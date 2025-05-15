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

    let mut request = client
        .post(url)
        .body(request_body)
        .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON);

    if let Some(session) = session {
        request = request.bearer_auth(session);
    }

    let response = request.send()?;

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
