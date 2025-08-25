mod framebuffer;
mod player;
mod raycaster;
mod textures;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycaster::{render3d, load_maze};
use rodio::Source;
use textures::Textures;

use std::time::Instant;

// --- Texto con rusttype ---
use rusttype::{Font, Scale, point};
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const BLOCK_SIZE: usize = 64; // Debe coincidir con raycaster y colisiones
const TOTAL_ITEMS: u32 = 3; // Total de objetos a recolectar

fn main() {
    let mut window = Window::new(
        "Raycaster con Objetivos y Texturas",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Cargar fuente una sola vez
    let font = load_font("fonts/Arial.ttf");
    
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    start_screen(&mut window, &mut framebuffer, &font);

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut maze = load_maze("./maze.txt");
    let textures = Textures::new();

    let mut player = Player::new(
        150.0, // x
        150.0, // y
        0.0,   // ángulo
        std::f32::consts::FRAC_PI_2, // FOV ~ 90°
    );

    // --- Música de fondo ---
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Cargar archivo de música
    let file = BufReader::new(File::open("assets/music.ogg").unwrap());
    let source = Decoder::new(file).unwrap();

    // Reproducir en bucle
    sink.append(source.repeat_infinite());
    sink.play();

    // Velocidades
    let base_speed = 4.0;      // velocidad normal
    let run_multiplier = 1.8;  // factor de correr
    let rot_speed = 0.05;

    // FPS
    let mut last_frame_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Delta time y FPS
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time);
        last_frame_time = now;
        let fps = 1.0 / delta_time.as_secs_f32();

        // Limpiar
        framebuffer.clear(0x000000);

        // --- VELOCIDAD VARIABLE ---
        let mut move_speed = base_speed;
        if window.is_key_down(Key::LeftShift) {
            move_speed *= run_multiplier; // correr con SHIFT
        }

        // Controles
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            player.rotate_left(rot_speed);
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            player.rotate_right(rot_speed);
        }
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            player.move_forward(move_speed, &maze, BLOCK_SIZE);
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            player.move_backward(move_speed, &maze, BLOCK_SIZE);
        }
        if window.is_key_pressed(Key::X, minifb::KeyRepeat::No) {
            // Lanzar un rayo hacia adelante para detectar objetos
            let hit = raycaster::cast_ray(&maze, &player, player.a, BLOCK_SIZE);
            
            if let Some(obj_type) = hit.object_type {
                if hit.distance < 50.0 { // Solo si está cerca
                    println!("Recolectado objeto: {}", obj_type);
                    player.collect_item();
                    
                    // "Eliminar" el objeto del mapa (reemplazar con espacio)
                    let obj_x = (player.pos.x + player.a.cos() * hit.distance) as usize / BLOCK_SIZE;
                    let obj_y = (player.pos.y + player.a.sin() * hit.distance) as usize / BLOCK_SIZE;
                    
                    if obj_y < maze.len() && obj_x < maze[0].len() {
                        maze[obj_y][obj_x] = ' ';
                    }
                    
                    // Reproducir sonido de recolección
                    play_sound("assets/collect.wav");
                }
            }
        }
        
        // Detectar si el jugador está mirando un objeto y presiona E para recolectar
        if window.is_key_pressed(Key::E, minifb::KeyRepeat::No) {
            // Lanzar un rayo hacia adelante para detectar objetos
            let hit = raycaster::cast_ray(&maze, &player, player.a, BLOCK_SIZE);
            
            if let Some(obj_type) = hit.object_type {
                if hit.distance < 50.0 { // Solo si está cerca
                    println!("Recolectado objeto: {}", obj_type);
                    player.collect_item();
                    
                    // "Eliminar" el objeto del mapa (reemplazar con espacio)
                    let obj_x = (player.pos.x + player.a.cos() * hit.distance) as usize / BLOCK_SIZE;
                    let obj_y = (player.pos.y + player.a.sin() * hit.distance) as usize / BLOCK_SIZE;
                    
                    if obj_y < maze.len() && obj_x < maze[0].len() {
                        maze[obj_y][obj_x] = ' ';
                    }
                    
                    // Reproducir sonido de recolección
                    play_sound("assets/collect.wav");
                }
            }
        }

        // Render 3D
        render3d(&mut framebuffer, &player, &maze, BLOCK_SIZE, &textures);

        // Minimap (esquina superior izquierda)
        render_minimap(&mut framebuffer, &player, &maze, 8, 8, 6);

        // Mostrar contador de objetos recolectados
        let items_text = format!("Objetos: {}/{}", player.get_collected_items(), TOTAL_ITEMS);
        draw_text(
            &mut framebuffer,
            &font,
            &items_text,
            14,
            14,
            0xFFFF00,
            18.0,
        );

        // FPS (esquina superior derecha)
        let fps_text = format!("FPS: {:.0}", fps);
        draw_text(
            &mut framebuffer,
            &font,
            &fps_text,
            WIDTH.saturating_sub(140),
            14,
            0xFFFF00,
            18.0,
        );

        // Volcar al buffer lineal y mostrar
        framebuffer.flush_to(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        // Comprobar si se han recolectado todos los objetos
        if player.has_all_items(TOTAL_ITEMS) {
            victory_screen(&mut window, &mut framebuffer, &font);
            break;
        }
    }
}

