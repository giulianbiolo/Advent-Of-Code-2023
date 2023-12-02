const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>
}

fn load_games(filename: &str) -> Result<Vec<Game>, std::io::Error> {
    let mut games: Vec<Game> = Vec::new();
    let inputfile = std::fs::read_to_string(filename).unwrap();
    for line in inputfile.lines() {
        let id_data: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let id: u32 = id_data[0].replace("Game ", "").trim().parse::<u32>().unwrap();
        let rgb_array: Vec<&str> = id_data[1].split(";").collect::<Vec<&str>>(); // [ "3 blue, 4 red", "1 red, 2 green, 6 blue", ... ]
        let mut red: Vec<u32> = Vec::new();
        let mut green: Vec<u32> = Vec::new();
        let mut blue: Vec<u32> = Vec::new();
        for rgb in rgb_array {
            let rgb_data: Vec<&str> = rgb.trim().split(",").collect::<Vec<&str>>(); // [ "3 blue", "4 red" ]
            for color in rgb_data {
                let color_data: Vec<&str> = color.trim().split(" ").collect::<Vec<&str>>();
                let color_count: u32 = color_data[0].parse::<u32>().unwrap();
                let color_name: &str = color_data[1];
                match color_name {
                    "red" => red.push(color_count),
                    "green" => green.push(color_count),
                    "blue" => blue.push(color_count),
                    _ => println!("Error: Unknown color {}", color_name)
                }
            }
        }
        games.push(Game { id, red, green, blue });
    }
    Ok(games)
}

fn part_one(games: &[Game]) -> u32 {
    let mut total: u32 = 0;
    for game in games {
        let maxred: u32 = game.red.iter().max().unwrap().to_owned();
        let maxgreen: u32 = game.green.iter().max().unwrap().to_owned();
        let maxblue: u32 = game.blue.iter().max().unwrap().to_owned();
        let impossible: bool = maxred > MAX_RED || maxgreen > MAX_GREEN || maxblue > MAX_BLUE;
        if impossible { println!("\x1b[93mGame {}: {:?} red, {:?} blue, {:?} green | max(red): {}, max(blue): {}, max(green): {}\x1b[0m", game.id, game.red, game.blue, game.green, maxred, maxblue, maxgreen); }
        else { total += game.id; println!("Game {}: {:?} red, {:?} blue, {:?} green | max(red): {}, max(blue): {}, max(green): {}", game.id, game.red, game.blue, game.green, maxred, maxblue, maxgreen); }
    }
    total
}

fn part_two(games: &[Game]) -> u32 {
    let mut total: u32 = 0;
    for game in games {
        let maxred: u32 = game.red.iter().max().unwrap().to_owned();
        let maxgreen: u32 = game.green.iter().max().unwrap().to_owned();
        let maxblue: u32 = game.blue.iter().max().unwrap().to_owned();
        println!("Power Set of Game {}: {} MaxRed, {} MaxGreen, {} MaxBlue | {} Set", game.id, maxred, maxgreen, maxblue, maxred * maxgreen * maxblue);
        total += maxred * maxgreen * maxblue;
    }
    total
}

fn main() -> Result<(), std::io::Error> {
    let games: Vec<Game> = load_games("input.txt")?;
    let total_partone: u32 = part_one(&games);
    println!("Total for Part One: {}", total_partone);
    let total_parttwo: u32 = part_two(&games);
    println!("Total for Part Two: {}", total_parttwo);

    Ok(())
}
