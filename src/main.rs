use tokio::io::AsyncReadExt;
use tokio::time;
use tokio::time::Duration;
use tokio::runtime::Runtime;
use tokio::fs::File;
use log::Level;

// the bottleneck is the cpu for this funciton so it will slow down the other async tasks running 
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2)
    }
}

async fn sleeper() {
    log::info!("Sleeping");
    time::sleep(Duration::from_secs(1)).await;
    log::info!("Awake");
}

async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = File::open("ooout.txt").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());

    // slows down the other async tasks running 
    // fib(40);
    // spawn a task that will run on a different thread
    // tokio::task::spawn_blocking(move || {
    //     log::info!("Computing fib(40)");
    //     fib(40);
    //     log::info!("Done computing fib(40)");
    // }).await.unwrap();
    fib(0);
}

async fn run() {
    // asynchronous, running them concurrently
    tokio::join!(
        sleeper(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader()
    );

    // synchronous
    // sleeper().await;
    // reader().await;

    // doesn't run because missing await
    // sleeper();

    // execute the future and not wait around for the completion, "fire and forget"
    tokio::spawn(async {
        sleeper().await;
    });
}

#[tokio::main]
async fn main() {
    let _ = simple_logger::init_with_level(Level::Info);
    
    /*
        replaced by #[tokio::main] macro, this turns our main function to an async function that returns a future
        a future represents a computation that will be completed in the future
        futures are like promises but they are lazy, which means they don't execute as soon as you create them 
        instead only when they are explicity polled by the tokio runtime which is when the await keyword is used
    */
    // let rt = Runtime::new().unwrap();
    // let future = run();
    
    let start = std::time::Instant::now();
    // rt.block_on(future);
    run().await;
    let end = std::time::Instant::now();

    log::info!("Took {:?} seconds", end - start);
}