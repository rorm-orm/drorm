use std::env::temp_dir;
use std::fs;
use std::future::{poll_fn, Future};
use std::hash::{BuildHasher, Hasher, RandomState};
use std::pin::pin;
use std::task::Poll;
use std::time::Duration;

use example_forum_server::{run_main, Cli, Command};
use serde_json::json;
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn test_example_forum_server() {
    let working_dir = temp_dir().join(format!(
        "test-rorm-example-forum-server-{}",
        RandomState::new().build_hasher().finish()
    ));
    fs::create_dir(&working_dir).unwrap();

    let db_sqlite = working_dir.join("db.sqlite").display().to_string();
    let db_config = working_dir.join("db_config.json").display().to_string();
    serde_json::to_writer(
        fs::File::create(&db_config).unwrap(),
        &json!({
            "Driver": "SQLite",
            "Filename": db_sqlite,
        }),
    )
    .unwrap();

    let migrations_dir = working_dir.join("migrations").display().to_string();
    fs::create_dir(&migrations_dir).unwrap();
    run_main(Cli {
        db_config: Some(db_config.clone()),
        command: Command::MakeMigrations {
            migrations_dir: migrations_dir.clone(),
        },
    })
    .await
    .unwrap();
    run_main(Cli {
        db_config: Some(db_config.clone()),
        command: Command::Migrate {
            migrations_dir: migrations_dir.clone(),
        },
    })
    .await
    .unwrap();

    let mut server_future = pin!(run_main(Cli {
        db_config: Some(db_config.clone()),
        command: Command::Start {},
    }));
    let mut client_future = pin!(async move {
        sleep(Duration::from_millis(500)).await;
        run_main(Cli {
            db_config: Some(db_config.clone()),
            command: Command::Test {},
        })
        .await?;
        run_main(Cli {
            db_config: Some(db_config.clone()),
            command: Command::Test {},
        })
        .await
    });

    timeout(
        Duration::from_secs(10),
        poll_fn(|ctx| {
            match (
                client_future.as_mut().poll(&mut *ctx),
                server_future.as_mut().poll(&mut *ctx),
            ) {
                (Poll::Pending, Poll::Pending) => Poll::Pending,
                (Poll::Ready(client_result), _) => Poll::Ready(client_result),
                (Poll::Pending, Poll::Ready(Err(server_error))) => Poll::Ready(Err(server_error)),
                (Poll::Pending, Poll::Ready(Ok(()))) => panic!("Server should not shut down"),
            }
        }),
    )
    .await
    .unwrap()
    .unwrap();

    fs::remove_dir_all(&working_dir).unwrap();
}
