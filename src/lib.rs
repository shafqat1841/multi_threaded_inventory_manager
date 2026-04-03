mod constants;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle, ThreadId},
};

use crate::constants::THREAD_COUNT;

type AllItemsType = Arc<Mutex<HashMap<&'static str, usize>>>;
type HandlerType = JoinHandle<(ThreadId, usize, usize)>;

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

    let mut handlers: Vec<HandlerType> = Vec::new();

    for _ in 0..THREAD_COUNT {
        let all_items_clone = Arc::clone(&all_items);

        let handler: JoinHandle<(ThreadId, usize, usize)> = thread::spawn(move || {
            let thread_id = thread::current().id();

            let mut all_items = match all_items_clone.lock() {
                Ok(items) => items,
                Err(e) => {
                    eprintln!("Failed to acquire lock: {}", e);
                    let count1: usize = 0;
                    let count2: usize = 0;
                    let res: (ThreadId, usize, usize) = (thread_id, count1, count2);
                    return res;
                }
            };
            {
                let laptop_count = all_items.entry("Laptops").or_insert(0);
                if *laptop_count > 0 {
                    *laptop_count -= 1;
                } else {
                    eprintln!("No laptops left to decrement for thread: {:?}", thread_id);
                }
            }
            {
                let phone_count = all_items.entry("Phones").or_insert(0);
                if *phone_count > 0 {
                    *phone_count -= 1;
                } else {
                    eprintln!("No phones left to decrement for thread: {:?}", thread_id);
                }
            }
            let res = (thread_id, all_items["Laptops"], all_items["Phones"]);
            return res;
        });

        handlers.push(handler);
    }

    handlers
        .into_iter()
        .for_each(|handler| match handler.join() {
            Ok(res) => {
                println!(
                    "Thread: {:?}, laptop count: {:?}, phone count: {:?}",
                    res.0, res.1, res.2
                );
            }
            Err(err) => {
                eprintln!("Thread panicked: {:?}", err);
                let main_thread_id = thread::current().id();
                println!(
                    "Main Thread: {:?}, laptop count: {:?}, phone count: {:?}",
                    main_thread_id, 0, 0
                );
            }
        });

    let main_res = all_items.lock();

    match main_res {
        Ok(res) => {
            println!("Final Inventory: {:?}", res);
        }
        Err(e) => {
            eprintln!("Failed to acquire lock in main thread: {}", e);
        }
    }
}
