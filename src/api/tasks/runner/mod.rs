use crate::api::tasks::tasks::FTaskState;
use crate::frb_generated::StreamSink;
use crate::utils::channel::broadcast_to_stream;

pub mod refresh;

/// Register a task state change listener.
///
/// This function should only be called once.
/// Ignore the null returned value.
pub async fn get_change_receiver(stream: StreamSink<Option<(i64, FTaskState)>>) {
    let mut receiver = wlist_native::tasks::runner::get_change_receiver();
    loop {
        broadcast_to_stream(&mut receiver, &stream, |a| a.map(|(id, state)| (id, state.into()))).await;
    }
}

/// Register a listener for new pending refresh tasks.
///
/// Since [run_pending_refresh](refresh::run_pending_refresh) returned null means no more tasks,
/// after this function returned a new value, a new pending refresh is ready.
pub async fn get_refresh_listener(stream: StreamSink<()>) {
    let mut receiver = wlist_native::tasks::runner::get_refresh_listener();
    loop {
        broadcast_to_stream(&mut receiver, &stream, std::convert::identity).await;
    }
}