fn start_screen(window: &mut Window, framebuffer: &mut Framebuffer, font: &Font) {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Cargar imagen de fondo (asegúrate que exista en assets/)
    let mut img = image::open("assets/Radioheadkida.png")
    .expect("No se pudo cargar la imagen de fondo de inicio")
    .resize_exact(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Nearest);

    // Convertir a RGB y escalar a la resolución de la ventana
    let img = img.resize(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Triangle);
    let img_buf = img.to_rgb8();

    loop {
        // Copiar imagen como fondo
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let px = img_buf.get_pixel(x as u32, y as u32);
                let r = px[0] as u32;
                let g = px[1] as u32;
                let b = px[2] as u32;
                framebuffer.point(x, y, (r << 16) | (g << 8) | b);
            }
        }

        // Texto encima
        draw_text(
            framebuffer,
            font,
            "atrapa el radio objeto",
            WIDTH / 2 - 160,
            HEIGHT / 2 - 50,
            0xFFFFFF,
            40.0,
        );

        draw_text(
            framebuffer,
            font,
            "Presiona ENTER para comenzar",
            WIDTH / 2 - 200,
            HEIGHT / 2 + 20,
            0xFFFF00,
            24.0,
        );

        // Mostrar en ventana
        framebuffer.flush_to(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Esperar ENTER
        if window.is_key_down(Key::Enter) {
            break;
        }

        if !window.is_open() || window.is_key_down(Key::Escape) {
            std::process::exit(0);
        }
    }
}

fn victory_screen(window: &mut Window, framebuffer: &mut Framebuffer, font: &Font) {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    
    // Cargar imagen de victoria
    let mut img = image::open("assets/victory.png")
        .unwrap_or_else(|_| image::DynamicImage::new_rgb8(WIDTH as u32, HEIGHT as u32))
        .resize_exact(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Nearest);
    
    let img_buf = img.to_rgb8();
    
    loop {
        // Fondo con imagen
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let px = img_buf.get_pixel(x as u32, y as u32);
                let r = px[0] as u32;
                let g = px[1] as u32;
                let b = px[2] as u32;
                framebuffer.point(x, y, (r << 16) | (g << 8) | b);
            }
        }
        
        draw_text(
            framebuffer,
            font,
            "¡FELICIDADES!",
            WIDTH / 2 - 150,
            HEIGHT / 2 - 50,
            0x00FF00,
            40.0,
        );
        
        draw_text(
            framebuffer,
            font,
            "Has recolectado todos los objetos",
            WIDTH / 2 - 200,
            HEIGHT / 2 + 20,
            0xFFFFFF,
            24.0,
        );
        
        draw_text(
            framebuffer,
            font,
            "Presiona ESC para salir",
            WIDTH / 2 - 150,
            HEIGHT / 2 + 70,
            0xFFFF00,
            20.0,
        );

        framebuffer.flush_to(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        if window.is_key_down(Key::Escape) {
            break;
        }
    }
}

