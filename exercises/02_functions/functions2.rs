// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
fn main() {
    call_me(3);
}

fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
//explication
// la fonction call me est mal definie .La fonction call me est censée prendre un argument num de type entier. Cependant, dans le code fourni, le type de num n'est pas spécifié, ce qui est une erreur . j'ai juste précisé le type de la variable num pendant la declaration de la fonction call me 