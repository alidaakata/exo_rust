// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main(){}
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
//correction 
    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}

///explication
/// Le problème est que les tests unitaires ne sont pas complets, car les assertions dans le test correct_width_and_height ne sont pas définies
/// ma solution consiste à compléter les assertions dans le test correct_width_and_height en vérifiant que les attributs width et height du rectangle sont égaux aux valeurs attendues.
/// Ce choix de solution est évident car nous devons vérifier que les attributs width et height du rectangle sont bien initialisés avec les valeurs attendues. En utilant les assertions assert_eq! avec les attributs width et height du rectangle, nous vérifions que les valeurs sont correctes.