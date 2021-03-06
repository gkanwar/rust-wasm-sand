use sand_game::physics::step;
use sand_game::sand::{Game, Particle};

pub fn main() {
    let mut game = Game::new(128, 128);
    game.particle_system.particles.add(Particle {
        kind: 0,
        position: (0, 0),
        velocity: (0.0, 0.0),
    });
    game.running = true;
    step(&mut game.particle_system);
}
