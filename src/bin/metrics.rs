

#[tokio::main]
async fn main() {
    let metrics = tokio::runtime::Handle::current().metrics();

    tokio::spawn(async {
        let mut init = 0.0;
        loop {
            init += 0.0000000005;
            if init % 99.0 == 0.0 {
                println!("Running {init}");
            }
        }
    });

    let n = metrics.num_alive_tasks();
    println!("{n} Tasks are alive");
}