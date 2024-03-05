# Async Utils

Simple crate created out of need to manage my Tokio JoinHandles. Async runtimes get to decide how they treat spawned tasks and
Tokio went with the model that detaches spawned Tasks. Regardless of the fact if You think that's good choice, to avoid leaking
tasks and for easier task management it's useful to have tasks 'attached' to some handle and cancel them on handle drop.
This makes structured concurrency much easier, as you don't have to manually send values down some channels and instead you just
need to make sure that parent task is holding TaskHandle of child and doesn't drop it.
That's where this crate is primarily coming from. Right now it provides 3 (actually 2) structs to help to manage concurrency.

## Provided types
- TaskHandle - wrapper around JoinHandle, that can be obtained by calling `to_task_handle()` on tokio::task::JoinHandle. Aborts inner task on Drop.
- TaskSet - alias for Tokio JoinSet. Tokio JoinSet already aborts tasks on Drop.
- TaskMap - wrapper around HashMap<K, TaskHandle<V>>. Useful if you want more control over your 'Set' of tasks as it makes it easier to see if
there is already some task spawned.

All those types abort tasks when they are dropped. In the future I might put here more 'async' utilities that I find useful in my every day coding.
