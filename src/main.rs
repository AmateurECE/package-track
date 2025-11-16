#![feature(trim_prefix_suffix)]
#![feature(iter_intersperse)]

use std::env;

use lettre::message::Mailbox;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use crate::db::Database;
use crate::email::IntoMessage;
use crate::git::get_git_versions;
use crate::outdated::find_outdated;

mod component;
mod db;
mod email;
mod git;
mod outdated;
mod version;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let domain = env::var("MAIL_DOMAIN")?;
    let recipient = env::var("MAIL_RECIPIENT")?;
    let relay_host = env::var("MAIL_RELAY_HOST")?;
    let url = env::var("DATABASE_URL")?;

    let pool = sqlx::PgPool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let db = Database::new(pool);

    let outdated_components = match find_outdated(db, get_git_versions).await {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&relay_host).build();

    // Send the email
    let recipient: Mailbox = recipient.parse()?;
    for component in outdated_components {
        let message = component.into_message(recipient.clone(), &domain)?;
        mailer.send(message).await?;
    }
    Ok(())
}
