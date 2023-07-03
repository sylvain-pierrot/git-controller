use std::process::{exit, Command};

#[derive(Debug, PartialEq)]
struct Acl {
    user: String,
    repo: String,
}

fn main() {
    let key = "SSH_ORIGINAL_COMMAND";

    // Extract args there should be the command itself and a string to identify the user
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("[paastech] Unexpected error");
        exit(1);
    }

    // Acls for demo purpose
    let acls: Vec<Acl> = vec![
        Acl {
            user: "userA".to_string(),
            repo: "/srv/git/repoA.git".to_string(),
        },
        Acl {
            user: "userB".to_string(),
            repo: "/srv/git/repoB.git".to_string(),
        },
    ];

    // Extract ssh_command
    let raw_ssh_command = std::env::var(key).unwrap_or_else(|_| {
        eprintln!("[paastech] Unexpected error");
        std::process::exit(1);
    });

    let ssh_command = Vec::from_iter(raw_ssh_command.split_whitespace());

    // Checks if the command executed when connecting over ssh is git-receive-pack
    if ssh_command.len() != 2 || ssh_command[0] != "git-receive-pack" {
        eprintln!("[paastech] Bad request, you can only access this server via the git cli");
        exit(1);
    }

    let acl = Acl {
        user: args[1].to_owned(),
        repo: ssh_command[1].to_owned().replace('\'', ""),
    };

    if !acls.contains(&acl) {
        eprintln!("[paastech] Forbidden");
        exit(1);
    }

    // Execute git-receive-pack
    Command::new("git-receive-pack")
        .arg(&acl.repo)
        .status()
        .ok();
}
