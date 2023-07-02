use clap::Parser;
use color_eyre::eyre::Result;
use roux::saved::SavedData;
use roux::util::FeedOption;
use roux::Reddit;
use std::fs::{create_dir_all, File};
use std::io::{prelude::*, stdout, BufWriter};
use tracing::debug;

mod cli;
mod op;

use crate::cli::Cli;
use crate::op::reddit_credentials;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();

    debug!("Logging into Reddit...");

    let credentials = reddit_credentials("reddit")?;

    let user_agent = format!("macOS:saved-exporter:0.1 (by {})", credentials.username);

    let reddit = Reddit::new(&user_agent, &credentials.client_id, &credentials.client_secret)
        .username(&credentials.username)
        .password(&format!("{}:{}", &credentials.password, &credentials.totp))
        .login()?;

    let mut buffer: BufWriter<Box<dyn Write>> = match cli.output {
        Some(path) => {
            create_dir_all(&path)?;
            BufWriter::new(Box::new(File::create(&path)?))
        }
        None => BufWriter::new(Box::new(stdout())),
    };

    debug!("Extracting saved posts...");

    let mut after = None;

    loop {
        let posts = reddit.saved(Some(FeedOption {
            after: after.clone(),
            before: None,
            count: None,
            period: None,
            limit: Some(50),
        }))?;

        if posts.data.children.is_empty() {
            println!("loaded all new posts, ending");
            break;
        }

        for post in posts.data.children.iter() {
            if let SavedData::Submission(ref submission) = post.data {
                if cli.subreddit.clone().is_some_and(|sub| submission.subreddit != sub) {
                    continue;
                }

                writeln!(buffer, "https://reddit.com{}", submission.permalink)?
            }
        }

        if posts.data.after.is_none() {
            debug!("No more posts, ending loop.");
            break;
        }

        after = posts.data.after;
    }

    debug!("Finished Downloading!");

    Ok(buffer.flush()?)
}
