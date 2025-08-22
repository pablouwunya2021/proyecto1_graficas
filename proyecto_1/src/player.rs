pub struct Player {
    pub pos: Position,
    pub a: f32,   // ángulo (rad)
    pub fov: f32, // campo de visión (rad)
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32, fov: f32) -> Self {
        Player {
            pos: Position { x, y },
            a,
            fov,
        }
    }
}
