mod platform;
mod context;
mod app;

extern crate winit;

use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow::{Exit, Poll};
use crate::app::{App, AppContext, WindowConfig};

struct TestApp {
}

impl App for TestApp {
    fn draw(&mut self) {
    }
}

fn main() {
    println!("Hello, world!");

    let mut app: TestApp = TestApp{};

    let window_config = WindowConfig{
        ..Default::default()
    };

    let mut app_context: Rc<AppContext> = Rc::new(AppContext::new(window_config));
    app_context.run(app);

    // let event_loop = winit::event_loop::EventLoop::new();
    //
    // let window = winit::window::WindowBuilder::new()
    //     .with_inner_size(LogicalSize::new(800, 800))
    //     .build(&event_loop).unwrap();
    //



}
