// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2; 
    }
    v
}


fn vec_map(v: &Vec<i32>) -> Vec<i32> {
     // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
    v.iter().map(|&element| element * 2).collect() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
///explication 
///Le problème réside dans le fait que les TODOs dans les fonctions `vec_loop` et `vec_map` ne sont pas implémentés, ce qui entraîne une erreur de compilation.
/// Pour résoudre ce problème, nous avons remplacer les `???` par le code nécessaire pour multiplier chaque élément de la Vec par 2.
/// Dans la fonction `vec_loop`, nous avons utiliser `*element *= 2` pour multiplier chaque élément mutatif de la Vec par 2.
/// Dans la fonction `vec_map`, nous avons simplement retourner `element * 2` à partir de la fermeture passée à la méthode `map`.
/// Nous avons cette solution car elle est simple et directe. Dans la fonction `vec_loop`, nous modifions mutativement chaque élément de la Vec en le multipliant par 2. Dans la fonction `vec_map`, nous utilisons la méthode `map` pour appliquer la multiplication à chaque élément de la Vec sans modifier la Vec d'origine.