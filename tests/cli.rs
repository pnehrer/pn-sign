use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn curl_post_body() {
    let url = "https://ps.pndsn.com/v3/pam/demo/grant\
        ?PoundsSterling=£13.37\
        &timestamp=123456789";

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

    let publish_key = "demo";
    let secret_key = "wMfbo9G0xVUG8yfTfYw5qIdfJkTd7A";
    let sig = "v2.W7Vim_epW4RyuT427E7cS2HiMADRP0wLP6-RkTWPtaM";
    let out_url = format!(
        "https://ps.pndsn.com/v3/pam/demo/grant\
        ?PoundsSterling=%C2%A313.37\
        &timestamp=123456789&signature={}",
        sig
    );

    Command::cargo_bin("pn-sign")
        .unwrap()
        .args(&[
            url,
            "--method",
            "POST",
            "--body",
            body,
            "--curl",
            "--pub",
            publish_key,
            "--sec",
            secret_key,
            "--v2",
        ])
        .assert()
        .stdout(
            predicate::eq(
                format!(
                    "'{}' -H 'content-type: application/json' -d '{}'",
                    out_url, body
                )
                .as_str(),
            )
            .trim(),
        );
}

#[test]
fn curl_delete() {
    let url = "https://ps.pndsn.com/v3/history/sub-key/mySubKey/channel/channelfoo\
        ?start=14993928130000000\
        &end=14992200130000000\
        &timestamp=123456789";

    let publish_key = "myPubKey";
    let secret_key = "mySecKey";
    let sig = "v2.-JnWOAWcEDCQ0FNJNf4geNck5YW73m3xT-qpYEA2Oyk";
    let out_url = format!("{}&signature={}", url, sig);

    Command::cargo_bin("pn-sign")
        .unwrap()
        .args(&[
            url,
            "--method",
            "DELETE",
            "--curl",
            "--pub",
            publish_key,
            "--sec",
            secret_key,
            "--v2",
        ])
        .assert()
        .stdout(predicate::eq(format!("'{}' -X DELETE", out_url).as_str()).trim());
}
