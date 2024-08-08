use std::thread;
use std::time::Duration;


fn main() {

    
       let handle =  thread::spawn(|| {
            for _ in 1..5
            {
                print!(".");
                thread::sleep(Duration::from_millis(1000));
            }
        });
    handle.join().unwrap();
}
