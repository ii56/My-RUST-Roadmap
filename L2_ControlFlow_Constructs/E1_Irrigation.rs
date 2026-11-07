// Program for farmer to control Irrigation.

use std::io;

const DRY_LIMIT: u8 = 30;
const WET_LIMIT: u8 = 60;

fn main() {
    loop {
        println!("Enter the soil moisture value: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let moisture: u8 = input.trim().parse().expect("Invalid input");

        println!("Enter the Rain guage level: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let rain_level: u8 = input2.trim().parse().expect("Invalid input");

        if rain_level > 0 {
            println!("Rain detected. No irrigation needed.");
        } else if moisture < DRY_LIMIT {
            println!("Soil is dry. Irrigation needed.");
        } else if moisture > WET_LIMIT {
            println!("Soil is wet. No irrigation needed.");
        } else {
            println!("Soil moisture is within the optimal range. No irrigation needed.");
        }
    }
}