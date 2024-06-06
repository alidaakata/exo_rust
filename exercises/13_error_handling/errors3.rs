// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
// correction
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

/// Explication
///Afin qu'une fonction puisse gérer correctement les erreurs, elle doit retourner un type Result ou Option, ce qui permet de traiter les valeurs valides et les erreurs de manière explicite.
/// Ainsi, pour garantir que notre code puisse gérer les erreurs de conversion, j'ai spécifié que la fonction principale (main) doit renvoyer un type Result, indiquant soit une valeur valide, soit une erreur de type ParseIntError.






