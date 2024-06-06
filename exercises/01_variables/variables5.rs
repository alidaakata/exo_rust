// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number : i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
//explication
// Le problème avec ce code est que la variable number se voit d'abord attribuer une valeur de type string, puis une valeur entière.
//Pour résoudre ce problème, j'ai préciser le type entier  i32 pour quelle soit spécifier explicitement 