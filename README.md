# Async Utils

Simple crate created out of need to manage my Tokio JoinHandles. Async runtimes get to decide how they treat spawned tasks and
Tokio went with the model that detaches spawned Tasks. Regardless of the fact if You think that's good choice, to avoid leaking
tasks and for easier task management it's useful to have tasks 'attached' to some handle and cancel them on handle drop. That's where
this crate is primarily coming from. Right now it provides 3 (actually 2) structs to help to manage concurrency.

## Provided types
- TaskHandle (wrapper around JoinHandle, that can be obtained by calling `to_task_handle()` on tokio::task::JoinHandle) 
- TaskSet (alias for Tokio JoinSet)
- TaskMap (wrapper around HashMap<K, TaskHandle<V>>)

All those types abort tasks when they are dropped. In the future I might put more 'async' things that I find useful in my every day coding.
