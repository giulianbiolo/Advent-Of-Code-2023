use std::{fs::File, io::{Read, Error}};


fn load_grid() -> Result<Vec<Vec<char>>, Error> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    let mut line: String = String::new();
    let mut file: File = File::open("input.txt")?;
    file.read_to_string(&mut line)?;
    for c in line.chars() {
        if c == '\n' {
            grid.push(row);
            row = Vec::new();
        }
        if c == '\r' || c == '\n' { continue; }
        row.push(c);
    }
    grid.push(row);
    Ok(grid)
}

fn find_numbers(grid: &Vec<Vec<char>>) -> Vec<(usize, usize, usize)> {
    let mut numbers: Vec<(usize, usize, usize)> = Vec::new(); // (row_start, col_start, length)
    for (i, row) in grid.iter().enumerate() {
        let mut j: usize = 0;
        while j < row.len() {
            if row[j].is_digit(10) {
                let mut length: usize = 1;
                while j + length < row.len() && row[j + length].is_digit(10) { length += 1; }
                numbers.push((i, j, length));
                j += length;
            } else { j += 1; }
        }
    }
    numbers
}

fn get_symbol_neighbours(grid: &Vec<Vec<char>>, number: (usize, usize, usize)) -> Vec<(usize, usize)> {
    let (row_start, col_start, length) = number;
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    // first we check all the cells above and below the number
    for i in col_start..col_start + length {
        if row_start > 0 && grid[row_start - 1][i] != '.' { neighbours.push((row_start - 1, i)); }
        if row_start + 1 < grid.len() && grid[row_start + 1][i] != '.' { neighbours.push((row_start + 1, i)); }
    }
    // then we check all the cells to the left and right of the number
    if col_start > 0 && grid[row_start][col_start - 1] != '.' { neighbours.push((row_start, col_start - 1)); }
    if col_start + length < grid[row_start].len() && grid[row_start][col_start + length] != '.' { neighbours.push((row_start, col_start + length)); }
    // finally we check the cells in the corners
    if row_start > 0 && col_start > 0 && grid[row_start - 1][col_start - 1] != '.' { neighbours.push((row_start - 1, col_start - 1)); }
    if row_start > 0 && col_start + length < grid[row_start].len() && grid[row_start - 1][col_start + length] != '.' { neighbours.push((row_start - 1, col_start + length)); }
    if row_start + 1 < grid.len() && col_start > 0 && grid[row_start + 1][col_start - 1] != '.' { neighbours.push((row_start + 1, col_start - 1)); }
    if row_start + 1 < grid.len() && col_start + length < grid[row_start].len() && grid[row_start + 1][col_start + length] != '.' { neighbours.push((row_start + 1, col_start + length)); }
    neighbours
}

fn is_part_number(grid: &Vec<Vec<char>>, neighbours: Vec<(usize, usize)>) -> bool {
    neighbours.iter().any(|&neighbour| !grid[neighbour.0][neighbour.1].is_digit(10))
}

fn get_number(grid: &Vec<Vec<char>>, number: (usize, usize, usize)) -> usize {
    let (row_start, col_start, length) = number;
    let mut number: usize = 0;
    for i in col_start..col_start + length {
        number *= 10;
        number += grid[row_start][i].to_digit(10).unwrap() as usize;
    }
    number
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let numbers: Vec<(usize, usize, usize)> = find_numbers(&grid);
    let neighbours: Vec<Vec<(usize, usize)>> = numbers.iter().map(|&number| get_symbol_neighbours(&grid, number)).collect::<Vec<_>>();
    let part_numbers: Vec<&(usize, usize, usize)> = numbers.iter().enumerate().filter(|(idx, _)| is_part_number(&grid, neighbours[*idx].clone())).map(|(_, number)| number).collect::<Vec<_>>();
    let total: usize = part_numbers.iter().map(|&number| get_number(&grid, *number)).sum::<usize>();
    total
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let numbers: Vec<(usize, usize, usize)> = find_numbers(&grid);
    let neighbours: Vec<Vec<(usize, usize)>> = numbers.iter().map(|&number| get_symbol_neighbours(&grid, number)).collect::<Vec<_>>();
    let part_numbers: Vec<&(usize, usize, usize)> = numbers.iter().enumerate().filter(|(idx, _)| is_part_number(&grid, neighbours[*idx].clone())).map(|(_, number)| number).collect::<Vec<_>>();
    let parts_neighbours: Vec<Vec<(usize, usize)>> = part_numbers.iter().map(|&number| get_symbol_neighbours(&grid, *number)).collect::<Vec<_>>();
    // Now we check every part number with every other part number to see if they share a neighbour == "*", if so we multiply them together
    let mut total: usize = 0;
    for i in 0..part_numbers.len() {
        for j in i + 1..part_numbers.len() {
            let mut shared_neighbours: Vec<(usize, usize)> = Vec::new();
            for neighbour in parts_neighbours[i].iter() {
                if parts_neighbours[j].contains(neighbour) { shared_neighbours.push(*neighbour); }
            }
            if shared_neighbours.iter().any(|&neighbour| grid[neighbour.0][neighbour.1] == '*') {
                total += get_number(&grid, *part_numbers[i]) * get_number(&grid, *part_numbers[j]);
            }
        }
    }
    total
}

fn main() -> Result<(), Error> {
    let grid: Vec<Vec<char>> = load_grid()?;
    let total: usize = part_one(&grid);
    println!("Part One: {}", total);
    let total: usize = part_two(&grid);
    println!("Part Two: {}", total);
    Ok(())
}
