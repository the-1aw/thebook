use std::collections::HashMap;

const BLUE_TEAM: &str = "Blue";
const YELLOW_TEAM: &str = "Yellow";
const GREEN_TEAM: &str = "Green";

fn main() {
    let mut scores = HashMap::new();
    scores.insert(BLUE_TEAM, 10);
    scores.insert(YELLOW_TEAM, 50);
    let blue_score = scores.get(BLUE_TEAM).copied().unwrap_or(0);
    println!("Blue team score is: {blue_score}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Ovewrite on insert
    scores.insert(BLUE_TEAM, 20);
    scores.insert(BLUE_TEAM, 25);

    // Insert if new
    scores.entry(BLUE_TEAM).or_insert(300);
    let blue_score = scores.get(BLUE_TEAM).copied().unwrap_or(0);
    scores.entry(GREEN_TEAM).or_insert(300);
    let green_score = scores.get(GREEN_TEAM).copied().unwrap_or(0);
    println!("Blue {blue_score} - {green_score} Green");
    let green_score_entry = scores.entry(GREEN_TEAM).or_insert(0);
    *green_score_entry *= 12;
    println!("{green_score_entry}");
}
