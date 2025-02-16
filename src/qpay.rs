use color_eyre::eyre::Context as _;
use color_eyre::Result;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use std::{collections::HashMap, env::var};

use crate::APP_USER_AGENT;

#[derive(Deserialize, Debug)]
pub struct QPayResponse {
    #[serde(alias = "allMemberships")]
    pub all_memberships: Vec<QPayMember>,
}

#[derive(Deserialize, Debug)]
pub struct QPayMember {
    pub sortindex: i64,
    pub membershipid: i64,
    pub created: String,
    pub phonenumber: String,
    pub isvalid: i64,
    pub pricepaid: String,
    pub membershiptype: String,
    pub refundtext: String,
    pub responses: HashMap<String, String>,
    pub fullname: String,
    pub updated: String,
    pub email: String,
    pub paymentmethod: String,
}

pub fn qpay_request() -> Result<QPayResponse> {
    let client = reqwest::blocking::Client::new();

    let session_id = var("QPAY_SESSION_ID").with_context(|| "fetching QPAY_SESSION_ID from env")?;
    let email = var("QPAY_EMAIL").with_context(|| "fetching QPAY_SESSION_ID from env")?;

    let body = format!("{{ 'sessionid': '{}', 'email': '{}' }}", session_id, email);
    let body = urlencoding::encode(&body).into_owned();
    let body = format!("details={}", body);

    let res = client
        .post("https://appserver.getqpay.com:9090/AppServerSwapnil/getSocietyPortalMembershipList")
        .body(body)
        .header(USER_AGENT, APP_USER_AGENT)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .with_context(|| "qpay appserver post request")?
        .text()
        .with_context(|| "extracting text body")?;

    serde_json::from_str(&res)
        .with_context(|| "parsing response body")
        .with_context(|| res)
}

impl QPayMember {
    pub fn origination(&self) -> Option<&'_ str> {
        self.responses
            .get("Are you a domestic or international student?")
            .map(|s| s.as_str())
    }

    pub fn discord(&self) -> Option<&'_ str> {
        self.responses
            .get("Do you have a discord username? If so, what is it?")
            .map(|s| s.as_str())
    }
}
