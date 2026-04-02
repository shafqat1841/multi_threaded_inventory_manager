mod constants;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle, ThreadId},
};

use crate::constants::THREAD_COUNT;

type AllItemsType = Arc<Mutex<HashMap<&'static str, usize>>>;

pub fn run() {
    let all_items: AllItemsType = Arc::new(Mutex::new(HashMap::new()));

    match all_items.lock() {
        Ok(mut items) => {
            items.insert("Laptops", 10);
            items.insert("Phones", 20);
        }
        Err(e) => {
            eprintln!("Failed to acquire lock: {}", e);
            return;
        }
    }

    println!("{:?}", all_items);

    let mut handlers: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..THREAD_COUNT {
        let all_items_clone = Arc::clone(&all_items);

        let handler = thread::spawn(move || {
            let mut all_items = match all_items_clone.lock() {
                Ok(items) => items,
                Err(e) => {
                    eprintln!("Failed to acquire lock: {}", e);
                    return;
                }
            };
            {
                let laptop_count = all_items.entry("Laptops").or_insert(0);
                *laptop_count -= 1;
            }
            {
                let phone_count = all_items.entry("Phones").or_insert(0);
                *phone_count -= 1;
            }
            println!(
                "Thread: {:?}, laptop count: {:?}, phone count: {:?}",
                thread::current().id(),
                all_items["Laptops"],
                all_items["Phones"]
            );

        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap_or_else(|err| {
            eprintln!("Thread panicked: {:?}", err);
        });
    }

    println!("{:?}", all_items);
}
