// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
fn main(){}
pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE

//correction
fn compare_license_types(software: &dyn Licensed, software_two: &dyn Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}

///explication
///J'ai utilisé &dyn Licensed comme type pour software et software_two car cela nous permet d'appeler la méthode licensing_info() sur ces arguments.
///En utilisant &dyn Licensed, nous disons à Rust que nous voulons accepter tout objet qui implémente le trait Licensed, et nous voulons appeler la méthode licensing_info() sur celui-ci.
///Cela est nécessaire car nous avons deux structs différents, SomeSoftware et OtherSoftware, qui implémentent tous deux le trait Licensed. En utilisant &dyn Licensed, nous pouvons écrire une seule fonction compare_license_types qui peut fonctionner avec les deux types de structs.