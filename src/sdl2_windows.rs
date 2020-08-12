use bevy_window::{Window, WindowId, CreateWindow, BevyRawWindowHandle};
use std::collections::HashMap;
use raw_window_handle::HasRawWindowHandle;

#[derive(Default)]
pub struct Sdl2Windows {
    pub windows: HashMap<u32, sdl2::video::Window>,
    pub bevy_id_to_sdl2_id: HashMap<WindowId, u32>,
    pub sdl2_id_to_bevy_id: HashMap<u32, WindowId>,
}

impl Sdl2Windows {
    pub fn create_window(
        &mut self,
        sdl2_video_subsystem: &sdl2::VideoSubsystem,
        window_event: &CreateWindow,
    ) -> Window {
        let sdl2_window = sdl2_video_subsystem
            .window(&window_event.descriptor.title, window_event.descriptor.width, window_event.descriptor.height)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to create window");


        let raw_window_handle = BevyRawWindowHandle(sdl2_window.raw_window_handle());

        let sdl2_id = sdl2_window.id();
        self.windows.insert(sdl2_id, sdl2_window);
        self.bevy_id_to_sdl2_id.insert(window_event.id, sdl2_id);
        self.sdl2_id_to_bevy_id.insert(sdl2_id, window_event.id);

        Window::new(
            window_event.id,
            raw_window_handle,
            &window_event.descriptor
        )
    }

    pub fn get_window(&self, id: WindowId) -> Option<&sdl2::video::Window> {
        self.bevy_id_to_sdl2_id
            .get(&id)
            .and_then(|id| self.windows.get(id))
    }

    pub fn get_window_id(&self, id: u32) -> Option<WindowId> {
        self.sdl2_id_to_bevy_id.get(&id).map(|x| *x)
    }
}
