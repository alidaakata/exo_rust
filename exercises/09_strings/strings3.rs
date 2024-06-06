
// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE


//explication
//juste le main pour tester
fn main() {
    let espace = trim_me("   Bonjour!   ");
    let composed = compose_me("Bonjour");
    let replaced = replace_me("Je pense que les voitures sont cool");

    println!("space : {}", espace);
    println!("Composé : {}", composed);
    println!("Remplacé : {}", replaced);
}


//correction

fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!
    input.trim().to_string()
}

//explication
//La fonction trim_me utilise la méthode trim pour supprimer les espaces blancs à la fin et au début de la chaîne de caractères. La méthode trim renvoie une slice (&str) qui pointe vers la partie de la chaîne de caractères sans les espaces blancs. Pour convertir ce résultat en une valeur de type String, nous utilisons la méthode to_string.

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There are multiple ways to do this!
    format!("{} world!", input)
}

//explication
//nous utilisons format! pour créer une nouvelle chaîne de caractères avec le contenu de l'input et le suffixe " world!. format! est une manière concise et lisible de créer des chaînes de caractères en combinant des valeurs


fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

//explication
// La méthode replace renvoie une nouvelle chaîne de caractères avec les occurrences de "cars" remplacées par "balloons".

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
