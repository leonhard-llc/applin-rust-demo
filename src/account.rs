#![allow(clippy::module_name_repetitions)]
use crate::HOME_PAGE_KEY;
use applin::{
    applin_response, checkbox, form, form_button, nav_button, nav_page, push, replace_all, rpc,
    scroll, textfield, user_error,
};
use safe_regex::{regex, Matcher0};
use serde::Deserialize;
use servlin::{Error, Request, Response};

pub const CREATE_ACCOUNT_KEY: &str = "/create_account";
pub fn create_account(req: &Request) -> Result<Response, Error> {
    #[derive(Deserialize)]
    struct Input {
        #[serde(default)]
        username: String,
        #[serde(default)]
        agree: bool,
    }
    let input: Input = req.json()?;
    if !input.agree {
        return Ok(user_error("You must agree to the terms"));
    }
    let username = input.username.trim();
    let matcher: Matcher0<_> = regex!(br"[a-z0-9]+");
    if !matcher.is_match(username.as_bytes()) {
        return Ok(user_error("Please enter letters and numbers"));
    }
    Ok(Response::ok_200())
}

pub const NEW_ACCOUNT_PAGE_KEY: &str = "/new_account_page";
pub fn new_account_page() -> Response {
    applin_response(nav_page(
        "New Account",
        scroll(form((
            textfield("username").with_label("Username"),
            nav_button("Terms", [push("/terms")]),
            nav_button("Privacy", [push("/privacy")]),
            checkbox("agree").with_text("I agree"),
            form_button(
                "Create Account",
                [rpc("/create_account"), replace_all(HOME_PAGE_KEY)],
            ),
        ))),
    ))
    .unwrap()
}
