#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Ground,
    Start
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Vertical => '║',
            Tile::Horizontal => '═',
            Tile::BendNorthEast => '╚',
            Tile::BendNorthWest => '╝',
            Tile::BendSouthWest => '╗',
            Tile::BendSouthEast => '╔',
            Tile::Ground => '.',
            Tile::Start => 'S'
        };
        write!(f, "{}", c)
    }
}

fn tiles_connect(tile1: Tile, pos1: (u32, u32), tile2: Tile, pos2: (u32, u32)) -> bool {
    match tile1 {
        Tile::Vertical => {
            match tile2 {
                Tile::Vertical => pos1.1 == pos2.1,
                Tile::Horizontal => false,
                Tile::BendNorthEast => pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1,
                Tile::BendNorthWest => pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1,
                Tile::BendSouthWest => pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1,
                Tile::BendSouthEast => pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1,
                Tile::Ground => false,
                Tile::Start => pos1.1 == pos2.1
            }
        },
        Tile::Horizontal => {
            match tile2 {
                Tile::Vertical => false,
                Tile::Horizontal => pos1.0 == pos2.0,
                Tile::BendNorthEast => pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1,
                Tile::BendNorthWest => pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1,
                Tile::BendSouthWest => pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1,
                Tile::BendSouthEast => pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1,
                Tile::Ground => false,
                Tile::Start => pos1.0 == pos2.0
            }
        },
        Tile::BendNorthEast => {
            match tile2 {
                Tile::Vertical => pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1,
                Tile::Horizontal => pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1,
                Tile::BendNorthEast => false,
                Tile::BendNorthWest => pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1,
                Tile::BendSouthWest => (pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1) || (pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1),
                Tile::BendSouthEast => pos1.1 == pos2.1 && pos1.0 == pos2.0 + 1,
                Tile::Ground => false,
                Tile::Start => (pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1) || (pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1)
            }
        },
        Tile::BendNorthWest => {
            match tile2 {
                Tile::Vertical => pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1,
                Tile::Horizontal => pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1,
                Tile::BendNorthEast => pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1,
                Tile::BendNorthWest => false,
                Tile::BendSouthWest => pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1,
                Tile::BendSouthEast => (pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1) || (pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1),
                Tile::Ground => false,
                Tile::Start => (pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1) || (pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1)
            }
        },
        Tile::BendSouthWest => {
            match tile2 {
                Tile::Vertical => pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1,
                Tile::Horizontal => pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1,
                Tile::BendNorthEast => (pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1) || (pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1),
                Tile::BendNorthWest => pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1,
                Tile::BendSouthWest => false,
                Tile::BendSouthEast => pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1,
                Tile::Ground => false,
                Tile::Start => (pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1) || (pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1)
            }
        },
        Tile::BendSouthEast => {
            match tile2 {
                Tile::Vertical => pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1,
                Tile::Horizontal => pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1,
                Tile::BendNorthEast => pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1,
                Tile::BendNorthWest => (pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1) || (pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1),
                Tile::BendSouthWest => pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1,
                Tile::BendSouthEast => false,
                Tile::Ground => false,
                Tile::Start => (pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1) || (pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1)
            }
        },
        Tile::Ground => false,
        Tile::Start => {
            match tile2 {
                Tile::Vertical => pos1.1 == pos2.1,
                Tile::Horizontal => pos1.0 == pos2.0,
                Tile::BendNorthEast => (pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1) || (pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1),
                Tile::BendNorthWest => (pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1) || (pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1),
                Tile::BendSouthWest => (pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1) || (pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1),
                Tile::BendSouthEast => (pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1) || (pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1),
                Tile::Ground => false,
                Tile::Start => false
            }
        }
    }
}

fn load_data(filename: &str) -> Result<Vec<Vec<Tile>>, std::io::Error> {
    let mut data: Vec<Vec<Tile>> = Vec::new();
    let file = std::fs::read_to_string(filename)?;
    for line in file.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            let tile: Tile = match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::BendNorthEast,
                'J' => Tile::BendNorthWest,
                '7' => Tile::BendSouthWest,
                'F' => Tile::BendSouthEast,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => panic!("Unknown tile: {}", c)
            };
            row.push(tile);
        }
        data.push(row);
    }
    Ok(data)
}

