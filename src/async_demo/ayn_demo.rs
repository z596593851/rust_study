use std::future::Future;
use futures::join;
use tokio::time::{Duration, sleep};

pub struct Song {}

async fn learn_song() -> Song {
    for i in (0..4) {
        println!("learn_song");
        sleep(Duration::from_millis(300)).await;
    }
    Song{}
}

async fn sing_song(song: Song) {
    for i in (0..4) {
        println!("sing_song");
        sleep(Duration::from_millis(200)).await;
    }
}

async fn dance() {
    for i in (0..4) {
        println!("dance");
        sleep(Duration::from_millis(100)).await;
    }
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    join!(f1, f2);
}



#[tokio::test]
async fn test() {
    //async_main().await;
    let f1 = learn_and_sing();
    let f2 = dance();
    join!(f1, f2);
}