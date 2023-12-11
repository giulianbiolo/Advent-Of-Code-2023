#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile { Empty, Galaxy }

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Galaxy => write!(f, "#"),
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn load_data(filename: &str) -> Result<Map, std::io::Error> {
    let file: String = std::fs::read_to_string(&filename)?;
    let mut map: Map = Vec::new();
    for line in file.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Tile::Empty),
                '#' => row.push(Tile::Galaxy),
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid character")),
            }
        }
        map.push(row);
    }
    Ok(map)
}

fn distance(a: (usize, usize), b: (usize, usize), map: &Map, amount: i128) -> i128 {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    for row in a.1.min(b.1)..a.1.max(b.1) { if map[row].iter().all(|&tile| tile == Tile::Empty) { empty_rows.push(row); } }
    for col in a.0.min(b.0)..a.0.max(b.0) { if map.iter().all(|row| row[col] == Tile::Empty) { empty_cols.push(col); } }
    let dx: i128 = (a.0 as i128) - (b.0 as i128);
    let dy: i128 = (a.1 as i128) - (b.1 as i128);
    dx.abs() + dy.abs() + (empty_rows.len() as i128 * (amount - 1_i128)) + (empty_cols.len() as i128 * (amount - 1_i128))
}

fn get_galaxies(map: &Map) -> Vec<(usize, usize)> {
    map
        .iter()
        .enumerate()
        .map(|(y, row)| 
            row
                .iter()
                .enumerate()
                .filter(|(_, tile)| **tile == Tile::Galaxy)
                .map(move |(x, _)| (x, y))
        )
        .flatten()
        .collect()
}

fn part_one(map: &Map) -> i128 {
    get_galaxies(map)
        .iter()
        .enumerate()
        .map(|(i, galaxy)| {
            get_galaxies(map)[i+1..]
                .iter()
                .map(|other| distance(*galaxy, *other, &map, 2))
                .sum::<i128>()
        }).sum()
}

fn part_two(map: &Map) -> i128 {
    get_galaxies(map)
    .iter()
    .enumerate()
    .map(|(i, galaxy)| {
        get_galaxies(map)[i+1..]
            .iter()
            .map(|other| distance(*galaxy, *other, &map, 1_000_000))
            .sum::<i128>()
    }).sum()
}

fn main() -> Result<(), std::io::Error> {
    let map: Map = load_data("input.txt")?;
    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
    Ok(())
}
