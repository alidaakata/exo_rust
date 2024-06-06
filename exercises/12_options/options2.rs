// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction
fn main (){}
#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        /*word = optional_target {
            assert_eq!(word, target); */

            ///explication 
            ///J'ai utilisé l'instruction if let pour extraire la valeur de l'option optional_target si elle est de type Some. Si c'est le cas, la valeur est stockée dans la variable word et on peut alors vérifier qu'elle est égale à target.
            if let Some(word) = optional_target {
                assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        /*  integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;*/

            
            while let Some(Some(integer)) = optional_integers.pop() {
                assert_eq!(integer, cursor);
                cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
///explication
///J'ai utilisé l'instruction while let pour extraire les valeurs du vecteur optional_integers tant que le vecteur n'est pas vide. À chaque itération, on vérifie que la valeur extraite est égale à la valeur du curseur, puis on décrémente le curseur. 
///L'instruction while let est particulièrement utile ici car elle permet de gérer les options imbriquées de manière sécurisée.