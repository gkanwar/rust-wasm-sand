use crate::{sand::{Game, ParticleSystem, Brush, Particle, BrushKind, ParticleKind}, util::Coord};

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
    fn delete_particle(&mut self, i: usize) {
        let particle: &Particle = self.particles.get(i).expect("Missing particle");
        self.grid.set(particle.position.0, particle.position.1, None);
        self.particles.delete(i);
    }

    fn create_particle(&mut self, x: usize, y: usize, kind: ParticleKind) {
        let i = self.particles.add(Particle {
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
            BrushKind::Element(kind) => {
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
            for y in y0..y1 {
                if (x*x + y*y) as f64 <= brush.radius {
                    self.draw_point(x, y, &brush.kind);
                }
            }
        }
    }

    pub fn fill_line(&mut self, start: Coord, end: Coord, brush: &Brush) {
        // TODO
    }
}