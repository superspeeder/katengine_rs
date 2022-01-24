use crate::context;

use ash::{vk::{self,SurfaceKHR},Instance};
use winit::{window::Window,platform::windows::WindowExtWindows};
use windows::Win32::Foundation::HWND;
use crate::app::{App, AppContext};
use std::cell::RefCell;
use std::ffi::CString;
use context::{VKContext, PlatformedVKContext};
use std::borrow::Borrow;
use std::ops::Deref;

pub struct GLContext {

}

pub fn platform_required_instance_extensions() -> Vec<String> {
    vec![String::from("VK_KHR_surface"), String::from("VK_KHR_win32_surface")]
}

impl PlatformedVKContext for VKContext {
    fn create_surface(&mut self, window: &mut Window) {
        // create the surface
        let create_info = vk::Win32SurfaceCreateInfoKHR {
            hwnd: (window as &mut dyn WindowExtWindows).hwnd(),
            hinstance: (window as &mut dyn WindowExtWindows).hinstance(),
            ..Default::default()
        };

        let inst = self.get_instance();
        let entry = self.entry.borrow();

        self.surface_api = Some(ash::extensions::khr::Surface::new(entry.deref(), &inst));

        self.surface = Some(unsafe {
            ash::extensions::khr::Win32Surface::new(entry.deref(), &inst)
                .create_win32_surface(&create_info, None)
        }.expect("Failed to create surface"));

        println!("Created window surface!");
    }


}

