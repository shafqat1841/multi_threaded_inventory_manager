mod constants;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::constants::THREAD_COUNT;

type AllItemsType = Arc<Mutex<HashMap<&'static str, usize>>>;

pub fn run() {
    let all_items: AllItemsType = Arc::new(Mutex::new(HashMap::new()));
    all_items.lock().unwrap().insert("Laptops", 10);
    all_items.lock().unwrap().insert("Phones", 20);
    println!("{:?}", all_items.lock().unwrap());

    let mut handlers: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..THREAD_COUNT {
        let all_items_clone = Arc::clone(&all_items);

        let handler = thread::spawn(move || {
            let mut all_items = all_items_clone.lock().unwrap();
            // let laptop_count = all_items.get_mut("Laptops").unwrap();
            // *laptop_count -= 1;
            // let phone_count = all_items.get_mut("Phones").unwrap();
            // *phone_count -= 1;

            for (item, count) in all_items.iter_mut() {
                if item == &"Laptops" || item == &"Phones" {
                    println!("Thread {:?} is updating {} count", thread::current().id(), item);
                }
                *count -= 1;
            }
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("{:?}", all_items.lock().unwrap());
}
