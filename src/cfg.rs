#[test]
fn test() {
    use crate::*;
    let thread_pool: ThreadPool = ThreadPool::new(1);
    let first_res: SendResult = thread_pool.execute(
        || {
            println!("first");
        },
        |_err| {},
    );
    println!("{:?}", first_res);
    let panic_res: SendResult = thread_pool.execute(
        || {
            panic!("[panic]");
        },
        |err| {
            println!("Catch panic {}", err);
        },
    );
    println!("{:?}", panic_res);
    let second_res: SendResult = thread_pool.execute(
        || {
            println!("second");
        },
        |_err| {},
    );
    println!("{:?}", second_res);
    loop {}
}
