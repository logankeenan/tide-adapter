use std::str::FromStr;
use rora_javascript_adapter::{JsRequest, JsResponse};
use tide::http::{Method, Url, Request as TideRequest, Response as TideResponse};

pub async fn to_response(mut tide_response: TideResponse) -> JsResponse {
    let mut response = JsResponse::new();
    response.body = Some(tide_response.body_string().await.unwrap());
    response.status_code = tide_response.status().to_string();

    tide_response.header_names().for_each(|header_name| {
        let header_value = tide_response.header(header_name).unwrap();
        response.headers.insert(
            String::from(header_name.as_str()),
            String::from(header_value.as_str()),
        );
    });

    response
}

pub fn to_tide_request(js_request: JsRequest) -> TideRequest {
    let method = Method::from_str(js_request.method.to_string().as_str()).unwrap();
    let url = Url::from_str(js_request.uri.as_str()).unwrap();
    let mut tide_request: TideRequest = TideRequest::new(method, url);

    js_request.headers.into_iter().for_each(|(key, value)| {
        tide_request.insert_header(key.as_str(), value.as_str());
    });

    if js_request.body.is_some() {
        tide_request.set_body(js_request.body.unwrap());
    }

    tide_request
}