fn play_sound(path: &str) {
    // Implementación básica para reproducir sonido
    if let Ok(file) = File::open(path) {
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(source.convert_samples());
    }
}

// ============================================================================
// Minimap (con jugador correctamente escalado)
// ============================================================================
fn render_minimap(
    fb: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    x_off: usize,
    y_off: usize,
    scale: usize, // píxeles por celda en el minimapa
) {
    // Colores por tipo de pared SOLO para el minimapa
    fn wall_color(c: char) -> u32 {
        match c {
            '#' => 0x808080, // gris
            'A' => 0xCC3333, // rojo
            'B' => 0x33CC33, // verde
            'C' => 0x3333CC, // azul
            '1' => 0xFF0000, // objeto 1 rojo
            '2' => 0x00FF00, // objeto 2 verde
            '3' => 0x0000FF, // objeto 3 azul
            _   => 0x606060, // otros
        }
    }

    // Celdas del mapa
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            let color = if cell == ' ' { 0x000000 } else { wall_color(cell) };
            for dx in 0..scale {
                for dy in 0..scale {
                    let px = x_off + i * scale + dx;
                    let py = y_off + j * scale + dy;
                    if px < fb.width && py < fb.height {
                        fb.point(px, py, color);
                    }
                }
            }
        }
    }

    // Jugador: convertir coordenadas del mundo -> minimapa
    let px = x_off as f32 + (player.pos.x / BLOCK_SIZE as f32) * scale as f32;
    let py = y_off as f32 + (player.pos.y / BLOCK_SIZE as f32) * scale as f32;

    // Dibujar al jugador como un disco pequeño
    let r = (scale as i32 / 3).max(2); // radio
    for dy in -r..=r {
        for dx in -r..=r {
            if dx*dx + dy*dy <= r*r {
                let mx = px as i32 + dx;
                let my = py as i32 + dy;
                if mx >= 0 && my >= 0 && (mx as usize) < fb.width && (my as usize) < fb.height {
                    fb.point(mx as usize, my as usize, 0xFFFF00);
                }
            }
        }
    }

    // Pequeño "heading" (línea) indicando hacia dónde mira
    let len = (scale as f32 * 0.9).max(4.0);
    let tip_x = px + player.a.cos() * len;
    let tip_y = py + player.a.sin() * len;
    draw_line(fb, px as i32, py as i32, tip_x as i32, tip_y as i32, 0xFFFF00);
}

// Bresenham para la línea del heading en el minimapa
fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as usize) < fb.width && (y0 as usize) < fb.height {
            fb.point(x0 as usize, y0 as usize, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

// ============================================================================
// Texto (FPS) con rusttype
// ============================================================================

fn load_font(path: &str) -> Font<'static> {
    let mut data = Vec::new();
    File::open(path).expect("No se pudo abrir la fuente (fonts/Arial.ttf)")
        .read_to_end(&mut data)
        .expect("No se pudo leer la fuente");
    Font::try_from_vec(data).expect("Fuente inválida o corrupta")
}

/// Dibuja texto sólido (sin blending con fondo) con color RGB 0xRRGGBB
fn draw_text(fb: &mut Framebuffer, font: &Font<'_>, text: &str, x: usize, y: usize, color: u32, size: f32) {
    let scale = Scale { x: size, y: size };
    let v_metrics = font.v_metrics(scale);
    let mut cursor_x = x as f32;
    let baseline_y = y as f32 + v_metrics.ascent;

    for ch in text.chars() {
        let glyph = font.glyph(ch).scaled(scale).positioned(point(cursor_x, baseline_y));
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                if v == 0.0 { return; }
                let px = bb.min.x + gx as i32;
                let py = bb.min.y + gy as i32;
                if px >= 0 && py >= 0 && (px as usize) < fb.width && (py as usize) < fb.height {
                    // Escalamos el color por la cobertura 'v' para antialias
                    let r = ((color >> 16) & 0xFF) as f32 * v;
                    let g = ((color >> 8) & 0xFF) as f32 * v;
                    let b = (color & 0xFF) as f32 * v;
                    let col = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                    fb.point(px as usize, py as usize, col);
                }
            });
        }
        cursor_x += glyph.unpositioned().h_metrics().advance_width;
    }
}