use wapc_guest::HandlerResult;
extern crate wasmcloud_actor_core as core;
extern crate wasmcloud_actor_http_client as httpclient;
extern crate wasmcloud_actor_http_server as httpserver;
extern crate wasmcloud_actor_logging as logging;

const API_URL: &str = "https://ifconfig.io";

#[no_mangle]
pub fn wapc_init() {
    httpserver::Handlers::register_handle_request(get_proxy);
    core::Handlers::register_health_request(health);
    logging::enable_macros();
}

fn health(_: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}

fn get_proxy(msg: httpserver::Request) -> HandlerResult<httpserver::Response> {
    if msg.method == "GET".to_string() {
        logging::default().write_log("", "debug", "GOT GET");
        let res = httpclient::default().request(msg.method, API_URL.to_string(), msg.header, vec![])?;
        Ok(httpserver::Response {
            status_code: res.status_code,
            status: res.status,
            header: res.header,
            body: res.body,
        })
    } else {
        Ok(httpserver::Response::internal_server_error("Only GET requests can be proxied with this actor"))
    }
}
