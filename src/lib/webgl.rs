use crate::sand;
use js_sys::ArrayBuffer;
use js_sys::Float32Array;
use js_sys::Uint16Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use web_sys::{WebGl2RenderingContext as WebGL, WebGlShader, WebGlSampler, WebGlProgram};
use rand::Rng;

// Texture storage settings
const TEX_BYTES_PER_PIXEL: usize = 4;
const TEX_INTERNAL_FORMAT: u32 = WebGL::RGBA;
const TEX_FORMAT: u32 = WebGL::RGBA;
const TEX_NUM_TYPE: u32 = WebGL::UNSIGNED_BYTE;
// Texture pixel to screen pixel ratio
const TEX_SCALE: u32 = 4;

fn get_shader_compile_err(gl: &WebGL, shader: &web_sys::WebGlShader, shader_type: &'static str) -> JsError {
    match gl.get_shader_info_log(shader) {
        Some(message) => {
            JsError::new(&format!("Failed to compile {} shader: {}", shader_type, message))
        }
        _ => {
            JsError::new(&format!("Failed to compile {} shader.", shader_type))
        }
    }
}

fn get_program_link_err(gl: &WebGL, program: &web_sys::WebGlProgram) -> JsError {
    match gl.get_program_info_log(program) {
        Some(message) => {
            JsError::new(&format!("Failed to link shader program: {}", message))
        }
        _ => {
            JsError::new("Failed to link shader program.")
        }
    }
}

fn init_shaders(gl: &WebGL) -> Result<(WebGlShader, WebGlShader), JsError> {
    let vert_shader: web_sys::WebGlShader = gl.create_shader(WebGL::VERTEX_SHADER).ok_or(
        JsError::new("Failed to create vertex shader"))?;
    let frag_shader: web_sys::WebGlShader = gl.create_shader(WebGL::FRAGMENT_SHADER).ok_or(
        JsError::new("Failed to create fragment shader"))?;
    gl.shader_source(&vert_shader, VERT_SHADER_SOURCE);
    gl.shader_source(&frag_shader, FRAG_SHADER_SOURCE);
    gl.compile_shader(&vert_shader);
    gl.compile_shader(&frag_shader);

    let vert_shader_ok = gl.get_shader_parameter(&vert_shader, WebGL::COMPILE_STATUS).as_bool().ok_or(
        JsError::new("Failed to fetch vert shader status")
    )?;
    let frag_shader_ok = gl.get_shader_parameter(&frag_shader, WebGL::COMPILE_STATUS).as_bool().ok_or(
        JsError::new("Failed to fetch frag shader status")
    )?;
    if !vert_shader_ok {
        return Err(get_shader_compile_err(gl, &vert_shader, "vertex").into());
    }
    if !frag_shader_ok {
        return Err(get_shader_compile_err(gl, &frag_shader, "fragment").into());
    }
    Ok((vert_shader, frag_shader))
}

fn init_shader_program(
        gl: &WebGL, vert_shader: &WebGlShader, frag_shader: &WebGlShader)
        -> Result<WebGlProgram, JsError> {
    let program: WebGlProgram = match gl.create_program() {
        Some(program) => { program }
        None => {
            gl.delete_shader(Some(&vert_shader));
            gl.delete_shader(Some(&frag_shader));
            return Err(JsError::new("Failed to create shader program"));
        }
    };

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);
    gl.delete_shader(Some(&vert_shader));
    gl.delete_shader(Some(&frag_shader));

    let link_ok = gl.get_program_parameter(&program, WebGL::LINK_STATUS).as_bool().ok_or_else(|| {
        gl.delete_program(Some(&program));
        JsError::new("Failed to fetch program link status")
    })?;
    if !link_ok {
        let err = get_program_link_err(gl, &program);
        gl.delete_program(Some(&program));
        return Err(err.into());
    }

    Ok(program)
}

fn get_attrib_location(gl: &WebGL, program: &WebGlProgram, name: &str)
        -> Result<u32, JsValue> {
    let i = gl.get_attrib_location(program, name);
    if i < 0 {
        Err(format!("Invalid attrib array {}", name).into())
    }
    else {
        Ok(i as u32)
    }
}

fn get_uniform_location(gl: &WebGL, program: &WebGlProgram, name: &str)
        -> Result<WebGlUniformLocation, JsValue> {
    gl.get_uniform_location(program, name).ok_or(format!("Invalid uniform {}", name).into())
}

fn bind_shader_buffers(
        gl: &WebGL, program: &WebGlProgram, gl_data: &RendererBuffers)
        -> Result<(), JsValue> {
    gl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&gl_data.vertex_buffer));
    let i = get_attrib_location(gl, program, "vertexPosition")?;
    gl.vertex_attrib_pointer_with_i32(i as u32, 3, WebGL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(i as u32);

    gl.bind_buffer(WebGL::ELEMENT_ARRAY_BUFFER, Some(&gl_data.index_buffer));
    gl.bind_buffer(WebGL::ARRAY_BUFFER, Some(&gl_data.tex_vertex_buffer));
    let i = get_attrib_location(gl, program, "vert_texCoord")?;
    gl.vertex_attrib_pointer_with_i32(i as u32, 2, WebGL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(i as u32);

    let i = get_uniform_location(gl, program, "texSampler")?;
    gl.active_texture(WebGL::TEXTURE0);
    gl.bind_texture(WebGL::TEXTURE_2D, Some(&gl_data.texture));
    gl.bind_sampler(0, Some(&gl_data.sampler));
    gl.use_program(Some(program));
    gl.uniform1i(Some(&i), 0);
    Ok(())
}

fn clear_screen(gl: &WebGL, b: f32) {
    gl.clear_color(0.0, b, b, 1.0);
    gl.clear(WebGL::COLOR_BUFFER_BIT | WebGL::DEPTH_BUFFER_BIT);
}

const QUAD_VERTICES: [f32; 12] = [
    -1.0, -1.0, 0.0,
    1.0, -1.0, 0.0,
    1.0, 1.0, 0.0,
    -1.0, 1.0, 0.0
];
const QUAD_TEX_VERTICES: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    0.0, 1.0
];
const INDICES_TYPE: u32 = WebGL::UNSIGNED_SHORT;
const QUAD_INDICES: [u16; 6] = [
    3, 2, 1, 3, 1, 0
];

