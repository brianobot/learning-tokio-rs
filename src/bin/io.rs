use tokio::{fs::File, io::AsyncReadExt};



#[tokio::main]
async fn main() {
    let mut f = File::open("current_feature.md").await.unwrap();
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await.unwrap();

    println!("The bytes: {:?}", &buffer[..n]);
}