fn find_loop(data: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut loop_tiles: Vec<(usize, usize)> = Vec::new();
    if let Some((row, _)) = data.iter().enumerate().find(|(_, row)| row.contains(&Tile::Start)) {
        if let Some((col, _)) = data[row].iter().enumerate().find(|(_, tile)| **tile == Tile::Start) {
            loop_tiles.push((row, col));
            let max_rows: usize = data.len();
            let max_cols: usize = data[0].len();
            let neighbours = |row: usize, col: usize| -> Vec<(usize, usize)> {
                let mut neighbours: Vec<(usize, usize)> = Vec::new();
                if row + 1 < max_rows { neighbours.push((row + 1, col)); }
                if col + 1 < max_cols { neighbours.push((row, col + 1)); }
                if row > 0 { neighbours.push((row - 1, col)); }
                if col > 0 { neighbours.push((row, col - 1)); }
                neighbours
            };
            let nn: Vec<(usize, usize)> = neighbours(row, col);
            let first_neighbour: &(usize, usize) = nn
                .iter()
                .filter(|(r, c)| tiles_connect(Tile::Start, (row as u32, col as u32), data[*r][*c], (*r as u32, *c as u32)))
                .next()
                .unwrap();
            loop_tiles.push(*first_neighbour);
            loop {
                let current_position: &(usize, usize) = loop_tiles.last().unwrap();
                let current_tile: Tile = data[current_position.0][current_position.1];
                let nn: Vec<(usize, usize)> = neighbours(current_position.0, current_position.1);
                let next_neighbour: Option<&(usize, usize)> = nn
                    .iter()
                    .filter(|(r, c)| !loop_tiles.contains(&(*r, *c)))
                    .filter(|(r, c)| tiles_connect(current_tile, (current_position.0 as u32, current_position.1 as u32), data[*r][*c], (*r as u32, *c as u32)))
                    .next();
                if let Some(next) = next_neighbour {
                    if next == &(row, col) { break; }
                    loop_tiles.push(*next);
                } else { println!("Reached the end! (Check correctness yourself. Here, have a nice picture :D)"); break; }
            }
            loop_tiles
        } else { panic!("Start not found in row"); }
    } else { panic!("Start not found in data"); }
}

fn print_loop(data: &Vec<Vec<Tile>>, loop_tiles: &Vec<(usize, usize)>, enclosed_tiles: &Vec<(usize, usize)>) {
    for (nrow, row) in data.iter().enumerate() {
        for (ntile, tile) in row.iter().enumerate() {
            if loop_tiles.last().unwrap_or(&(1000, 1000)) == &(nrow, ntile) { print!("\x1b[01m\x1b[31m{}\x1b[0m", tile); }
            else if loop_tiles.contains(&(nrow, ntile)) { print!("\x1b[01m\x1b[32m{}\x1b[0m", tile); }
            else if enclosed_tiles.contains(&(nrow, ntile)) { print!("\x1b[01m\x1b[33m{}\x1b[0m", tile); }
            else { print!("{}", tile); }
        }
        println!();
    }
}

fn isinside(p: (usize, usize), polygon: &Vec<(usize, usize)>) -> bool {
    let mut inside: bool = false;
    let mut j: usize = polygon.len() - 1;
    for i in 0..polygon.len() {
        if (polygon[i].1 > p.1) != (polygon[j].1 > p.1)
            && p.0 < (polygon[j].0 - polygon[i].0) * (p.1 - polygon[i].1)
            / (polygon[j].1 - polygon[i].1) + polygon[i].0 { inside = !inside; }
        j = i;
    }
    inside
}

fn part_one(loop_tiles: &Vec<(usize, usize)>) -> usize { loop_tiles.len() / 2 }

fn part_two(data: &Vec<Vec<Tile>>, loop_tiles: &Vec<(usize, usize)>) -> (usize, Vec<(usize, usize)>) {
    let mut enclosed: usize = 0;
    let mut enclosed_tiles: Vec<(usize, usize)> = Vec::new();
    for (nrow, row) in data.iter().enumerate() {
        for (ncol, _) in row.iter().enumerate() {
            if loop_tiles.contains(&(nrow, ncol)) { continue; }
            if !isinside((nrow, ncol), &loop_tiles) { continue; }
            enclosed += 1;
            enclosed_tiles.push((nrow, ncol));
        }
    }
    (enclosed, enclosed_tiles)
}

fn main() -> Result<(), std::io::Error> {
    let data: Vec<Vec<Tile>> = load_data("input.txt")?;
    let loop_tiles: Vec<(usize, usize)> = find_loop(&data);
    let (enclosed, enclosed_tiles): (usize, Vec<(usize, usize)>) = part_two(&data, &loop_tiles);
    print_loop(&data, &loop_tiles, &enclosed_tiles);
    println!("Part One: {}", part_one(&loop_tiles));
    println!("Part Two: {}", enclosed);
    Ok(())
}
