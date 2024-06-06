// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

//correction
fn main() {
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */ (name, age) = cat;

    println!("{} is {} years old.", name, age);
}

//explication
//Le problème avec ce code est que les variables name et age ne sont pas définies avant d'être utilisées dans l'instruction println!
//j'ai deifini les variables nom et âge et suis assigné à cat