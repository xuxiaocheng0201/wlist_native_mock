pub async fn watch_to_stream<T: Clone>(rx: &mut tokio::sync::watch::Receiver<T>, stream: &crate::frb_generated::StreamSink<T>) {
    if rx.changed().await.is_ok() {
        let item = rx.borrow_and_update().clone();
        let _ = stream.add(item);
    }
    tokio::task::yield_now().await;
}
