use cch23_weshnaw::router::router;
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::fmt().without_time().init();
    sqlx::migrate!().run(&pool).await.unwrap();

    Ok(router(persist, pool).into())
}
