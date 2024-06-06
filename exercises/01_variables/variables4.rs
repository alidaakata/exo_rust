// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//Correction
fn main() {
    let mut x = 3;  // X a été déclaré comme modifiable avec mut
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

//explication
//  La variable x est déclarée comme immutable avec la syntaxe let x = 3;. Ensuite, on essayait de modifier la valeur de x avec la ligne x = 5 or que la valeur ne peut pas être modifiée après la déclaration. du coup j'ai juste ajouter mut pour que la variable x soit modifiable 