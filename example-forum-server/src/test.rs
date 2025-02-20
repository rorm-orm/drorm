use rorm::fields::types::MaxStr;
use tracing::info;

use crate::handler::thread::{CreateThreadRequest, ListResponse, MakePostRequest};
use crate::handler::user::{LoginRequest, ProfileResponse, RegisterRequest};

const ORIGIN: &str = "http://localhost:8000";
const USERNAME: &str = "alice";
const PASSWORD: &str = "password";
const THREAD_NAME: &str = "Cats";

pub async fn run_test_client() -> anyhow::Result<()> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let response = client
        .post(format!("{ORIGIN}/api/user/login"))
        .json(&LoginRequest {
            username: USERNAME.to_string(),
            password: PASSWORD.to_string(),
        })
        .send()
        .await?;
    assert!(!response.status().is_server_error());
    if response.status().is_client_error() {
        client
            .post(format!("{ORIGIN}/api/user/register"))
            .json(&RegisterRequest {
                username: MaxStr::new(USERNAME.to_string()).unwrap(),
                password: PASSWORD.to_string(),
            })
            .send()
            .await?;

        info!("Created new user account");
    } else {
        info!("Logged into existing user account")
    }

    let ProfileResponse {
        username,
        role,
        posts: _,
    } = client
        .get(format!("{ORIGIN}/api/user/profile/{USERNAME}"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    assert_eq!(username, USERNAME);
    assert_eq!(role, "user");

    let ListResponse { threads } = client
        .get(format!("{ORIGIN}/api/thread/list"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let thread_id = if let Some(x) = threads.iter().find(|thread| thread.name == THREAD_NAME) {
        info!("Found existing thread");
        x.identifier.clone()
    } else {
        let id = client
            .post(format!("{ORIGIN}/api/thread/create"))
            .json(&CreateThreadRequest {
                name: THREAD_NAME.to_string(),
            })
            .send()
            .await?
            .error_for_status()?
            .json::<String>()
            .await?;

        info!("Created new thread");
        id
    };

    client
        .post(format!("{ORIGIN}/api/thread/posts/{thread_id}"))
        .json(&MakePostRequest {
            message: "Look at this cute cat picture I found".to_string(),
            reply_to: None,
        })
        .send()
        .await?
        .error_for_status()?;

    info!("Submitted a new post");

    Ok(())
}
