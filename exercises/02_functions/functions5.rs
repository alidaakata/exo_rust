// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction 
fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}
//explication
// la fonction square ne renvoie pas de valeur . or que toutes les fonctions doivent renvoyer une valeur.  sauf si elles sont marquées comme () ne renvoient rien. 
// j'ai ajouté le mot-clé return avant num * pour renvoyer la valeur calculée.