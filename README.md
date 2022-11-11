# sendgrid_thin

A thin wrapper around the SendGrid V3 API.

It does not use the crate `tokio` or `hyper` and is therefore very lightweight and do not interfere with your existing runtime.

You can use it inside your Actix, Axum or Rocket application without any problems.

To get the API key, you need to create an account on SendGrid and create an API key.

I recommend the [dotenvy](https://crates.io/crates/dotenvy) crate to load the API key from an environment variable.

## Usage

```rust
     use sendgrid_thin::Sendgrid;

     fn main() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
        sendgrid.set_content_type(ContentType::Html);
        sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);

        match sendgrid.send("subject text", "body content") {
            Ok(_) => println!("Email sent successfully"),
            Err(err) => println!("Error sending email: {}", err),
        }
     }
```
