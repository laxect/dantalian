use super::types::{Episode, SubjectBase, SubjectMedium};
use anyhow::{Context, Result};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use percent_encoding::{utf8_percent_encode, CONTROLS};
use serde::{de::DeserializeOwned, Deserialize};

pub async fn search_anime(keyword: String) -> Result<Vec<SubjectBase>> {
    println!("search_subject: {}", keyword);
    let encoded_keyword = utf8_percent_encode(&keyword, &CONTROLS);
    let path = format!("/search/subject/{}?type=2", encoded_keyword);
    let res_obj: SearchResponse = request(&path).await?;
    println!("obj: {:?}", &res_obj);
    Ok(res_obj.list)
}

pub async fn get_subject_info(id: u32) -> Result<SubjectMedium> {
    println!("get_subject_info: {}", id);
    let path = format!("/subject/{}?responseGroup=medium", id);
    let subject: SubjectMedium = request(&path).await?;
    println!("subject: {:#?}", &subject);
    Ok(subject)
}

pub async fn get_subject_episode(id: u32) -> Result<Vec<Episode>> {
    println!("get_subject_info: {}", id);
    let path = format!("/subject/{}/ep", id);
    let res: EpisodeResponse = request(&path).await?;
    for ep in &res.eps {
        println!("subject ep: {:#?}", &ep);
    }
    Ok(res.eps)
}

const BASE_URL: &str = "https://api.bgm.tv";

#[derive(Deserialize, Debug)]
struct SearchResponse {
    results: u32,
    list: Vec<SubjectBase>,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeResponse {
    // ignore SubjectBase
    pub eps: Vec<Episode>,
}

async fn request<T: DeserializeOwned>(path: &str) -> Result<T> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url: Uri = format!("{}{}", BASE_URL, path)
        .parse()
        .with_context(|| "parse url")?;
    println!("url = {}", url);
    let res = client.get(url).await.with_context(|| "get request")?;
    println!("status: {}", res.status());
    let buf = hyper::body::to_bytes(res).await?;
    let res_obj: T = serde_json::from_slice(&buf)?;
    Ok(res_obj)
}