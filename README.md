# sendgrid_thin

[![Tests](https://github.com/OLoKo64/sendgrid_thin/actions/workflows/rust-workflow.yml/badge.svg)](https://github.com/OLoKo64/sendgrid_thin/actions/workflows/rust-workflow.yml)

A thin wrapper around the SendGrid V3 API.

It exposes a simple API to send emails with SendGrid with a blocking or non-blocking way.

You can use it inside your Actix, Axum or Rocket application without any problems.

To get the API key, you need to create an account on SendGrid and create an API key.

I recommend the [dotenvy](https://crates.io/crates/dotenvy) crate to load the API key from an environment variable.

## Usage


```rust
    use sendgrid_thin::{Sendgrid, ContentType};

    #[tokio::main]
    async fn main() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY")
            // Required
            .set_to_emails(["to_email_1@example.com", "to_email_2@example.com"])
            .set_from_email("from_email@example.com")
            .set_subject("subject of email")
            .set_body("body of email")
            // Optional
            .set_content_type(ContentType::Html)
            .set_send_at(1668281500)
            .set_cc_emails(["cc_email_1@example.com", "cc_email_2@example.com"]);

        // Send the email with a non-blocking client
        match sendgrid.send().await {
            Ok(message) => println!("{message}"),
            Err(err) => println!("Error sending email: {err}"),
        }

        // Send the email with a blocking client (in this case the main function cannot be async)
        match sendgrid.send_blocking() {
            Ok(message) => println!("{message}"),
            Err(err) => println!("Error sending email: {err}"),
        }
    }
```