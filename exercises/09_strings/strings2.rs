// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction
fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

//explication
//Le problème est que la fonction is_a_color_word attendait un argument de type &str, mais que nous passions un argument de type String à la fonction main. Pour résoudre ce problème, nous devions modifier la façon dont nous appelons la fonction is_a_color_word en passant une référence à la chaîne de caractères word au lieu de la chaîne de caractères elle-même.
//Pour résoudre le problème, j'ai passé une référence à la chaîne de caractères word au lieu de la chaîne de caractères elle-même. En ajoutant l'opérateur & avant word dans l'appel de la fonction is_a_color_word, on créé une référence à la chaîne de caractères word qui est compatible avec le type &str attendu par la fonction is_a_color_word.