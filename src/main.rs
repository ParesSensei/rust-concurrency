use std::thread;

fn main() {
    let curren_thread = thread::current();
    println!("{} : Hello, world!", curren_thread.name().unwrap());
}

#[cfg(test)]
mod tests {

    use ::std::thread;
    use ::std::time::Duration;
    use std::sync::{Arc, Barrier, Once};
    use std::thread::{spawn, JoinHandle};

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application finish");
        thread::sleep(Duration::from_secs(6));
    }

    #[test]
    fn test_join_thread() {
        let handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0 ;
            for i in 1..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }
            return counter;
        });

        println!("Waiiting handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Total Counter: {}", counter),
            Err(error) => println!("Error: {:?}", error),
        }

        println!("Application finish");
    }

    fn calculate() -> i32 {
        let mut counter = 0 ;
        let current = thread::current();
        for i in 1..=5 {
            match current.name() {
                None => { println!("{:?} : Counter : {}", current.id(), i); }
                Some(name) => { println!("{} : Counter : {}", name, i); }
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();
        println!("Total counter 1 : {}", result1);
        println!("Total counter 2 : {}", result2);
        println!("Application finish")
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => { println!("Total counter 1 : {}", counter) }
            Err(error) => { println!("Error: {:?}", error) }
        }

        match result2 {
            Ok(counter) => { println!("Total counter 2 : {}", counter) }
            Err(error) => { println!("Error: {:?}", error ) }
        }

        println!("Application finish")
    }

    #[test]
    fn test_closure() {
        let curren_thread = thread::current();
        println!("Current thread: {}", curren_thread.name().unwrap());


        let name = String::from("Eko");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello {}", name);
        };

        let handle = thread::spawn(closure);
        handle.join().unwrap();
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My thread".to_string());

        let handle = factory.spawn(calculate).expect("Failed to create a new thread");
        let total = handle.join().unwrap();

        println!("Total counter : {}", total);
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
           thread::sleep(Duration::from_secs(2));
            sender.send("Hello from thread!".to_string())
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message);
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for _i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender.send("Hello from thread!".to_string());
            }
            let _ = sender.send("Exit".to_string());
        });

        let handle2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();
                if message == "Exit" {
                    break;
                }
                println!("{}", message);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for _i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender.send("Hello from thread!".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handle3 = thread::spawn(move || {
            for _i in 0..5 {
                thread::sleep(Duration::from_secs(1));
                let _ =sender2.send("Hello from sender2".to_string());
            }
        });

        let handle1 = thread::spawn(move || {
            for _i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                let _ =sender.send("Hello from sender1".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
        let _ = handle3.join();
    }

    static mut COUNTER : i32 = 0;

    #[test]
    fn test_race_condition() {
        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(|| unsafe {
                for _ in 0..1000000 {
                    COUNTER += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Counter : {}", unsafe {COUNTER});
    }

    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI32, Ordering};
        static COUNTER : AtomicI32 = AtomicI32::new(0);

        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(||{
                for _ in 0..1000000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Counter : {}", COUNTER.load(Ordering::Relaxed));
    }

    #[test]
    fn test_atomic_reference() {
        use std::sync::atomic::{AtomicI32, Ordering};
        let counter : Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn( move ||{
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Counter : {}", counter.load(Ordering::Relaxed));
    }

    #[test]
    fn test_mutex() {
        use std::sync::{Arc, Mutex};
        let counter : Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn( move ||{
                for _ in 0..1000000 {
                    let mut data =counter_clone.lock().unwrap();
                    *data += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Counter : {}", *counter.lock().unwrap());
    }

    use std::cell::{RefCell};
    use tokio::runtime::Runtime;

    thread_local! {
        pub static NAME: RefCell<String> = RefCell::new("Default".to_string());
    }
    thread_local! {
        pub static OTHER_NAME: RefCell<String> = RefCell::new("Default".to_string());
    }

    #[test]
    fn test_thread_local() {
        let handle = thread::spawn(|| {
            NAME.with_borrow_mut(|name| {
                *name = "Budi".to_string()
            });


            NAME.with_borrow(|name| {
                println!("hello : {}", name);
            });
        });

        handle.join().unwrap();

        NAME.with_borrow(|name| {
            println!("hello : {}", name);
        });
    }

    #[test]
    fn test_thread_panic() {
        let handle = thread::spawn(|| {
            panic!("Oops! something went wrong")
        });

        match handle.join() {
            Ok(_) => println!("Thread Finish"),
            Err(_) => println!("Thread panic"),
        }

        println!("Application finish")
    }

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handles = vec![];

        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                println!("Join game-{}", i);
                barrier_clone.wait();
                println!("Gamer-{} Start", i)
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
          TOTAL_INIT.call_once(|| {
              println!("Call once");
              TOTAL_COUNTER += 1;
          });
          TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = thread::spawn(move|| {
                let total = get_total();
                println!("Total : {}", total);

            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    async fn get_async_data() -> String {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from async");
        return "Hello from async!".to_string();
    }

    #[tokio::test]
    async fn test_async() {
        let function = get_async_data();
        println!("finish call async");
        let data = function.await;
        println!("{}", data);
    }

    async fn get_database_data(wait: u64) -> String {
        println!("{:?} : get database data", thread::current().id());
        tokio::time::sleep(Duration::from_secs(wait)).await;
        println!("{:?} : get database data", thread::current().id());
        return "Hello from database!".to_string();
    }

    #[tokio::test]
    async fn test_concurrent() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = tokio::spawn(get_database_data(i));
            handles.push(handle);
        }

        for handle in handles {
            let data= handle.await.unwrap();
            println!("response : {}", data)
        }
    }

    async fn run_concurrent(runtime: Arc<Runtime>) {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = runtime.spawn(get_database_data(i));
            handles.push(handle);
        }

        for handle in handles {
            let data= handle.await.unwrap();
            println!("response : {}", data)
        }
    }

    #[test]
    fn test_runtime() {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_time()
                .build().unwrap()
        );

        runtime.block_on(run_concurrent(Arc::clone(&runtime)));
    }
}