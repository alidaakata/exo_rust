// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

//correction

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
/* explication 
"blue" est une chaîne de caractères littérale, donc de type &str, donc on appelle string_slice.
"red".to_string() est une chaîne de caractères convertie en String, donc on appelle string.
String::from("hi") est une chaîne de caractères créée à partir d'une chaîne littérale, donc on appelle string.
"rust is fun!".to_owned() est une chaîne de caractères littérale convertie en String, donc on appelle string.
"nice weather".into() est une chaîne de caractères littérale convertie en String, donc on appelle string.
format! est une chaîne de caractères créée à l'aide de la macro format!, qui renvoie un String, donc on appelle string.
&String::from("abc")[0..1] est un slice de chaîne de caractères extrait d'un String, donc on appelle string_slice.
" hello there ".trim() est une chaîne de caractères littérale dont les espaces sont supprimés, donc on appelle string_slice.
"Happy Monday!".to_string().replace("Mon", "Tues") est une chaîne de caractères littérale convertie en String, puis modifiée, donc on appelle string.
"mY sHiFt KeY iS sTiCkY".to_lowercase() est une chaîne de caractères littérale convertie en minuscules, donc on appelle string.
J'ai choisi cette solution en analysant le type de chaque argument et en déterminant si la fonction string_slice ou string devait être appelée en conséquence.
*/