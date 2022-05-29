use wasm_bindgen::prelude::*;

use crate::input;
use crate::physics;
use crate::sand;
use crate::util;
use crate::webgl;

#[wasm_bindgen]
pub struct WasmGameContext {
    game: sand::Game,
    renderer: Option<webgl::Renderer>,
    step_time_ms: f64,
}

const STEP_TIME_MS_120FPS: f64 = 1000.0 / 120.0;

#[wasm_bindgen]
impl WasmGameContext {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            game: sand::Game::new(width, height),
            renderer: None,
            step_time_ms: STEP_TIME_MS_120FPS,
        }
    }
    pub fn bind_canvas(&mut self, canvas: web_sys::HtmlCanvasElement)
        -> Result<(), JsValue> {
        self.renderer = Some(webgl::Renderer::new(canvas)?);
        Ok(())
    }
    pub fn set_running(&mut self, running: bool, timestamp: f64) {
        self.game.running = running;
        self.game.last_tick = timestamp;
    }
    pub fn render(&self) {
        if let Some(renderer) = &self.renderer {
            renderer.render(&self.game);
        }
    }
    pub fn update(&mut self, timestamp: f64) {
        let dt = timestamp - self.game.last_tick;
        let n_ticks = (dt / self.step_time_ms).floor() as usize;
        if n_ticks == 0 {
            return;
        }
        if n_ticks > 1 {
            web_sys::console::log_2(&"Dropping frames".into(), &(n_ticks - 1).into());
        }
        physics::step(&mut self.game.particle_system);
    }
    pub fn mouse_down(&mut self, x: f64, y: f64) {
        let coord = util::Coord::new(x, y);
        input::handle_mouse_click(coord, &mut self.game);
        self.game.mouse_state = input::MouseState::Down(coord);
    }
    pub fn mouse_move(&mut self, x: f64, y: f64) {
        let coord = util::Coord::new(x, y);
        match self.game.mouse_state {
            input::MouseState::Up => {
                input::handle_mouse_click(coord, &mut self.game);
            }
            input::MouseState::Down(old_coord) => {
                input::handle_mouse_drag(old_coord, coord, &mut self.game);
            }
        }
        self.game.mouse_state = input::MouseState::Down(coord);
    }
    pub fn mouse_up(&mut self, x: f64, y: f64) {
        let coord = util::Coord::new(x, y);
        match self.game.mouse_state {
            input::MouseState::Up => {
                input::handle_mouse_click(coord, &mut self.game);
            }
            input::MouseState::Down(old_coord) => {
                input::handle_mouse_drag(old_coord, coord, &mut self.game);
            }
        }
        self.game.mouse_state = input::MouseState::Up;
    }
}
