use crate::dyn_store::DynStore;
use crate::input::MouseState;

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
                kind: BrushKind::Element(0),
                radius: 10.0,
            },
            particle_system: ParticleSystem {
                particles: DynStore::<Particle>::new(512),
                grid: Grid::new(width, height),
            },
        }
    }
}

pub struct Brush {
    pub kind: BrushKind,
    pub radius: f64,
}

pub enum BrushKind {
    Eraser,
    Element(ParticleKind),
}

pub struct ParticleSystem {
    // TODO: boundaries, etc?
    pub particles: DynStore<Particle>,
    pub grid: Grid,
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

pub type ParticleKind = i64;

#[derive(Debug)]
pub struct Particle {
    pub kind: ParticleKind,
    pub position: (usize, usize),
    pub velocity: (f64, f64),
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            position: Default::default(),
            velocity: Default::default(),
        }
    }
}
