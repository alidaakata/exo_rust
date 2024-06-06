// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
//correction
#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();  // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
///explication
/// Le problème est que les ??? dans le code doivent être remplacés par des valeurs appropriées pour que le code compile et passe les tests.
/// step1: nous créons un itérateur my_iterable_fav_fruits à partir du vecteur my_fav_fruits en utilisant la méthode iter(). Cela nous permet de parcourir les éléments du vecteur.cette solution permet de créer un itérateur qui peut parcourir les éléments du vecteur
/// step2: j'ai remplacer le ? par la valeur attendue pour le deuxieme element de l'iterateur qui est custard apple 
/// step3: j'ai remplacer le ? par la valeur attendue pour le quatrieme element de l'iterateur qui est peach . je l'ai choisie car elle correspond au 4 eme element dans my_fav_fruits
/// step4: j'ai remplacer le ? par la valeur attendue pour le quatrieme element de l'iterateur qui est none . car il n'y a plus d'éléments dans l'itérateu, l'iterateur est epuisé