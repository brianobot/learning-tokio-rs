use tokio::fs::File;



#[tokio::main]
async fn main() {
    let mut f = File::open("current_feature.md").await?;
    let mut buffer = [0; 10];

    let 
}