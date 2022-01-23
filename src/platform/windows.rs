extern crate windows;

use crate::context;

use ash::vk;
use ash::vk::SurfaceKHR;
use winit::window::Window;
use windows::Win32::Foundation::HWND;

struct GLContext {

}

struct VKContextWindows {
    hwnd: HWND,
    surface: vk::SurfaceKHR
}

impl context::VKContext for VKContextWindows {
    fn get_surface(&mut self) -> SurfaceKHR {
        return self.surface;
    }
}

