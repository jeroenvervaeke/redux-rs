#[cfg(feature = "async_tokio")]
pub use async_tokio::*;

#[cfg(feature = "async_tokio")]
mod async_tokio {
    use std::future::Future;

    pub type SpawnResult = tokio::task::JoinHandle<()>;

    pub fn spawn<T>(future: T) -> SpawnResult
    where
        T: Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(future)
    }
}

#[cfg(feature = "async_wasm")]
pub use async_wasm::*;

#[cfg(feature = "async_wasm")]
mod async_wasm {
    use std::future::Future;

    pub type SpawnResult = ();

    pub fn spawn<T>(future: T) -> SpawnResult
    where
        T: Future<Output = ()> + Send + 'static,
    {
        wasm_bindgen_futures::spawn_local(future)
    }
}
