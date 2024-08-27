use axum_test::TestServer;
use ctor::dtor;
use tokio::sync::OnceCell;
use website::app::new_app;

use testcontainers::{core::WaitFor, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt};

type PostgresContainer = OnceCell<ContainerAsync<GenericImage>>;

static INSTANCE_POSTGRES: PostgresContainer = OnceCell::const_new();
pub async fn postgres() -> &'static ContainerAsync<GenericImage> {
    INSTANCE_POSTGRES
        .get_or_init(|| async {
            GenericImage::new("postgres", "14.1")
                .with_wait_for(WaitFor::message_on_stdout(
                    "database system is ready to accept connections",
                ))
                .with_wait_for(WaitFor::seconds(5))
                .with_env_var("POSTGRES_USER", "test_user")
                .with_env_var("POSTGRES_PASSWORD", "test_password")
                .with_env_var("POSTGRES_DB", "test_db")
                .with_container_name("integration_test_postgres")
                .start()
                .await
                .expect("Postgres start")
        })
        .await
}

#[dtor]
fn shutdown() {
    std::process::Command::new("docker")
        .arg("kill")
        .arg("integration_test_postgres")
        .output()
        .expect("failed to kill container");

    std::process::Command::new("docker")
        .arg("rm")
        .arg("integration_test_postgres")
        .output()
        .expect("failed to kill container");
}

pub async fn setup() -> TestServer {
    let conteiner = postgres().await;

    let port = conteiner.get_host_port_ipv4(5432).await.unwrap();

    let db_connection = format!("postgres://test_user:test_password@localhost:{port}/test_db");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(&db_connection, assets_path).await;

    TestServer::new(app).unwrap()
}
