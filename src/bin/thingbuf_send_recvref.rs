use std::{
    thread,
    time::{Duration, Instant},
};

use thingbuf::mpsc::errors::TrySendError;
use tokio::task;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    env_logger::init();

    let (tx, rx) = thingbuf::mpsc::channel(128);

    // This thread sends items to the channel in a synchronous way every 10ms and busy-waits
    // for a free slot if the channel is full.
    let _feeder_thread = {
        let tx = tx.clone();

        thread::spawn(move || loop {
            loop {
                match tx.try_send(1) {
                    Ok(_) => break,
                    Err(TrySendError::Full(_)) => {
                        log::warn!("Channel full. Waiting...");
                        thread::sleep(Duration::from_millis(3));
                        continue;
                    }
                    Err(TrySendError::Closed(_)) => panic!("channel closed"),
                    Err(_) => panic!("other error"),
                }
            }

            thread::sleep(Duration::from_millis(10));
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

    tokio::try_join!(async_recv_task, async_sender_task, async_alive_logger).unwrap();
}
