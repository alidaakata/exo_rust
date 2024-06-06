// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
///explication
/// Le problème est que les structures ColorClassicStruct et ColorTupleStruct sont déclarées mais pas implémentées, et les instances de ces structures ne sont pas créées dans les tests. De plus, la structure UnitLikeStruct est correctement déclarée, mais elle n'est pas instanciée non plus.
/// Pour ColorClassicStruct, nous avonsdéclarer les champs red, green, et blue de type u8
/// Pour ColorTupleStruct, nous avons déclarer les champs du tuple comme des types de données
/// Pour UnitLikeStruct, nous n'avons pas besoin de déclarer de champs, car il s'agit d'une structure de type unité. Nous avons simplement instancier cette structure