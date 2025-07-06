#![allow(clippy::module_name_repetitions)]
use applin::action::{push, replace_all, rpc};
use applin::page::nav_page;
use applin::widget::{checkbox, error_text, form, form_button, nav_button, scroll, textfield};
use applin::{applin_response, user_error};
use safe_regex::{regex, Matcher0};
use serde::Deserialize;
use servlin::{Error, Request, Response};

use crate::HOME_PAGE_KEY;

pub const CREATE_ACCOUNT_KEY: &str = "/create_account";

#[derive(Deserialize)]
struct Input {
    #[serde(default)]
    username: String,
    #[serde(default)]
    agree: bool,
}
impl Input {
    pub fn check(&self) -> Result<(), &'static str> {
        if !self.agree {
            return Err("You must agree to the terms");
        }
        let username = self.username.trim();
        let matcher: Matcher0<_> = regex!(br"[a-z0-9]+");
        if !matcher.is_match(username.as_bytes()) {
            return Err("Please enter letters and numbers");
        }
        Ok(())
    }
}

pub fn create_account(req: &Request) -> Result<Response, Error> {
    let input: Input = req.json()?;
    input.check().map_err(user_error)?;
    // Write new account to database...
    Ok(Response::ok_200())
}

pub const NEW_ACCOUNT_PAGE_KEY: &str = "/new_account_page";
pub fn new_account_page(req: &Request) -> Result<Response, Error> {
    let opt_input: Option<Input> = req.opt_json()?;
    let error = opt_input.and_then(|input| input.check().err());
    applin_response(nav_page(
        "New Account",
        scroll(form((
            textfield("username").with_label("Username"),
            nav_button("Terms", [push("/terms")]),
            nav_button("Privacy", [push("/privacy")]),
            checkbox("agree").with_text("I agree"),
            error.map(error_text),
            form_button(
                "Create Account",
                [rpc("/create_account", true), replace_all(HOME_PAGE_KEY)],
            ),
        ))),
    ))
}
