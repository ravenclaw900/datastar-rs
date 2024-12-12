use axum_core::{
    body::Body,
    response::{IntoResponse, Response},
};
use futures_util::{Stream, StreamExt};
use http::header;

use crate::response::DatastarResponse;

impl<S> IntoResponse for DatastarResponse<S>
where
    S: Stream<Item = String> + Send + 'static,
{
    fn into_response(self) -> Response {
        let body = Body::from_stream(self.map(Ok::<_, std::convert::Infallible>));

        let headers = [
            (header::CACHE_CONTROL, "nocache"),
            (header::CONNECTION, "keep-alive"),
            (header::CONTENT_TYPE, "text/event-stream"),
        ];

        (headers, body).into_response()
    }
}
