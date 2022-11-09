# sendgrid_thin

A thin wrapper around the SendGrid API.

It does not use the crate `tokio` or `hyper` and is therefore very lightweight and do not interfere with your existing runtime.

You can use it inside your Actix, Axum or Rocket application without any problems.

It's also very easy to use.

## Usage

```rust
     use sendgrid_thin::Sendgrid;

     fn main() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
        sendgrid.set_content_type(ContentType::Html);

        match sendgrid.send_mail("subject text", "body content") {
            Ok(_) => println!("Email sent successfully"),
            Err(err) => println!("Error sending email: {}", err),
        }
     }
