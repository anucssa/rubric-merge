use ::postgres::{Client, NoTls};
use color_eyre::{eyre::Context as _, Result};
use std::env::var;

mod postgres;
mod qpay;

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

    for mut qpay_user in members {
        if qpay_user.in_membership_db(&mut pg, &table) {
            continue;
        }

        qpay_user
            .create_membership(&mut pg, &table)
            .with_context(|| format!("{:#?}", qpay_user))?;
    }

    Ok(())
}
