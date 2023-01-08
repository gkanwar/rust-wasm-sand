use crate::{sand::{Game, ParticleSystem, Brush, Particle, BrushKind, ParticleKind, ParticleInd}, util::Coord};
use core::cmp::{max, min};

pub enum MouseState {
    Up,
    Down(Coord),
}

pub fn handle_mouse_click(coord: Coord, game: &mut Game) {
    game.particle_system.fill_circle(coord, &game.brush);
}

pub fn handle_mouse_drag(start: Coord, end: Coord, game: &mut Game) {
    game.particle_system.fill_line(start, end, &game.brush)
}


impl ParticleSystem {
    fn delete_particle(&mut self, i: ParticleInd) {
        let particle: &Particle = self.particles.get(i).expect("Missing particle");
        self.grid.set(particle.position.0, particle.position.1, None);
        self.particles.remove(i);
    }

    fn create_particle(&mut self, x: usize, y: usize, kind: ParticleKind) {
        let i = self.particles.insert(Particle {
            kind: kind,
            position: (x, y),
            velocity: (0.0, 0.0),
        });
        self.grid.set(x, y, Some(i));
    }

    pub fn draw_point(&mut self, x: usize, y: usize, brush_kind: &BrushKind) {
        let cur_val = self.grid.get(x, y);
        match brush_kind {
            BrushKind::Eraser => {
                if let Some(i) = cur_val {
                    self.delete_particle(i);
                }
            }
            BrushKind::Draw(kind) => {
                if let None = cur_val {
                    self.create_particle(x, y, *kind);
                }
            }
        }
    }

    pub fn fill_circle(&mut self, coord: Coord, brush: &Brush) {
        let x0 = (coord.x - brush.radius).floor().clamp(0.0, f64::INFINITY) as usize;
        let y0 = (coord.y - brush.radius).floor().clamp(0.0, f64::INFINITY) as usize;
        let x1 = (coord.x + brush.radius).ceil().clamp(0.0, f64::INFINITY) as usize;
        let y1 = (coord.y + brush.radius).ceil().clamp(0.0, f64::INFINITY) as usize;
        for x in x0..x1 {
            let dx = x as f64 - coord.x; // TODO: offset by half pixel?
            for y in y0..y1 {
                let dy = y as f64 - coord.y;
                if dx*dx + dy*dy <= brush.radius*brush.radius {
                    self.draw_point(x, y, &brush.kind);
                }
            }
        }
    }

    pub fn fill_line(&mut self, start: Coord, end: Coord, brush: &Brush) {
        self.fill_circle(start, brush);
        self.fill_circle(end, brush);

        let lx = end.x - start.x;
        let ly = end.y - start.y;
        let norm = (lx*lx + ly*ly).sqrt();
        let left_unclamp = (start.x.min(end.x) - brush.radius) as i64;
        let right_unclamp = (start.x.max(end.x) + brush.radius) as i64;
        let top_unclamp = (start.y.max(end.y) + brush.radius) as i64;
        let bottom_unclamp = (start.y.min(end.y) - brush.radius) as i64;
        let left = max(0, left_unclamp) as usize;
        let right = min(self.grid.width as i64 - 1, right_unclamp) as usize;
        let bottom = max(0, bottom_unclamp) as usize;
        let top = min(self.grid.height as i64 - 1, top_unclamp) as usize;

        for y in bottom..top+1 {
            for x in left..right+1 {
                let vx = x as f64 - start.x;
                let vy = y as f64 - start.y;
                // change basis to line coords
                let v_tangent = (lx * vx + ly * vy) / (norm*norm);
                let v_perp = (-ly * vx + lx * vy) / (norm*brush.radius);
                if v_tangent >= 0.0 && v_tangent <= 1.0 && v_perp >= -1.0 && v_perp <= 1.0 {
                    self.draw_point(x, y, &brush.kind);
                }
            }
        }
    }
}