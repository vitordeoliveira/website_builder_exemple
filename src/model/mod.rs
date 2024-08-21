use sqlx::PgPool;
use sqlx::Row;
use tracing::instrument;
// use tracing::Instrument;

#[instrument]
pub async fn exemple_service(pg_pool: &PgPool) -> String {
    let result = sqlx::query(r#"select 'hello world from pg' as "message!""#)
        .fetch_one(pg_pool)
        // .instrument(tracing::info_span!("db_query"))
        .await
        .unwrap();

    result.get("message!")
}
