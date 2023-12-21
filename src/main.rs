use std::sync::Arc;

use applin::user_error;
use include_dir::include_dir;
use servlin::log::log_request_and_response;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{socket_addr_all_interfaces, Error, HttpServerBuilder, Request, Response};
use temp_dir::TempDir;

mod account;
mod index;
mod pages;
mod widgets;

struct State {}

const PUBLIC_DIR: include_dir::Dir = include_dir!("public/");

// fn index(_state: Arc<State>, req: Request) -> Result<Response, Error> {
//     #[derive(Deserialize)]
//     struct Input {
//         name: String,
//     }
//     let input: Input = req.json()?;
//     Ok(Response::json(200, json!({"message": format!("Hello, {}!", input.name)}))
//         .unwrap())
// }

pub const OK_KEY: &str = "/ok";
pub const USER_ERROR_KEY: &str = "/user_error";
pub const SERVER_ERROR_KEY: &str = "/server_error";
pub const HOME_PAGE_KEY: &str = "/";
pub const PLACEHOLDER_IMAGE_KEY: &str = "/placeholder-200x200.png";

#[allow(clippy::needless_pass_by_value)]
fn handle_req(_state: Arc<State>, req: Request) -> Result<Response, Error> {
    match (req.method(), req.url().path()) {
        ("GET", "/healthz") => Ok(Response::text(200, "success")),
        ("GET" | "POST", OK_KEY) => Ok(Response::new(200)),
        ("GET" | "POST", USER_ERROR_KEY) => Err(user_error("example error")),
        ("GET" | "POST", SERVER_ERROR_KEY) => Err(Error::server_error("server error")),
        ("POST", account::CREATE_ACCOUNT_KEY) => account::create_account(&req),
        ("GET" | "POST", account::NEW_ACCOUNT_PAGE_KEY) => Ok(account::new_account_page()),
        ("GET", HOME_PAGE_KEY) => Ok(index::index_page()),
        ("GET", pages::NAV_PAGE_PAGE_KEY) => Ok(pages::nav_page_page()),
        ("GET", pages::PLAIN_PAGE_PAGE_KEY) => Ok(pages::plain_page_page()),
        ("GET", pages::INERT_PAGE_KEY) => Ok(pages::inert_page()),
        ("GET", pages::POLLED_PAGE_KEY) => Ok(pages::polled_page()),
        ("GET", widgets::BACK_BUTTON_DEFAULT_PAGE_KEY) => Ok(widgets::back_button_default_page()),
        ("GET", widgets::BACK_BUTTON_DISABLED_PAGE_KEY) => Ok(widgets::back_button_disabled_page()),
        ("GET", widgets::BACK_BUTTON_MISSING_PAGE_KEY) => Ok(widgets::back_button_missing_page()),
        ("GET", widgets::BACK_BUTTON_PAGE_KEY) => Ok(widgets::back_button_page()),
        ("GET", widgets::BACK_BUTTON_RPC_ERROR_PAGE_KEY) => {
            Ok(widgets::back_button_rpc_error_page())
        }
        ("GET", widgets::BACK_BUTTON_RPC_OK_PAGE_KEY) => Ok(widgets::back_button_rpc_ok_page()),
        ("GET", widgets::BUTTON_PAGE_KEY) => Ok(widgets::button_page()),
        ("GET", widgets::BUTTON_PRESSED_PAGE_KEY) => Ok(widgets::button_pressed_page()),
        ("GET" | "POST", widgets::CHECKBOX_PAGE_KEY) => Ok(widgets::checkbox_page()),
        ("GET", widgets::ERROR_TEXT_PAGE_KEY) => Ok(widgets::error_text_page()),
        ("GET", widgets::FORM_BUTTON_PAGE_KEY) => Ok(widgets::form_button_page()),
        ("GET", widgets::FORM_SECTION_PAGE_KEY) => Ok(widgets::form_section_page()),
        ("GET", widgets::GROUPED_ROW_TABLE_PAGE_KEY) => Ok(widgets::grouped_row_table_page()),
        ("GET", widgets::IMAGE_PAGE_KEY) => Ok(widgets::image_page()),
        ("GET", widgets::NAV_BUTTON_PAGE_KEY) => Ok(widgets::nav_button_page()),
        ("GET", widgets::TABLE_PAGE_KEY) => Ok(widgets::table_page()),
        ("GET", widgets::TEXT_PAGE_KEY) => Ok(widgets::text_page()),
        ("GET" | "POST", widgets::TEXTFIELD_PAGE_KEY) => Ok(widgets::textfield_page()),
        ("GET", _) => Response::include_dir(&req, &PUBLIC_DIR),
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
