use std::{fmt::Error, env::VarError, io, fs};
use futures::StreamExt;
use hyper::body::Buf;
use reqwest::{Client};
use scraper::{Html, Selector};
use lazy_static::lazy_static;
use tokio::{fs::{OpenOptions, File}, io::AsyncWriteExt};


lazy_static!{
    static ref HttpClient: Client = Client::new();
}


pub async fn getHttpDocument(uri:& str) -> Result<scraper::Html, VarError>
{
    eprintln!("Getting document for page with uri {}", uri);
    let mut resp = HttpClient.get(uri).send().await;
    if (resp.is_err())
    {
        Err::<String, VarError>(VarError::NotPresent);
    }
    let res_t = resp.expect("resp unavailable, throwing.").text().await.expect("resp unavailable, throwing.");
    
    Ok(Html::parse_document(res_t.as_str()))
    
}
pub fn getEmptyDoc() -> Html
{
    Html::new_document()
}
pub async fn downloadTo(url: &str, to: &str)
{
    eprintln!("Trying download {}->{}", url, to);
    let mut bytes = HttpClient.get(url).send().await.unwrap().bytes_stream();
    let mut f = fs::File::create(to).unwrap();
    eprintln!("Sent request and opened file...");
    while let Some(buf) = bytes.next().await
    {
        eprintln!("outb -> {}", to);
        io::copy(&mut buf.unwrap().reader(), &mut f);
    }
    eprintln!("fin {}", to);

    
}