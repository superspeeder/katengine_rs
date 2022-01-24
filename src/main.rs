mod platform;
mod context;
mod app;

extern crate winit;
extern crate ash;
extern crate windows;

use std::cell::RefCell;
use winit::event_loop::ControlFlow::Poll;
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use crate::app::{App, AppContext, WindowConfig};
use context::{VKContext, Version, PlatformedVKContext};

struct TestApp {
}

impl App for TestApp {
    fn draw(&mut self) {
    }
}

fn main() {
    println!("Hello, world!");

    let mut app = RefCell::new(TestApp{});

    let window_config = WindowConfig{
        ..Default::default()
    };

    let mut event_loop = EventLoop::new();

    let mut app_context = RefCell::new(AppContext::new(window_config, &mut event_loop, app));
    let mut vk_context = RefCell::new(VKContext::new("TestApp".to_string(), Version::new(0,1,0), vec![], vec![]));

    vk_context.get_mut().select_default_gpu();
    vk_context.get_mut().create_surface(app_context.get_mut().get_window());

    run_app!(app_context,event_loop);
}
