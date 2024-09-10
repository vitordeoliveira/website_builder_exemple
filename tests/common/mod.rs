// use std::sync::{Arc, Mutex, MutexGuard};

use std::sync::RwLock;

use axum_test::TestServer;
use ctor::dtor;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;
use website::app::new_app;

use testcontainers::{core::WaitFor, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt};

// type PostgresContainer = OnceCell<ContainerAsync<GenericImage>>;
//
// static INSTANCE_POSTGRES: PostgresContainer = OnceCell::const_new();
// pub async fn postgres() -> &'static ContainerAsync<GenericImage> {
//     INSTANCE_POSTGRES
//         .get_or_init(|| async {
//             GenericImage::new("postgres", "14.1")
//                 .with_wait_for(WaitFor::message_on_stdout(
//                     "database system is ready to accept connections",
//                 ))
//                 .with_wait_for(WaitFor::seconds(5))
//                 .with_env_var("POSTGRES_USER", "test_user")
//                 .with_env_var("POSTGRES_PASSWORD", "test_password")
//                 .with_env_var("POSTGRES_DB", "test_db")
//                 .with_container_name("integration_test_postgres")
//                 .start()
//                 .await
//                 .expect("Postgres start")
//         })
//         .await
// }

static STATIC_INSTANCE: Lazy<RwLock<Option<ContainerAsync<GenericImage>>>> = Lazy::new(|| {
    RwLock::new(None) // Initialize with None
});

async fn init() {
    let mut instance = STATIC_INSTANCE.write().unwrap();

    if instance.is_none() {
        *instance = Some(
            GenericImage::new("postgres", "14.1")
                .with_wait_for(WaitFor::message_on_stdout(
                    "database system is ready to accept connections",
                ))
                .with_wait_for(WaitFor::seconds(5))
                .with_env_var("POSTGRES_USER", "test_user")
                .with_env_var("POSTGRES_PASSWORD", "test_password")
                .with_env_var("POSTGRES_DB", "test_db")
                .start()
                .await
                .expect("Postgres start"),
        );
    };
}

//
#[dtor]
fn drop_instance() {
    println!("Program terminating, dropping static instance...");

    let container = STATIC_INSTANCE.read().unwrap();

    let id = container.as_ref().unwrap().id();

    std::process::Command::new("docker")
        .arg("kill")
        .arg(id)
        .output()
        .expect("failed to kill container");

    std::process::Command::new("docker")
        .arg("rm")
        .arg(id)
        .output()
        .expect("failed to kill container");
}

pub async fn setup() -> TestServer {
    // let container = postgres().await;

    let _ = init().await;

    let container = STATIC_INSTANCE.read().unwrap();

    let port = container
        .as_ref()
        .unwrap()
        .get_host_port_ipv4(5432)
        .await
        .unwrap();

    let db_connection = format!("postgres://test_user:test_password@localhost:{port}/test_db");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(&db_connection, assets_path).await.unwrap();

    TestServer::new(app).unwrap()
}
