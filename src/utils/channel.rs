use crate::frb_generated::{SseEncode, StreamSink};

pub async fn watch_to_stream<A: Clone, B: SseEncode>(rx: &mut tokio::sync::watch::Receiver<A>, stream: &StreamSink<B>, mut func: impl FnMut(A) -> B) -> bool {
    if rx.changed().await.is_ok() {
        let item = rx.borrow_and_update().clone();
        let item = func(item);
        stream.add(item).expect("failed to send item to dart");
        tokio::task::yield_now().await;
        false
    } else {
        true
    }
}

pub async fn broadcast_to_stream<A: Clone, B: SseEncode>(rx: &mut tokio::sync::broadcast::Receiver<A>, stream: &StreamSink<B>, mut func: impl FnMut(A) -> B) {
    match rx.recv().await {
        Ok(item) => {
            let item = func(item);
            stream.add(item).expect("failed to send item to dart");
            tokio::task::yield_now().await;
        },
        Err(tokio::sync::broadcast::error::RecvError::Closed) => std::future::pending().await,
        Err(tokio::sync::broadcast::error::RecvError::Lagged(_n)) => {},
    }
}
