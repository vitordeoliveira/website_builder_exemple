use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pg_pool: PgPool,
}
