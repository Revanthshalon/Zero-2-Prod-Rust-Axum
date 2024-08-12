use zero2prod::run;

#[tokio::main]
async fn main() {
    println!("hello world!);
    run().await.expect("error running the application")
}
