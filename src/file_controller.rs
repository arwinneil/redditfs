use crate::reddit::{Client, Post};
use convert_case::{Case, Casing};
use fuser::{FileAttr, FileType};
use std::time::{Duration, UNIX_EPOCH};

pub struct FileController {}

pub struct PostFile {
    pub post: Post,
    pub filename: String,
    pub fileattr: FileAttr,
}

impl FileController {
    pub fn generate_post_flies(subreddit: String, limit: String) -> Vec<PostFile> {
        let posts = Client::get_posts(&subreddit, &limit);

        let mut post_files: Vec<PostFile> = Vec::new();

        for post in posts.into_iter() {
            println!("Retrieved Post : {}", post.title);

            post_files.push(PostFile {
                filename: FileController::extract_filenames(&post.title),
                fileattr: FileController::get_file_attr(&post.created, post.score),
                post: post,
            })
        }

        return post_files;
    }

    fn extract_filenames(title: &str) -> String {
        let mut filename = title.replace(&['.'][..], "");
        filename = filename.replace(&['(', ')','’','“','”', ',', '\"', '?', ';', ':','!', '\''][..], ""); //TODO @arwinneil RegEx that
        filename = [&(filename.to_case(Case::Snake)), ".txt"].concat();

        println!("Generated Filename : {}", filename);
        return filename;
    }

    fn get_file_attr(created: &str, score : u64) -> FileAttr {
        let duration = Duration::from_secs(created.parse::<u64>().unwrap());

        let attr = FileAttr {
            ino: 2,
            size: score,
            blocks: 1,
            atime: UNIX_EPOCH + duration,
            mtime: UNIX_EPOCH + duration,
            ctime: UNIX_EPOCH + duration,
            crtime: UNIX_EPOCH + duration,
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 501,
            gid: 9,
            rdev: 0,
            flags: 0,
            blksize: 512,
            padding: 0,
        };

        return attr;
    }
}
