use rand::Rng;
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

fn rand_string(s: i32) -> String {
    let lowercase = "abcdefghijklmnopqrstunwxyz";
    let mut result = String::new();
    for _i in 0..s {
        let num: usize = rand::thread_rng().gen_range(0, 26);
        result.push(lowercase.as_bytes()[num] as char);
    }
    result
}

fn main() {
    let m = Arc::new(Mutex::new(HashMap::new()));
    let mut handlers = vec![];
    for _i in 0..1000 {
      let mp = m.clone();
        handlers.push(thread::spawn(move || {
            let key = rand_string(1);
            *mp.lock().unwrap().entry(key).or_insert(0) += 1;
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }
    println!("{:?}", m.lock().unwrap());
}
