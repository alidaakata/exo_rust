// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(3+3, 6);
    }
}

///explication
/// Le problème avec ce code est que la macro assert_eq! est appelée sans arguments, ce qui ne permet pas de vérifier si deux valeurs sont égales.
///  nous devons fournir deux arguments à la macro assert_eq! : les deux valeurs que nous voulons comparer.
/// nous appelons la macro assert_eq! avec deux arguments : 2 + 2 et 4. La macro vérifie si les deux expressions sont égales, et si c'est le cas, le test passe.