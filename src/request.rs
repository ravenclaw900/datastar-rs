//! Axum extractors for a datastar GET Request.

use ::core::marker::Send;
use async_trait::async_trait;

use axum_core::{
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use http::{request::Parts, StatusCode, Uri};
use serde::de::DeserializeOwned;

pub struct FailedToDeserializeDatastarQueryString;
pub struct FailedToDeserializeInnerJson;

/// An error that can occur while extracting datastar query string from a GET request sent by datastar.
#[non_exhaustive]
pub enum DatastarQueryRejection {
    FailedToDeserializeDatastarQueryString,
    DatastarQueryNotFound,
    FailedToDeserializeDatastarInnerJson,
}

impl IntoResponse for DatastarQueryRejection {
    /// Create an axum response from the error type DatastarQueryRejection
    fn into_response(self) -> Response {
        let msg = match self {
            DatastarQueryRejection::FailedToDeserializeDatastarQueryString => {
                "Failed to deserialize datastar query string"
            }
            DatastarQueryRejection::FailedToDeserializeDatastarInnerJson => {
                "Failed to deserialize inner json of datastar query string"
            }
            DatastarQueryRejection::DatastarQueryNotFound => {
                "Query string with the format `?datastar=<json> was not found`"
            }
        };

        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(msg.into())
            .unwrap()
    }
}

/// The parsed query of the datastar GET Request
pub struct DatastarQuery<T>(pub T);

impl<T> DatastarQuery<T>
where
    T: DeserializeOwned,
{
    fn try_from_uri(value: &Uri) -> Result<Self, DatastarQueryRejection> {
        let query_string = value.query().unwrap_or_default();
        let query_params = serde_urlencoded::from_str::<Vec<(String, String)>>(query_string)
            .map_err(|_| DatastarQueryRejection::FailedToDeserializeDatastarQueryString)?;

        let datastar_json_string = query_params
            .iter()
            .find(|(key, _)| key == "datastar")
            .map(|(_, value)| value)
            .ok_or_else(|| DatastarQueryRejection::DatastarQueryNotFound)?;

        let datastar_store = serde_json::from_str::<T>(&datastar_json_string)
            .map_err(|_| DatastarQueryRejection::FailedToDeserializeDatastarInnerJson)?;

        Ok(DatastarQuery(datastar_store))
    }
}

/// A GET Request submitted by datastar will be off the form:
/// [url]?datastar=[json]

/// # Example:
/// Consider a sample request GET /sse?datastar={"theme":"","hidden":true}

/// ```
/// use datastar::request::DatastarQuery;
/// use axum_core::response::IntoResponse;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Store {
///     theme: String,
///     hidden: bool,
/// }
///
/// // Create an axum handler with DatastarQuery extractor
///
/// async fn handle_request(DatastarQuery(Store { theme, hidden }): DatastarQuery<Store>) -> impl IntoResponse {
///     // Do something with theme and hidden
///     todo!()
/// }
/// ```
#[async_trait]
impl<T, S> FromRequestParts<S> for DatastarQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = DatastarQueryRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::try_from_uri(&parts.uri)
    }
}
