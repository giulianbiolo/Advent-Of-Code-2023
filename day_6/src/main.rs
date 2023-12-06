fn load_input(filename: &str) -> Result<Vec<(u64, u64)>, std::io::Error> {
    let file: String = std::fs::read_to_string(filename)?;
    let mut lines = file.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("Time: ")
        .trim()
        .split("  ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance: ")
        .trim()
        .split("  ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    Ok(times.iter().zip(distances.iter()).map(|(x, y)| (*x, *y)).collect()) 
}

fn part_one(races: &Vec<(u64, u64)>) -> u64 {
    races
        .iter()
        .fold(1, |acc, (time, distance)| {
        (0..=*time)
            .filter(|i| (time - i) * i > *distance)
            .count() as u64 * acc
    })
}

fn part_two(races: &Vec<(u64, u64)>) -> u64 {
    let (time, distance): (u64, u64) = races
        .iter()
        .fold((0, 0), |(acc_time, acc_distance), (time, distance)| {
        (
            format!("{}{}", acc_time, time).parse::<u64>().unwrap(),
            format!("{}{}", acc_distance, distance).parse::<u64>().unwrap()
        )
    });
    (0..=time).filter(|i| (time - i) * i > distance).count() as u64
}

fn main() -> Result<(), std::io::Error> {
    let input: Vec<(u64, u64)> = load_input("input.txt")?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    Ok(())
}
