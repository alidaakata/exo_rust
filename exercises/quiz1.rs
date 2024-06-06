// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// I AM NOT DONE

// Put your function here!
// fn calculate_price_of_apples {
fn main (){}
//correction
fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity <= 40 {
        quantity * 2
    } else {
        quantity
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
///explication
/// Nous devons écrire une fonction calculate_price_of_apples qui prend la quantité de pommes comme entrée et retourne le prix total de la commande.
/*  La fonction `calculate_price_of_apples` prend en paramètre `quantity` de type `i32`, représentant le nombre de pommes que Mary souhaite acheter, et retourne un `i32` représentant le prix total de la commande.
Si `quantity` est inférieur ou égal à 40, chaque pomme coûte 2 rustbucks, donc le prix total est `quantity * 2`. Si `quantity` est supérieur à 40, chaque pomme coûte 1 rustbuck, donc le prix total est simplement `quantity`.*/
///Cette solution est choisie parce qu'elle est simple, efficace et facile à comprendre. L'instruction if est un moyen naturel d'exprimer la logique conditionnelle du problème, et les opérations arithmétiques sont straightforward. La fonction est également concise et facile à lire, ce qui la rend maintenable 