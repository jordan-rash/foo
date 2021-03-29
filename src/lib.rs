use wapc_guest::HandlerResult;
extern crate wasmcloud_actor_core as core;
extern crate wasmcloud_actor_http_client as httpclient;
extern crate wasmcloud_actor_http_server as httpserver;
extern crate wasmcloud_actor_logging as logging;
use std::collections::HashMap;

const API_URL: &str = "http://worldtimeapi.org/api/timezone/America/Denver";

#[core::init]
fn init() {
    logging::default().write_log("", "info", "Actor Init");
    httpserver::Handlers::register_handle_request(get_ip);
    logging::enable_macros();
}

fn get_ip(msg: httpserver::Request) -> HandlerResult<httpserver::Response> {
    logging::default().write_log("", "info", "Received Request");
    match msg.method.as_str() {
        "GET" => {
            logging::default().write_log("", "info", "Got GET");
            let res = match httpclient::default().request(
                "GET".to_string(),
                API_URL.to_string(),
                msg.header,
                vec![],
            ) {
                Ok(res) => {
                    logging::default().write_log("", "debug", "Returning Response");
                    let h = HashMap::new();
                    //h.insert("User-Agent".to_string(), "curl/7.64.1".to_string());
                    //h.insert("Accept".to_string(), "*/*".to_string());
                    //h.insert("Host".to_string(), "worldtimeapi.org".to_string());
                    return Ok(httpserver::Response {
                        status_code: res.status_code,
                        status: res.status,
                        header: h,
                        body: res.body,
                    });
                }
                Err(e) => {
                    logging::default().write_log("", "error", &e.to_string());
                    let h = HashMap::new();
                    return Ok(httpserver::Response {
                        status_code: 404,
                        status: "Not Found".to_string(),
                        header: h,
                        body: vec![],
                    });
                }
                _ => {
                    logging::default().write_log("", "debug", "Hit default");
                    let h = HashMap::new();
                    return Ok(httpserver::Response {
                        status_code: 404,
                        status: "Not Found".to_string(),
                        header: h,
                        body: vec![],
                    });
                }
            };
            logging::default().write_log("", "debug", "Got to return statement");
            return res;
        }
        "POST" => {
            logging::default().write_log("", "info", "Got POST");
            return Ok(httpserver::Response {
                status_code: 200,
                status: "OK".to_string(),
                header: msg.header,
                body: vec![],
            });
        }
        _ => {
            logging::default().write_log("", "debug", "2");
            Ok(httpserver::Response::internal_server_error(
                "Only GET requests can be proxied with this actor",
            ))
        }
    }
}
