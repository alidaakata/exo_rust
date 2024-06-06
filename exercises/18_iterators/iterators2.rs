// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), c.as_str()),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in words {
        result.push(capitalize_first(word));
    }
    result
}
// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut result: String = String::new();
    for word in words {
        result.push_str(&capitalize_first(word));
        result.push(' ');
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
fn main (){}
///explication
/// Le code contient des parties manquantes, représentées par des ??? dans la fonction capitalize_first et des tableaux vides dans les fonctions capitalize_words_vector et capitalize_words_string.
/*step1: La solution utilise l'itérateur chars pour parcourir les caractères de la chaîne d'entrée. Si la chaîne est vide, la fonction retourne une chaîne vide. Sinon, elle prend le premier caractère, le met en majuscule avec to_uppercase, et le concatène avec le reste de la chaîne d'entrée.
 j'ai choisi cette solution car elle est efficace et facile à comprendre. L'utilisation de l'itérateur chars permet de parcourir les caractères de la chaîne d'entrée de manière efficace, et la méthode to_uppercase permet de mettre le premier caractère en majuscule de manière simple.*/

 /*step2: La solution utilise une boucle for pour parcourir le tableau de chaînes de caractères. Pour chaque chaîne, elle applique la fonction capitalize_first pour capitaliser le premier caractère, puis ajoute le résultat à un tableau de chaînes de caractères.
J'ai choisi cette solution car elle est facile à comprendre et à implémenter. La boucle for permite de parcourir le tableau de chaînes de caractères de manière efficace, et la fonction capitalize_first est appliquée à chaque chaîne pour capitaliser le premier caractère.*/

/*step3: La solution utilise une boucle for pour parcourir le tableau de chaînes de caractères. Pour chaque chaîne, elle applique la fonction capitalize_first pour capitaliser le premier caractère, puis ajoute le résultat à une chaîne de caractères. Enfin, elle utilise la méthode trim pour supprimer les espaces en fin de chaîne et to_string pour retourner une chaîne de caractères.
J'ai choisi cette solution car elle est facile à comprendre et à implémenter. La boucle for permite de parcourir le tableau de chaînes de caractères de manière efficace, et la fonction capitalize_first est appliquée à chaque chaîne pour capitaliser le premier caractère. La méthode trim permet de supprimer les espaces en fin de chaîne et to_string permet de retourner une chaîne de caractères.*/
