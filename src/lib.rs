extern crate wasm_bindgen;
use core::{f32};
use std::panic::RefUnwindSafe;
//use app_state::AppState;
use web_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use rand::Rng;
//use std::{thread, time};
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
    //program_color_2d: programs::Color2D,
    program_color_2d_gradient: programs::Color2DGradient,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            //program_color_2d: programs::Color2D::new(&gl),
            program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            gl:gl,
        }
    }

    pub fn update(&mut self, time:f32, height:f32, width:f32) -> Result<(),JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }
    pub fn render(&self, program: &str) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        match program {
            "squares" => render_squares(self),
            _ => render_squares(self)
        }
        //render_background(self);
    }    
}

fn render_squares(client:&Client) {
    let curr_state = app_state::get_curr_state();
    let mut rng = rand::thread_rng();
    let num_width = 100;
    let num_height = 100;
    let w = 10.;
    let h = 10.;
    let offset_X = curr_state.canvas_width/2. - (num_width as f32*w)/2.;
    let offset_y = curr_state.canvas_height/2. - (num_height as f32*h)/2.;

    //for i in 1..num_height {
    //    for j in 1..100 {
    //        let mut col = 1.;
    //        if i%2 == 0 {
    //            col = 0.;
    //        } if j%2 == 0 {
    //            col = 0.;
    //        }
    //        client.program_color_2d.render(&client.gl,
    //            offsetX + i as f32*w, 
    //            offsetY + j as f32*h, 
    //            w, 
    //            h, 
    //            (rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>(),0.8))
    //            //(col, col, col, 1.))
    //    }
    //}
    let val = curr_state.mouse_x;
    let val2 = curr_state.mouse_y;
    let in_max = curr_state.canvas_width;
    let in_max2 = curr_state.canvas_height;
    let in_min = 0.;
    let out_min = 0.;
    let out_max = 1.;
    let a = (val-in_min)*(out_max)/(in_max-in_min) + out_min;
    let b = (val2-in_min)*(out_max)/(in_max2-in_min) + out_min;
    let c = 1.-a*b;
    // log(&a.to_string());
    // log(&b.to_string());
    let _gradient:[f32;16] = [
        c, b, a, 1.,
        b, b, a, 1., 
        c, b, a, 1.,
        b, b, a, 1.,
    ];
    let mut gradient:[f32;16] = [
        209./256., 107./256., 117./256., 1.,
        237./256., 126./256., 126./256., 1.,
        247./256., 171./256., 117./256., 1.,
        255./256., 253./256., 117./256., 1.,
    ];
    let mut _gradient:[f32;16] = [
        1., 0., 0., 1.,
        0., 1., 0., 1., 
        0., 0., 1., 1., 
        0., 1., 1., 1., 
    ];
    client.program_color_2d_gradient.render(&client.gl,
                0., 
                0. ,
                curr_state.canvas_width, 
                curr_state.canvas_height, 
                Some(gradient))

    //for i in 1..num_height {
    //    for j in 1..100 {
    //        if rng.gen::<f32>() < 0.4 {
    //        client.program_color_2d_gradient.render(&client.gl,
    //            offset_X + i as f32*w, 
    //            offset_y + j as f32*h, 
    //            w, 
    //            h, 
    //            Some(gradient))
    //            //(col, col, col, 1.))               
    //        }
    //    }
    //}
    //client.program_color_2d_gradient.render(
    //    &client.gl,
    //    (curr_state.canvas_width/2.)-500.,
    //    (curr_state.canvas_height/2.)-500.,
    //    1000.,
    //    1000.,
    //    Some(gradient),
    //);
    //client.program_color_2d.render(
    //    &client.gl,
    //    (curr_state.canvas_width/2.)-250.,
    //    (curr_state.canvas_height/2.)-250.,
    //    500.,
    //    500.,
    //    (0.,0.,0.,1.)
    //);
//
    //client.program_color_2d_gradient.render(
    //    &client.gl,
    //    (curr_state.canvas_width/2.)-50.,
    //    (curr_state.canvas_height/2.)-50.,
    //    100.,
    //    100.,
    //    None,
    //);
}

fn render_background(client:&Client) {
    let curr_state = app_state::get_curr_state();
    let num_width = 100;
    let num_height = 100;
    let w = 10.;
    let h = 10.;

    let val = curr_state.mouse_x;
    let val2 = curr_state.mouse_y;
    let in_max = curr_state.canvas_width;
    let in_max2 = curr_state.canvas_height;
    let in_min = 0.;
    let out_min = 0.;
    let out_max = 1.;
    let a = (val-in_min)*(out_max)/(in_max-in_min) + out_min;
    let b = (val2-in_min)*(out_max)/(in_max2-in_min) + out_min;
    let c = 1.-a*b;
    let _gradient:[f32;16] = [
        c, b, a, 1.,
        b, b, a, 1., 
        c, b, a, 1.,
        b, b, a, 1.,
    ];
    let mut gradient:[f32;16] = [
        209./256., 107./256., 117./256., 1.,
        237./256., 126./256., 126./256., 1.,
        247./256., 171./256., 117./256., 1.,
        255./256., 253./256., 117./256., 1.,
    ];
    let mut _gradient:[f32;16] = [
        1., 0., 0., 1.,
        0., 1., 0., 1., 
        0., 0., 1., 1., 
        0., 1., 1., 1., 
    ];
    client.program_color_2d_gradient.render(&client.gl,
                0., 
                0. ,
                curr_state.canvas_width, 
                curr_state.canvas_height, 
                Some(gradient))
}
