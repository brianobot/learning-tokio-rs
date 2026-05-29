

#[tokio::main]
async fn main() {
    let metrics = tokio::runtime::Handle::current().metrics();

    tokio::spawn(async {
        let mut init = 0.0;
        loop {
            // println!("Running {init}");
            init += 1.5;
        }
    });

    let n = metrics.num_alive_tasks();
    println!("{n} Tasks are alive");
}