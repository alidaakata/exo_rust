// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    let y_clone = y.clone();

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y_clone; // Fix without deleting this line.
}

///explication
///J'ai ajouté l'attribut #[derive(Clone)] à la structure Point parce que la méthode clone n'est pas disponible par défaut pour les structures personnalisées dans Rust
//En ajoutant l'attribut #[derive(Clone)] à la structure Point, je dis à Rust d'implémenter automatiquement le trait Clone pour la structure Point.

///explication
/// La ligne y; à la fin du code cause probleme car elle essaie d'utiliser la valeur de y après que le bloc match l'a consommée.
///En Rust, les valeurs sont déplacées lorsqu'elles sont utilisées dans un bloc match. Cela signifie que la valeur de y n'est plus disponible après le bloc match.
///pour resoudre le problème , j'ai cloner la valeur de y avant de lutiliser dans match . j'ai use plus precisement la methode clone pour creer une copie 
