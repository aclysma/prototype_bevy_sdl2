
use bevy_app::prelude::*;
use bevy_window::{CreateWindow, Windows, WindowCreated, Window, WindowCloseRequested};
use bevy_app::AppExit;
use bevy_ecs::Resources;
use crate::sdl2_windows::Sdl2Windows;
use bevy_input::keyboard::{KeyboardInput, ElementState};

mod sdl2_windows;

/// Adds SDL2 windowing backend to Apps.
#[derive(Default)]
pub struct Sdl2Plugin;

impl Plugin for Sdl2Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .set_runner(sdl2_runner);
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
        &mut create_window_event_reader
    );

    log::debug!("Entering SDL2 event loop");

    'running: loop {
        if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
            if app_exit_event_reader.latest(&app_exit_events).is_some() {
                break 'running;
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                // sdl2::event::Event::Quit => {
                //     let mut window_close_requested_events = app
                //         .resources
                //         .get_mut::<Events<WindowCloseRequested>>()
                //         .unwrap();
                //     let winit_windows = app.resources.get_mut::<WinitWindows>().unwrap();
                //     let window_id = winit_windows.get_window_id(winit_window_id).unwrap();
                //     window_close_requested_events.send(WindowCloseRequested { id: window_id });
                // },
                sdl2::event::Event::Window {
                    timestamp,
                    window_id,
                    win_event
                } => {
                    match win_event {
                        sdl2::event::WindowEvent::Close => {
                            let mut window_close_requested_events = app
                                .resources
                                .get_mut::<Events<WindowCloseRequested>>()
                                .unwrap();
                            let window_id = sdl2_windows.get_window_id(window_id).unwrap();
                            window_close_requested_events.send(WindowCloseRequested { id: window_id });
                        },
                        _ => {}
                    }
                },
                sdl2::event::Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat
                } => {
                    let mut keyboard_input_events =
                        app.resources.get_mut::<Events<KeyboardInput>>().unwrap();
                    keyboard_input_events.send(KeyboardInput {
                        scan_code: 0,
                        key_code: 0,
                        state: ElementState::Pressed
                    });
                },
                sdl2::event::Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat
                } => {
                    let mut keyboard_input_events =
                        app.resources.get_mut::<Events<KeyboardInput>>().unwrap();
                    keyboard_input_events.send(KeyboardInput {
                        scan_code: 0,
                        key_code: 0,
                        state: ElementState::Released
                    });
                }
                _ => {}
            }
        }

        handle_create_window_events(
            &mut app.resources,
            &video_subsystem,
            &mut sdl2_windows,
            &mut create_window_event_reader
        );
        app.update();
    }
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
