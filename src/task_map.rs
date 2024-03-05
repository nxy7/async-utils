use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::TaskHandle;

/// Map of tasks that can store TaskHandles based on any key type.
#[derive(Debug)]
pub struct TaskMap<K, V> {
    inner: HashMap<K, TaskHandle<V>>,
}

impl<K, V> Deref for TaskMap<K, V> {
    type Target = HashMap<K, TaskHandle<V>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<K, V> DerefMut for TaskMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> TaskMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::{sync::broadcast, time::sleep};

    use crate::TaskExt;

    use super::*;

    #[tokio::test]
    async fn is_dropped_correctly() {
        let (tx, _) = broadcast::channel(20);
        let mut map = TaskMap::new();
        let mut rx_clone = tx.subscribe();
        map.insert(
            1,
            tokio::spawn(async move { while let Ok(_) = rx_clone.recv().await {} })
                .to_task_handle(),
        );
        let mut rx_clone = tx.subscribe();
        map.insert(
            2,
            tokio::spawn(async move { while let Ok(_) = rx_clone.recv().await {} })
                .to_task_handle(),
        );

        let r = tx.send(true);
        assert!(r.is_ok());
        drop(map);
        // i guess we need to wait until tokio runtime drops inner task
        sleep(Duration::from_millis(1)).await;
        let r = tx.send(false);
        assert!(r.is_err(), "expected error, but got ok");
    }
}
