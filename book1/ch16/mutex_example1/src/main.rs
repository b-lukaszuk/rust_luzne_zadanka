use std::sync::Mutex; // mutex - mutual exclusion
                      // only one thread at a time can access the data

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;

        println!("m in inner scope = {m:?}");
    }

    println!("m in outer scope = {m:?}");
}
