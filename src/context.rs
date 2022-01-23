use std::rc::Rc;
// Platform agnostic context traits & structs
use ash::vk;
use crate::App;
use crate::app::AppContext;

pub trait VKContext {
    fn get_surface(&mut self) -> vk::SurfaceKHR;
}


pub trait PlatformedAppContext {
    fn new(app_context: Rc<AppContext>) -> Self;
}