use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

pub struct Philosopher {
    pub name: String,
    pub left: usize,
    pub right: usize,
}

impl Philosopher {
    pub fn new(name: &str, left: usize, right: usize) -> Philosopher {
        return Philosopher{
            name: name.to_string(),
            left: left,
            right: right,
        };
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} picks up their left fork.", self.name);

        thread::sleep(Duration::from_millis(100));

        let _right = table.forks[self.right].lock().unwrap();
        println!("{} picks up their right fork.", self.name);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        ]
    });

    let philosophers = vec![
        Philosopher::new("Karl Marx", 0, 1),
        Philosopher::new("Friedrich Engels", 1, 2),
        Philosopher::new("Michel Foucault", 2, 3),
        Philosopher::new("Socrates", 3, 4),
        // Philosopher::new("Mark Walberg", 0, 4),  // Left handed to prevent deadlock
        Philosopher::new("Mark Walberg", 4, 0),  // RIght handed will lock
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
