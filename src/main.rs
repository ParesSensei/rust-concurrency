fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use ::std::thread;
    use ::std::time::Duration;
    use std::thread::JoinHandle;

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
        for i in 1..=5 {
            println!("Counter : {}", i);
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
}