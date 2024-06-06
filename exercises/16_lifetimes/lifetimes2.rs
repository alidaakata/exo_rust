
// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz"); // Déclarer string2 avant result
    let result = longest(string1.as_str(), string2.as_str());
    println!("La chaîne la plus longue est '{}'", result);
}

///explication
///Le problème avec le code original est que string2 est libérée à la fin de la portée interne, mais result conserve toujours une référence à elle. C'est un problème car result est utilisé après que string2 est libérée, ce qui entraînerait une référence pendante.
///En déclarant string2 avant result, nous nous assurons que string2 vit au moins aussi longtemps que result. C'est parce que string2 est maintenant dans la même portée que result, donc elle ne sera pas libérée jusqu'à la fin de la fonction main.
///J'ai corrigé le code en déclarant string2 avant la déclaration de result. Cela est nécessaire car string2 est utilisé pour calculer la valeur de result, et nous devons nous assurer que string2 est toujours valide lorsque nous utilisons result.