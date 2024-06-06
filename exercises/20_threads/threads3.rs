// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
//use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let q1 = q.clone();
    let q2 = q.clone();

    thread::spawn(move || {
        for val in q1.first_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in q2.second_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

///explication
/// Le problème avec ce code réside dans le partage de la même instance de `Queue` entre les deux threads créés dans la fonction `send_tx`. Cela peut entraîner des problèmes de concurrence car les deux threads modifient simultanément les valeurs de `first_half` et `second_half`, pouvant causer des erreurs de synchronisation.
/// nous avons créer des copies des parties de la file d'attente pour chaque thread, afin qu'ils n'aient pas à partager les mêmes données. Nous avons créé deux copies de l'instance de Queue avec clone(), q1 et q2. Chaque thread utilise une copie différente, ce qui évite les problèmes de concurrence.
/// Nous avons choisi cette solution parce qu'elle est simple et efficace. En créant des copies des données, nous pouvons éviter les problèmes de synchronisation et garantir que les threads travaillent de manière indépendante.