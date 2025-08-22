use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0;

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            };
        }

        d += 1.0;
    }
}

/// Colores por tipo de pared
fn wall_color(c: char) -> u32 {
    match c {
        '#' => 0x808080, // gris
        'A' => 0xFF0000, // rojo
        'B' => 0x0000FF, // azul
        'C' => 0x00FF00, // verde
        _ => 0xFFFFFF,   // blanco por defecto
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let block_size = 64;
    let num_rays = framebuffer.width;

    let hh = framebuffer.height as f32 / 2.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        let intersect = cast_ray(maze, player, a, block_size);

        let distance_to_wall = intersect.distance * (player.a - a).cos();
        let distance_to_projection_plane = hh;

        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        let color = wall_color(intersect.impact);

        for y in stake_top..stake_bottom {
            framebuffer.set_color_point(i, y, color);
        }
    }
}

/// Cargar mapa desde archivo
pub fn load_maze(path: &str) -> Vec<Vec<char>> {
    use std::fs::read_to_string;
    let contents = read_to_string(path).expect("No se pudo leer el archivo");
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
