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
- Tokio tasks are very lightweight, under the hood they require only a single allocation of 64 bytes of memory
- Applications should feel free to spawn thosands or even millions of tasks
- All spawned task must have a lifetime of 'static, if not the task would fail raise an error
- if a piece of data must be accessible from more than one task concurrently, then it must be shared using synchronization primities such as Arc
- Tasks are required to be static because tokio does not know how long the task would run and if it must keep the task for the duration of the program, the static lifetime is a guarantee of that reality
- Tasks spawned by tokio::spawn must implement the Send trait, this allows tokio runtime to move the tasks between threads while they are suspended at an `.await`
- About the point above, None Send types can be used in the task, but they must be used across awaits, so that when task is suspended it's Send

Example below
```rust
use tokio; // 1.52.2; // 0.13.3
use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        // in this example, Rc is not Send, but it does not exist across an await boundary
        // so this would work fine
       {
           let rc = Rc::new(5);
           println!("Rc: {rc:?}");
       } 
       
       yield_now().await;
       
    });
}
```

- The same rule that applies to using none Send data in task applies to 