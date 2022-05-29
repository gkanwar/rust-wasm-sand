use crate::sand;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext as WebGL, WebGlShader, WebGlProgram};

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

fn init_shader_program(gl: &WebGL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, JsError> {
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

fn clear_screen(gl: &WebGL) {
    gl.clear_color(0.0, 0.3, 0.3, 1.0);
    gl.clear(WebGL::COLOR_BUFFER_BIT | WebGL::DEPTH_BUFFER_BIT);
}

pub struct Renderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: WebGL,
    program: WebGlProgram,
}
impl Renderer {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl_opt: Option<js_sys::Object> = canvas.get_context("webgl2")?;
        let gl_js_obj = gl_opt.ok_or(
            JsError::new("Failed to create webGL context"))?;
        let gl: WebGL = gl_js_obj.dyn_into()?;
        let (vert_shader, frag_shader) = init_shaders(&gl)?;
        let program = init_shader_program(&gl, &vert_shader, &frag_shader)?;
        clear_screen(&gl);
        Ok(Self {
            canvas: canvas,
            gl: gl,
            program: program,
        })
    }


    pub fn render(&self, game: &sand::Game) {
        // FORNOW: do it the dumb way by clearing and re-drawing everything
        clear_screen(&self.gl);
        self.gl.use_program(Some(&self.program));
        // TODO!
    }
}

const VERT_SHADER_SOURCE: &'static str = include_str!("../shaders/vertex.glsl");
const FRAG_SHADER_SOURCE: &'static str = include_str!("../shaders/fragment.glsl");