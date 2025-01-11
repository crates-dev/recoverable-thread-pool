#[test]
fn test() {
    use crate::*;
    use std::{thread::sleep, time::Duration};
    let thread_pool: ThreadPool = ThreadPool::new(1);
    let first_res: SendResult = thread_pool.execute(
        || {
            println!("first");
        },
        |_err| {},
        || {
            println!("finally");
        },
    );
    println!("{:?}", first_res);
    let panic_res: SendResult = thread_pool.execute(
        || {
            panic!("[panic]");
        },
        |err| {
            println!("Catch panic {}", err);
            panic!("[panic]");
        },
        || {
            println!("finally");
        },
    );
    println!("{:?}", panic_res);
    let second_res: SendResult = thread_pool.execute(
        || {
            panic!("[panic]");
        },
        |_err| {
            panic!("[panic]");
        },
        || {
            println!("finally");
        },
    );
    println!("{:?}", second_res);
    sleep(Duration::from_secs(10));
}
