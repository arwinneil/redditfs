use crate::reddit::{Client, Post};
use convert_case::{Case, Casing};

pub struct FileController {}

pub struct PostFile {
    pub post: Post,
    pub filename: String,
}

impl FileController {
    pub fn generate_post_flies(subreddit: String, limit: String) -> Vec<PostFile> {
        let posts = Client::get_posts(&subreddit, &limit);

        let mut post_files: Vec<PostFile> = Vec::new();

        for post in posts.into_iter() {
            println!("Retrieved Post {}", post.title);

            post_files.push(PostFile {
                filename: FileController::extract_filenames(&post.title),
                post: post,
            })
        }

        return post_files;
    }

    fn extract_filenames(title: &str) -> String {
        let mut filename = title.replace(&['.'][..], "");
        filename = [&(filename.to_case(Case::Snake)), ".txt"].concat();
        filename = filename.replace(&['(', ')', ',', '\"', '?', ';', ':', '\''][..], ""); //TODO @arinnwil RegEx that

        //TODO Shorten long names

        println!("Generated Filename : {}", filename);
        return filename;
    }
}
