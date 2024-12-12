use std::future::Future;

use asynk_strim::stream_fn;
use futures_core::Stream;
use pin_project_lite::pin_project;

use crate::generator::ServerSentEventGenerator;

pin_project! {
    pub struct DatastarResponse<S> {
        #[pin]
        inner: S,
    }
}

pub fn new_response<F, Fut>(func: F) -> DatastarResponse<impl Stream<Item = String>>
where
    F: FnOnce(ServerSentEventGenerator) -> Fut,
    Fut: Future<Output = ()>,
{
    let stream = stream_fn(|yielder| {
        let generator = ServerSentEventGenerator { yielder };
        func(generator)
    });

    DatastarResponse { inner: stream }
}

impl<S: Stream<Item = String>> Stream for DatastarResponse<S> {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();

        this.inner.poll_next(cx)
    }
}
