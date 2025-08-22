use crate::config::BLOCK_SIZE;
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.clear();

    // Cielo y piso (gris oscuro) opcionales simples
    // cielo
    for y in 0..(framebuffer.height / 2) {
        for x in 0..framebuffer.width {
            framebuffer.put(x, y, 0x303050);
        }
    }
    // piso
    for y in (framebuffer.height / 2)..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.put(x, y, 0x202020);
        }
    }

    // Ray casting
    for i in 0..num_rays {
        let t = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * t);

        let inter = cast_ray(maze, player, a, BLOCK_SIZE);

        // corrección “fish-eye”
        let distance = inter.distance * (player.a - a).cos();
        if distance <= 0.0 {
            continue;
        }

        // Altura del “stake”
        let stake_height = (BLOCK_SIZE as f32 * hh) / distance;

        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom =
            (hh + (stake_height / 2.0)).min(framebuffer.height as f32 - 1.0) as usize;

        // color base según tipo de pared (por ahora 2 colores simples para distinguir)
        let color = match inter.impact {
            '#' => 0xE0E0E0, // pared sólida
            _ => 0xA0A0A0,   // otros
        };

        for y in stake_top..stake_bottom {
            framebuffer.put(i, y, color);
        }
    }
}

pub fn cast_ray(
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0_f32;
    let step = 1.0_f32;

    loop {
        let x = player.pos.x + a.cos() * d;
        let y = player.pos.y + a.sin() * d;

        if x < 0.0 || y < 0.0 {
            return Intersect {
                distance: d,
                impact: '#',
            };
        }

        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        if j >= maze.len() || i >= maze[0].len() {
            // chocó con el límite del mundo
            return Intersect {
                distance: d,
                impact: '#',
            };
        }

        let cell = maze[j][i];
        if cell != ' ' {
            return Intersect {
                distance: d,
                impact: cell,
            };
        }

        d += step;
        if d > 5000.0 {
            // límite de seguridad
            return Intersect {
                distance: d,
                impact: ' ',
            };
        }
    }
}
