use ::postgres::{Client, NoTls};
use color_eyre::{eyre::Context as _, Result};
use std::env::var;

mod crobot;
mod postgres;
mod qpay;

pub const APP_USER_AGENT: &str = "CSSA rubric merge";

fn main() -> Result<()> {
    color_eyre::install()?;

    let host = var("PG_HOST").with_context(|| "fetching postgres host from env")?;
    let user = var("PG_USER").with_context(|| "fetching postgres user from env")?;
    let database = var("PG_DATABASE").with_context(|| "fetching postgres database from env")?;
    let table = var("PG_TABLE").with_context(|| "fetching postgres table from env")?;

    let mut pg = Client::connect(&format!("host={host} user={user} dbname={database}"), NoTls)
        .with_context(|| "Connecting to postgres database")?;

    let qpay_details = qpay::qpay_request()?;
    let members = qpay_details.all_memberships;

    let mut crobot_updates = Vec::new();

    for qpay_user in members {
        match qpay_user.in_membership_db(&mut pg, &table) {
            postgres::InDb::Empty => {
                qpay_user
                    .create_membership(&mut pg, &table)
                    .with_context(|| format!("{:#?}", qpay_user))?;

                crobot_updates.push(
                    qpay_user
                        .add_username(&mut pg, &table)
                        .with_context(|| format!("{:#?}", qpay_user))?,
                );
            }
            postgres::InDb::NeedsDiscord => crobot_updates.push(
                qpay_user
                    .add_username(&mut pg, &table)
                    .with_context(|| format!("{:#?}", qpay_user))?,
            ),
            _ => (),
        }
    }

    crobot::send_webhook(crobot_updates)?;

    Ok(())
}
