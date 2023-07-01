mod postgres_fs;
mod ssh_key;

use dotenv::dotenv;
use postgres_fs::PostgresFS;

// TODO SQL SYNC OU ASYNC FUSE
// Dockerisation possible ? pas sur
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // This line loads the environment variables

    // Initialize the filesystem
    let url_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let filesystem = PostgresFS::new(url_connection_string).await?;

    // URL connection string
    // git_auth_binary_path: std::env::var(AUTH_LAYER_PATH).expect("AUTH_LAYER_PATH must be set."),

    // filesystem.connection.
    // Mount the filesystem
    let mountpoint = std::env::var("MOUNT_POINT").expect("MOUNTPOINT must be set.");
    PostgresFS::mount(filesystem, mountpoint);

    Ok(())
}
