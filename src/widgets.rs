use applin::action::{pop, push, rpc};
use applin::applin_response;
use applin::option::{Allow, AutoCapitalize, Disposition, HAlignment};
use applin::page::nav_page;
use applin::widget::{
    back_button, button, checkbox, empty, error_text, form, form_button, form_section,
    grouped_row_table, image, nav_button, scroll, table, text, textfield,
};
use servlin::internal::FormatTime;
use servlin::Response;
use std::time::{Duration, SystemTime};

use crate::{OK_KEY, PLACEHOLDER_IMAGE_KEY, SERVER_ERROR_KEY};

pub const BACK_BUTTON_DEFAULT_PAGE_KEY: &str = "/back_button_default_page";
pub fn back_button_default_page() -> Response {
    applin_response(nav_page("Default", scroll(empty()))).unwrap()
}

pub const BACK_BUTTON_DISABLED_PAGE_KEY: &str = "/back_button_disabled_page";
pub fn back_button_disabled_page() -> Response {
    applin_response(
        nav_page("Disabled", scroll(button("Back", [pop()]))).with_start(back_button([])),
    )
    .unwrap()
}
pub const BACK_BUTTON_MISSING_PAGE_KEY: &str = "/back_button_missing_page";
pub fn back_button_missing_page() -> Response {
    applin_response(nav_page("Missing", scroll(button("Back", [pop()]))).with_empty_start())
        .unwrap()
}

pub const BACK_BUTTON_PAGE_KEY: &str = "/back_button_page";
pub fn back_button_page() -> Response {
    applin_response(nav_page(
        "Back Button",
        scroll(form((
            nav_button("Default", [push(BACK_BUTTON_DEFAULT_PAGE_KEY)]),
            nav_button("Disabled", [push(BACK_BUTTON_DISABLED_PAGE_KEY)]),
            nav_button("Missing", [push(BACK_BUTTON_MISSING_PAGE_KEY)]),
            nav_button("RPC", [push(BACK_BUTTON_RPC_OK_PAGE_KEY)]),
            nav_button("RPC Error", [push(BACK_BUTTON_RPC_ERROR_PAGE_KEY)]),
        ))),
    ))
    .unwrap()
}

pub const BACK_BUTTON_RPC_ERROR_PAGE_KEY: &str = "/back_button_rpc_error_page";
pub fn back_button_rpc_error_page() -> Response {
    applin_response(
        nav_page("RPC Error", scroll(button("Back without RPC", [pop()])))
            .with_start(back_button([rpc(SERVER_ERROR_KEY, false), pop()])),
    )
    .unwrap()
}

pub const BACK_BUTTON_RPC_OK_PAGE_KEY: &str = "/back_button_rpc_ok_page";
pub fn back_button_rpc_ok_page() -> Response {
    applin_response(
        nav_page("RPC", scroll(button("Back without RPC", [pop()])))
            .with_start(back_button([rpc(OK_KEY, false), pop()])),
    )
    .unwrap()
}

pub const BUTTON_PAGE_KEY: &str = "/button_page";
pub fn button_page() -> Response {
    applin_response(nav_page(
        "Button",
        scroll(form((
            button("Button", [push(BUTTON_PRESSED_PAGE_KEY)]),
            button(
                "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                [push(BUTTON_PRESSED_PAGE_KEY)],
            ),
            button(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                [push(BUTTON_PRESSED_PAGE_KEY)],
            ),
            button("Disabled Button", []),
        ))),
    ))
    .unwrap()
}

pub const BUTTON_PRESSED_PAGE_KEY: &str = "/button_pressed_page";
pub fn button_pressed_page() -> Response {
    applin_response(nav_page("Button Pressed", scroll(empty()))).unwrap()
}

