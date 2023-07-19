// src\main.rs
use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::SyndicationClient
    };

fn main() -> windows::core::Result<()> {
    let uri = Uri::CreateUri(h!("http://blogs.windows.com/feed"))?;
    let client = SyndicationClient::new()?;

    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; WOW64; Trident/6.0)"),
    )?;

        let feed = client.RetrieveFeedAsync(&uri)?.get()?;

        for item in feed.Items()? {
            println!("{}", item.Title()?.Text()?);
        }
    

    Ok(())
}
