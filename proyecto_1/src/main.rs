mod framebuffer;
mod player;
mod raycaster;
mod map;
mod config;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycaster::render3d;
use map::load_map;
use config::{BLOCK_SIZE, MOVE_SPEED, ROT_SPEED};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new(
        "Raycaster - Paso 1 (movimiento + colisiones)",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("Error creando la ventana: {e}"));

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Cargar mapa una vez
    let maze = load_map("./maze.txt");

    // Posición inicial (debe caer en espacio vacío del mapa)
    let mut player = Player::new(150.0, 150.0, 0.0, std::f32::consts::FRAC_PI_2);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- INPUT & MOVIMIENTO ---

        // Rotación
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            player.a -= ROT_SPEED;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            player.a += ROT_SPEED;
        }

        // Adelante
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            let nx = player.pos.x + player.a.cos() * MOVE_SPEED;
            let ny = player.pos.y + player.a.sin() * MOVE_SPEED;
            if !is_wall(nx, ny, &maze) {
                player.pos.x = nx;
                player.pos.y = ny;
            }
        }

        // Atrás
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            let nx = player.pos.x - player.a.cos() * MOVE_SPEED;
            let ny = player.pos.y - player.a.sin() * MOVE_SPEED;
            if !is_wall(nx, ny, &maze) {
                player.pos.x = nx;
                player.pos.y = ny;
            }
        }

        // --- RENDER ---
        render3d(&mut framebuffer, &player, &maze);

        framebuffer.flush_to(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn is_wall(x: f32, y: f32, maze: &Vec<Vec<char>>) -> bool {
    if x < 0.0 || y < 0.0 {
        return true;
    }
    let i = (x as usize) / BLOCK_SIZE;
    let j = (y as usize) / BLOCK_SIZE;

    if j >= maze.len() || i >= maze[0].len() {
        true // fuera de rango = pared
    } else {
        maze[j][i] != ' ' // cualquier char distinto de espacio es pared
    }
}

