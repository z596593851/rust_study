use tokio::time::{sleep, Duration};
use tokio::join;

async fn task_one() {
    println!("Task one started");
    sleep(Duration::from_secs(2)).await;
    println!("Task one finished");
}

async fn task_two() {
    println!("Task two started");
    sleep(Duration::from_secs(1)).await;
    println!("Task two finished");
}

#[tokio::test]
async fn test() {
    let t1 = task_one();
    let t2 = task_two();
    join!(t1,t2);
    println!("main task finish");
}
