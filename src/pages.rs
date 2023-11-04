use applin::{applin_response, button, form, nav_page, plain_page, pop, scroll, text};
use servlin::Response;

pub fn nav_page_page() -> Response {
    applin_response(nav_page(
        "Nav Page",
        scroll(form((text("Hello"), text("Hello2")))),
    ))
    .unwrap()
}

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
