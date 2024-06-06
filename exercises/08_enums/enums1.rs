// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE


//correction
#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}


fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
//explication
//Le code tente d'imprimer quatre valeurs de l'énumération différentes : Quit, Echo, Move et ChangeColor. Cependant, ces valeurs ne sont pas encore définies dans l'énumération Message
//j'ai juste ajouter les valeurs 