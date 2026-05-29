

#[tokio::main]
async fn main() {
    let metrics = tokio::runtime::Handle::current().metrics();

    tokio::spawn(async {
        
    });

    let n = metrics.num_alive_tasks();
    println!("{n} Tasks are alive");
}