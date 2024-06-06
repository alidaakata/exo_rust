
// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
//correction
fn main (){}
pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}

//explication
//Le problème avec le code est que la variable identifiant a un nombre à virgule 2.0 lorsque animal est "gopher" et unknown au niveau de else ce qui n'est pas compatible avec les autres valeurs attribuées à identifiant. cela retourne une erreur de type > 
//Pour corriger le code on attribue une valeur unique (entière)  à identifiant pour que ça soit cohérent et identique partout