use glam::{dvec2, DVec2};
use rand::{self, Rng};

#[derive(Debug, PartialEq, Clone)]
pub struct Species {
    organisms: Vec<Organism>
}
impl Species {
    pub fn new() -> Species {
        Species {organisms: vec![]}
    }
    pub fn update(&mut self) {
        let orgs = self.clone();
        for organism in self.organisms.iter_mut() {
            organism.update(orgs.clone());
        }
    }
    pub fn add(&mut self, org: Organism) {
        self.organisms.push(org);
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Organism {
    energy: f64,
    metabolism: f64,
    speed: f64,
    vision: i32,
    pos: DVec2,
}
impl Organism {
    pub fn new(metabolism: f64, speed: f64, vision: i32, pos: DVec2) -> Self {
        Self { energy: 100.0, metabolism, speed, vision, pos }
    }
    fn movement(&mut self) {
        let mut rng = rand::rng();

        let r1: f64 = rng.random();
        let r2: f64 = rng.random();

        let movement: f64 = rng.random();

        let mov = dvec2(r1, r2);
        self.pos += mov.normalize() * self.speed * (movement*2.0);
    }
    fn vision(&mut self, spec: Species) {
        for org in spec.organisms.iter() {
            if org != self && self.pos.distance(org.pos) <= self.vision.into() {
                //behaviour in here
            }
        }
    }
    pub fn update(&mut self, spec: Species) {
        self.movement();
        self.vision(spec);
    }
}