use futures_core::{FusedStream, Stream};

use crate::message::DatastarMessage;

#[derive(Debug, Clone)]
pub struct FullDatastarResponse(String);

impl FullDatastarResponse {
    pub fn new_empty() -> Self {
        Self(String::new())
    }

    pub fn push_msg(&mut self, msg: DatastarMessage) {
        self.0.push_str(&msg.into_string())
    }
}

impl From<DatastarMessage> for FullDatastarResponse {
    fn from(value: DatastarMessage) -> Self {
        Self(value.into_string())
    }
}

#[cfg(feature = "axum")]
impl axum_core::response::IntoResponse for FullDatastarResponse {
    fn into_response(self) -> axum_core::response::Response {
        use http::header;

        let headers = [
            (header::CACHE_CONTROL, "no-cache"),
            (header::CONTENT_TYPE, "text/event-stream"),
            (header::CONNECTION, "keep-alive"),
        ];

        (headers, self.0).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct StreamingDatastarResponse<S>(S);

impl<S> StreamingDatastarResponse<S> {
    pub fn new(stream: S) -> Self {
        Self(stream)
    }

    pub fn into_stream(self) -> impl Stream<Item = DatastarMessage>
    where
        S: Stream<Item = DatastarMessage>,
    {
        self.0
    }

    pub fn into_fused_stream(self) -> impl FusedStream<Item = DatastarMessage>
    where
        S: FusedStream<Item = DatastarMessage>,
    {
        self.0
    }
}

#[cfg(feature = "axum")]
impl<S> axum_core::response::IntoResponse for StreamingDatastarResponse<S>
where
    S: Stream<Item = DatastarMessage> + Send + 'static,
{
    fn into_response(self) -> axum_core::response::Response {
        use axum_core::body::Body;
        use futures_util::StreamExt;
        use http::header;

        let headers = [
            (header::CACHE_CONTROL, "no-cache"),
            (header::CONTENT_TYPE, "text/event-stream"),
            (header::CONNECTION, "keep-alive"),
        ];

        let body = Body::from_stream(
            self.0
                .map(|msg| Ok::<bytes::Bytes, std::convert::Infallible>(msg.into_string().into())),
        );

        (headers, body).into_response()
    }
}
