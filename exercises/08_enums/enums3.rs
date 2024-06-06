// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

//correction
fn main (){}
enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit,
   
   }
   
   struct Point {
       x: u8,
       y: u8,
   }
   
   struct State {
       color: (u8, u8, u8),
       position: Point,
       quit: bool,
       message: String,
   }
   
   impl State {
       fn change_color(&mut self, color: (u8, u8, u8)) {
           self.color = color;
       }
   
       fn quit(&mut self) {
           self.quit = true;
       }
   
       fn echo(&mut self, s: String) {
           self.message = s
       }
   
       fn move_position(&mut self, p: Point) {
           self.position = p;
       }
   
       fn process(&mut self, message: Message) {
           // TODO: create a match expression to process the different message variants
           // Remember: When passing a tuple as a function argument, you'll need extra parentheses:
           // fn function((t, u, p, l, e))
           match message {
               Message::ChangeColor(a, b, c) => self.change_color((a, b, c)),
               Message::Echo(s) => self.echo(s),
               Message::Move(p) => self.move_position(p),
               Message::Quit => self.quit(),
           }
       }
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn test_match_message_call() {
           let mut state = State {
               quit: false,
               position: Point { x: 0, y: 0 },
               color: (0, 0, 0),
               message: "hello world".to_string(),
           };
           state.process(Message::ChangeColor(255, 0, 255));
           state.process(Message::Echo(String::from("Hello world!")));
           state.process(Message::Move(Point { x: 10, y: 15 }));
           state.process(Message::Quit);
   
           assert_eq!(state.color, (255, 0, 255));
           assert_eq!(state.position.x, 10);
           assert_eq!(state.position.y, 15);
           assert_eq!(state.quit, true);
           assert_eq!(state.message, "Hello world!");
       }
   }
   //explication
   //J'ai déduit les variants de l'enum Message en examinant les tests et les méthodes de State
   //Les appels à state.process dans les tests : dans les tests, j'ai vu que state.process était appelé avec différents arguments, tels que Message::ChangeColor(255, 0, 255), Message::Echo(String::from("Hello world!")), Message::Move(Point { x: 10, y: 15 }), et Message::Quit. Cela m'a indiqué que ces arguments devaient être des variants de l'enum Message.
   //pour le fn proccess je me suis référé au impl State et aux appels à state.process 