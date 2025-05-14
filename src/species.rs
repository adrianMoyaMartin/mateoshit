#![expect(clippy::collapsible_match)]
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

    pub fn update(&mut self, ft: f64, food_field: &mut FoodField) {
        self.update_movement_and_clear_vision(ft);
        self.update_vision();
        self.update_behaviour(food_field);
        self.cleanup_dead(ft);
    }

    pub fn add(&mut self, org: Organism) {
        self.organisms.push(org);
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
            d.draw_text(
                &format!("{}", object.stomach.len()),
                object.pos.x as i32,
                object.pos.y as i32 - 10,
                10,
                Color::BLACK,
            );
        }
    }

    fn update_movement_and_clear_vision(&mut self, ft: f64) {
        for organism in &mut self.organisms {
            organism.movement(ft);
            organism.clear_vision();
        }
    }

    fn update_vision(&mut self) {
        let n = self.organisms.len();
        for i in 0..n {
            for j in 0..i {
                let [a, b] = self.organisms.get_disjoint_mut([i, j]).unwrap();
                a.add_vision(b);
                b.add_vision(a);
            }
        }
    }

    fn update_behaviour(&mut self, food_field: &mut FoodField) {
        for organism in &mut self.organisms {
            organism.add_visible_food(&food_field.items);
            organism.try_gather_food(food_field);
        }
    }

    fn cleanup_dead(&mut self, ft: f64) {
        self.organisms.retain_mut(|organism| {
            organism.vision();
            organism.energy_consumption(ft);
            if !organism.stomach.is_empty() {
                println!("{:?}, {}", organism.stomach, organism.energy);
            }
            organism.energy > 0.0
        });
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
    visible_creatures: Vec<PerceivedEntity>,
    stomach: Vec<FoodTypes>,
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
            stomach: vec![],
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
            CurrentBehaviour::Gather(move_dir) => {
                self.pos += move_dir * self.speed * metabolic_rate * 1.4 * ft;
            } //            CurrentBehaviour::Rest => todo!(),
        }
    }

    fn energy_consumption(&mut self, ft: f64) {
        self.digest_food(ft);

        let activity_multiplier = match self.current_behaviour {
            CurrentBehaviour::Idle => 1.0,
            CurrentBehaviour::Active(_) => 2.0,
            CurrentBehaviour::Gather(_) => 1.5,
            //            CurrentBehaviour::Rest => 0.5,
        };

        let total_loss = self.metabolism * activity_multiplier;
        self.energy -= total_loss * ft;
    }

    fn try_gather_food(&mut self, food_field: &mut FoodField) {
        food_field.items.retain(|food| {
            if self.pos.distance(food.pos) <= 1.0 {
                println!("Gathering food: {:?}", food.kind);
                self.stomach.push(food.kind.clone());
                false
            } else {
                true
            }
        });
    }

    fn digest_food(&mut self, ft: f64) {
        self.stomach.retain_mut(|food| {
            let energy_left = match food {
                FoodTypes::Plant(energy) => energy,
                FoodTypes::Meat(energy) => energy,
            };

            if *energy_left <= 0.0 {
                return false;
            }
            let digest_amount = (self.metabolism * (6.5 / 15.0) * ft).min(*energy_left);
            *energy_left -= digest_amount;
            self.energy += digest_amount;

            *energy_left > 0.0
        });
    }

    fn vision(&mut self) {
        match self.behaviour_type {
            BehaviourArchetypes::Prey => {
                self.prey_vision();
            }
            BehaviourArchetypes::Hunter => {
                self.hunter_vision();
            }
        }
    }

    fn prey_vision(&mut self) {
        let mut flee_dir = DVec2::ZERO;
        let mut closest_plant: Option<DVec2> = None;
        let mut min_plant_dist = f64::MAX;

        for entity in &self.visible_creatures {
            match entity {
                PerceivedEntity::Organism { pos, kind } if *kind == BehaviourArchetypes::Hunter => {
                    flee_dir += self.pos - *pos;
                }
                PerceivedEntity::Food { pos, food_type } => {
                    if let FoodTypes::Plant(_) = food_type {
                        let dist = self.pos.distance(*pos);
                        if dist < min_plant_dist {
                            min_plant_dist = dist;
                            closest_plant = Some(*pos);
                        }
                    }
                }
                _ => {}
            }
        }

        if flee_dir.length_squared() > 0.0001 {
            self.current_behaviour = CurrentBehaviour::Active(flee_dir.normalize());
        } else if let Some(target) = closest_plant {
            self.current_behaviour = CurrentBehaviour::Gather((target - self.pos).normalize());
        } else {
            self.current_behaviour = CurrentBehaviour::Idle;
        }
    }
    fn hunter_vision(&mut self) {
        let mut closest_prey: Option<DVec2> = None;
        let mut min_prey_dist = f64::MAX;
        let mut closest_meat: Option<DVec2> = None;
        let mut min_meat_dist = f64::MAX;

        for entity in &self.visible_creatures {
            match entity {
                PerceivedEntity::Organism { pos, kind } if *kind == BehaviourArchetypes::Prey => {
                    let dist = self.pos.distance(*pos);
                    if dist < min_prey_dist {
                        min_prey_dist = dist;
                        closest_prey = Some(*pos);
                    }
                }
                PerceivedEntity::Food { pos, food_type } => {
                    if let FoodTypes::Meat(_) = food_type {
                        let dist = self.pos.distance(*pos);
                        if dist < min_meat_dist {
                            min_meat_dist = dist;
                            closest_meat = Some(*pos);
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(prey_pos) = closest_prey {
            let dir = (prey_pos - self.pos).normalize();
            self.current_behaviour = CurrentBehaviour::Active(dir);
        } else if let Some(meat_pos) = closest_meat {
            let dir = (meat_pos - self.pos).normalize();
            self.current_behaviour = CurrentBehaviour::Gather(dir);
        } else {
            self.current_behaviour = CurrentBehaviour::Idle;
        }
    }

    fn add_vision(&mut self, other: &Organism) {
        if self.pos.distance(other.pos) <= self.vision as f64 {
            self.visible_creatures.push(PerceivedEntity::Organism {
                pos: other.pos,
                kind: other.behaviour_type.clone(),
            });
        }
    }

    fn add_visible_food(&mut self, food_list: &[Food]) {
        for food in food_list {
            if self.pos.distance(food.pos) <= self.vision as f64 {
                self.visible_creatures.push(PerceivedEntity::Food {
                    pos: food.pos,
                    food_type: food.kind.clone(),
                });
            }
        }
    }
    fn clear_vision(&mut self) {
        self.visible_creatures.clear();
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
    //    Rest,
}
#[derive(Debug, Clone)]
pub struct Food {
    pub pos: DVec2,
    pub kind: FoodTypes,
}

pub struct FoodField {
    pub items: Vec<Food>,
}

impl FoodField {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, food: Food) {
        self.items.push(food);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for food in &self.items {
            let color = match food.kind {
                FoodTypes::Plant(_) => Color::LIME,
                FoodTypes::Meat(_) => Color::BROWN,
            };
            d.draw_rectangle_v(
                Vector2 {
                    x: food.pos.x as f32,
                    y: food.pos.y as f32,
                },
                Vector2 { x: 3.0, y: 3.0 },
                color,
            );
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FoodTypes {
    Plant(f64),
    Meat(f64),
}
#[derive(Debug, PartialEq, Clone)]
pub enum PerceivedEntity {
    Organism {
        pos: DVec2,
        kind: BehaviourArchetypes,
    },
    Food {
        pos: DVec2,
        food_type: FoodTypes,
    },
}
