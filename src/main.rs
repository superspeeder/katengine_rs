mod platform;
mod context;
mod app;

extern crate winit;

use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow::{Exit, Poll};
use winit::event_loop::EventLoop;
use crate::app::{App, AppContext, WindowConfig};

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = Poll;

        // match event {
        //     Event::WindowEvent {
        //         event: WindowEvent::CloseRequested,
        //         ..
        //     } => {
        //         println!("Closing Window!");
        //         *control_flow = Exit;
        //     },
        //     Event::MainEventsCleared => {
        //         app.draw();
        //     },
        //     _ => ()
        // }

        app_context.borrow_mut().on_event(&event, control_flow);
    });

    // let event_loop = winit::event_loop::EventLoop::new();
    //
    // let window = winit::window::WindowBuilder::new()
    //     .with_inner_size(LogicalSize::new(800, 800))
    //     .build(&event_loop).unwrap();
    //



}
