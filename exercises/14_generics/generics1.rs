// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}

/* fn main() {
    let mut shopping_list: Vec<String> = Vec::new();
    shopping_list.push("milk".to_string());
}*/

///explication 
///Le problème avec ce code est que le type des éléments du vecteur n'est pas spécifié. ici c'est un car nous savons que nous allons stocker des chaînes de caractères (comme milk ype str) dans le vecteur. j'ai choisie fait les deux str et string j'ai commenter celui avec string 
/// pour résoudre j'ai remplacé Vec<?> par Vec<String> sans oublier le .to_string pour convertir milk en string ou  le type str vec<&Str>