use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Garden {
    seeds: Vec<u64>,
    maps: Vec<Vec<Vec<u64>>>,
}

#[inline(always)]
fn seed_convert(seed: u64, map: &Vec<Vec<u64>>) -> u64 {
    for row in map {
        if row[1] <= seed && seed <= row[1] + row[2] - 1 { // The -1 is because 98 2 is the range 98-99 and not 98-100
            return row[0] + (seed - row[1]);
        }
    }
    seed
}

impl Garden {
    fn new(seeds: Vec<u64>, maps: Vec<Vec<Vec<u64>>>) -> Self {
        Self { seeds, maps }
    }
    fn part_one(&self) -> u64 {
        let mut best_seed: u64 = u64::MAX;
        for seed in &self.seeds {
            best_seed = self.maps
                .iter()
                .fold(*seed, |acc, map| seed_convert(acc, map))
                .min(best_seed);
        }
        best_seed
    }
    fn part_two(&self) -> u64 {
        let seeds: &Vec<(u64, u64)> = &self.seeds.chunks(2).map(|x| (x[0], x[1])).collect::<Vec<(u64, u64)>>();
        let best_seed = seeds
            .par_iter()
            .fold(|| u64::MAX, |acc, x| {
                let (start, len) = x;
                let mut best_seed: u64 = u64::MAX;
                for idx in 0..*len {
                    best_seed = self.maps
                        .iter()
                        .fold(start + idx, |acc, map| seed_convert(acc, map))
                        .min(best_seed);
                }
                best_seed.min(acc)
            })
            .min()
            .unwrap();
        best_seed
    }
}

fn load_input() -> Result<Garden, std::io::Error> {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let mut lines: std::str::Lines<'_> = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut map: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        if line.trim() == "" {
            maps.push(map);
            map = Vec::new();
        } else {
            if line.contains("map") { continue; }
            map.push(line.trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
        }
    }
    maps.remove(0); // Remove the first element wich is empty
    maps.push(map); // Add the last map
    Ok(Garden::new(seeds, maps))
}

fn main() {
    let garden: Garden = load_input().unwrap();
    let starttime: std::time::Instant = std::time::Instant::now();
    println!("Part one: {:?} | Calculated in {} seconds", garden.part_one(), starttime.elapsed().as_secs_f64());
    let starttime: std::time::Instant = std::time::Instant::now();
    println!("Part two: {:?} | Calculated in {} seconds", garden.part_two(), starttime.elapsed().as_secs_f64());
}
