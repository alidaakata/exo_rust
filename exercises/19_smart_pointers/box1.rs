// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}

//explication
//le probleme est que Le type `List` est défini de manière récursive, chaque élément contenant un pointeur vers le suivant. Cela pose un problème pour le compilateur Rust, car il ne peut pas déterminer la taille de chaque élément à l'avance, puisque la taille dépend du prochain élément.
/* step 1: nous avons utiliser un Box, qui est un pointeur intelligent qui stocke ses données sur la heap au lieu de la stack. Cela nous permet de définir le type List de manière récursive sans que le compilateur Rust ne sorte des erreurs
J'ai choisi d'utiliser `Box` car ils stockent les données sur la heap, résolvant ainsi le problème de récursivité. De plus, étant des pointeurs intelligents, ils gèrent automatiquement la mémoire, éliminant le besoin de gestion manuelle.*/
/* step2: Maintenant que nous avons défini le type List de manière correcte, nous avons implémenter les fonctions create_empty_list et create_non_empty_list.
J'ai choisi cette solution pour créer des listes vides et non vides répondant aux tests. `create_empty_list` retourne `List::Nil`, représentant une liste vide. `create_non_empty_list` crée une liste non vide avec deux éléments: le premier a la valeur 1 et pointe vers le deuxième, qui a la valeur 2 et pointe vers `List::Nil`.*/