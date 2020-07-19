use chrono::Utc;
use headers::{
    ContentType,
    Header,
};

use pn_sign::{
    built_info,
    sign_request_v1,
    sign_request_v2,
};

use structopt::StructOpt;
use url::Url;

const PARAM_TIMESTAMP: &str = "timestamp";

#[derive(Debug, StructOpt)]
#[structopt(
    rename_all = "kebab",
    version = Box::leak(version().into_boxed_str()) as &str,
)]
struct Opt {
    /// Subscribe key
    #[structopt(long = "sub", required_unless("v2"))]
    sub_key: Option<String>,
    /// Publish key
    #[structopt(long = "pub")]
    pub_key: String,
    /// Secret key
    #[structopt(long = "sec")]
    sec_key: String,
    /// Method
    #[structopt(short, long, default_value = "GET")]
    method: String,
    /// Body
    #[structopt(short, long, default_value)]
    body: String,
    /// Use signature algorithm version 2
    #[structopt(long)]
    v2: bool,
    /// Generate arguments for a curl command
    #[structopt(long)]
    curl: bool,
    /// Request URL
    url: Url,
}

fn version() -> String {
    format!(
        "{} ({}, {} build, {} [{}], {})",
        env!("CARGO_PKG_VERSION"),
        built_info::GIT_VERSION.unwrap_or("unknown"),
        built_info::PROFILE,
        built_info::CFG_OS,
        built_info::CFG_TARGET_ARCH,
        built_info::BUILT_TIME_UTC,
    )
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let opt = Opt::from_args();

    let mut url = opt.url;
    if !url.query_pairs().any(|(key, _)| key == PARAM_TIMESTAMP) {
        url.query_pairs_mut()
            .append_pair(PARAM_TIMESTAMP, &Utc::now().timestamp().to_string());
    }

    let sig = if opt.v2 {
        sign_request_v2(
            opt.method,
            opt.pub_key,
            url.path(),
            url.query_pairs(),
            &opt.body,
            opt.sec_key,
        )
    } else {
        sign_request_v1(
            opt.sub_key.expect("sub_key not supplied"),
            opt.pub_key,
            url.path(),
            url.query_pairs(),
            opt.sec_key,
        )
    }?;

    if opt.curl {
        url.query_pairs_mut().append_pair("signature", &sig);
        let mut curl = format!(r#"'{}'"#, url);
        if !opt.body.is_empty() {
            curl = format!(
                "{} -H '{}: {}' -d '{}'",
                curl,
                ContentType::name(),
                ContentType::json(),
                opt.body
            );
        }

        println!("{}", curl);
    } else {
        println!("{}", sig);
    }

    Ok(())
}
