use rand;

const BASE_VISION: i32 = 30;
const BASE_SPEED: i32 = 10;
pub struct Species {
    organisms: Vec<Organism>
}
impl Species {
    pub fn new() -> Species {
        Species {organisms: vec![]}
    }
}
pub struct Organism {
    energy: f32,
    metabolism: f32,
    speed: f32,
    vision: i32,
    pos: Vector2,
}
impl Organism {
    pub fn new(metabolism: f32, speed: f32, vision: i32, pos: Vector2) -> Self {
        Self { energy: 100.0, metabolism, speed, vision, pos }
    }
    fn movement(&mut self) {
        let num: f32 = rand::random();
        self.pos.x += num * self.speed;
        self.pos.y += num * self.speed;
    }
    pub fn update(&mut self) {
        
    }
}
pub struct Vector2 {
    x: f32,
    y: f32
}
impl Vector2 {
    pub fn new(x: f32, y:f32) -> Self {
        Vector2 {x, y}
    }
}