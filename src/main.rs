use glam::dvec2;
use raylib::prelude::*;
use species::{BehaviourArchetypes, Food, FoodField, Organism, Species};

mod species;
fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 960).title("Hello, World").build();
    rl.set_target_fps(60);
    let mut species = Species::new();
    let mut food = FoodField::new();
    let fpood = Food {
        pos: dvec2(110.0, 310.0),
        kind: species::FoodTypes::Plant(100.0),
    };
    let fpood2 = Food {
        pos: dvec2(130.0, 310.0),
        kind: species::FoodTypes::Plant(100.0),
    };
    let fpood3 = Food {
        pos: dvec2(120.0, 310.0),
        kind: species::FoodTypes::Plant(100.0),
    };
    let org = Organism::new(20.0, 150, dvec2(640.0, 440.0), BehaviourArchetypes::Hunter);
    let org2 = Organism::new(10.0, 200, dvec2(100.0, 300.0), BehaviourArchetypes::Prey);
    let org3 = Organism::new(10.0, 200, dvec2(600.0, 300.0), BehaviourArchetypes::Prey);

    species.add(org);
    species.add(org2);
    species.add(org3);

    food.add(fpood);
    food.add(fpood2);
    food.add(fpood3);

    while !rl.window_should_close() {
        let ft = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        food.draw(&mut d);
        species.draw(&mut d);
        species.update(ft as f64, &mut food);

        d.clear_background(Color::WHITE);
    }
}
