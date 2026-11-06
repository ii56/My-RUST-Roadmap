// Simple player menu in a Game

const GAME_TITLE: &str = "Rust Quest";
const MAX_HEALTH: u32 = 100;

fn main() {
    let player_name: &str = "Sonquick";
    let player_title: &str = "Paladin";
    let mut health: u32 = MAX_HEALTH;
    health = 70; // Player took damage, health reduced to 70

    let mana: u32 = 50;
    let level: u32 = 3;
    let mut score: u32 = 1500;
    score = score + 50; // Player gained points, score increased to 2000

    println!("Welcome to {}!", GAME_TITLE);
    println!("=============================");
    println!("Player Name: {}", player_name);
    println!("Player Title: {}", player_title);
    println!("Health: {}", health);
    println!("Mana: {}", mana);
    println!("Level: {}", level);
    println!("Score: {}", score);
}