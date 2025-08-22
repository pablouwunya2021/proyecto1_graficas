pub struct Player {
    pub pos: Pos,
    pub a: f32,   // ángulo
    pub fov: f32, // campo de visión
}

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32, fov: f32) -> Self {
        Self {
            pos: Pos { x, y },
            a,
            fov,
        }
    }

    pub fn move_forward(&mut self, step: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let nx = self.pos.x + self.a.cos() * step;
        let ny = self.pos.y + self.a.sin() * step;

        if !is_wall(maze, nx, ny, block_size) {
            self.pos.x = nx;
            self.pos.y = ny;
        }
    }

    pub fn move_backward(&mut self, step: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let nx = self.pos.x - self.a.cos() * step;
        let ny = self.pos.y - self.a.sin() * step;

        if !is_wall(maze, nx, ny, block_size) {
            self.pos.x = nx;
            self.pos.y = ny;
        }
    }

    pub fn rotate_left(&mut self, angle: f32) {
        self.a -= angle;
    }

    pub fn rotate_right(&mut self, angle: f32) {
        self.a += angle;
    }
}

/// Verifica si una posición está dentro de una pared
fn is_wall(maze: &Vec<Vec<char>>, x: f32, y: f32, block_size: usize) -> bool {
    let i = (x as usize) / block_size;
    let j = (y as usize) / block_size;

    if j >= maze.len() || i >= maze[0].len() {
        return true; // fuera del mapa = pared
    }

    maze[j][i] != ' '
}
