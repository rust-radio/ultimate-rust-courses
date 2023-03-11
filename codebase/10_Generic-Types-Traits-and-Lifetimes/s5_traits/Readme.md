# 10.2 Traits: Defining Shared Behavior

-   [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)

## Defining, implementing traits on type and create an instance

For example, let’s say we have multiple structs that hold various kinds and amounts of text: a NewsArticle struct that holds a news story filed in a particular location and a Tweet that can have at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet.

We want to make a media s5_traits library crate named `s5_traits` that can display `summaries` of data that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance.

Trying to use the `Summary` trait and the `Tweet` struct from the `aggregator` crate in your `main.rs` file, you need to: 

1.  `src/lib.rs` - Implementing a Trait on a Type

2.  `src/main.rs` - Create an instance of the Tweet struct and call the summarize method on it:




