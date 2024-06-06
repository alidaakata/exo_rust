// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    if let Some(val) = arg.as_mut().as_mut() {
        *val *= *val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
///explication
///Le problème se trouve dans l'utilisation de références et de pointeurs dans les fonctions `byte_counter`, `char_counter` et `num_sq`. Pour résoudre ce problème, nous devons ajouter le trait `AsRef` ou `AsMut` comme contrainte de trait pour les types génériques utilisés dans ces fonctions.
/// Pour byte_counter et char_counter, nous avons utiliser AsRef pour les références et les types de chaînes, car nous voulons pouvoir traiter les chaînes de caractères et les références de manière uniforme.
/// Pour num_sq, nous avons utilisons AsMut pour que la fonction puisse modifier le contenu de la référence de manière sûre.
/// Nous avons choisie  ces solutions car elles permettent une utilisation polyvalente des types de données, en autorisant les références et les types de chaînes, tout en assurant la sécurité de la mutation avec AsMut.




