use glam::dvec2;
use species::{Organism, Species};

mod species;
fn main() {
    let mut species = Species::new();
    let org = Organism::new(10.0, 10.0, 10, dvec2(0.0, 0.0));

    species.add(org);
    let mut i = 0;
    while i < 10 {
        println!("a");
        species.update();
        i += 1;
    }
}
