// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE
//correction
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}


/// Explication
/// Afin d'éviter que la fonction get_char ne prenne possession des éléments de la chaîne, nous ajoutons le symbole & devant string, créant ainsi une référence à la chaîne plutôt que de la passer par valeur.
/// En revanche, pour permettre à get_char de prendre possession de la chaîne et de ses éléments, nous retirons simplement le symbole &, annulant ainsi la référenciation et transmettant la chaîne par valeur à la fonction.
