use crate::downloader::http;
use futures::{Future, StreamExt};
use lazy_static::{__Deref, lazy_static};
use scraper::{ElementRef, Selector};
use std::{borrow::Cow, convert::TryInto, fmt::format, path::Path};
use tokio::runtime::Runtime;
use url::{Url, UrlQuery};
static BASE_URI: &str = "http://safebooru.org/";
static SEARCH_PAGE: &str = "?page=post&s=list";
lazy_static! {
    static ref SEARCH_IMAGE_SELECTOR: Selector =
        getSelector("span.thumb[id] a[id][href*=\"s=view\"]");
    static ref IMAGE_POST_IMAGE: Selector = getSelector("#image");
    static ref IMAGE_POST_HD_LINK: Selector = getSelector("a[href*=\"image\"]");
    static ref SYNC_RUNTIME: Runtime = Runtime::new().unwrap();
}
fn getSelector(selector: &str) -> Selector {
    let sel = Selector::parse(selector).unwrap();
    sel
}
pub async fn test() {
    let doc = http::getHttpDocument("http://safebooru.org/")
        .await
        .ok()
        .unwrap();
}
fn addTag(mut url: &str, tag: &str) -> String {
    let mut uri = Url::parse(url).expect("url parse while addTag failed");
    let tags_str = uri
        .query_pairs()
        .find(|x| x.0 == "tags")
        .unwrap_or_else(|| (Cow::Borrowed("tags"), Cow::Borrowed("")))
        .1
        .split("+")
        .chain([tag])
        .fold(String::new(), |a, b| a + "+" + b);
    let a = uri
        .query_pairs_mut()
        .append_pair("tags", &tags_str)
        .finish();
    let b = a.as_str().to_owned();
    b
}
async fn testTag(tag: &str) -> bool {
    let mut ret = false;
    http::getHttpDocument(addTag(format!("{}{}", BASE_URI, SEARCH_PAGE).as_str(), tag).as_str())
        .await
        .unwrap_or_else(|r| http::getEmptyDoc())
        .select(&*SEARCH_IMAGE_SELECTOR)
        .for_each(|elem| ret = true);
    ret
}
fn getImagePostPageUri(elem: ElementRef<'_>) -> &'_ str {
    let st = elem.value().attr("href").unwrap_or_else(|| "");
    st
}
async fn getImageUriFromPost(href: &'_ str) -> String {
    let doc = http::getHttpDocument(href)
        .await
        .unwrap_or_else(|s| http::getEmptyDoc());
    let elem = doc.select(&*IMAGE_POST_IMAGE).next();
    let mut final_url = "";
    if elem.is_none() {
        return String::new();
    }
    let elem = elem.unwrap();
    let elem_has_sample = elem
        .value()
        .attr("src")
        .unwrap_or_else(|| "sample")
        .contains("sample");
    if elem_has_sample {
        let elem = doc.select(&*IMAGE_POST_HD_LINK).next();
        if elem.is_none() {
            return String::new();
        }
        let elem = elem.unwrap();
        final_url = elem.value().attr("href").unwrap_or_else(|| "");
    } else {
        final_url = elem.value().attr("src").unwrap_or_else(|| "");
    }
    let x = format!("{}", final_url);
    x
}
fn getSearchUri(oldUri: &str) -> String {
    let mut uri = Url::parse(oldUri).expect("url parse while addTag failed");
    let mut tags_str: u32 = uri
        .query_pairs()
        .find(|x| x.0 == "pid")
        .unwrap_or_else(|| (Cow::Borrowed("pid"), Cow::Borrowed("0")))
        .1
        .parse()
        .unwrap();
    uri.set_query(Option::Some(
        replace_if(
            Cow::Owned(uri.query().unwrap().to_string()),
            format!("{}={}", "pid", (tags_str).to_string()).as_str(),
            format!("{}={}", "pid", (tags_str + 40).to_string()).as_str(),
        )
        .deref(),
    ));
    uri.to_string()
}
fn replace_if<'a>(s: Cow<'a, str>, from: &str, to: &str) -> Cow<'a, str> {
    if s.contains(from) {
        Cow::Owned(s.replace(from, to))
    } else {
        s
    }
}
async fn getPostsOnPage(uri: &str) -> [ElementRef<'_>] {
    http::getHttpDocument(uri)
        .await
        .unwrap_or_else(|r| http::getEmptyDoc())
        .select(&*SEARCH_IMAGE_SELECTOR)
}
async fn processPost(elem: ElementRef<'_>) -> String {
    getImageUriFromPost(getImagePostPageUri(elem))
}
pub async fn runDownloader(tags_str: &str, out: &str, num: &i32) {
    eprintln!(
        "running SafeBooru download function with tags {}, out {}, num {}",
        &tags_str, &out, &num
    );
    unsafe{
        let mut tags = tags_str.split(" ").into_iter();
    tags.for_each(|f| {
        (&*SYNC_RUNTIME)
        .block_on(async {
            if !testTag(f).await {
               let mut tags = tags.filter(|g| g != &f);
            }
        });
        ()
    });
    let uri = format!(
        "{}{}&tags={}",
        BASE_URI,
        SEARCH_PAGE,
        tags.fold(String::new(), |f, x| format!("{}+{}", f, x))
    );
    if num.is_negative() {
        num = &0;
    }
    let downloadStreams: Vec<(String, String)> = Vec::new();
    let downloaded = 0i32;
    eprintln!("fully initialized, entering discovery loop.");
    while downloaded < num || num == &0i32 {
        let iterator: [ElementRef<'_>; 50] = getPostsOnPage(uri).await;
        while let Ok(elem) = iterator.next() {
            let url = elem;
            downloadStreams.
        }
        break;
    }
    let all_done = futures::stream::iter(downloadStreams.map())
        .buffer_unordered(8)
        .into_future();
    eprintln!("discovery loop done and future stream compiled. downloading now.");
    *SYNC_RUNTIME.block_on(&all_done)
    }
    
}