fn make_gl_buffer(data: &ArrayBuffer, gl: &WebGL, buf_kind: u32) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(buf_kind, Some(&buffer));
    gl.buffer_data_with_opt_array_buffer(
        buf_kind, Some(data), WebGL::STATIC_DRAW);
    gl.bind_buffer(buf_kind, None);
    Ok(buffer)
}

fn make_gl_texture(tex_array: &[u8], width: i32, height: i32, gl: &WebGL) -> Result<WebGlTexture, JsValue> {
    let texture = gl.create_texture().ok_or("Could not create texture.")?;
    gl.active_texture(WebGL::TEXTURE0);
    gl.bind_texture(WebGL::TEXTURE_2D, Some(&texture));
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGL::TEXTURE_2D, 0, TEX_INTERNAL_FORMAT as i32, width, height, 0,
        TEX_FORMAT, TEX_NUM_TYPE, Some(tex_array))?;
    Ok(texture)
}

fn make_gl_sampler(gl: &WebGL) -> Result<WebGlSampler, JsValue> {
    let sampler = gl.create_sampler().ok_or("Failed to create sampler")?;
    gl.sampler_parameteri(&sampler, WebGL::TEXTURE_MIN_FILTER, WebGL::NEAREST as i32);
    gl.sampler_parameteri(&sampler, WebGL::TEXTURE_MAG_FILTER, WebGL::NEAREST as i32);
    Ok(sampler)
}

struct RendererBuffers {
    index_buffer: WebGlBuffer,
    vertex_buffer: WebGlBuffer,
    tex_vertex_buffer: WebGlBuffer,
    texture: WebGlTexture,
    sampler: WebGlSampler
}
impl RendererBuffers {
    fn new(gl: &WebGL, tex_array: &[u8], width: i32, height: i32) -> Result<Self, JsValue> {
        let js_quad_indices = Uint16Array::from(&QUAD_INDICES[..]);
        let js_quad_vertices = Float32Array::from(&QUAD_VERTICES[..]);
        let js_quad_tex_vertices = Float32Array::from(&QUAD_TEX_VERTICES[..]);
        Ok(Self {
            index_buffer: make_gl_buffer(&js_quad_indices.buffer(), gl, WebGL::ELEMENT_ARRAY_BUFFER)?,
            vertex_buffer: make_gl_buffer(&js_quad_vertices.buffer(), gl, WebGL::ARRAY_BUFFER)?,
            tex_vertex_buffer: make_gl_buffer(&js_quad_tex_vertices.buffer(), gl, WebGL::ARRAY_BUFFER)?,
            texture: make_gl_texture(tex_array, width, height, gl)?,
            sampler: make_gl_sampler(gl)?
        })
    }
}

fn make_tex_array(width: u32, height: u32) -> Vec<u8> {
    let length = (width * height) as usize * TEX_BYTES_PER_PIXEL;
    let mut rng = rand::thread_rng();
    let mut out: Vec<u8> = vec![0x0f; length];
    for i in 0..(width*height) as usize {
        for j in 0..TEX_BYTES_PER_PIXEL-1 {
            let ind = TEX_BYTES_PER_PIXEL*i + j;
            out[ind] = rng.gen();
        }
        out[TEX_BYTES_PER_PIXEL*i+TEX_BYTES_PER_PIXEL-1] = 0x0f;
    }
    assert_eq!(out.len(), length);
    out
    // vec![0xff; length]
}

pub struct Renderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: WebGL,
    program: WebGlProgram,
    gl_data: RendererBuffers
}

impl Renderer {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl_opt: Option<js_sys::Object> = canvas.get_context("webgl2")?;
        let gl_js_obj = gl_opt.ok_or(
            JsError::new("Failed to create webGL context"))?;
        let gl: WebGL = gl_js_obj.dyn_into()?;
        let (vert_shader, frag_shader) = init_shaders(&gl)?;
        let program = init_shader_program(&gl, &vert_shader, &frag_shader)?;
        let tex_width = canvas.width() / TEX_SCALE;
        let tex_height = canvas.height() / TEX_SCALE;
        let tex_array = make_tex_array(tex_width, tex_height);
        let gl_data = RendererBuffers::new(
            &gl, &tex_array, tex_width as i32, tex_height as i32)?;
        bind_shader_buffers(&gl, &program, &gl_data)?;
        clear_screen(&gl, 0.3);
        Ok(Self {
            canvas: canvas,
            gl: gl,
            program: program,
            gl_data: gl_data
        })
    }


    pub fn render(&self, game: &sand::Game) {
        // FORNOW: do it the dumb way by clearing and re-drawing everything
        clear_screen(&self.gl, 1.0);
        self.gl.use_program(Some(&self.program));
        self.gl.draw_elements_with_i32(
            WebGL::TRIANGLES, QUAD_INDICES.len() as i32, INDICES_TYPE, 0);
    }
}

const VERT_SHADER_SOURCE: &'static str = include_str!("../shaders/vertex.glsl");
const FRAG_SHADER_SOURCE: &'static str = include_str!("../shaders/fragment.glsl");