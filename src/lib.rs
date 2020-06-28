use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct MealInfo {
    pub meal_name: String,
    pub line_number: usize,
    pub total_lines: usize,
}

pub fn suggest_meal() -> Result<MealInfo, Box<dyn Error>> {
    let lines = read_lines("./meals.txt")?;

    let random_index = rand::thread_rng().gen_range(0, lines.len());
    let line = lines.get(random_index)
        .expect("index out of bounds - wtf how?")
        .to_string();

    Ok(MealInfo {
        meal_name: line,
        line_number: random_index + 1,
        total_lines: lines.len(),
    })
}

fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut lines = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line?);
    }

    Ok(lines)
}