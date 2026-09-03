use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Settings {
    key: String,
}
