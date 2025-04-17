use glam::{DVec2, dvec2};
use rand::{self, Rng};
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
                            x: object.pos.x as f32,
                            y: object.pos.y as f32,
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
                            x: object.pos.x as f32,
                            y: object.pos.y as f32,
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
    pub fn update(&mut self) {
        let n = self.organisms.len();
        for organism in &mut self.organisms {
            organism.movement();
            organism.clear_vision();
        }
        for i in 0..n {
            for j in 0..i {
                let [a, b] = self.organisms.get_disjoint_mut([i, j]).unwrap();
                a.add_vision(b);
                b.add_vision(a);
            }
        }
        for organism in &mut self.organisms {
            organism.vision();
        }
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
    pos: DVec2,
    visible_creatures: Vec<DVec2>,
}
impl Organism {
    pub fn new(
        metabolism: f64,
        speed: f64,
        vision: i32,
        pos: DVec2,
        behaviour_type: BehaviourArchetypes,
    ) -> Self {
        Self {
            energy: 100.0,
            metabolism,
            speed,
            vision,
            pos,
            behaviour_type,
            current_behaviour: CurrentBehaviour::Idle,
            visible_creatures: vec![],
        }
    }
    fn movement(&mut self) {
        let mut rng = rand::rng();
        let movement: f64 = rng.random();

        match self.current_behaviour {
            CurrentBehaviour::Active(flee_dir) => {
                match self.behaviour_type {
                    BehaviourArchetypes::Prey => self.pos -= flee_dir.normalize() * movement,
                    BehaviourArchetypes::Hunter => self.pos += flee_dir.normalize() * 1.0,
                };
            }
            CurrentBehaviour::Idle => {
                let r1: f64 = rng.random_range(-1.0..1.0);
                let r2: f64 = rng.random_range(-1.0..1.0);
                let mov = dvec2(r1, r2);

                if mov.length_squared() > 0.0001 {
                    self.pos += mov.normalize() * self.speed * (movement * 2.0);
                }
            }
        }
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
                    self.current_behaviour = CurrentBehaviour::Active(flee_dir.normalize());
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
                        self.current_behaviour = CurrentBehaviour::Active(dir);
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
    Idle,
}
