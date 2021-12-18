use super::super::common_funcs as cf;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Graph3D {
    pub program: WebGlProgram,
    pub indices_buffer: WebGlBuffer,
    pub index_count: i32,
    pub normals_buffer: WebGlBuffer,
    pub position_buffer: WebGlBuffer,
    pub u_normals_rotation: WebGlUniformLocation,
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
    pub y_buffer: WebGlBuffer,
}

impl Graph3D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            &super::super::shaders::vertex::graph_3d::SHADER,
            &super::super::shaders::fragment::varying_color_from_vertex::SHADER,
        ).unwrap();


        let positions_and_indices = cf::get_position_grid_n_by_n(super::super::constants::GRID_SIZE);
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = positions_and_indices.0.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + positions_and_indices.0.len() as u32,
        );

        let buffer_position = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_position));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let indices_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

        let indices_location = positions_and_indices.1.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + positions_and_indices.1.len() as u32,
        );
        let buffer_indices = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);

        Self {
            u_normals_rotation: gl.get_uniform_location(&program, "uNormalsRotation").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),
            program: program,

            indices_buffer: buffer_indices,
            index_count: indices_array.length() as i32,
            normals_buffer: gl.create_buffer().ok_or("failed normals create buffer").unwrap(),
            position_buffer: buffer_position,
            y_buffer: gl.create_buffer().ok_or("failed to create buffer").unwrap(),
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_angle_x_axis: f32,
        rotation_angle_y_axis: f32,
        y_vals: &Vec<f32>,
    ) {
        gl.use_program(Some(&self.program));
    }
}