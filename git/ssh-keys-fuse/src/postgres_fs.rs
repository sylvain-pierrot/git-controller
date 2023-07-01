use crate::ssh_key::SSHKey;
use fuse3::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request,
};
use libc::ENOENT;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use async_trait::async_trait;

const TTL: Duration = Duration::from_secs(1); // 1 second

pub struct PostgresFS {
    // git_auth_binary_path: String,
    connection: Pool<Postgres>,
}

// définition du fichier .ssh
const DOT_SSH_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH,
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o600,
    nlink: 2,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

// definition du fichier .ssh/authorized_keys
// TODO rendre ca dynamique
const AUTHORIZED_KEYS_ATTR: FileAttr = FileAttr {
    ino: 2,
    // important pour read le fichier, quand on cat par exemple, le buffer pour lire est de la taille spécifiée ici.
    size: 5,
    blocks: 1,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o600,
    nlink: 1,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

#[async_trait]
impl Filesystem for PostgresFS {

    async fn init(&self, _req: Request) -> Result<()> {
        Ok(())
    }

    async fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("authorized_keys") {
            reply.entry(&TTL, &AUTHORIZED_KEYS_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
        
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &DOT_SSH_DIR_ATTR),
            2 => reply.attr(&TTL, &AUTHORIZED_KEYS_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        if ino == 2 {
            // reply.data(&self.get_content().await.as_bytes()[offset as usize..]);
            reply.data(&"hello world\n".as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "authorized_keys"),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                break;
            }
        }

        reply.ok();
    }
}

impl PostgresFS {
    pub async fn new(url_connection_string: String) -> Result<Self, Error> {
        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url_connection_string)
            .await?;

        Ok(PostgresFS { connection })
    }

    async fn get_ssh_keys(&self) -> Result<Vec<SSHKey>, sqlx::Error> {
        let result = sqlx::query_as::<Postgres, SSHKey>("SELECT * FROM ssh_keys")
            .fetch_all(&self.connection)
            .await?;

        Ok(result)
    }

    pub fn mount(filesystem: PostgresFS, mountpoint: String) {
        fuser::mount2::<PostgresFS, String>(filesystem, mountpoint, &[MountOption::AutoUnmount])
            .unwrap();
    }

    // /// Query
    // async fn get_ssh_keys(&self) -> Result<Vec<SSHKey>, Error> {
    //     self.connection.fetch_all("SELECT user_id, value FROM ssh_keys;").await?
    // }k
    // /// Build le contenu du fichier à partir de la requête
    // async fn get_content(&self) -> String {
    //     let content: Vec<SSHKey> = match self.query_content().await {
    //         Ok(content) => content,
    //         Err(_) => Vec::new(),
    //     };

    //     let mut result = String::new();
    //     for row in content {
    //         result += &format!(
    //             r#"command="{} {}" {}"#,
    //             &self.git_auth_binary_path, row.user_id, row.value
    //         );
    //         result += "\n";
    //     }

    //     result
    // }
    // /// Récupérer la taille de authorized keys
    // async fn get_content_size(&self) -> usize {
    //     self.get_content().await.len()
    // }
}
