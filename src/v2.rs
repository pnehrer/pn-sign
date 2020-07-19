//! PubNub signature v2
use base64::{
    encode_config,
    URL_SAFE_NO_PAD,
};

use log::*;

use crate::common::{
    normalize_query_string,
    sign,
    CryptoError,
};

/// Computes the signature for a request with the given HTTP method, path, query string parameters,
/// and request body using the supplied publish and secret keys.
pub fn sign_request(
    method: impl AsRef<str>,
    pub_key: impl AsRef<str>,
    path: impl AsRef<str>,
    qs: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    body: impl AsRef<str>,
    sec_key: impl AsRef<str>,
) -> Result<String, CryptoError> {
    let nqs = normalize_query_string(qs);
    let input = format!(
        "{}\n{}\n{}\n{}\n{}",
        method.as_ref(),
        pub_key.as_ref(),
        path.as_ref(),
        nqs,
        body.as_ref()
    );

    debug!("input: {}", input);

    let code = sign(input, sec_key)?;
    let encoded = encode_config(code.as_ref(), URL_SAFE_NO_PAD);
    debug!("encoded: {}", encoded);

    Ok(format!("v2.{}", encoded))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sign_request() {
        let body = r#"{
    "ttl": 1440,
    "permissions": {
        "resources" : {
            "channels": {
                "inbox-jay": 3
            },
            "groups": {},
            "users": {},
            "spaces": {}
        },
        "patterns" : {
            "channels": {},
            "groups": {},
            "users": {},
            "spaces": {}
        },
        "meta": {
            "user-id": "jay@example.com",
            "contains-unicode": "The 來 test."
        }
    }
}"#;

        let secret_key = "wMfbo9G0xVUG8yfTfYw5qIdfJkTd7A";
        let qs = vec![("timestamp", "123456789"), ("PoundsSterling", "£13.37")];

        let actual =
            super::sign_request("POST", "demo", "/v3/pam/demo/grant", qs, body, secret_key)
                .expect("Failed to sign request!");

        let expected = "v2.W7Vim_epW4RyuT427E7cS2HiMADRP0wLP6-RkTWPtaM";

        assert_eq!(expected, actual);
    }
}
