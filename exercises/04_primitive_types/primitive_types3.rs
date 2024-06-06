// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE
//correction

fn main() {
    let a = [0; 100];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}

//explication
//Le problème est que le code ne spécifie pas le type et la valeur de la variable a. Pour résoudre ce problème, nous devons définir un tableau avec au moins 100 éléments.
//j'ai défini un tableau a avec 100 éléments initier à 0 le code vérifie si la longueur du tableau est supérieure ou égale à 100 et affiche le message approprié.