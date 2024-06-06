// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

///explication
///Le problème était que la struct Book détenait des références à des chaînes de caractères (&str) mais ne spécifiait pas la durée de vie de ces références.
///J'ai corrigé le code en ajoutant un paramètre de durée de vie 'a à la struct Book. C'est nécessaire car les champs author et title de la struct Book détiennent des références à des chaînes de caractères, et Rust doit savoir combien de temps ces références sont valides.
///En ajoutant le paramètre de durée de vie 'a à la struct Book, nous informons Rust que les références détenues par les champs author et title sont valides pendant au moins autant de temps que la durée de vie 'a. Cela permet à Rust de s'assurer que les références sont valides lorsque nous les utilisons dans la fonction main.