// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers.clone());
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
///explication
/// Le premier TODO est à remplacer par une valeur qui permet de partager le vecteur numbers entre les threads. Le deuxième TODO est à remplacer par une initialisation de la variable child_numbers qui permettra à chaque thread d'accéder au vecteur partagé.
/*  todo1 : Pour partager le vecteur numbers entre les threads, nous avons utiliser un Arc qui est un type de Rust qui permet de partager des données entre les threads de manière thread-safe.
Nous utilisons Arc::new() pour créer un nouvel Arc qui pointe vers une copie du vecteur numbers afin de conserver la propriété du vecteur original. Cela permet de partager une copie entre les threads sans affecter le vecteur original. L'Arc garantit la sécurité des données partagées entre les threads en empêchant les modifications concurrentes. La méthode clone() est utilisée pour créer une copie du vecteur car nous ne voulons pas donner la propriété du vecteur à l'Arc*/
/* todo2 : Pour que chaque thread puisse accéder au vecteur partagé, nous avons initialiser la variable child_numbers avec une référence à l'Arc créé précédemment. 
Nous utilisons `Arc::clone()` pour permettre à chaque thread d'accéder de manière thread-safe au même vecteur partagé. Cela crée une nouvelle référence à l'Arc existant sans copier le vecteur, ce qui est plus efficace en termes de mémoire et de gestion que de créer des copies pour chaque thread.*/