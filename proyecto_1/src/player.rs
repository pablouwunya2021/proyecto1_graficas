pub struct Player {
    pub pos: Position,
    pub a: f32,     // ángulo (rad)
    pub fov: f32,   // campo de visión (rad)
    pub radius: f32 // radio de colisión (en unidades del mundo)
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32, fov: f32) -> Self {
        Self {
            pos: Position { x, y },
            a,
            fov,
            radius: 10.0, // ajusta al gusto
        }
    }

    pub fn rotate_left(&mut self, angle: f32) { self.a -= angle; }
    pub fn rotate_right(&mut self, angle: f32) { self.a += angle; }

    pub fn move_forward(&mut self, step: f32, maze: &Vec<Vec<char>>, block: usize) {
        let dx = self.a.cos() * step;
        let dy = self.a.sin() * step;
        self.try_move(dx, dy, maze, block);
    }

    pub fn move_backward(&mut self, step: f32, maze: &Vec<Vec<char>>, block: usize) {
        let dx = -self.a.cos() * step;
        let dy = -self.a.sin() * step;
        self.try_move(dx, dy, maze, block);
    }

    /// Movimiento con "deslizamiento": prueba eje X y eje Y por separado
    fn try_move(&mut self, dx: f32, dy: f32, maze: &Vec<Vec<char>>, block: usize) {
        let nx = self.pos.x + dx;
        if !collides(maze, nx, self.pos.y, self.radius, block) {
            self.pos.x = nx;
        }
        let ny = self.pos.y + dy;
        if !collides(maze, self.pos.x, ny, self.radius, block) {
            self.pos.y = ny;
        }
    }
}

/// ¿El círculo del jugador colisiona con alguna pared?
fn collides(maze: &Vec<Vec<char>>, x: f32, y: f32, r: f32, block: usize) -> bool {
    // chequea las 4 esquinas del bounding box del círculo
    is_wall_cell(maze, x - r, y - r, block) ||
    is_wall_cell(maze, x + r, y - r, block) ||
    is_wall_cell(maze, x - r, y + r, block) ||
    is_wall_cell(maze, x + r, y + r, block)
}

fn is_wall_cell(maze: &Vec<Vec<char>>, x: f32, y: f32, block: usize) -> bool {
    if x < 0.0 || y < 0.0 { return true; }
    let i = (x as usize) / block;
    let j = (y as usize) / block;
    if j >= maze.len() || i >= maze[0].len() { return true; }
    maze[j][i] != ' ' // cualquier char != espacio es pared
}
