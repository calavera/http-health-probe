use httpmock::prelude::*;
use std::process::Command;

#[test]
fn test_successful_probe() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/");
        then.status(200);
    });

    let url = server.url("/");
    let output = Command::new(env!("CARGO_BIN_EXE_http-health-probe"))
        .arg(url)
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
}

#[test]
fn test_probe_with_wrong_status() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/");
        then.status(404);
    });

    let url = server.url("/");
    let output = Command::new(env!("CARGO_BIN_EXE_http-health-probe"))
        .arg(url)
        .output()
        .expect("failed to run binary");

    assert!(!output.status.success());
}

#[test]
fn test_probe_with_custom_expected_status() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/");
        then.status(201);
    });

    let url = server.url("/");
    let output = Command::new(env!("CARGO_BIN_EXE_http-health-probe"))
        .args([url.as_str(), "--expected-status", "201"])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
}

#[test]
fn test_probe_with_post_method() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200);
    });

    let url = server.url("/");
    let output = Command::new(env!("CARGO_BIN_EXE_http-health-probe"))
        .args([url.as_str(), "--method", "POST"])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
}
