// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
// correction
// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
fn main () {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 1 {
            return Err(if value == 0 { CreationError::Zero } else { CreationError::Negative });
        }
        Ok(PositiveNonzeroInteger(value as u64))
    }
}


#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

///explication
/// nitialement, la fonction renvoyait toujours Ok car elle ne vérifiait pas si le nombre passé en argument était négatif ou nul.
/// Afin de résoudre ce problème, j'ai inclus une condition if pour vérifier si le nombre est négatif, nul ou positif, et j'ai ajusté le retour de la fonction en conséquence : Negative est retourné si la valeur est négative, Zero si elle est nulle, et Ok si elle est positive.




