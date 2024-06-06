// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

// correction
#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

///Explication
///L'erreur survient parce qu'il est interdit d'avoir plusieurs références mutables pointant vers la même variable simultanément.
///Ainsi, pour remédier à cela, nous avons supprimé les références mutables à partir de y et z, levant ainsi la référenciation mutable sur la variable concernée.