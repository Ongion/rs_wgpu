#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

use rs_wgpulib::run;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        // TODO: Wow, really? Can't create a full-screen canvas for web? That seems dumb!
        // Can probably be done with CSS, this is probably only true if you're injecting a canvas where one didn't previously exist. 
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
        
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }

}