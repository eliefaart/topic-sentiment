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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();

    let twitter_client = TwitterClient::new(args.token);
    let tweets = twitter_client.get_tweets_for_topic(&args.topic).await?;
    let n_tweets = tweets.len() as f64;

    let mut score_neutral = 0f64;
    let mut score_positive = 0f64;
    let mut score_negative = 0f64;

    for tweet in tweets {
        let scores = analyzer.polarity_scores(&tweet.text);
        score_neutral += scores.get("neu").unwrap_or(&0f64);
        score_positive += scores.get("pos").unwrap_or(&0f64);
        score_negative += scores.get("neg").unwrap_or(&0f64);

        println!("{}\n---", &tweet.text.replace("\n", " "));
    }

    score_neutral = score_neutral / n_tweets;
    score_positive = score_positive / n_tweets;
    score_negative = score_negative / n_tweets;

    println!("\nTopic: {}", &args.topic);
    println!("\tneutral: {score_neutral}");
    println!("\tpositive: {score_positive}");
    println!("\tnegative: {score_negative}");

    Ok(())
}
