// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// I AM NOT DONE

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object


impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts[0];
        let age_str = parts[1];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = age_str.parse::<usize>().map_err(|e| ParsePersonError::ParseInt(e))?;
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}
fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
///explication
/// Le problème réside dans l'implémentation incomplète de la méthode `from_str` pour la structure `Person`. Cette méthode doit être complétée pour pouvoir convertir une chaîne de caractères en une instance de `Person`. De plus, des erreurs doivent être retournées en cas de conditions invalides, telles qu'une chaîne vide, un nombre incorrect de champs, un champ de nom vide ou une erreur de conversion lors du parsing de l'âge.

/// Pour résoudre ce problème, nous avons compléter l'implémentation de la méthode `from_str` pour la structure `Person`. Dans cette implémentation, nous vérifions les différentes conditions mentionnées ci-dessus et retournons une erreur appropriée si l'une d'entre elles est détectée. Sinon, nous extrayons le nom et l'âge de la chaîne fournie et tentons de les convertir en une instance de `Person`. Si une erreur survient pendant le processus de conversion, nous la renvoyons également.
/// Nous avons choisie cette solution car elle complète de manière adéquate l'implémentation de `from_str` en vérifiant toutes les conditions requises et en retournant des erreurs appropriées le cas échéant. De plus, elle utilise le trait `FromStr` de manière appropriée pour faciliter la conversion d'une chaîne en une instance de `Person`.