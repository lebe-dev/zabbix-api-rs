use log::{debug, error};
use reqwest::blocking::Client;
use serde::Serialize;

use crate::error::ZabbixApiError;

const CONTENT_TYPE_HEADER: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";

pub fn send_post_request<T: Serialize>(
    client: &Client,
    url: &str,
    session: Option<&str>,
    request: T,
) -> Result<String, ZabbixApiError> {
    debug!("send post request to '{url}'");

    let request_body = serde_json::to_string(&request)?;

    let mut http_request_builder = client
        .post(url)
        .body(request_body)
        .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON);

    if let Some(auth_token) = session {
        #[cfg(feature = "v7")]
        {
            // For v7, add token as Bearer auth header
            http_request_builder = http_request_builder.bearer_auth(auth_token);
        }
        // If only v6 feature is enabled (and not v7), token is expected in the JSON body
        // (handled by ZabbixApiRequest<T> for v6) and not as a Bearer token.
    }

    let response = http_request_builder.send()?;

    let response_status = response.status();
    let response_text = response.text()?;

    debug!("---[HTTP RESPONSE]----");
    debug!("{}", response_text);
    debug!("---[/HTTP RESPONSE]----");

    if response_status == reqwest::StatusCode::OK {
        Ok(response_text)
    } else {
        error!("unexpected server response code {}", response_status);
        Err(ZabbixApiError::BadRequestError)
    }
}
