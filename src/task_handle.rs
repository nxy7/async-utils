use std::ops::Deref;

use tokio::task::JoinHandle;

/// TaskHandle is simple wrapper around Tokio task::JoinHandle that aborts tasks on Handle drop.
/// The easiest way to obtain TaskHandle is importing TaskExt trait and running `to_task_handle()` on Tokio task::JoinHandle.
///
/// # Example
/// ```rs
/// let (tx, mut rx) = mpsc::channel(20);
/// let handle =
///     tokio::spawn(async move { while let Some(_) = rx.recv().await {} }).to_task_handle();
///
/// let r = tx.send(true).await;
/// // drop handle so the inner task is aborted
/// drop(handle);
///
/// // sadly seems like we need to wait so Tokio runtime has time to actually drop all variables
/// sleep(Duration::from_millis(1)).await;
/// let r = tx.send(false).await;
/// assert!(r.is_err(), "'rx' along with task inside 'handle' should be dropped at this point so tx.send fails");
/// ```
pub struct TaskHandle<T>(pub JoinHandle<T>);

impl<T> Deref for TaskHandle<T> {
    type Target = JoinHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait TaskExt<T> {
    fn to_task_handle(self) -> TaskHandle<T>;
}

impl<T> TaskExt<T> for JoinHandle<T> {
    fn to_task_handle(self) -> TaskHandle<T> {
        TaskHandle(self)
    }
}

impl<T> Drop for TaskHandle<T> {
    fn drop(&mut self) {
        self.0.abort()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::{sync::mpsc, time::sleep};

    use super::*;

    #[tokio::test]
    async fn is_dropped_correctly() {
        let (tx, mut rx) = mpsc::channel(20);
        let handle =
            tokio::spawn(async move { while let Some(_) = rx.recv().await {} }).to_task_handle();

        let r = tx.send(true).await;
        assert!(r.is_ok());
        drop(handle);
        // i guess we need to wait until tokio runtime drops inner task
        sleep(Duration::from_millis(1)).await;
        let r = tx.send(false).await;
        assert!(r.is_err(), "expected error, but got ok");
    }
}
