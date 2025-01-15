## recoverable-thread-pool

[![](https://img.shields.io/crates/v/recoverable-thread-pool.svg)](https://crates.io/crates/recoverable-thread-pool)
[![](https://docs.rs/recoverable-thread-pool/badge.svg)](https://docs.rs/recoverable-thread-pool)
[![](https://img.shields.io/crates/l/recoverable-thread-pool.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/recoverable-thread-pool/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/recoverable-thread-pool/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/recoverable-thread-pool/)

[Api Docs](https://docs.rs/recoverable-thread-pool/latest/recoverable_thread_pool/)

> A thread pool that supports automatic recovery from panics, allowing threads to restart after a panic. Useful for resilient and fault-tolerant concurrency in network and web programming.

## Installation

To use this crate, you can run cmd:

```shell
cargo add recoverable-thread-pool
```

## Use

### Sync

```rust
use recoverable_thread_pool::*;
use std::{thread::sleep, time::Duration};
let thread_pool: ThreadPool = ThreadPool::new(1);
let first_res: SendResult = thread_pool.execute(|| {
    println!("first");
});
println!("{:?}", first_res);
let panic_res: SendResult = thread_pool.execute_with_catch(
    || {
        panic!("[panic]");
    },
    |err| {
        println!("Catch panic {}", err);
    },
);
println!("{:?}", panic_res);
let second_res: SendResult = thread_pool.execute_with_catch_finally(
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
```

### Async

```rust
use recoverable_thread_pool::*;
use std::{thread::sleep, time::Duration};
let thread_pool: ThreadPool = ThreadPool::new(1);
let first_res: SendResult = thread_pool.async_execute(|| async {
    println!("first");
});
println!("{:?}", first_res);
let panic_res: SendResult = thread_pool.async_execute_with_catch(
    || async {
        panic!("[panic]");
    },
    |err| async move {
        println!("Catch panic {}", err);
    },
);
println!("{:?}", panic_res);
let second_res: SendResult = thread_pool.async_execute_with_catch_finally(
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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
