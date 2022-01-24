use std::rc::Rc;
use ash::{vk, Instance, Entry};
use crate::App;
use crate::app::AppContext;
use std::ffi::CString;
use std::cell::RefCell;
use ash::vk::{SurfaceKHR, PhysicalDevice};
use winit::window::Window;
use crate::platform::internal::platform_required_instance_extensions;
use std::ops::Deref;

// Platform agnostic context traits & structs

#[derive(Copy, Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32
}

impl Version {
    pub fn new(major_: u32, minor_: u32, patch_: u32) -> Version {
        Version{ major: major_, minor: minor_, patch: patch_ }
    }

    pub fn to_vk(&self) -> u32 {
        vk::make_api_version(0, self.major, self.minor, self.patch)
    }
}

#[derive(Clone)]
pub struct CStringArray {
    cstrings: Vec<CString>,
    cstrings_p: Vec<*const i8>,
    pub p_array: *const *const i8
}

impl CStringArray {
    pub fn new(strs: &Vec<String>) -> CStringArray {
        let css: Vec<CString> = strs.iter().map(|str| CString::new(str.deref()).unwrap()).collect();
        let mut cssp: Vec<_> = css.iter().map(|str| str.as_ptr()).collect();
        cssp.push(std::ptr::null());
        let parr = cssp.as_ptr();
        CStringArray {
            cstrings: css,
            cstrings_p: cssp,
            p_array: parr
        }
    }
}

pub struct VKContext {
    pub(crate) entry: RefCell<Entry>,
    pub(crate) surface: Option<SurfaceKHR>,
    instance: Instance,
    gpu: Option<PhysicalDevice>,
    pub(crate) surface_api: Option<ash::extensions::khr::Surface>,
}

pub trait PlatformedVKContext {
    fn create_surface(&mut self, window: &mut Window);
}

fn create_instance(entry: RefCell<Entry>,app_name: String, app_version: Version, extensions: Vec<String>, layers: Vec<String>) -> ash::Instance {
    let app_name_cstr = CString::new(app_name).expect("Failed to convert app_name string into a c string");
    let engine_name_cstr = CString::new("").expect("Failed to create a c string");
    let engine_ver = Version::new(1,0,0);

    let mut exts = extensions;

    let rexts = platform_required_instance_extensions();

    for ext in rexts {
        if !exts.contains(&ext) {
            exts.push(ext.clone());
        }
    }

    println!("Instance Extensions:");
    for ext in exts.iter() {
        println!("- {}", ext);
    }

    println!("Instance Layers:");
    for lyr in layers.iter() {
        println!("- {}", lyr);
    }

    let ext_csarr = CStringArray::new(exts.as_ref());
    let layer_csarr = CStringArray::new(layers.as_ref());

    let app_info = vk::ApplicationInfo {
        api_version: vk::API_VERSION_1_2,
        application_version: app_version.to_vk(),
        engine_version: engine_ver.to_vk(),
        p_application_name: app_name_cstr.as_ptr(),
        p_engine_name: engine_name_cstr.as_ptr(),
        ..Default::default()
    };

    let inst_cinfo = vk::InstanceCreateInfo {
        enabled_extension_count: exts.len() as u32,
        enabled_layer_count: layers.len() as u32,
        pp_enabled_extension_names: ext_csarr.p_array,
        pp_enabled_layer_names: layer_csarr.p_array,
        p_application_info: &app_info,
        ..Default::default()
    };

    unsafe { entry.borrow().create_instance(&inst_cinfo, None) }.expect("Failed to create instance")
}

impl VKContext {

    pub fn new(app_name: String, app_version: Version, extensions: Vec<String>, layers: Vec<String>) -> VKContext {
        let entry_ = RefCell::new(unsafe { Entry::load() }.expect("Failed to load Vulkan!"));

        let instance_ = create_instance(entry_.clone(), app_name, app_version, extensions, layers);

        println!("Created Vulkan Instance!");

        VKContext {
            entry: entry_,
            surface: None,
            instance: instance_.clone(),
            gpu: None,
            surface_api: None,
        }
    }

    pub fn select_default_gpu(&mut self) {
        let gpus = unsafe { self.instance.enumerate_physical_devices() }.expect("Couldn't load physical devices!");

        // for now just pick the first

        self.gpu = Some(gpus[0]);

        let mut props = unsafe { self.instance.get_physical_device_properties(self.gpu.unwrap()) };
        let name = unsafe { CString::from_raw(props.device_name.as_mut_ptr()) };
        println!("Selected GPU 0: {}", name.into_string().unwrap());
    }

    pub fn select_gpu(&mut self, id: usize) {
        let gpus = unsafe { self.instance.enumerate_physical_devices() }.expect("Couldn't load physical devices!");

        self.gpu = Some(gpus[if id >= gpus.len() { 0 } else { id }]);
    }

    pub fn get_surface(&mut self) -> Option<SurfaceKHR> {
        return self.surface;
    }

    pub fn get_instance(&mut self) -> Instance {
        self.instance.clone()
    }

    pub fn get_gpu(&mut self) -> Option<PhysicalDevice> {
        self.gpu
    }
}

impl Drop for VKContext {
    fn drop(&mut self) {
        if self.surface_api.is_some() && self.surface.is_some() {
            unsafe { self.surface_api.as_ref().unwrap().destroy_surface(self.surface.unwrap(), None); }
        }

        unsafe { self.instance.destroy_instance(None); }
    }
}