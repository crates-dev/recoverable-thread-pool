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

#[test]
fn async_test() {
    use crate::*;
    use std::{thread::sleep, time::Duration};
    let thread_pool: ThreadPool = ThreadPool::new(1);
    let first_res: SendResult = thread_pool.async_execute(
        || async {
            println!("first");
        },
        |_err| async {},
        || async {
            println!("finally");
        },
    );
    println!("{:?}", first_res);
    let panic_res: SendResult = thread_pool.async_execute(
        || async {
            panic!("[panic]");
        },
        |err| async move {
            println!("Catch panic {}", err);
            panic!("[panic]");
        },
        || async {
            println!("finally");
        },
    );
    println!("{:?}", panic_res);
    let second_res: SendResult = thread_pool.async_execute(
        || async {
            panic!("[panic]");
        },
        |_err| async {
            panic!("[panic]");
        },
        || async {
            println!("finally");
        },
    );
    println!("{:?}", second_res);
    sleep(Duration::from_secs(10));
}
