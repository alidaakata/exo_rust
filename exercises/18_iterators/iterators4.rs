// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
fn main(){}
///explication
/// Le problème est que la fonction factorial n'est pas implémentée
/*J'ai choisi d'utiliser les itérateurs pour résoudre ce problème. L'itérateur (1..=num) génère une séquence d'entiers allant de 1 à num inclus. La méthode fold permet de réduire cette séquence en un seul élément en appliquant une fonction de accumulation.
Dans ce cas, la fonction de accumulation est |acc, x| acc * x, qui multiplie l'accumulateur acc par l'élément courant x. L'accumulateur initial est défini à 1.*/
///J'ai choisi cette solution parce qu'elle est concise, efficace et respecte les contraintes imposées. Les itérateurs sont une façon élégante de traiter les séquences en Rust, et la méthode fold est particulièrement utile pour réduire une séquence en un seul élément