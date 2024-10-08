use crate::{
    azure::{ConnectionStringUtility, CONNECTION_STRING, HUB_NAME, NAMESPACE},
    RegisterPayload,
};
use axum::{
    http::{HeaderMap, HeaderValue},
    Json,
};
use reqwest::Client;

pub async fn register(Json(payload): Json<RegisterPayload>) -> () {
    let client = Client::new();
    println!("Payload: {:?}", payload);

    let connection_string = ConnectionStringUtility::new(CONNECTION_STRING);
    // let url = format!(
    //     "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NotificationHubs/namespaces/{}/notificationHubs/{}?api-version=2023-09-01",
    //     SUBSCRIPTION_ID, RESOURCE_GROUP, NAMESPACE, HUB_NAME
    // );
    let url = format!(
        "https://{}.servicebus.windows.net/{}/registrations/?api-version=2015-01",
        NAMESPACE, HUB_NAME
    );
    // let uri = format!("{}{}", connection_string.endpoint, HUB_NAME);

    let authorization_token = connection_string.get_sas_token(&connection_string.endpoint, 10);
    let mut header_map = HeaderMap::new();
    let body = format!(
        "{}{}{}{}{}{}{}",
        r#"<?xml version="1.0" encoding="utf-8"?>
	<entry xmlns="http://www.w3.org/2005/Atom">
		<content type="application/xml">
			<BrowserRegistrationDescription xmlns:i="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://schemas.microsoft.com/netservices/2010/10/servicebus/connect">
				<Endpoint> "#,
        payload.endpoint,
        r#"
				</Endpoint>
				<P256DH>"#,
        payload.keys.p256dh,
        r#"</P256DH>
				<Auth>"#,
        payload.keys.auth,
        r#"</Auth>
				<BodyTemplate>{"title":"asdf","message":"xts"}</BodyTemplate> 
			</BrowserRegistrationDescription>
		</content>
	</entry>"#
    );

    header_map.insert(
        "Authorization",
        HeaderValue::from_str(&authorization_token).unwrap(),
    );
    header_map.insert("x-ms-version", HeaderValue::from_str("2015-01").unwrap());
    header_map.insert(
        "Content-Type",
        HeaderValue::from_str("application/atom+xml;type=entry;charset=utf-8").unwrap(),
    );

    let response = client
        .post(&url)
        .headers(header_map)
        .body(body)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Subscription registered successfully!");
        let res = response.text().await.unwrap();
        println!("Response: {:?}", res);
    } else {
        println!(
            "Failed to register subscription: {:?}",
            response.text().await.unwrap()
        );
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_register_subscription() {
        // let res = super::get_registrations().await;
        // println!("Response: {:?}", res);
        // let result = super::register_subscription().await.unwrap();
        // let res = result.into_response();
        // assert!(res.status().is_success());
    }
}
