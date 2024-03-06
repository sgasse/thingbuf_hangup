use std::time::{Duration, Instant};

use tokio::task;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    env_logger::init();

    let (tx, rx) = thingbuf::mpsc::channel(128);

    let feeder_task = {
        let tx = tx.clone();

        task::spawn(async move {
            loop {
                tx.send(1).await.ok();
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
    };

    // This task receives from the channel. After 10s, we simulate a slowdown by sleeping for 10ms
    // between receives.
    let async_recv_task = task::spawn(async move {
        let start = Instant::now();
        let mut slowed_down = false;

        while let Some(_data) = rx.recv_ref().await {
            if slowed_down {
                tokio::time::sleep(Duration::from_millis(10)).await;
            } else if start.elapsed() > Duration::from_secs(10) {
                log::debug!("Slowdown starting...");
                slowed_down = true;
            }
        }
    });

    // This task starts sending items in a loop after 20s.
    let async_sender_task = {
        let tx = tx.clone();

        task::spawn(async move {
            tokio::time::sleep(Duration::from_secs(20)).await;
            log::debug!("Starting async feeding...");

            loop {
                tx.send(2).await.ok();
            }
        })
    };

    // This task logs an alive message once per second to show that other async tasks than
    // receiving and sending still work.
    let async_alive_logger = {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        task::spawn(async move {
            loop {
                interval.tick().await;
                log::info!("Still alive: {:?}", Instant::now());
            }
        })
    };

    tokio::try_join!(
        feeder_task,
        async_recv_task,
        async_sender_task,
        async_alive_logger
    )
    .unwrap();
}
