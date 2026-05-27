# Learning Tokio

Reference Source Material: https://tokio.rs/tokio/tutorial

Tokio is an asynchronous runtime for Rust Programming Language.
- a runtime for executing asynchronous code
- an asynchronous version of the standard library
- a large ecosystem


# Advantages:
[Here](https://tokio.rs/tokio/tutorial)

## Tips:
- When deciding to optimize compile times or reduce the binary footprint, you can intentionally opt into specific features in the tokio crate


## Notes
- Tasks: Are the unit of execution managed by the scheduler, task can be started with the `tokio::spawn` function
- When task is spawned, it is passed to the worker thread which was created at the start of the program by the async runtime
- The task might be executed on the same thread as it was spawned or in another thread and can be moved between threads