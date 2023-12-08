use cch23_weshnaw::router::router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(router().into())
}
