use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time;

use super::ActionMessage;

pub fn system_millis() -> u128 {
    let start = SystemTime::now();
    let epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    epoch.as_millis()
}

pub fn task_timer_loop(tx: &mpsc::Sender<ActionMessage>, duration: u64) -> task::JoinHandle<()> {
    let timer_tx = tx.clone();
    task::spawn(async move {
        loop {
            time::sleep(Duration::from_millis(duration)).await;
            if timer_tx
                .send(ActionMessage {
                    topic: "SYSMR/timer".into(),
                    payload: system_millis().to_string().into(),
                })
                .await
                .is_err()
            {
                // If cannot send because channel closed, just ignore and exit.
                break;
            }
        }
        log::info!("Exiting timer task...");
    })
}
