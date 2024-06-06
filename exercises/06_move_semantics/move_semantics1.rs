// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

//correction

#[test]
fn main() {
    let mut vec0 = vec![22, 44, 66];

    fill_vec(&mut vec0);

    assert_eq!(vec0, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

/// Explication
/// Le problème observé est que lorsque la fonction fill_vec est appelée, elle prend possession du vecteur vec0, ce qui a pour conséquence de le vider, et ainsi nous perdons la valeur initiale du vecteur.
/// Afin de remédier à cette situation, j'ai ajusté la fonction fill_vec pour qu'elle accepte désormais une référence au vecteur initial plutôt que de prendre possession de ce vecteur. De cette manière, la fonction peut modifier le contenu du vecteur sans en perdre la valeur initiale.