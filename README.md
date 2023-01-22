# sendgrid_thin

[![Tests](https://github.com/OLoKo64/sendgrid_thin/actions/workflows/rust-workflow.yml/badge.svg)](https://github.com/OLoKo64/sendgrid_thin/actions/workflows/rust-workflow.yml)

A thin wrapper around the SendGrid V3 API.

It does not use the crate `tokio` or `hyper` and is therefore very lightweight and do not interfere with your existing runtime.

You can use it inside your Actix, Axum or Rocket application without any problems.

To get the API key, you need to create an account on SendGrid and create an API key.

I recommend the [dotenvy](https://crates.io/crates/dotenvy) crate to load the API key from an environment variable.

## Usage


```rust
    use sendgrid_thin::{Sendgrid, ContentType};

    fn main() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");

        // Required
        sendgrid
            .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
            .set_from_email("from_email@example.com")
            .set_subject("subject of email")
            .set_body("body of email");

        // Optional
        sendgrid
            .set_content_type(ContentType::Html)
            .set_send_at(1_668_281_500)
            .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"]);

        // Send the email
        match sendgrid.send() {
            Ok(message) => println!("{}", message),
            Err(err) => println!("Error sending email: {}", err),
        }
    }
```