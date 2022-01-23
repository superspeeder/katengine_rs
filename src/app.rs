use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow::{Exit, Poll};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Icon, Window, WindowBuilder};

pub enum FullscreenType {
    Windowed(Option<usize>), Exclusive(Option<usize>)
}

pub struct WindowConfig {
    pub size: (u32, u32),
    pub resizable: bool,
    pub fullscreen: Option<FullscreenType>,
    pub title: String,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub decorated: bool,
    pub floating: bool,
    pub icon: Option<Icon>,
}

pub trait App {
    fn draw(&mut self);
}

pub struct AppContext {
    window: Window,
    event_loop: EventLoop<()>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            size: (800, 800),
            resizable: true,
            fullscreen: None,
            title: "Window".to_string(),
            maximized: false,
            visible: true,
            transparent: false,
            decorated: true,
            floating: false,
            icon: None
        }
    }
}


impl AppContext {
    pub fn new(cfg: WindowConfig) -> AppContext {
        let evtl = winit::event_loop::EventLoop::new();

        let mut fullscr: Option<Fullscreen> = None;
        if cfg.fullscreen.is_some() {
            let ft = cfg.fullscreen.unwrap();
            match ft {
                FullscreenType::Windowed(monitor_id_) => {
                    let monitor_id = monitor_id_.unwrap_or(0);
                    let monitors: Vec<MonitorHandle> = evtl.available_monitors().collect();
                    if monitors.len() > monitor_id {
                        let monitor_: MonitorHandle = monitors[monitor_id].clone();
                        fullscr = Some(Fullscreen::Borderless(Some(monitor_)));
                    } else {
                        fullscr = Some(Fullscreen::Borderless(None));
                    }
                },
                FullscreenType::Exclusive(monitor_id_) => {
                    let monitor_id = monitor_id_.unwrap_or(0);
                    let monitors: Vec<MonitorHandle> = evtl.available_monitors().collect();
                    if monitors.len() > monitor_id {
                        let monitor_: MonitorHandle = monitors[monitor_id].clone();
                        let videomode = monitor_.video_modes().next();
                        fullscr = Some(Fullscreen::Exclusive(videomode.unwrap()));
                    } else {
                        let monitor_: MonitorHandle = monitors[0].clone();
                        let videomode = monitor_.video_modes().next();
                        fullscr = Some(Fullscreen::Exclusive(videomode.unwrap()));
                    }
                }
            }
        }


        return AppContext {
            window: WindowBuilder::new()
                .with_inner_size(LogicalSize::new(cfg.size.0, cfg.size.1))
                .with_decorations(cfg.decorated)
                .with_title(cfg.title)
                .with_always_on_top(cfg.floating)
                .with_maximized(cfg.maximized)
                .with_transparent(cfg.transparent)
                .with_resizable(cfg.resizable)
                .with_visible(cfg.visible)
                .with_window_icon(cfg.icon)
                .with_fullscreen(fullscr)
                .build(&evtl).unwrap(),
            event_loop: evtl,
        }

    }

    pub fn run<T: App>(&mut self, app: T) {
        let eloop = &mut self.event_loop;

        eloop.run(move |event, _, control_flow| {
            *control_flow = Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Closing Window!");
                    *control_flow = Exit;
                },
                Event::MainEventsCleared => {
                    app_.draw();
                },
                _ => ()
            }
        });
    }
}

