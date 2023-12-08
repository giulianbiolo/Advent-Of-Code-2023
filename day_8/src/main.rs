use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Map<'a> {
    instructions: Vec<char>,
    nodes: HashMap::<&'a str, (&'a str, &'a str)>,
}
impl<'a> Map<'a> {
    fn new<'b>(instructions: Vec<char>, nodes: HashMap<&'b str, (&'b str, &'b str)>) -> Map<'a> 
    where 'b: 'a,
    { Map { instructions, nodes } }
}
fn load_data<'a>(file: &'a str) -> Result<Map<'a>, std::io::Error> {
    let mut lines = file
        .lines()
        .filter(|line| !line.is_empty());
    let instructions = lines
        .next()
        .expect("Error reading 'instructions' field in file!")
        .chars()
        .collect::<Vec<char>>();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::<&str, (&str, &str)>::new();
    for l in lines.clone() {
        let nodeval_children: Vec<&str> = l.split(" = (").collect::<Vec<&str>>();
        let nodeval: &str = nodeval_children[0];
        let children: Vec<&str> = nodeval_children[1].split(", ").collect::<Vec<&str>>();
        let (left, right): (&str, &str) = (children[0].trim(), children[1].trim_end_matches(")").trim());
        nodes.insert(nodeval, (left, right));
    }
    Ok(Map::new(instructions, nodes))
}

fn travel(map: &Map, current_node: &(&str, &(&str, &str)), end: &str) -> u128 {
    let mut travelled: u128 = 0;
    let mut current_node: (&str, &(&str, &str)) = current_node.clone();
    loop {
        if current_node.0.ends_with(end) { break; }
        if map.instructions[travelled as usize % map.instructions.len()] == 'L' { current_node = (current_node.1.0, map.nodes.get(&current_node.1.0).unwrap()); }
        else if map.instructions[travelled as usize % map.instructions.len()] == 'R' { current_node = (current_node.1.1, map.nodes.get(&current_node.1.1).unwrap()); }
        else { panic!("Invalid instruction: {}", map.instructions[travelled as usize % map.instructions.len()]); }
        travelled += 1;
    }
    travelled
}

fn part_one(map: &Map) -> u128 { travel(map, &("AAA", map.nodes.get("AAA").unwrap()), "ZZZ") }

fn part_two(map: &Map) -> u128 {
    let current_nodes: Vec<(&str, &(&str, &str))> = map.nodes
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .map(|(k, v)| (*k, v))
        .collect::<Vec<(&str, &(&str, &str))>>();
    let mut travelled: Vec<u128> = vec![0; current_nodes.len()];
    for (idx, (k, v)) in current_nodes.iter().enumerate() { travelled[idx] = travel(map, &(*k, *v), "Z"); }
    travelled.iter().fold(1, |acc, &x| num::integer::lcm(acc, x))
}

fn main() -> Result<(), std::io::Error> {
    let file: String = std::fs::read_to_string("input.txt")?;
    let map: Map = load_data(&file)?;
    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
    Ok(())
}
