use fuser::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use libc::ENOENT;
use std::env;
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
mod file_controller;
mod reddit;

const TTL: Duration = Duration::from_secs(1); // 1 second

const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
    padding: 0,
};

const HELLO_TXT_CONTENT: &str = "Hello World!\n";

struct RedditFS {
    post_files: Vec<file_controller::PostFile>,
}

impl Filesystem for RedditFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 {
            let index = self
                .post_files
                .iter()
                .position(|post| name.to_str().unwrap().to_string() == post.filename);

            reply.entry(&TTL, &self.post_files[index.unwrap()].fileattr, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => (),
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
        reply: ReplyData,
    ) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
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

        let mut entries = vec![
            (1, FileType::Directory, ".".to_string()),
            (1, FileType::Directory, "..".to_string()),
        ];

        for i in 0..self.post_files.len() {
            entries.push((
                1,
                FileType::RegularFile,
                self.post_files[i].filename.clone(),
            ));
        }

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            reply.add(
                entry.0,
                (i + 1) as i64,
                entry.1,
                entry.2.as_str().to_owned(),
            );
        }
        reply.ok();
    }
}

fn main() {
    //env_logger::init();
    let mountpoint = env::args_os().nth(2).unwrap().into_string().unwrap();

    let options = ["-o", "ro", "-o", "fsname=redditfs"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    fuser::mount(
        RedditFS {
            post_files: file_controller::FileController::generate_post_flies(
                env::args_os().nth(1).unwrap().into_string().unwrap(),
                env::args_os().nth(3).unwrap().into_string().unwrap(),
            ),
        },
        mountpoint,
        &options,
    )
    .unwrap();
}
