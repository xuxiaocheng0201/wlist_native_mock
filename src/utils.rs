pub async fn watch_to_stream<A: Clone + crate::frb_generated::SseEncode>(rx: &mut tokio::sync::watch::Receiver<A>, stream: &crate::frb_generated::StreamSink<A>) {
    if rx.changed().await.is_ok() {
        let item = rx.borrow_and_update().clone();
        stream.add(item).expect("failed to send item to dart");
        tokio::task::yield_now().await;
    } else {
        std::future::pending().await
    }
}

pub async fn broadcast_to_stream<A: Clone, B: crate::frb_generated::SseEncode>(rx: &mut tokio::sync::broadcast::Receiver<A>, stream: &crate::frb_generated::StreamSink<B>, mut func: impl FnMut(A) -> B) {
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
