// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

//correction
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(88);

    vec
}

/// Explication
/// Le problème rencontré provient du fait que lors du transfert des valeurs de vec0 vers la fonction fill_vec, cela entraîne la vidange de vec0, le laissant ainsi vide.
/// Afin de remédier à cette situation, nous modifions la fonction fill_vec pour qu'elle prenne une référence sur le vecteur vec0 sans en prendre possession. Cela permet à fill_vec de manipuler le contenu de vec0 sans le vider.




