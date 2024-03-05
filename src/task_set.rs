use tokio::task::JoinSet;

/// TaskSet is simple alias for JoinSet. Because JoinSet already aborts futures on drop
/// I didn't feel like it's necessary to modify it.
pub type TaskSet<T> = JoinSet<T>;
