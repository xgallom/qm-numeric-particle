#![cfg(target_arch = "wasm32")]

use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

pub mod geometry;
mod shaders;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Engine {
    size: usize,
    canvas: HtmlCanvasElement,
    context: WebGlRenderingContext,
    vert_buffer: WebGlBuffer,
}

#[wasm_bindgen]
impl Engine {
    pub fn on_resize(&mut self) -> Result<(), wasm_bindgen::JsValue> {
        self.canvas
            .set_width(self.canvas.client_width().try_into().unwrap());
        self.canvas
            .set_height(self.canvas.client_height().try_into().unwrap());

        let w = self.canvas.client_width();
        let h = self.canvas.client_height();
        self.context.viewport(0, 0, w, h);

        Ok(())
    }

    pub fn on_request_animation_frame(&mut self, time: f64) -> Result<(), wasm_bindgen::JsValue> {
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        self.context
            .draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (18 / 3) as i32);

        Ok(())
    }

    pub fn new(size: usize) -> Result<Engine, wasm_bindgen::JsValue> {
        let canvas = JsCast::dyn_into::<web_sys::HtmlCanvasElement>(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap(),
        )?;

        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        let vert_shader = Engine::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            shaders::vertex_shader(),
        )?;

        let frag_shader = Engine::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            shaders::fragment_shader(),
        )?;

        let program = Engine::link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        context.clear_color(0.0, 0.0, 0.0, 1.0);

        let vert_buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vert_buffer));
        let vertices: [f32; 18] = [
            -1.0, -1.0, 0.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0,
            -1.0, 0.0,
        ];
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);

        let color_buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        let colors: [f32; 6 * 4] = [
            1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
        ];
        unsafe {
            let color_array = js_sys::Float32Array::view(&colors);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &color_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(1, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(1);

        //        let frag_buffer = context.create_buffer().ok_or("failed to c reate buffer")?;
        //        context.bind_buffer(WebGlRenderingContext::)

        let mut engine = Engine {
            size,
            canvas,
            context,
            vert_buffer,
        };

        engine.on_resize()?;

        Ok(engine)
    }

    fn compile_shader(
        context: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
