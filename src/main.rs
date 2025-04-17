use glam::dvec2;
use raylib::prelude::*;
use species::{BehaviourArchetypes, Organism, Species};

mod species;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    rl.set_target_fps(30);
    let mut species = Species::new();
    let org = Organism::new(10.0, 1.0, 1000, dvec2(300.0, 300.0), BehaviourArchetypes::Hunter);
    let org2 = Organism::new(10.0, 1.0, 10, dvec2(100.0, 300.0), BehaviourArchetypes::Prey);
    let org3 = Organism::new(10.0, 1.0, 10, dvec2(600.0, 300.0), BehaviourArchetypes::Prey);

    
    species.add(org);
    species.add(org2);
    species.add(org3);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        species.draw(&mut d);
        species.update();
        d.clear_background(Color::WHITE);
    }
}
