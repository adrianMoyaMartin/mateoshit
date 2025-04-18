use glam::{DVec2, dvec2};
use rand::{self, Rng, rng};
use raylib::{
    color::Color,
    ffi::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Species {
    organisms: Vec<Organism>,
}
impl Species {
    pub fn new() -> Species {
        Species { organisms: vec![] }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for object in self.organisms.iter() {
            match object.behaviour_type {
                BehaviourArchetypes::Prey => {
                    d.draw_rectangle_pro(
                        Rectangle {
                            x: object.pos.x as f32 - 2.5,
                            y: object.pos.y as f32 - 2.5,
                            width: 5.0,
                            height: 5.0,
                        },
                        Vector2 { x: 0.0, y: 0.0 },
                        0.0,
                        Color::GREEN,
                    );
                }
                BehaviourArchetypes::Hunter => {
                    d.draw_rectangle_pro(
                        Rectangle {
                            x: object.pos.x as f32 - 2.5,
                            y: object.pos.y as f32 - 2.5,
                            width: 5.0,
                            height: 5.0,
                        },
                        Vector2 { x: 0.0, y: 0.0 },
                        0.0,
                        Color::RED,
                    );
                }
            }
        }
    }
    pub fn update(&mut self, ft: f64) {
        let n = self.organisms.len();
        for organism in &mut self.organisms {
            organism.movement(ft);
            organism.clear_vision();
        }
        for i in 0..n {
            for j in 0..i {
                let [a, b] = self.organisms.get_disjoint_mut([i, j]).unwrap();
                a.add_vision(b);
                b.add_vision(a);
            }
        }
        self.organisms.retain_mut(|organism| {
            organism.vision();
            organism.consume_energy(ft);
            organism.energy > 0.0
        });
    }
    pub fn add(&mut self, org: Organism) {
        self.organisms.push(org);
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Organism {
    energy: f64,
    metabolism: f64,
    speed: f64,
    vision: i32,
    behaviour_type: BehaviourArchetypes,
    current_behaviour: CurrentBehaviour,
    idle_dir: DVec2,
    pos: DVec2,
    visible_creatures: Vec<DVec2>,
    stomach: Vec<FoodTypes>
}
impl Organism {
    pub fn new(
        metabolism: f64,
        vision: i32,
        pos: DVec2,
        behaviour_type: BehaviourArchetypes,
    ) -> Self {
        let rand: f64 = rng().random();
        let rand2: f64 = rng().random();
        Self {
            energy: 100.0,
            metabolism,
            speed: metabolism * 1.25,
            vision,
            pos,
            behaviour_type,
            current_behaviour: CurrentBehaviour::Idle,
            idle_dir: dvec2(rand, rand2),
            visible_creatures: vec![],
            stomach: vec![]
        }
    }
    fn movement(&mut self, ft: f64) {
        let mut rng = rand::rng();
        let metabolic_rate = self.metabolism / 10.0;
        match self.current_behaviour {
            CurrentBehaviour::Active(move_dir) => {
                        self.pos += move_dir * self.speed * metabolic_rate * 1.5 * ft;
                    }
            CurrentBehaviour::Idle => {
                        if rng.random_bool(0.05) {
                            let jitter = dvec2(rng.random_range(-0.5..=0.5), rng.random_range(-0.5..=0.5));
                            let new_dir = self.idle_dir + jitter;
                            if new_dir.length_squared() > 0.0001 {
                                self.idle_dir = new_dir.normalize();
                            }
                        }

                        self.pos += self.idle_dir * self.speed * metabolic_rate * ft;
                    }
            CurrentBehaviour::Gather(_dvec2) => todo!(),
            CurrentBehaviour::Rest => todo!(),
        }
    }
    fn consume_energy(&mut self, ft: f64) {

        self.digest_food();

        let activity_multiplier = match self.current_behaviour {
            CurrentBehaviour::Idle => 1.0,
            CurrentBehaviour::Active(_) => 2.0,
            CurrentBehaviour::Gather(_) => 1.5,
            CurrentBehaviour::Rest => 0.5,
        };
    
        let total_loss = self.metabolism * activity_multiplier;
        self.energy -= total_loss * ft;
    }
    fn digest_food(&mut self) {
        self.stomach.retain_mut(|food| {
            match food {
                FoodTypes::Plant(energy_left) => {
                    self.energy += self.metabolism;
                    *energy_left -= self.metabolism;
                    if *energy_left <= 0.0 {
                        self.energy += *energy_left;

                    }
                    *energy_left > 0.0
                },
                FoodTypes::Meat(energy_left) => {
                    self.energy += self.metabolism;
                    *energy_left -= self.metabolism;
                    if *energy_left <= 0.0 {
                        self.energy += *energy_left;
                    }
                    *energy_left > 0.0
                },
            }
        });
    }
    fn clear_vision(&mut self) {
        self.visible_creatures.clear();
    }
    fn add_vision(&mut self, organism: &Organism) {
        if self.pos.distance(organism.pos) <= self.vision as f64 {
            self.visible_creatures.push(organism.pos);
        }
    }
    fn vision(&mut self) {
        match self.behaviour_type {
            BehaviourArchetypes::Prey => {
                if self.visible_creatures.is_empty() {
                    self.current_behaviour = CurrentBehaviour::Idle;
                    return;
                }

                let mut flee_dir = DVec2::ZERO;
                for threat in &self.visible_creatures {
                    let to_threat = *threat - self.pos;
                    if to_threat.length_squared() > 0.0001 {
                        flee_dir -= to_threat.normalize();
                    }
                }

                if flee_dir.length_squared() > 0.0001 {
                    let mut angle = flee_dir.to_angle();
                    angle += rng().random::<f64>() / 2.0;
                    self.current_behaviour = CurrentBehaviour::Active(DVec2::from_angle(angle));
                } else {
                    self.current_behaviour = CurrentBehaviour::Idle;
                }
            }

            BehaviourArchetypes::Hunter => {
                let closest_prey: Option<DVec2> = self
                    .visible_creatures
                    .iter()
                    .min_by(|a, b| a.distance(self.pos).total_cmp(&b.distance(self.pos)))
                    .copied();

                if let Some(target) = closest_prey {
                    let raw_dir = target - self.pos;
                    if raw_dir.length_squared() > 0.0001 {
                        let dir = raw_dir.normalize();
                        self.current_behaviour = CurrentBehaviour::Active(1.01 * dir);
                    } else {
                        self.current_behaviour = CurrentBehaviour::Idle;
                    }
                } else {
                    self.current_behaviour = CurrentBehaviour::Idle;
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum BehaviourArchetypes {
    Prey,
    Hunter,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CurrentBehaviour {
    Active(DVec2),
    Gather(DVec2),
    Idle,
    Rest,
}
#[derive(Debug, PartialEq, Clone)]
pub enum FoodTypes {
    Plant(f64),
    Meat(f64),
}
