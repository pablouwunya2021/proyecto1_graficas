mod framebuffer;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycaster::{render3d, load_maze};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const BLOCK_SIZE: usize = 64; // debe coincidir con raycaster y minimapa

fn main() {
    let mut window = Window::new(
        "Raycaster con Minimap + Colisiones",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("Error creando la ventana: {e}"));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let maze = load_maze("./maze.txt");

    // jugador con radio de colisión
    let mut player = Player::new(150.0, 150.0, 0.0, std::f32::consts::FRAC_PI_2);

    // velocidades
    let move_speed = 4.0;
    let rot_speed = 0.05;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(0x000000);

        // ROTACIÓN
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            player.rotate_left(rot_speed);
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            player.rotate_right(rot_speed);
        }

        // MOVIMIENTO CON COLISIONES
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            player.move_forward(move_speed, &maze, BLOCK_SIZE);
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            player.move_backward(move_speed, &maze, BLOCK_SIZE);
        }

        // Render 3D + minimapa
        render3d(&mut framebuffer, &player, &maze, BLOCK_SIZE);

        framebuffer.flush_to(&mut buffer);
        window.update_with_buffer(&mut buffer, WIDTH, HEIGHT).unwrap();
    }
}
