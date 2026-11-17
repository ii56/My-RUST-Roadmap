// Create a project solar_facts that prints the names of the 8 planets, one per line, each right-aligned in 10 columns.

fn main() {
    let no_planets = 8;

    let planet1 = "Mercury";
    let planet2 = "Venus";
    let planet3 = "Mars";
    let planet4 = "Earth";
    let planet5 = "Jupider";
    let planet6 = "Saturn";
    let planet7 = "Uranus";
    let planet8 = "Neptune";
    
    println!("The solar system have a total of {} planets.", no_planets);

    println!("The planets are: ");
    println!("First planet: {:>5} | {}", planet1);
}