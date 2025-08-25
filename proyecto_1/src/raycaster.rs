use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::Textures;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub object_type: Option<char>, // Para identificar objetos
    pub hit_x: f32,
    pub hit_y: f32,
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
            return Intersect { 
                distance: d, 
                impact: '#',
                object_type: None,
                hit_x: x,
                hit_y: y,
            }; 
        }

        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        if j >= maze.len() || i >= maze[0].len() { 
            return Intersect { 
                distance: d, 
                impact: '#',
                object_type: None,
                hit_x: x,
                hit_y: y,
            }; 
        }

        let cell = maze[j][i];
        
        // Detectar objetos (1, 2, 3)
        if cell == '1' || cell == '2' || cell == '3' {
            return Intersect { 
                distance: d, 
                impact: ' ', // No es una pared
                object_type: Some(cell),
                hit_x: x,
                hit_y: y,
            };
        }
        
        if cell != ' ' { 
            return Intersect { 
                distance: d, 
                impact: cell,
                object_type: None,
                hit_x: x,
                hit_y: y,
            }; 
        }

        d += step;
        if d > 5000.0 { 
            return Intersect { 
                distance: d, 
                impact: ' ',
                object_type: None,
                hit_x: x,
                hit_y: y,
            }; 
        }
    }
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    textures: &Textures
) {
    let w = framebuffer.width;
    let h = framebuffer.height;
    let hh = h as f32 / 2.0;

    // cielo y piso
    for y in 0..(h / 2) { for x in 0..w { framebuffer.point(x, y, 0x303050); } }
    for y in (h / 2)..h { for x in 0..w { framebuffer.point(x, y, 0x202020); } }

    // Renderizar paredes
    for x in 0..w {
        let t = x as f32 / w as f32;
        let a = player.a - (player.fov / 2.0) + player.fov * t;

        let hit = cast_ray(maze, player, a, block_size);
        
        if hit.object_type.is_some() {
            continue; // los objetos se renderizan después
        }

        let distance = hit.distance * (player.a - a).cos();
        if distance <= 0.0 { continue; }

        let stake_h = (block_size as f32 * hh) / distance;
        let top = (hh - stake_h / 2.0).max(0.0) as usize;
        let bot = (hh + stake_h / 2.0).min((h - 1) as f32) as usize;

        let wall_x = hit.hit_x % block_size as f32 / block_size as f32;

        for y in top..=bot {
            let v = (y - top) as f32 / (bot - top + 1) as f32;
            let color = textures.sample(hit.impact, wall_x, v);
            framebuffer.point(x, y, color);
        }
    }

    // Renderizar sprites/objetos
    render_sprites(framebuffer, player, maze, block_size, textures);

    // Renderizar minimapa
    render_minimap(framebuffer, player, maze, 10, 10, 4, block_size);
}

fn render_sprites(
    fb: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    textures: &Textures,
) {
    let w = fb.width;
    let h = fb.height;
    let hh = h as f32 / 2.0;

    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '1' && cell != '2' && cell != '3' { continue; }

            let obj_x = (x * block_size) as f32 + block_size as f32 / 2.0;
            let obj_y = (y * block_size) as f32 + block_size as f32 / 2.0;

            let dx = obj_x - player.pos.x;
            let dy = obj_y - player.pos.y;
            let dist = (dx*dx + dy*dy).sqrt();
            let mut angle = dy.atan2(dx) - player.a;

            while angle > std::f32::consts::PI { angle -= 2.0 * std::f32::consts::PI; }
            while angle < -std::f32::consts::PI { angle += 2.0 * std::f32::consts::PI; }

            if angle.abs() < player.fov / 2.0 && dist > 0.5 {
                let screen_x = (0.5 + angle / player.fov) * w as f32;
                let sprite_size = (block_size as f32 * hh / dist) as usize;
                let half_size = sprite_size / 2;

                let start_x = (screen_x as isize - half_size as isize).max(0) as usize;
                let end_x = (screen_x as isize + half_size as isize).min(w as isize - 1) as usize;
                let start_y = (hh as isize - half_size as isize).max(0) as usize;
                let end_y = (hh as isize + half_size as isize).min(h as isize - 1) as usize;

                for sy in start_y..end_y {
                    for sx in start_x..end_x {
                        let u = (sx as f32 - (screen_x - half_size as f32)) / sprite_size as f32;
                        let v = (sy as f32 - (hh - half_size as f32)) / sprite_size as f32;
                        let color = textures.sample_sprite(cell, u, v);
                        if color != 0x000000 { fb.point(sx, sy, color); }
                    }
                }
            }
        }
    }
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
    fn wall_color(c: char) -> u32 {
        match c {
            '#' => 0x808080,
            'A' => 0xCC3333,
            'B' => 0x33CC33,
            'C' => 0x3333CC,
            '1' => 0xFF0000,
            '2' => 0x00FF00,
            '3' => 0x0000FF,
            _   => 0x606060,
        }
    }

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

    let px = x_off as f32 + (player.pos.x / block_size as f32) * scale as f32;
    let py = y_off as f32 + (player.pos.y / block_size as f32) * scale as f32;

    let r = (scale as i32 / 3).max(2);
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

    let len = (scale as f32 * 0.9).max(4.0);
    let tip_x = px + player.a.cos() * len;
    let tip_y = py + player.a.sin() * len;
    draw_line(fb, px as i32, py as i32, tip_x as i32, tip_y as i32, 0xFFFF00);
}

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

pub fn load_maze(path: &str) -> Vec<Vec<char>> {
    use std::fs::read_to_string;
    let contents = read_to_string(path).expect("No se pudo leer el archivo");
    let mut lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let max_w = lines.iter().map(|r| r.len()).max().unwrap_or(0);
    for r in lines.iter_mut() {
        if r.len() < max_w { r.resize(max_w, ' '); }
    }
    lines
}
