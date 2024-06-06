// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}

///explication
/// Le problème est que les fonctions de test ne sont pas implémentées correctement. Les macros assert! sont utilisées pour affirmer que des conditions sont vraies, mais il n'est pas clair quelle condition doit être affirmée.
/// j'ai juse fournie les affirmations correctes.Dans is_true_when_even, nous affirmons que is_even(4) renvoie true, car 4 est un nombre pair. Dans is_false_when_odd, nous affirmons que is_even(7) renvoie false, car 5 est un nombre impair. 
/*Nous avons choisi cette solution parce qu'elle teste correctement la fonction is_even avec des valeurs d'entrée paires et impaires. En affirmant les valeurs de retour attendues, nous nous assurons que la fonction se comporte comme attendu.

En particulier, nous utilisons les macros assert! pour affirmer que la fonction renvoie les valeurs attendues. Le symbole ! est utilisé pour nier le résultat de is_even(5) dans le deuxième test, car nous attendons que la fonction renvoie false pour une entrée impaire.

 */