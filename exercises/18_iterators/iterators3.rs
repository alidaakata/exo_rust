// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        /// Si b est égal à 0, nous retournons une erreur de division par zéro
        Err(DivisionError::DivideByZero)
    } else if a % b == 0 {
        /// Si a est divisible par b, nous retournons le résultat de la division
        Ok(a / b)
    } else {
        /// Si a n'est pas divisible par b, nous retournons une erreur de division non divisible
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }))
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
///explication
/// Le problème est que les fonctions divide, result_with_list et list_of_results ne sont pas implémentées correctement. La fonction divide doit être complétée pour gérer les cas de division possible et impossible, tandis que les fonctions result_with_list et list_of_results doivent être modifiées pour retourner les résultats attendus.
/// step1: j'ai geree les trois cas possible si b=o retourner une erreur de division par zero, si a est divisible par b retourner le resultat de division , si a n'est pas divisible par b retourner une erreur de division. 
/*step2: nous avons utiliser la methode map pour appliquer la fonction divide à chaque élément de la liste numbers, puis utiliser la méthode collect pour collecter les résultats dans un vecteur.
J'ai choisi cette solution car elle utilise les méthodes map et collect pour traiter la liste de nombres de manière efficace et concise. */
/*step3: nous avons utiliser la méthode map pour appliquer la fonction divide à chaque élément de la liste numbers, puis utiliser la méthode collect pour collecter les résultats dans un vecteur.
J'ai choisi cette solution car elle utilise les méthodes map et collect pour traiter la liste de nombres de manière efficace et concise.*/