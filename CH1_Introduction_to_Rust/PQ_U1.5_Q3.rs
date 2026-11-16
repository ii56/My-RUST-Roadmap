// Print a small table with the element names right-aligned in 10 columns and their symbols after a | , for hydrogen (H) and oxygen (O).

fn main() {
    let atom1  = "Hydrogen";
    let atom1_symb = "H";

    let atom2 = "Oxygen";
    let atom2_symb = "O";

    println!("{:>10} | {}", atom1, atom1_symb);
    println!("{:>10} | {}", atom2, atom2_symb);
}