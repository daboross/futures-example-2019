#![feature(async_await)]
use std::str;

use futures::{
    compat::{Future01CompatExt, Stream01CompatExt},
    future, FutureExt, TryFutureExt, TryStreamExt,
};
use hyper::{Chunk, Client};
use hyper_tls::HttpsConnector;

// use this as our error type
type Error = Box<std::error::Error>;

fn main() {
    tokio::run(
        entry()
            .then(|res| {
                if let Err(e) = res {
                    panic!("error occurred: {0} ({0:?})", e);
                }

                future::ok(())
            })
            .boxed()
            .compat(),
    );
}

async fn entry() -> Result<(), Error> {
    let hyper = Client::builder().build::<_, hyper::Body>(HttpsConnector::new(4)?);

    let response = hyper
        .get("https://jsonplaceholder.typicode.com/todos/1".parse()?)
        .compat()
        .await?;

    let mut body = response.into_body().compat();

    let mut whole_contents = Chunk::default();

    // example of iterating a stream
    while let Some(part) = body.try_next().await? {
        whole_contents.extend(part);
    }

    println!("response: {}", str::from_utf8(&whole_contents)?);

    Ok(())
}
