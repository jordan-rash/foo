use wapc_guest::HandlerResult;
extern crate wasmcloud_actor_core as core;
extern crate wasmcloud_actor_http_client as httpclient;
extern crate wasmcloud_actor_http_server as httpserver;
extern crate wasmcloud_actor_logging as logging;

const API_URL: &str = "https://ifconfig.io";

#[core::init]
fn init() {
    httpserver::Handlers::register_handle_request(get_ip);
    logging::enable_macros();
}

fn get_ip(msg: httpserver::Request) -> HandlerResult<httpserver::Response> {
    logging::default().write_log("", "info", "Received Request");
    if msg.method == "GET".to_string() {
        let res = match httpclient::default().request(
            msg.method,
            API_URL.to_string(),
            msg.header,
            vec![],
        ) {
            Ok(res) => {
                logging::default().write_log("", "info", "Got GET");
                return Ok(httpserver::Response {
                    status_code: res.status_code,
                    status: res.status,
                    header: res.header,
                    body: res.body,
                });
            }
            Err(e) => { 
                logging::default().write_log("", "error", &e.to_string());
                return Err(e);
            },
        };
    } else {
        Ok(httpserver::Response::internal_server_error(
            "Only GET requests can be proxied with this actor",
        ))
    }
}
