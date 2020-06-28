use whats_for_dinner::{suggest_meal, MealInfo};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let meal = suggest_meal()?;
    display(meal);

    Ok(())
}

fn display(meal: MealInfo) {
    println!();
    println!("[Meal {}/{}]", meal.line_number, meal.total_lines);
    println!("{}", meal.meal_name);
    println!();
}
