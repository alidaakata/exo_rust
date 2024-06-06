// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
fn main() {
    call_me(7);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

//explication
//la fonction call me est appelée dans la fonction main sans fournir le numéro d'argument requis. j'ai juste passé un argument non signee à la fonction call me
