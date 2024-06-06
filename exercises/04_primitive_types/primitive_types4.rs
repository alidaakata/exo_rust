#[test]

//correction
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}

fn main (){}
//explication
//j'ai remplacee les ? par &a[1..4]. Cela créé un extrait qui pointe vers les éléments du tableau a à partir de l'index 1 (inclus) jusqu'à l'index 4 (exclus).
//En Rust, la syntaxe &a[ start..end ] pour l'extraction