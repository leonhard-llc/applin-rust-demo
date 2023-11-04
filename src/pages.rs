use applin::{applin_response, button, form, nav_page, plain_page, pop, scroll, text};
use servlin::internal::FormatTime;
use servlin::Response;
use std::time::SystemTime;

pub const NAV_PAGE_PAGE_KEY: &str = "/nav_page_page";
pub fn nav_page_page() -> Response {
    applin_response(nav_page(
        "Nav Page",
        scroll(form((text("Hello"), text("Hello2")))),
    ))
    .unwrap()
}

pub const PLAIN_PAGE_PAGE_KEY: &str = "/plain_page_page";
pub fn plain_page_page() -> Response {
    applin_response(plain_page(
        "Nav Page",
        scroll(form((
            text("Hello"),
            text("Hello2"),
            button("Back", [pop()]),
        ))),
    ))
    .unwrap()
}

pub const INERT_PAGE_KEY: &str = "/inert_page";
pub fn inert_page() -> Response {
    applin_response(nav_page(
        "Inert",
        scroll(form((
            text("This page updates when you load or refresh it (pull to refresh)."),
            text(SystemTime::now().iso8601_utc()),
        ))),
    ))
    .unwrap()
}

pub const POLLED_PAGE_KEY: &str = "/polled_page";
pub fn polled_page() -> Response {
    applin_response(
        nav_page(
            "Polled",
            scroll(form((
                text("This page updates automatically every 2 seconds."),
                text(SystemTime::now().iso8601_utc()),
            ))),
        )
        .with_poll(2),
    )
    .unwrap()
}
