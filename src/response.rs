//! Helpers for creating `text/event-stream` responses.

use futures_core::{FusedStream, Stream};

use crate::message::DatastarMessage;

/// A complete datastar response, possibly consisting of multiple messages.
///
/// # Example
/// ```
/// # use datastar::response::FullDatastarResponse;
/// # use datastar::message::DatastarMessage;
/// let msg = DatastarMessage::new_redirect("/");
/// let resp = FullDatastarResponse::from(msg);
/// ```
#[derive(Debug, Clone)]
pub struct FullDatastarResponse(String);

impl FullDatastarResponse {
    /// Create an empty response that messages can be pushed onto later.
    pub fn new_empty() -> Self {
        Self(String::new())
    }

    /// Push a `DatastarMessage` onto this response.
    pub fn push_msg(&mut self, msg: DatastarMessage) {
        self.0.push_str(&msg.into_string())
    }

    /// Get this response as a [`String`].
    pub fn into_string(self) -> String {
        self.0
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

/// A streaming datastar response.
///
/// # Example
/// ```
/// # use datastar::response::StreamingDatastarResponse;
/// # use datastar::message::{DatastarMessage, FragmentConfig};
/// let stream = async_stream::stream! {
///     DatastarMessage::new_fragment(Some("<div>Running computation</div>"), FragmentConfig::new());
///     // Do stuff...
///     DatastarMessage::new_fragment(Some("<div>Done!</div>"), FragmentConfig::new());
/// };
/// let resp = StreamingDatastarResponse::new(stream);
/// ```
#[derive(Debug, Clone)]
pub struct StreamingDatastarResponse<S>(S);

impl<S> StreamingDatastarResponse<S> {
    /// Create a [`StreamingDatastarResponse`] from a [`Stream`] of [`DatastarMessage`]s.
    pub fn new(stream: S) -> Self {
        Self(stream)
    }

    /// Extract the messages as a [`Stream`].
    pub fn into_stream(self) -> impl Stream<Item = DatastarMessage>
    where
        S: Stream<Item = DatastarMessage>,
    {
        self.0
    }

    /// Extract the messages as a [`FusedStream`].
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
