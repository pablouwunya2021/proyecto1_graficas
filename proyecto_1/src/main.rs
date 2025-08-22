mod framebuffer;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycaster::{render3d, load_maze};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new("Raycaster - Colores", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Error creando la ventana: {}", e);
        });

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut player = Player::new(150.0, 150.0, 0.0, std::f32::consts::FRAC_PI_2);
    let maze = load_maze("./maze.txt");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- Controles ---
        if window.is_key_down(Key::Up) {
            player.move_forward(3.0, &maze, 64);
        }
        if window.is_key_down(Key::Down) {
            player.move_backward(3.0, &maze, 64);
        }
        if window.is_key_down(Key::Left) {
            player.rotate_left(0.05);
        }
        if window.is_key_down(Key::Right) {
            player.rotate_right(0.05);
        }

        // --- Render ---
        framebuffer.clear(0x87CEEB); // Cielo (azul claro)
        render3d(&mut framebuffer, &player, &maze);
        framebuffer.flush_to(&mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
