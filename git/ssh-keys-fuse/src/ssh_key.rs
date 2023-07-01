use sqlx::FromRow;

#[derive(FromRow)]
pub struct SSHKey {
    user_id: String,
    value: String,
}
