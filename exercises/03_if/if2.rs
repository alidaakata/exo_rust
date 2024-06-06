// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

//correction
pub fn fizz_if_foo(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } 
    else if fizzish == "fuzz" {
        "bar"
    }
        else {
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}

//explication
//La fonction foo_if_fizz renvoie des types string . Dans la branche if, elle renvoie une chaîne de caractères foo, tandis que dans la branche else, elle renvoie un entier 1. Cette incohérence de types empêche la compilation du code.
//j'ai modifié else pour qu'elle renvoie également une chaîne de caractères. De cette façon, la fonction renvoie toujours un str ce qui est cohérent et permet la compilation.
//j'ai renvoyé une chaîne de caractères dans les deux cas. Selon les tests, lorsque fizzish n'est pas égal à "fizz", nous devrions renvoyer "bar" si c'est égal à "fuzz" et "baz" sinon.