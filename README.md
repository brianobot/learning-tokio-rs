# Learning Tokio

Reference Source Material: https://tokio.rs/tokio/tutorial

Tokio is an asynchronous runtime for Rust Programming Language.
- a runtime for executing asynchronous code
- an asynchronous version of the standard library
- a large ecosystem


# Advantages:
Read [Here](https://tokio.rs/tokio/tutorial)

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

- The same rule that applies to using none Send data in task applies to types like Mutex, you can use them in your task alright
  but you can't use them across an await, one way to ensure this, is to scope them so that they are dropped before an await
  ```rust 
  // This works!
  async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
      {
          let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
          *lock += 1;
      } // lock goes out of scope here
  
      do_something_async().await;
  }
  ```

- it is also important to not try to circumvent the use of Mutex across .await, cause if a mutex guard holds the lock to mutex
  while the task is suspended at the await, another task might attempt to access that lock on the same thread, this would lead to a deadlock, because the task holding the lock is currently suspended.

- While using queuing in messaging passing, it is important to always use bounded queue to put a limit on the messages in the queue in order to not crash the system

## Async In Depth
- Futures: 
  - Unlike in other languages a Rust future does not represent a computation happening in the background, it's the computation itself, the owner of the future is responsible for advancing the computation by polling the future, this is done by calling the `Future::poll` method
  - Futures in rust are State machines
  - When implementing Future for a value, it's okay to wake the future more offten than needed, this would lead to a busy loop and wasted cpu cycles, but atleast the future would not hand indefinitely


## Notes
- You can also achieve concurrency with `tokio::select` macro, this takes multiple async computation and returns the first one that completes

- `tokio::join!` waits for multiple futures concurrently, this macros takes a list of task handles
- Errors in Task spawned with `tokio::spawn` do not propgate to the main thread unless the JoinHandle is await and checked for errors
- you can use `tokio::time::timeout` to enforce a deadline on a task execution duration
  - ```rust
    async fn slow_operation() -> String {
        tokio::time::sleep(Duration::from_secs(10)).await;
        "done".to_string()
    }
    
    #[tokio::main]
    async fn main() {
        match timeout(Duration::from_secs(2), slow_operation()).await {
            Ok(result) => println!("Completed: {}", result),
            Err(_) => println!("Timed out after 2 seconds"),
        }
    }
    ```

## Observablity in Tokio
In order to access advance runtime metrics for tokio, you must enable unstable feature in tokio
see example of using metrics [here](src/bin/metrics.rs)

the metrics include
- `num_alive_tasks()` -> usize: Returns the current number of alive tasks in the runtime. 
- `num_workers()` -> usize: Returns the number of worker threads used by the runtime.
- `global_queue_depth` -> usize: Returns the number of tasks currently scheduled in the runtime's
- `num_blocking_threads` -> usize: Returns the number of additional threads spawned by the runtime