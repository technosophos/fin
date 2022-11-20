use anyhow::{anyhow, Result};
use http::header::{HeaderMap, AUTHORIZATION};

// Authenticate a user and return the username
// If anything goes wrong during auth, this will throw an error
pub fn auth_user(headers: &HeaderMap) -> Result<String> {
    // Right now, we get the password from an env var. Not the best....
    let expect_password = std::env::var("ADMIN_PASSWORD")?;

    // Grab the HTTP header and see what we have.
    match headers.get(AUTHORIZATION) {
        Some(auth_header) => {
            // Printlns can be useful when you are not sure whether the issue is that the
            // env var isn't being loader or that the password does not match.
            println!("Found auth header");
            let basic_creds = auth_header.to_str()?.to_owned();
            let creds = http_auth_basic::Credentials::from_header(basic_creds)
                .map_err(|_| anyhow!("Failed to get auth from header"))?;
            println!("Got password");
            if creds.password == expect_password {
                Ok(creds.user_id)
            } else {
                println!("failed password");
                anyhow::bail!("Failed auth")
            }
        }
        None => {
            println!("No header found");
            anyhow::bail!("Failed auth")
        }
    }
}
