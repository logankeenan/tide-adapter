use std::str::FromStr;
use tide::http::{Method, Url, Request as TideRequest, Response as TideResponse};
use worker::{Headers, Request, Response};

pub async fn to_tide_request(mut request: Request) -> TideRequest {
    let method = Method::from_str(request.method().to_string().as_str()).unwrap();
    let url = Url::from_str(request.url().unwrap().as_str()).unwrap();
    let mut tide_request = TideRequest::new(method, url);

    let body_text = request.text().await.unwrap();
    tide_request.set_body(body_text);

    for (key, value) in request.headers() {
        tide_request.insert_header(key.as_str(), value.as_str());
    }

    tide_request
}

pub async fn to_response(mut tide_response: TideResponse) -> Response {
    let bytes = tide_response.body_bytes().await.unwrap();
    let response = Response::from_bytes(bytes).unwrap();
    let code: u16 = tide_response.status().to_string().parse().unwrap();
    let response = response.with_status(code);

    let mut headers = Headers::new();

    tide_response.header_names().for_each(|header_name| {
        let header_value = tide_response.header(header_name).unwrap();
        headers.set(header_name.as_str(), header_value.as_str()).unwrap();
    });
    let response = response.with_headers(headers);

    response
}