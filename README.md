# topic-sentiment
A simple command line program that outputs current sentiment about a certain topic based on the 100 most recent tweets about it. The output is a normalized, weighted composite score between -1 (negative) and +1 (positive). It uses [vader-sentiment-rust](https://github.com/ckw017/vader-sentiment-rust) to compute the score.

[![CI](https://github.com/eliefaart/topic-sentiment/actions/workflows/ci.yml/badge.svg)](https://github.com/eliefaart/topic-sentiment/actions/workflows/ci.yml)

## Usage
```topic-sentiment.exe --token <token> <topic> [-v]```

Where 'token' is a Twitter API bearer token.