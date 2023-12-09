fn load_data(filename: &str) -> Result<Vec<Vec<i64>>, std::io::Error> {
    let mut data: Vec<Vec<i64>> = Vec::new();
    let file: String = std::fs::read_to_string(filename)?;
    for line in file.lines() {
        data.push(
            line
                .split(' ')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        );
    }
    Ok(data)
}

fn part_one(data: &Vec<Vec<i64>>) -> i64 {
    let mut sum: i64 = 0;
    for history in data {
        let mut derivatives: Vec<Vec<i64>> = Vec::new();
        derivatives.push(history.clone());
        loop {
            if derivatives[derivatives.len() - 1].iter().all(|&x| x == 0) { break; }
            let mut new_derivative: Vec<i64> = Vec::new();
            for i in 0..derivatives[derivatives.len() - 1].len() - 1 {
                new_derivative.push(derivatives[derivatives.len() - 1][i + 1] - derivatives[derivatives.len() - 1][i]);
            }
            derivatives.push(new_derivative);
        }
        sum += derivatives
            .iter()
            .rev()
            .skip(1)
            .fold(0, |acc, x| acc + x[x.len() - 1]);
    }
    sum
}

fn part_two(data: &Vec<Vec<i64>>) -> i64 {
    let mut sum: i64 = 0;
    for history in data {
        let mut derivatives: Vec<Vec<i64>> = Vec::new();
        derivatives.push(history.clone());
        loop {
            if derivatives[derivatives.len() - 1].iter().all(|&x| x == 0) { break; }
            let mut new_derivative: Vec<i64> = Vec::new();
            for i in 0..derivatives[derivatives.len() - 1].len() - 1 {
                new_derivative.push(derivatives[derivatives.len() - 1][i + 1] - derivatives[derivatives.len() - 1][i]);
            }
            derivatives.push(new_derivative);
        }
        sum += derivatives
            .iter()
            .rev()
            .skip(1)
            .fold(0, |acc, x| -acc + x[0]);
    }
    sum
}

fn main() -> Result<(), std::io::Error> {
    let data: Vec<Vec<i64>> = load_data("input.txt")?;
    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
    Ok(())
}
