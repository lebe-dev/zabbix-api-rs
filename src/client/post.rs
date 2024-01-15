use log::{debug, error};
use reqwest::blocking::Client;
use serde::Serialize;

use crate::error::ZabbixApiError;

const CONTENT_TYPE_HEADER: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";

pub fn send_post_request<T: Serialize>(client: &Client,
                                       url: &str, request: T) -> Result<String, ZabbixApiError> {
    debug!("send post request to '{url}'");

    let request_body = serde_json::to_string(&request)?;

    let response = client.post(url)
        .body(request_body)
        .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
        .send()?;

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