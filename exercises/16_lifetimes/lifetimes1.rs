// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE


///correction
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("La chaîne la plus longue est '{}'", result);
}

///probleme 
///Le problème avec le code est que les références renvoyées par la fonction longest ne sont pas liées à une durée de vie spécifique

///explication
///J'ai ajouté une annotation de durée de vie 'a à la fonction longest. Cela signifie que les références x et y ont la même durée de vie, qui est liée à la durée de vie de la fonction longest.
///J'ai également ajouté la même annotation de durée de vie à la valeur de retour de la fonction longest. Cela signifie que la référence renvoyée par la fonction a la même durée de vie que les références x et y
