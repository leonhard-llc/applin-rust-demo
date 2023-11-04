use std::sync::Arc;

use applin::user_error;
use servlin::log::log_request_and_response;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{socket_addr_all_interfaces, Error, HttpServerBuilder, Request, Response};
use temp_dir::TempDir;

mod index;
mod pages;

struct State {}

// fn index(_state: Arc<State>, req: Request) -> Result<Response, Error> {
//     #[derive(Deserialize)]
//     struct Input {
//         name: String,
//     }
//     let input: Input = req.json()?;
//     Ok(Response::json(200, json!({"message": format!("Hello, {}!", input.name)}))
//         .unwrap())
// }

#[allow(clippy::needless_pass_by_value)]
fn handle_req(_state: Arc<State>, req: Request) -> Result<Response, Error> {
    match (req.method(), req.url().path()) {
        ("GET", "/healthz") => Ok(Response::text(200, "success")),
        ("GET", "/ok") => Ok(Response::new(200)),
        ("GET", "/user_error") => Ok(user_error("example error")),
        ("GET", "/server_error") => Err(Error::server_error("server error")),
        ("GET", "/") => Ok(index::index_page()),
        ("GET", "/nav_page_page") => Ok(pages::nav_page_page()),
        ("GET", "/plain_page_page") => Ok(pages::plain_page_page()),
        _ => Ok(Response::text(404, "Not found")),
    }
}

fn main() {
    dotenv::dotenv().ok();
    let port: u16 = std::env::var("PORT")
        .expect("PORT env var")
        .parse()
        .expect("failed parsing PORT env var");
    let listen_addr = socket_addr_all_interfaces(port);

    let state = Arc::new(State {});
    let request_handler =
        move |req: Request| log_request_and_response(req, |req| handle_req(state, req)).unwrap();
    let cache_dir = TempDir::new().unwrap();
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::new(1, 4).unwrap();
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(listen_addr)
                .max_conns(10)
                .small_body_len(64 * 1024)
                .receive_large_bodies(cache_dir.path())
                .spawn_and_join(request_handler),
        )
        .unwrap();
}
