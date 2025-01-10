#[test]
fn test() {
    use crate::*;
    let thread_pool: ThreadPool = ThreadPool::new(2);
    thread_pool.execute(|| {
        println!("first");
    });
    thread_pool.execute(|| {
        println!("second");
    });
}
