// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from ("blue")
}

//explication
//La fonction current_favorite_color est censée renvoyer une chaîne de caractères, mais elle renvoie une chaîne de caractères littérale blue de type &str et nom string 
//j'ai utilisé la fonction string: : from pour creer une chaine de caractère à partir de la chaine de caractères littéraux c’est-à-dire une instance de string à partir de chaine de caractère littéral