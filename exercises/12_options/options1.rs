// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 scoops left. At 10PM, someone eats it
// all, so there'll be no more left :(

    //correction
    fn main(){}
fn maybe_icecream(time_of_day: u16) -> Option<u16> {

    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0. The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day < 22 {
        Some(5)
    } else if time_of_day <= 23 {
        Some(0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        if let Some(value) = icecreams {
            assert_eq!(value, 5);
        } else {
            assert!(false);
        }
    }
}
///explication 
///J'ai corrigé le code en ajoutant les conditions pour déterminer le nombre de glaces restantes en fonction de l'heure du jour. Si l'heure est inférieure à 22h, il reste 5 boules. Si l'heure est égale à 22h ou 23h, il n'y a plus de glaces. Si l'heure est supérieure à 23h, la fonction retourne None car l'heure est invalide.
///Dans le test raw_value, j'ai ajouté un if let pour extraire la valeur contenue dans l'Option retournée par la fonction maybe_icecream. Si la valeur est présente, je la compare à 5. Si la valeur est absente, je lance une erreur avec assert!(false).