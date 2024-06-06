// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

//correction
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
        _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` in the abs_all() function returns a
        // reference to the same data as before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
///explication
/// Les tests unitaires reference_no_mutation, owned_no_mutation et owned_mutation ont des TODO qui doivent être remplacés par le bon code pour vérifier que le résultat de abs_all est bien un Cow owned ou borrowed.
/*todo1 : Dans ce test, nous passons un tableau slice `&[i32]` à `Cow::from`, créant un `Cow` qui emprunte le slice sans en avoir la propriété. La fonction `abs_all` ne modifie pas le tableau puisque tous les éléments sont déjà positifs, donc `Cow` continue de référencer le tableau original sans cloner les données. Nous vérifions que le résultat de `abs_all` est un `Cow::Borrowed(_)`, confirmant que le `Cow` a emprunté le slice, et le test réussit.
j'ai choisie cette solution car elle vérifie que Cow conserve la référence au tableau original lorsque le tableau n'est pas modifié. Cela confirme que Cow se comporte comme attendu et qu'il n'y a pas de clone inutile des données. */ 

/*todo2 : Nous passons un Vec<i32> à Cow::from, créant un Cow qui prend la propriété du vecteur. La fonction abs_all ne modifie pas les éléments positifs, donc Cow conserve la propriété sans cloner. Le test vérifie que le résultat de abs_all est un Cow::Owned(_), confirmant que Cow conserve la propriété des données non modifiées. 
j'ai choisie cette solution car Elle vérifie que Cow prend et conserve la propriété des données passées par valeur, même si elles ne sont pas modifiées.*/

/*todo3 : Nous passons un Vec<i32> à Cow::from, créant un Cow qui prend la propriété du vecteur. La fonction abs_all modifie les éléments négatifs en valeurs absolues, donc Cow clone les données et prend la propriété du nouveau vecteur. Le test vérifie que le résultat de abs_all est un Cow::Owned(_), confirmant que Cow prend la propriété des données modifiées.
j'ai choisie car Elle vérifie que Cow clone et prend la propriété des données lorsqu'elles sont modifiées, confirmant son comportement attendu. */
