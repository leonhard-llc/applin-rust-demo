#![allow(clippy::module_name_repetitions)]

use applin::action::{launch_url, push};
use applin::applin_response;
use applin::page::nav_page;
use applin::widget::{column, form_section, nav_button, scroll, text};
use servlin::Response;

use crate::account::NEW_ACCOUNT_PAGE_KEY;
use crate::pages::{INERT_PAGE_KEY, NAV_PAGE_PAGE_KEY, PLAIN_PAGE_PAGE_KEY, POLLED_PAGE_KEY};
use crate::widgets::{
    BACK_BUTTON_PAGE_KEY, BUTTON_PAGE_KEY, CHECKBOX_PAGE_KEY, ERROR_TEXT_PAGE_KEY,
    FORM_BUTTON_PAGE_KEY, FORM_SECTION_PAGE_KEY, GROUPED_ROW_TABLE_PAGE_KEY, IMAGE_PAGE_KEY,
    NAV_BUTTON_PAGE_KEY, TEXTFIELD_PAGE_KEY, TEXT_PAGE_KEY,
};
use crate::{SERVER_ERROR_KEY, USER_ERROR_KEY};

pub fn index_page() -> Response {
    applin_response(nav_page("Applin Rust Demo", scroll(column((
        form_section("About", (
            text("This app demonstrates how to use the applin-rails library to make a mobile app.\n\nThe demo server runs at\nhttps://rails-demo.applin.dev/."),
            nav_button(
                "Source Code of this App",
                [launch_url("https://github.com/leonhard-llc/applin-rails-demo")],
            ).with_sub_text("github.com/leonhard-llc/applin-rails-demo"),
        )),
        form_section("Pages", (
            nav_button("Nav Page", [push(NAV_PAGE_PAGE_KEY)]),
            nav_button("Plain Page", [push(PLAIN_PAGE_PAGE_KEY)]),
        )),
        form_section("Widgets", (
            nav_button("Back Button", [push(BACK_BUTTON_PAGE_KEY)]),
            nav_button("Button", [push(BUTTON_PAGE_KEY)]),
            nav_button("Checkbox", [push(CHECKBOX_PAGE_KEY)]),
            nav_button("Error Text", [push(ERROR_TEXT_PAGE_KEY)]),
            nav_button("Form Button", [push(FORM_BUTTON_PAGE_KEY)]),
            nav_button("Form Section", [push(FORM_SECTION_PAGE_KEY)]),
            nav_button("Grouped Row Table", [push(GROUPED_ROW_TABLE_PAGE_KEY)]),
            nav_button("Image", [push(IMAGE_PAGE_KEY)]),
            nav_button("Nav Button", [push(NAV_BUTTON_PAGE_KEY)]),
            nav_button("Text", [push(TEXT_PAGE_KEY)]),
            nav_button("Textfield", [push(TEXTFIELD_PAGE_KEY)]),
        )),
        form_section("Page Update Modes", (
            nav_button("Inert", [push(INERT_PAGE_KEY)]),
            nav_button("Polled", [push(POLLED_PAGE_KEY)]),
        )),
        form_section("Error Pages", (
            nav_button("App Error", [push("/applin_app_error")]),
            nav_button("Page Not Loaded", [push("/applin_page_not_loaded")]),
            nav_button("Network Error", [push("/applin_network_error")]),
            nav_button("Server Error", [push(SERVER_ERROR_KEY)]),
            nav_button("State Load Error", [push("/applin_state_load_error")]),
            nav_button("User Error", [push(USER_ERROR_KEY)]),
            nav_button("Error Details", [push("/error_details")]),
        )),
        form_section("Example Pages", (
            nav_button("New Account", [push(NEW_ACCOUNT_PAGE_KEY)]),
        )),
    )))).with_poll(30)).unwrap()
}
