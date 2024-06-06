// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    //correction
   pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
///explication 
///Le problème ici est que la fonction get_secret_recipe est déclarée comme non publique dans le module sausage_factory.cette fonction n'est donc pas accessible en dehors du module sausage_factory, y compris dans la fonction main
///j'ai déclarer la fonction make_sausage comme publique en ajoutant le mot-clé pub avant la déclaration de la fonction. Cela permettra à la fonction main d'accéder à la fonction make_sausage sans problème.