pub const CHECKBOX_PAGE_KEY: &str = "/checkbox_page";
pub fn checkbox_page() -> Response {
    applin_response(nav_page(
        "Checkbox",
        scroll(form((
            checkbox("checkbox").with_text("Checkbox"),
            checkbox("initial-checked")
                .with_text("Initially checked")
                .with_initial_bool(true),
            checkbox("with-rpc")
                .with_text("Does RPC on change")
                .with_actions([rpc(OK_KEY, false)]),
            checkbox("with-bad-rpc")
                .with_text("Does RPC on change, but it fails")
                .with_actions([rpc(SERVER_ERROR_KEY, false)]),
            checkbox("no-label-checkbox"),
            checkbox("mmmm-mmmm-checkbox")
                .with_text("MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM"),
            checkbox("mmmmmmmm-checkbox")
                .with_text("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
            text(format!(
                "Time of page poll: {}",
                SystemTime::now().iso8601_utc()
            )),
            checkbox("polls")
                .with_text("Polls page")
                .with_poll_delay(Duration::ZERO),
            checkbox("poll_delay")
                .with_text("Polls page after 1 second delay")
                .with_poll_delay(Duration::from_secs(1)),
        ))),
    ))
    .unwrap()
}

pub const ERROR_TEXT_PAGE_KEY: &str = "/error_text_page";
pub fn error_text_page() -> Response {
    applin_response(nav_page(
        "Error Text",
        scroll(form((
            error_text("Error Message"),
            error_text("MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM"),
            error_text("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
        ))),
    ))
    .unwrap()
}

pub const FORM_BUTTON_PAGE_KEY: &str = "/form_button_page";
pub fn form_button_page() -> Response {
    applin_response(nav_page(
        "Form Button",
        scroll(form((
            form_button("Button", [push(BUTTON_PRESSED_PAGE_KEY)]),
            form_button(
                "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                [push(BUTTON_PRESSED_PAGE_KEY)],
            ),
            form_button(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                [push(BUTTON_PRESSED_PAGE_KEY)],
            ),
            form_button("Disabled", []),
            form_button("Start Aligned", [push(BUTTON_PRESSED_PAGE_KEY)])
                .with_align(HAlignment::Start),
            form_button("Center Aligned", [push(BUTTON_PRESSED_PAGE_KEY)])
                .with_align(HAlignment::Center),
            form_button("End Aligned", [push(BUTTON_PRESSED_PAGE_KEY)]).with_align(HAlignment::End),
        ))),
    ))
    .unwrap()
}

pub const FORM_SECTION_PAGE_KEY: &str = "/form_section_page";
pub fn form_section_page() -> Response {
    applin_response(nav_page(
        "Form Section",
        scroll(form((
            form_section("Section 1", (text("text"), text("text"))),
            form_section("Section 2", (text("text"), text("text"))),
        ))),
    ))
    .unwrap()
}

pub const GROUPED_ROW_TABLE_PAGE_KEY: &str = "/grouped_row_table_page";
pub fn grouped_row_table_page() -> Response {
    applin_response(nav_page(
        "Grouped Row Table",
        scroll(form((
            form_section(
                "Two Groups",
                grouped_row_table((
                    (
                        (text("A1"), text("B1"), text("C1")),
                        (text("A2"), text("B2"), None),
                    ),
                    ((text("One"), None, text("Three")),),
                ))
                .with_spacing(8),
            ),
            form_section(
                "Long text",
                grouped_row_table(((
                    (
                        text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        ),
                        text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        ),
                    ),
                    (text("A2"), text("B2")),
                ),))
                .with_spacing(8),
            ),
        ))),
    ))
    .unwrap()
}

pub const IMAGE_PAGE_KEY: &str = "/image_page";
pub fn image_page() -> Response {
    applin_response(nav_page(
        "Image",
        scroll(form((
            text("Fit"),
            image(Disposition::Fit, 4.0, PLACEHOLDER_IMAGE_KEY),
            text("Cover"),
            image(Disposition::Cover, 4.0, PLACEHOLDER_IMAGE_KEY),
            text("Stretch"),
            image(Disposition::Stretch, 4.0, PLACEHOLDER_IMAGE_KEY),
            text("Not found"),
            image(Disposition::Fit, 4.0, "/nonexistent"),
        ))),
    ))
    .unwrap()
}

