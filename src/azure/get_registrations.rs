use axum::http::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::azure::{ConnectionStringUtility, CONNECTION_STRING, HUB_NAME, NAMESPACE};

pub async fn get_registrations() -> String {
    let client = Client::new();

    let connection_string = ConnectionStringUtility::new(CONNECTION_STRING);
    let url = format!(
        "https://{}.servicebus.windows.net/{}/registrations/?api-version=2015-01",
        NAMESPACE, HUB_NAME
    );

    let authorization_token = connection_string.get_sas_token(&connection_string.endpoint, 10);
    let mut header_map = HeaderMap::new();
    header_map.insert(
        "Authorization",
        HeaderValue::from_str(&authorization_token).unwrap(),
    );
    header_map.insert("x-ms-version", HeaderValue::from_str("2015-01").unwrap());
    header_map.insert(
        "Content-Type",
        HeaderValue::from_str("application/atom+xml;type=entry;charset=utf-8").unwrap(),
    );

    let response = client.get(&url).headers(header_map).send().await.unwrap();

    let status = response.status();
    let res = response.text().await.unwrap();
    println!("Response: {:?}", res);
    if status.is_success() {
        println!("Subscription registered successfully!");
    } else {
        println!("Failed to register subscription: {:?}", res);
    }

    res.to_string()
}
