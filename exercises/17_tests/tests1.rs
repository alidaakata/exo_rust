// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true); //or false
    }
}
///explication
/// Le problème avec ce code est que la macro assert! nécessite une expression booléenne comme argument, mais dans ce cas, elle est appelée sans argument
/// il suffit d'ajouter une expression booléenne à la macro assert!
/// J'ai choisi d'ajouter la littérale booléenne true comme argument à la macro assert! parce que c'est une façon simple et directe de faire compiler le test.