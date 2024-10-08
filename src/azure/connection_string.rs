use base64::Engine;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding;

use base64::engine::general_purpose;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct ConnectionStringUtility {
    pub endpoint: String,
    pub sas_key_name: String,
    pub sas_key_value: String,
}
type HmacSha256 = Hmac<Sha256>;

impl ConnectionStringUtility {
    pub fn new(connection_string: &str) -> Self {
        let mut endpoint = String::new();
        let mut sas_key_name = String::new();
        let mut sas_key_value = String::new();

        let parts: Vec<&str> = connection_string.split(';').collect();
        for part in parts {
            if part.starts_with("Endpoint") {
                endpoint = format!("https{}", &part[11..]);
            } else if part.starts_with("SharedAccessKeyName") {
                sas_key_name = part[20..].to_string();
            } else if part.starts_with("SharedAccessKey") {
                sas_key_value = part[16..].to_string();
            }
        }

        ConnectionStringUtility {
            endpoint,
            sas_key_name,
            sas_key_value,
        }
    }

    pub fn get_sas_token(&self, uri: &str, min_until_expire: i64) -> String {
        let target_uri = urlencoding::encode(&uri.to_lowercase()).to_lowercase();

        // Calculate expiration time in seconds
        let expires_on_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64
            + min_until_expire * 60;

        let to_sign = format!("{}\n{}", target_uri, expires_on_date);

        // Generate HMAC-SHA256 hash
        let mut hmac = HmacSha256::new_from_slice(self.sas_key_value.as_bytes())
            .expect("HMAC can take key of any size");
        hmac.update(to_sign.as_bytes());
        let signature = general_purpose::STANDARD.encode(hmac.finalize().into_bytes());

        // Construct the SAS token
        format!(
            "SharedAccessSignature sr={}&skn={}&sig={}&se={}",
            &target_uri,
            self.sas_key_name,
            urlencoding::encode(&signature),
            expires_on_date,
        )
    }
}

#[cfg(test)]

mod tests {

    use crate::azure::CONNECTION_STRING;

    use super::ConnectionStringUtility;

    #[test]
    fn test_new() {
        let connection_string_utility = ConnectionStringUtility::new(CONNECTION_STRING);
        assert_eq!(
            connection_string_utility.endpoint,
            "https://notification-hub-poc-test.servicebus.windows.net/"
        );
        assert_eq!(connection_string_utility.sas_key_name, "backend-poc");
        assert_eq!(
            connection_string_utility.sas_key_value,
            "BRTOkDLit3bFREhUwQ60vxz3k0IFbstTVrQyQBBICPQ="
        );
    }

    #[test]
    fn test_get_sas_token() {
        let connection_string = ConnectionStringUtility::new(CONNECTION_STRING);
        // let uri = format!("{}{}", connection_string.endpoint, HUB_NAME);
        let sas_token = connection_string.get_sas_token(&connection_string.endpoint, 10);
        println!("sas_token: {:?}", sas_token);
        // assert_eq!(sas_token, "SharedAccessSignature sig=}")
    }
}
