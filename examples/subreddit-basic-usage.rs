use roux::Subreddit;
#[cfg(feature = "async")]
use tokio;

// #[cfg_attr(feature = "async", tokio::main)]
#[tokio::main]
#[maybe_async::maybe_async]
async fn main() {
    let subreddit = Subreddit::new("rust");
    // Now you are able to:

    // Get moderators.
    let moderators = subreddit.moderators().await;

    // Get hot posts with limit = 25.
    let hot = subreddit.hot(25, None).await;

    // Get comments from a submission.
    let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
    let article_comments_result = subreddit.article_comments(article_id, None, Some(25)).await;

    // Comments = BasicListing<CommentData>
    // BasicListing<T> = BasicThing<Listing<BasicThing<T>>>
    // BasicThing<T> kind: Option<String>, data: T
    let article_comments = match article_comments_result {
        Ok(comments) => comments,
        Err(_) => {
            eprintln!("[ERROR]: unable to get comments from article id");
            std::process::exit(1);
        }
    };
    // listing: modhash, dist, after, before, children
    //  Listing<BasicThing<T>>>    // BasicThing
    let article_comments_listing = article_comments.data;
    //  Vec<BasicThing<CommentData>>                    <Listing<T>> where T = BasicThing<CommentData>
    let article_comments_listing_children = article_comments_listing.children;
    for basic_thing_comment_data in article_comments_listing_children {
        let option_author = basic_thing_comment_data.data.author;
        let author = match option_author {
            Some(a) => {
                println!("the author is: {}", a);
                Some(a)
            }
            None => {
                println!("no author, probably [deleted]");
                None
            }
        };
        

    }
    // let submission_data = hot.unwrap();

    // Get rising posts with limit = 30.
    let rising = subreddit.rising(30, None).await;

    // Get top posts with limit = 10.
    let top = subreddit.top(10, None).await;

    // Get latest comments.
    // `depth` and `limit` are optional.
    let latest_comments = subreddit.latest_comments(None, Some(25)).await;


}