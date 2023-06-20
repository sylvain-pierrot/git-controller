mod service;

use dotenv::dotenv;
use paastech_proto::gitsprout::git_sprout_server::GitSproutServer;
use service::{GitSproutService, GitSproutServiceConfig};
use tonic::transport::Server;

fn load_env_into_config() -> GitSproutServiceConfig {
    let git_repository_base_path_key = "GITSPROUT_GIT_REPOSITORY_BASE_PATH";
    GitSproutServiceConfig {
        git_repository_base_path: std::env::var(git_repository_base_path_key).unwrap(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port_key = "GITSPROUT_SERVER_PORT";
    let addr = format!("[::1]:{}", std::env::var(port_key).unwrap()).parse()?;

    let gitsprout_config = load_env_into_config();

    let gitsprout_service = GitSproutService {
        config: gitsprout_config,
    };

    Server::builder()
        .add_service(GitSproutServer::new(gitsprout_service))
        .serve(addr)
        .await?;

    Ok(())
}
