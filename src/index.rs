use applin::{
    applin_response, column, form_section, launch_url, nav_button, nav_page, push, scroll, text,
};
use servlin::Response;

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
            nav_button("Nav Page", [push("/nav_page_page")]),
            nav_button("Plain Page", [push("/plain_page_page")]),
        )),
        form_section("Widgets", (
            nav_button("Back Button", [push("/back_button_page")]),
            nav_button("Button", [push("/button_page")]),
            nav_button("Checkbox", [push("/checkbox_page")]),
            nav_button("Error Text", [push("/error_text_page")]),
            nav_button("Form Button", [push("/form_button_page")]),
            nav_button("Form Section", [push("/form_section_page")]),
            nav_button("Grouped Row Table", [push("/grouped_row_table_page")]),
            nav_button("Image", [push("/image_page")]),
            nav_button("Nav Button", [push("/nav_button_page")]),
            nav_button("Text", [push("/text_page")]),
            nav_button("Textfield", [push("/textfield_page")]),
        )),
        form_section("Page Update Modes", (
            nav_button("Inert", [push("/inert_page")]),
            nav_button("Polled", [push("/polled_page")]),
        )),
        form_section("Error Pages", (
            nav_button("App Error", [push("/applin_app_error")]),
            nav_button("Page Not Loaded", [push("/applin_page_not_loaded")]),
            nav_button("Network Error", [push("/applin_network_error")]),
            nav_button("Server Error", [push("/applin_server_error")]),
            nav_button("State Load Error", [push("/applin_state_load_error")]),
            nav_button("User Error", [push("/applin_user_error")]),
            nav_button("Error Details", [push("/error_details")]),
        )),
        form_section("Example Pages", (
            nav_button("New Account", [push("/new_account")]),
        )),
    )))).with_poll(30)).unwrap()
}
