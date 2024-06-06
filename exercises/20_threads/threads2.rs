// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}
//correction
fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_completed`
    println!("Jobs completed: {}", status.lock().unwrap().jobs_completed);
}

///explication
/// Le problème avec ce code est que les threads ne peuvent pas modifier la valeur partagée jobs_completed car elle n'est pas protégée contre les accès concurrents. De plus, quand les threads essaient de modifier cette valeur, ils vont essayer de la modifier simultanément, ce qui peut entraîner des problèmes de synchronization.
/*todo1 : pour la solution j'ai utiliser `Arc` qui permet de partager la propriété d'une valeur entre plusieurs threads, mais offre seulement un accès en lecture seule. Pour permettre la modification de la valeur partagée, il faut utiliser un `Mutex` pour protéger l'accès. En enveloppant `JobStatus` avec un `Mutex`, on garantit que seul un thread peut modifier la valeur à la fois.sans oublier Le type Mutex qui est importé dans la déclaration use au début du fichier. 
j'ai choisie cette solution car Elle permet de partager une valeur modifiable entre plusieurs threads de manière thread-safe, évitant les courses de données grâce au verrouillage du Mutex*/

/*todo2: solution : Avant de mettre à jour la valeur partagée, nous avons acquérie un verrou sur le Mutex pour nous assurer que seul un thread peut modifier la valeur à la fois. La méthode lock() renvoie un MutexGuard qui fournit un accès exclusif à la valeur. Nous utilisons ensuite la méthode unwrap() pour récupérer la valeur sous-jacente et l'incrémenter.
je l'ai choisie car Elle assure une mise à jour thread-safe de la valeur partagée en empêchant les autres threads d'accéder à la valeur pendant sa modification, garantissant une mise à jour atomique et cohérente. */

/*todo3: our imprimer la valeur de jobs_completed, on doit acquérir un verrou sur le Mutex pour obtenir un accès exclusif.
La méthode lock() permet d'obtenir un MutexGuard pour accéder à la valeur.
j'ai choisie la solution car Elle garantit une vue cohérente de la valeur partagée, en s'assurant que nous voyons la dernière valeur de jobs_completed et empêchant les modifications concurrentes pendant l'impression. */