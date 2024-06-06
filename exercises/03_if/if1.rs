// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
//corection

fn main() {
    
    println!("Le plus grand nombre est {}",bigger(20, 40));
}
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a >= b {
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

//explication
//Le code est incomplet et la fonction main est manquante. La fonction bigger est définie, mais elle n'a pas d'implémentation. La fonction devrait retourner le plus grand des deux nombres i32, mais elle ne contient pas l'intruction. En Rust, chaque programme exécutable doit avoir une fonction main, qui est le point d'entrée du programme
//pour résoudre on met la condition if : le code retourne à si à est supérieur ou égal à b . ou b si b est supérieur ou égal à a
//la fonction main appelle la fonction bigger avec les arguments 20 et 40, puis imprime le résultat à l'aide de println!