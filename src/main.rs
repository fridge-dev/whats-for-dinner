use std::error::Error;
use whats_for_dinner::{suggest_meal, MealInfo};

fn main() -> Result<(), Box<dyn Error>> {
    let meal = suggest_meal()?;
    display(meal);

    Ok(())
}

fn display(meal: MealInfo) {
    println!();
    println!(
        "[Meal {}/{}] {}",
        meal.line_number,
        meal.total_lines,
        meal.meal_name,
    );
    println!();
}
