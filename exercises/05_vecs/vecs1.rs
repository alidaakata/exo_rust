// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

//correction
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v =  vec![10, 20, 30, 40];  // TODO: declare your vector here with the macro for vectors

    (a, v)
}

fn main (){
    array_and_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

//explication
//j'ai remplacé le commentaire TODO par la déclaration du vecteur v en utilisant la macro vec! et j'ai rajouté la fonction main pour tester .