pub const NAV_BUTTON_PAGE_KEY: &str = "/nav_button_page";
pub fn nav_button_page() -> Response {
    applin_response(nav_page(
        "Nav Button",
        scroll(form((
            nav_button("Page 1", [push(BUTTON_PRESSED_PAGE_KEY)]),
            nav_button("Page 2", [push(BUTTON_PRESSED_PAGE_KEY)]).with_sub_text("A very nice page"),
            nav_button("Page 3", [push(BUTTON_PRESSED_PAGE_KEY)]).with_badge_text("5"),
            nav_button("Page 4", [push(BUTTON_PRESSED_PAGE_KEY)])
                .with_badge_text("123456789012345678901234567890"),
            nav_button("Page 5", [push(BUTTON_PRESSED_PAGE_KEY)])
                .with_photo_url(PLACEHOLDER_IMAGE_KEY),
            nav_button("Page 6", [push(BUTTON_PRESSED_PAGE_KEY)])
                .with_photo_url("/nonexistent.png"),
            nav_button("Disabled", []),
            nav_button(
                "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                [push(BUTTON_PRESSED_PAGE_KEY)],
            ),
            nav_button(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                [push(BUTTON_PRESSED_PAGE_KEY)],
            ),
        ))),
    ))
    .unwrap()
}

pub const TABLE_PAGE_KEY: &str = "/table_page";
pub fn table_page() -> Response {
    applin_response(nav_page(
        "Table",
        scroll(form((
            form_section(
                "Table",
                table((
                    (text("A1"), text("B1"), text("C1")),
                    (text("A2"), text("B2"), None),
                ))
                .with_spacing(8),
            ),
            form_section(
                "Long text",
                table((
                    (
                        text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        ),
                        text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        ),
                    ),
                    (text("A2"), text("B2")),
                ))
                .with_spacing(8),
            ),
        ))),
    ))
    .unwrap()
}

pub const TEXT_PAGE_KEY: &str = "/text_page";
pub fn text_page() -> Response {
    applin_response(nav_page(
        "Text",
        scroll(form((
            text("Hello world!"),
            text("MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM"),
            text("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
        ))),
    ))
    .unwrap()
}

pub const TEXTFIELD_PAGE_KEY: &str = "/textfield_page";
pub fn textfield_page() -> Response {
    applin_response(nav_page(
        "Text Field",
        scroll(form((
      textfield("field1").with_label("Field 1"),
      textfield("with-err").with_label("With an error message").with_error("An error message."),
      textfield("with-short-initial").with_label("With a short initial string").with_initial_string( "initial text"),
      textfield(
        "with-long-initial").with_label("With a long initial string").with_initial_string( "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
      ),
      textfield("numbers").with_label("Enter numbers").with_allow( Allow::Numbers),
      textfield("text").with_label("Enter text").with_allow( Allow::Ascii),
      textfield("tel").with_label("Enter tel").with_allow( Allow::Tel),
      textfield("email").with_label("Enter email").with_allow( Allow::Email),
      textfield("name").with_label("Enter name").with_auto_capitalize(AutoCapitalize::Names),
      textfield("sentences").with_label("Enter sentences").with_auto_capitalize(AutoCapitalize::Sentences),
      // TODO: Implement textfield.max_lines and uncomment.
      // textfield("field9").with_label("Enter one line").with_max_lines( 1),
      // textfield("field10").with_label("Enter up to three lines").with_max_lines( 3),
      text(format!(
          "Time of page poll: {}",
          SystemTime::now().iso8601_utc()
      )),
      textfield("polls").with_label("Polls page").with_poll_delay(Duration::ZERO),
      textfield("poll_delay").with_label("Polls page after 1 second delay").with_poll_delay(Duration::from_secs(1)),
        ))),
    ))
    .unwrap()
}
