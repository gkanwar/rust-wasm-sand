use crate::dyn_store::DynStore;
use crate::input::MouseState;
use crate::render::{Color, Pixels, EMPTY_COLOR};

pub struct Game {
    pub running: bool,
    pub last_tick: f64,
    pub mouse_state: MouseState,
    pub brush: Brush,
    pub particle_system: ParticleSystem,
}
impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            running: false,
            last_tick: 0.0,
            mouse_state: MouseState::Up,
            brush: Brush {
                kind: BrushKind::Draw(ParticleKind::Base(0)),
                radius: 10.0,
            },
            particle_system: ParticleSystem {
                particles: DynStore::<Particle>::new(512),
                grid: Grid::new(width, height),
                elements: Elements::new(),
            },
        }
    }
    pub fn draw(&self, pixels: &mut Pixels) {
        // FORNOW: Just assume top-left alignment between pixels space and
        // game space. Ideally the sizes would be equal also.
        let grid = &self.particle_system.grid;
        for x in 0..(pixels.width as usize).min(grid.width) {
            for y in 0..(pixels.height as usize).min(grid.height) {
                match grid.get(x, y) {
                    Some(i) => {
                        if let Some(particle) = self.particle_system.particles.get(i) {
                            pixels.draw(x as usize, y as usize, self.particle_system.elements.get(particle.kind).color);
                        }
                    }
                    None => {
                        pixels.draw(x as usize, y as usize, EMPTY_COLOR);
                    }
                };
            }
        }
    }
}

pub struct Brush {
    pub kind: BrushKind,
    pub radius: f64,
}

pub enum BrushKind {
    Eraser,
    Draw(ParticleKind),
}

pub struct ParticleSystem {
    // TODO: boundaries, etc?
    pub particles: DynStore<Particle>,
    pub grid: Grid,
    pub elements: Elements,
}

pub struct Element {
    name: String,
    color: Color
}

pub struct Elements {
    pub base_elements: Vec<Element>,
    pub custom_elements: Vec<Element>,
}
impl Elements {
    pub fn new() -> Self {
        Self {
            base_elements: create_base_elements(),
            custom_elements: vec![]
        }
    }
    pub fn get<'a>(&'a self, kind: ParticleKind) -> &'a Element {
        match kind {
            ParticleKind::Base(i) => &self.base_elements[i as usize],
            ParticleKind::Custom(i) => &self.custom_elements[i as usize]
        }
    }
}

fn create_base_elements() -> Vec<Element> {
    let mut elements = Vec::<Element>::new();
    elements.push(Element {
        name: "Sand".to_string(),
        color: Color::new_rgb(1.000, 0.835, 0.333)
    });
    elements.push(Element {
        name: "Water".to_string(),
        color: Color::new_rgb(0.000, 0.000, 1.000)
    });
    elements
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Option<usize>>,
}
impl Grid {
    fn ind(&self, x: usize, y: usize) -> usize {
        x * self.height + y
    }
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width: width,
            height: height,
            cells: vec![None; width * height],
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.cells[self.ind(x, y)]
        }
    }
    pub fn set(&mut self, x: usize, y: usize, val: Option<usize>) -> UpdateResult {
        if x >= self.width || y >= self.height {
            UpdateResult::Err("Out of bounds")
        }
        else {
            let i = self.ind(x, y);
            self.cells[i] = val;
            UpdateResult::Ok
        }
    }
}
pub enum UpdateResult {
    Ok,
    Err(&'static str)
}

#[derive(Clone,Copy,Debug)]
pub enum ParticleKind {
    Base(u16),
    Custom(u16)
}

#[derive(Debug)]
pub struct Particle {
    pub kind: ParticleKind,
    pub position: (usize, usize),
    pub velocity: (f64, f64),
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            kind: ParticleKind::Base(0),
            position: Default::default(),
            velocity: Default::default(),
        }
    }
}
