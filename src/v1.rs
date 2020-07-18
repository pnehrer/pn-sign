use base64::{
    encode_config,
    URL_SAFE,
};

use log::*;

use crate::common::{
    normalize_query_string,
    sign,
    CryptoError,
};

pub fn sign_request(
    sub_key: impl AsRef<str>,
    pub_key: impl AsRef<str>,
    path: impl AsRef<str>,
    qs: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    sec_key: impl AsRef<str>,
) -> Result<String, CryptoError> {
    let nqs = normalize_query_string(qs);
    let input = format!(
        "{}\n{}\n{}\n{}",
        sub_key.as_ref(),
        pub_key.as_ref(),
        path.as_ref(),
        nqs
    );

    debug!("input: {}", input);

    let code = sign(input, sec_key)?;
    let encoded = encode_config(code.as_ref(), URL_SAFE);
    debug!("encoded: {}", encoded);

    Ok(encoded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sign_request() {
        let publish_key = "demoPublishKey";
        let subscribe_key = "demoSubscribeKey";
        let secret_key = "secretKey";

        let qs = vec![
            ("uuid", "myUuid"),
            ("auth", "key1"),
            ("ttl", "15"),
            ("r", "1"),
            ("w", "0"),
            ("m", "0"),
            ("timestamp", "123456"),
        ];

        let actual = super::sign_request(
            subscribe_key,
            publish_key,
            "/v2/auth/grant/sub-key/demoSubscribeKey",
            qs,
            secret_key,
        )
        .expect("Failed to sign request!");

        let expected = "Cq6mq1-N0ww7nwow06gydMJogxVuBTMjEF3e8Hnv3L4=";

        assert_eq!(expected, actual);
    }
}
