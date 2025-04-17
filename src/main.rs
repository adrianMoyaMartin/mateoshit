use glam::dvec2;
use raylib::prelude::*;
use species::{BehaviourArchetypes, Organism, Species};

mod species;
fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 960).title("Hello, World").build();
    rl.set_target_fps(60);
    let mut species = Species::new();
    let org = Organism::new(
        10.0,
        65.0,
        150,
        dvec2(640.0, 440.0),
        BehaviourArchetypes::Hunter,
    );
    let org2 = Organism::new(
        10.0,
        50.0,
        200,
        dvec2(100.0, 300.0),
        BehaviourArchetypes::Prey,
    );
    let org3 = Organism::new(
        10.0,
        50.0,
        200,
        dvec2(600.0, 300.0),
        BehaviourArchetypes::Prey,
    );

    species.add(org);
    species.add(org2);
    species.add(org3);

    while !rl.window_should_close() {
        let ft = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        species.draw(&mut d);
        species.update(ft as f64);
        d.clear_background(Color::WHITE);
    }
}
