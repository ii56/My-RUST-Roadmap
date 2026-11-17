// Create a mutable variable cells starting at 1, double it three times with *= , and print it.

fn main() {
    let mut cell = 1;

    cell *= 3;

    println!("Now there are {} cells in the petri dish.", cell);
}