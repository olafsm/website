extern crate wasm_bindgen;
use core::f32;
use web_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

mod app_state;
mod utils;
mod gl_setup;
mod shaders;
mod programs;
mod common_funcs;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen] 
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name:&str) {
    alert(&format!("Hello, {}", name));
}

#[wasm_bindgen]
pub struct Client {
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
    program_color_2d_gradient: programs::Color2DGradient,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            program_color_2d: programs::Color2D::new(&gl),
            program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            gl:gl,
        }
    }

    pub fn update(&mut self, time:f32, height:f32, width:f32) -> Result<(),JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let curr_state = app_state::get_curr_state();
        self.program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
            (0.,0.,0.,1.)
        );
        let gradient:[f32;16] = [
            1., 0., 0., 1.,
            0., 1., 1., 1., 
            1., 1., 1., 1.,
            0., 0., 1., 1.,
        ];
        self.program_color_2d_gradient.render(
            &self.gl,
            curr_state.control_bottom + 20.,
            curr_state.control_top - 20.,
            curr_state.control_left + 20.,
            curr_state.control_right - 20.,
            curr_state.canvas_height,
            curr_state.canvas_width,
            Some(gradient),
        );

    }    
}