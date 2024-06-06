// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
fn main(){}

trait AppendBar {
    fn append_bar(self) -> Self;
}

//correction
// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

///explication
/* nous devons implémenter le trait AppendBar pour Vec<String> de cette manière pour bénéficier de la polymorphisme, de l'encapsulation, de la réutilisabilité, de la sécurité et de la lisibilité.
J'ai implémenté le trait AppendBar pour un vecteur de chaînes de caractères (Vec<String>). Cela est fait en utilisant le mot-clé impl suivi du nom du trait et du type qui l'implémente.
Dans l'implémentation, j'ai défini la méthode append_bar, qui prend self comme argument. Cette méthode ajoute une nouvelle chaîne de caractères "Bar" à la fin du vecteur en utilisant la méthode push.
J'ai retourné le vecteur modifié self à la fin de la méthode.   
*/