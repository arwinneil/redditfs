use reqwest;
use serde_json::Value;

pub struct Client {}

pub struct Post {
    pub title: String,
    pub text: String,
    pub url: String,
}

impl Client {
    pub fn get_posts(subreddit: &str, limit: &str) -> Vec<Post> {
        let respose = reqwest::blocking::get(
            &[
                "https://www.reddit.com/r/",
                subreddit,
                ".json?limit=",
                &limit,
            ]
            .concat(),
        )
        .unwrap()
        .text()
        .unwrap();

        let post_json: Value = serde_json::from_str(&respose).unwrap();

        let mut posts: Vec<Post> = Vec::new();

        for i in 0..limit.parse::<usize>().unwrap() {
            posts.push(Post {
                title: post_json["data"]["children"][i]["data"]["title"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                text: post_json["data"]["children"][i]["data"]["selftext"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                url: post_json["data"]["children"][i]["data"]["url"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            });
        }

        // for i in 0..limit.parse::<usize>().unwrap() {
        //     println!("Retrieved post : {}", posts[i].title);
        // }

        return posts;
    }
}
