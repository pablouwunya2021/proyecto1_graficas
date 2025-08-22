use std::fs::read_to_string;

pub fn load_map(path: &str) -> Vec<Vec<char>> {
    let contents = read_to_string(path).expect("No se pudo leer el archivo del mapa");
    let mut lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Validación simple: todas las filas deben tener el mismo ancho
    let max_w = lines.iter().map(|r| r.len()).max().unwrap_or(0);
    for r in lines.iter_mut() {
        if r.len() < max_w {
            r.resize(max_w, ' '); // rellenar con espacios
        }
    }

    lines
}
