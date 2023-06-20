use std::fs;
use std::process::Command;

use paastech_proto::gitsprout::git_sprout_server::GitSprout;
use paastech_proto::gitsprout::{RepositoryRequest, RepositoryResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GitSproutServiceConfig {
    pub git_repository_base_path: String,
}

#[derive(Debug, Default)]
pub struct GitSproutService {
    pub config: GitSproutServiceConfig,
}

type GitSproutResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl GitSprout for GitSproutService {
    async fn create_repository(
        &self,
        request: Request<RepositoryRequest>,
    ) -> GitSproutResult<RepositoryResponse> {
        let request_data = request.into_inner();

        let new_repo_path = format!(
            "{}/{}",
            self.config.git_repository_base_path.clone(),
            request_data.repository_path
        );

        // Check if the repository already exists, if it does return error otherwise continue
        if let Ok(_) = fs::metadata(new_repo_path.clone()) {
            return Err(Status::already_exists(format!(
                "repository {} already exists",
                new_repo_path.clone(),
            )));
        }

        // Initialize empty git repository if it fails rollback and return error otherwise continue
        if Command::new("sh")
            .arg("-c")
            .arg(format!("git init --bare {}", new_repo_path.clone()))
            .output()
            .is_err()
        {
            return Err(Status::unknown(
                "Unknown error occured : Failed initializing repository",
            ));
        }

        let reply = RepositoryResponse {
            message: format!("Created repository {}", request_data.repository_path).to_owned(),
        };

        Ok(Response::new(reply))
    }

    async fn delete_repository(
        &self,
        _: Request<RepositoryRequest>,
    ) -> Result<Response<RepositoryRequest>, Status> {
        Err(Status::unimplemented(""))
    }
}
