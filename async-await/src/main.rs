async fn hoge() -> i32 {
    println!("async hoge");
    let x = async { 0 };
    x.await
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let piyo = async move {
            println!("async piyo");
        };

        println!("main hoge {}", hoge().await);
        println!("main piyo");
        piyo.await;
        println!("wait");
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        Ok(())
    })
}
