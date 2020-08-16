mod converters;
use crate::sdl2_windows::Sdl2Windows;
use bevy_app::prelude::*;
use bevy_app::AppExit;
use bevy_ecs::Resources;
use bevy_input::keyboard::{ElementState, KeyboardInput};
use bevy_input::mouse::{MouseButtonInput, MouseMotion};
use bevy_math::prelude::*;
use bevy_window::{
    CreateWindow, CursorMoved, WindowCloseRequested, WindowCreated, WindowResized, Windows,
};

mod sdl2_windows;

/// Adds SDL2 windowing backend to Apps.
#[derive(Default)]
pub struct Sdl2Plugin;

impl Plugin for Sdl2Plugin {
    fn build(
        &self,
        app: &mut AppBuilder,
    ) {
        app.set_runner(sdl2_runner);
    }
}

pub fn sdl2_runner(mut app: App) {
    // Setup SDL
    let context = sdl2::init().expect("Failed to initialize sdl2");
    let video_subsystem = context
        .video()
        .expect("Failed to create sdl video subsystem");

    log::info!("Starting SDL2 window event loop");
    let mut event_pump = context
        .event_pump()
        .expect("Could not create sdl event pump");

    // This is not stored as a resource because SDL2 windows are not Send
    let mut sdl2_windows = Sdl2Windows::default();

    let mut create_window_event_reader = EventReader::<CreateWindow>::default();
    let mut app_exit_event_reader = EventReader::<AppExit>::default();

    handle_create_window_events(
        &mut app.resources,
        &video_subsystem,
        &mut sdl2_windows,
        &mut create_window_event_reader,
    );

    // SDL2 has a lot of events we can support, expose them all via an event
    app.resources
        .insert(Events::<sdl2::event::Event>::default());

    log::debug!("Entering SDL2 event loop");

    'running: loop {
        if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
            if app_exit_event_reader.latest(&app_exit_events).is_some() {
                break 'running;
            }
        }

        {
            let mut sdl2_events = app
                .resources
                .get_mut::<Events<sdl2::event::Event>>()
                .unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Window {
                        window_id,
                        win_event,
                        ..
                    } => match win_event {
                        sdl2::event::WindowEvent::Close => {
                            let mut window_close_requested_events = app
                                .resources
                                .get_mut::<Events<WindowCloseRequested>>()
                                .unwrap();
                            let window_id = sdl2_windows.get_window_id(window_id).unwrap();
                            window_close_requested_events
                                .send(WindowCloseRequested { id: window_id });
                        }
                        sdl2::event::WindowEvent::Resized(width, height) => {
                            let mut windows = app.resources.get_mut::<Windows>().unwrap();
                            let bevy_window_id = sdl2_windows.get_window_id(window_id).unwrap();
                            let mut window = windows.get_mut(bevy_window_id).unwrap();
                            window.width = width as u32;
                            window.height = height as u32;

                            let mut resize_events =
                                app.resources.get_mut::<Events<WindowResized>>().unwrap();
                            resize_events.send(WindowResized {
                                id: bevy_window_id,
                                height: window.height as usize,
                                width: window.width as usize,
                            });
                        }
                        _ => {}
                    },
                    sdl2::event::Event::KeyDown {
                        keycode, scancode, ..
                    } => {
                        send_key_event(&app, keycode, scancode, ElementState::Pressed);
                    }
                    sdl2::event::Event::KeyUp {
                        keycode, scancode, ..
                    } => {
                        send_key_event(&app, keycode, scancode, ElementState::Released);
                    }
                    sdl2::event::Event::MouseMotion {
                        window_id,
                        x,
                        y,
                        xrel,
                        yrel,
                        ..
                    } => {
                        let mut cursor_moved_events =
                            app.resources.get_mut::<Events<CursorMoved>>().unwrap();

                        let bevy_window_id = sdl2_windows.get_window_id(window_id).unwrap();
                        let window = sdl2_windows.get_window(bevy_window_id).unwrap();
                        let (_width, height) = window.size();
                        // move origin to bottom left
                        let y_position = height as i32 - y;
                        cursor_moved_events.send(CursorMoved {
                            id: bevy_window_id,
                            position: Vec2::new(x as f32, y_position as f32),
                        });

                        let mut mouse_motion_events =
                            app.resources.get_mut::<Events<MouseMotion>>().unwrap();
                        mouse_motion_events.send(MouseMotion {
                            delta: Vec2::new(xrel as f32, yrel as f32),
                        });
                    }
                    sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                        send_mouse_event(
                            &app,
                            mouse_btn,
                            bevy_input::keyboard::ElementState::Pressed,
                        );
                    }
                    sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                        send_mouse_event(
                            &app,
                            mouse_btn,
                            bevy_input::keyboard::ElementState::Released,
                        );
                    }
                    _ => {}
                }

                sdl2_events.send(event);
            }
        }

        handle_create_window_events(
            &mut app.resources,
            &video_subsystem,
            &mut sdl2_windows,
            &mut create_window_event_reader,
        );
        app.update();
    }
}

fn send_key_event(
    app: &App,
    keycode: Option<sdl2::keyboard::Keycode>,
    scancode: Option<sdl2::keyboard::Scancode>,
    element_state: bevy_input::keyboard::ElementState,
) {
    let mut keyboard_input_events = app.resources.get_mut::<Events<KeyboardInput>>().unwrap();

    // These options are due to conversion from C types to i32.
    // - Keycode is typedeffed as i32
    // - Scancode is a C array with max value of 512. It is intended that end-users
    //   can make arrays of this length.
    //
    // So it is expected that the i32 conversion will always succeed and these unwrap
    // are infallible
    let keycode = keycode.unwrap();
    let scancode = scancode.unwrap();
    keyboard_input_events.send(converters::convert_keyboard_input(
        keycode,
        scancode,
        element_state,
    ));
}

fn send_mouse_event(
    app: &App,
    mouse_btn: sdl2::mouse::MouseButton,
    element_state: bevy_input::keyboard::ElementState,
) {
    let mut mouse_button_input_events =
        app.resources.get_mut::<Events<MouseButtonInput>>().unwrap();
    mouse_button_input_events.send(MouseButtonInput {
        button: converters::convert_mouse_button(mouse_btn),
        state: element_state,
    });
}

fn handle_create_window_events(
    resources: &mut Resources,
    sdl2_video_subsystem: &sdl2::VideoSubsystem,
    sdl2_windows: &mut Sdl2Windows,
    create_window_event_reader: &mut EventReader<CreateWindow>,
) {
    let mut windows = resources.get_mut::<Windows>().unwrap();
    let create_window_events = resources.get::<Events<CreateWindow>>().unwrap();
    let mut window_created_events = resources.get_mut::<Events<WindowCreated>>().unwrap();
    for create_window_event in create_window_event_reader.iter(&create_window_events) {
        let window = sdl2_windows.create_window(sdl2_video_subsystem, &create_window_event);
        let window_id = window.id;
        windows.add(window);
        window_created_events.send(WindowCreated { id: window_id });
    }
}
