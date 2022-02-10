use clap::Parser;

use crate::twitter::TwitterClient;

mod twitter;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,

    #[clap()]
    topic: String,

    #[clap(short)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    let twitter_client = TwitterClient::new(args.token);

    let tweets = twitter_client.get_tweets_for_topic(&args.topic).await?;
    let n_tweets = tweets.len() as f64;

    let mut compound_sum = 0f64;
    for tweet in tweets {
        let scores = analyzer.polarity_scores(&tweet.text);
        compound_sum += scores.get("compound").unwrap_or(&0f64);

        if args.verbose {
            println!("{}\n{:?}\n---\n", &tweet.text.replace("\n", " "), scores);
        }
    }

    let score = compound_sum / n_tweets;
    println!("{score}");
    Ok(())
}
