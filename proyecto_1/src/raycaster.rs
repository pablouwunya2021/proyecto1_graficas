use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

fn wall_color(c: char) -> u32 {
    match c {
        '#' => 0xAAAAAA, // gris
        'A' => 0xFF5555, // rojo
        'B' => 0x55FF55, // verde
        'C' => 0x5555FF, // azul
        _   => 0xFFFFFF, // blanco
    }
}

pub fn cast_ray(
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0f32;
    let step = 1.0f32;

    loop {
        let x = player.pos.x + a.cos() * d;
        let y = player.pos.y + a.sin() * d;

        if x < 0.0 || y < 0.0 {
            return Intersect { distance: d, impact: '#' };
        }

        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        if j >= maze.len() || i >= maze[0].len() {
            return Intersect { distance: d, impact: '#' };
        }

        let cell = maze[j][i];
        if cell != ' ' {
            return Intersect { distance: d, impact: cell };
        }

        d += step;
        if d > 5000.0 {
            return Intersect { distance: d, impact: ' ' };
        }
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize) {
    let w = framebuffer.width;
    let h = framebuffer.height;
    let hh = h as f32 / 2.0;

    // cielo y piso
    for y in 0..(h / 2) {
        for x in 0..w { framebuffer.point(x, y, 0x303050); }
    }
    for y in (h / 2)..h {
        for x in 0..w { framebuffer.point(x, y, 0x202020); }
    }

    // columnas (ray casting)
    for x in 0..w {
        let t = x as f32 / w as f32;
        let a = player.a - (player.fov / 2.0) + player.fov * t;

        let hit = cast_ray(maze, player, a, block_size);

        let distance = hit.distance * (player.a - a).cos(); // corrección fish-eye
        if distance <= 0.0 { continue; }

        let stake_h = (block_size as f32 * hh) / distance;

        let top = (hh - stake_h / 2.0).max(0.0) as usize;
        let bot = (hh + stake_h / 2.0).min((h - 1) as f32) as usize;

        let color = wall_color(hit.impact);
        for y in top..=bot {
            framebuffer.point(x, y, color);
        }
    }

    // minimapa
    render_minimap(framebuffer, player, maze, 10, 10, 4, block_size);
}

pub fn render_minimap(
    fb: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    x_off: usize,
    y_off: usize,
    scale: usize,
    block_size: usize,
) {
    // celdas
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            let color = if cell == ' ' { 0x000000 } else { wall_color(cell) };
            for dx in 0..scale {
                for dy in 0..scale {
                    fb.point(x_off + i * scale + dx, y_off + j * scale + dy, color);
                }
            }
        }
    }

    // jugador como flecha
    let px = x_off as f32 + (player.pos.x / block_size as f32) * scale as f32;
    let py = y_off as f32 + (player.pos.y / block_size as f32) * scale as f32;

    let size = 5.0;
    let a = player.a;
    let tip_x = px + a.cos() * size;
    let tip_y = py + a.sin() * size;
    let base_l_x = px + (a + 2.5).cos() * (size * 0.5);
    let base_l_y = py + (a + 2.5).sin() * (size * 0.5);
    let base_r_x = px + (a - 2.5).cos() * (size * 0.5);
    let base_r_y = py + (a - 2.5).sin() * (size * 0.5);

    draw_line(fb, px, py, tip_x, tip_y, 0xFFFF00);
    draw_line(fb, px, py, base_l_x, base_l_y, 0xFFFF00);
    draw_line(fb, px, py, base_r_x, base_r_y, 0xFFFF00);
}

fn draw_line(fb: &mut Framebuffer, x0: f32, y0: f32, x1: f32, y1: f32, color: u32) {
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 {
            fb.point(x0 as usize, y0 as usize, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

pub fn load_maze(path: &str) -> Vec<Vec<char>> {
    use std::fs::read_to_string;
    let contents = read_to_string(path).expect("No se pudo leer el archivo");
    let mut lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    // normaliza a mismo ancho
    let max_w = lines.iter().map(|r| r.len()).max().unwrap_or(0);
    for r in lines.iter_mut() {
        if r.len() < max_w { r.resize(max_w, ' '); }
    }
    lines
}
