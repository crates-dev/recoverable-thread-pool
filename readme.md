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

```rust
use recoverable_thread_pool::*;
let thread_pool: ThreadPool = ThreadPool::new(2);
thread_pool.execute(|| {
    println!("first");
});
thread_pool.execute(|| {
    println!("second");
});
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
