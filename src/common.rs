//! Common types and functions
use hmac::{
    Hmac,
    Mac,
    NewMac,
};

use log::*;
use percent_encoding::{
    utf8_percent_encode,
    AsciiSet,
    NON_ALPHANUMERIC,
};

use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

const SIG_CHARS: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'~')
    .remove(b'-')
    .remove(b'_')
    .remove(b'.');

/// Error encountered while computing the signature
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CryptoError {
    /// The secret key is invalid
    #[error("invalid secret key")]
    SecretKey,
}

pub(crate) fn normalize_query_string(
    qs: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> String {
    let mut pairs: Vec<_> = qs
        .into_iter()
        .map(|(key, value)| (key.as_ref().to_string(), value.as_ref().to_string()))
        .collect();
    pairs.sort();

    let entries: Vec<String> = pairs
        .iter()
        .map(|(key, value)| {
            let key: String = utf8_percent_encode(key, SIG_CHARS).collect();
            let value: String = utf8_percent_encode(value, SIG_CHARS).collect();
            format!("{}={}", key, value)
        })
        .collect();

    entries.join("&")
}

pub(crate) fn sign(
    input: impl AsRef<[u8]>,
    sec_key: impl AsRef<str>,
) -> Result<impl AsRef<[u8]>, CryptoError> {
    let mut mac = HmacSha256::new_from_slice(sec_key.as_ref().as_bytes()).map_err(|err| {
        debug!("Error initializing HMAC: {}", err);
        CryptoError::SecretKey
    })?;

    mac.update(input.as_ref());
    let code = mac.finalize().into_bytes();
    Ok(code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_normalize_query_string() {
        let qs = vec![
            ("store", "1"),
            ("seqn", "1"),
            ("auth", "myAuth"),
            ("timestamp", "1535125017"),
            ("pnsdk", "PubNub-Go/4.1.2"),
            ("uuid", "myUuid"),
        ];

        let normalized = super::normalize_query_string(qs);
        assert_eq!(
            "auth=myAuth&pnsdk=PubNub-Go%2F4.1.2&seqn=1&store=1&timestamp=1535125017&uuid=myUuid",
            normalized
        );
    }
}
