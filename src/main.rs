use std::time::Duration;

use clap::Parser;
use miette::{Context, IntoDiagnostic};
use reqwest::Method;

#[derive(Debug, Parser)]
struct Args {
    /// The expected HTTP status code.
    #[clap(long, short, default_value_t = 200)]
    expected_status: u16,
    /// The HTTP method to use.
    #[clap(long, short, default_value_t = String::from("GET"))]
    method: String,
    /// The timeout in seconds.
    #[clap(long, short)]
    timeout: Option<u64>,
    /// The URL to probe.
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    let method = args.method.as_str().try_into().unwrap_or(Method::GET);

    let mut req = client.request(method, &args.url);
    if let Some(timeout) = args.timeout {
        req = req.timeout(Duration::from_secs(timeout));
    }

    let resp = req
        .send()
        .await
        .into_diagnostic()
        .wrap_err("failed to send probe request")?;

    if resp.status() != args.expected_status {
        return Err(miette::miette!(
            "expected status code {}, got {}",
            args.expected_status,
            resp.status()
        ));
    }

    Ok(())
}
