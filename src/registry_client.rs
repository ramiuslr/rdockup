use oci_client::{Client, Reference, secrets::RegistryAuth};
use regex::Regex;
use semver::Version;
use serde::Serialize;
use serde_json;
use std::error::Error;

#[derive(Serialize)]
struct LatestTags<'a> {
    name: &'a str,
    latest_tags: &'a [String],
}

/// Extracts the first MAJOR.MINOR.PATCH or MAJOR.MINOR from a tag.
/// If only MAJOR.MINOR is found, appends .0 for patch.
fn extract_version(tag: &str) -> Option<Version> {
    // Try to find MAJOR.MINOR.PATCH
    let re_patch = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    if let Some(mat) = re_patch.find(tag) {
        return Version::parse(mat.as_str()).ok();
    }
    // Try to find MAJOR.MINOR and treat as MAJOR.MINOR.0
    let re_minor = Regex::new(r"\d+\.\d+").unwrap();
    if let Some(mat) = re_minor.find(tag) {
        let version_str = format!("{}.0", mat.as_str());
        return Version::parse(&version_str).ok();
    }
    None
}

fn tag_matches_filters(tag: &str, include: Option<&[String]>, exclude: Option<&[String]>) -> bool {
    let includes = include.map_or(true, |incs| incs.iter().all(|inc| tag.contains(inc)));
    let excludes = exclude.map_or(true, |excs| excs.iter().all(|exc| !tag.contains(exc)));
    includes && excludes
}

async fn fetch_all_tags(
    client: &Client,
    image_ref: &Reference,
    auth: &RegistryAuth,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut all_tags = Vec::new();
    let mut last_tag: Option<String> = None;

    loop {
        let response = client
            .list_tags(image_ref, auth, Some(100), last_tag.as_deref())
            .await?;

        let tags = response.tags;
        if tags.is_empty() {
            break;
        }

        let is_last_page = tags.len() < 100;
        last_tag = tags.last().cloned();
        all_tags.extend(tags);

        if is_last_page {
            break;
        }
    }

    Ok(all_tags)
}

async fn get_sorted_tags(
    image: &str,
    include: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::new(Default::default());
    let image_ref: Reference = image.parse()?;
    let auth = RegistryAuth::Anonymous;

    let all_tags = fetch_all_tags(&client, &image_ref, &auth).await?;

    let mut parsed_tags: Vec<(String, Version)> = all_tags
        .into_iter()
        .filter(|tag| tag_matches_filters(tag, include, exclude))
        .filter_map(|tag| extract_version(&tag).map(|ver| (tag, ver)))
        .collect();

    parsed_tags.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(parsed_tags
        .into_iter()
        .take(10)
        .map(|(tag, _)| tag)
        .collect())
}

pub async fn get_tags(
    image: &str,
    include: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<String, Box<dyn Error>> {
    let tags = get_sorted_tags(image, include, exclude).await?;
    let output = LatestTags {
        name: image,
        latest_tags: &tags,
    };
    Ok(serde_json::to_string_pretty(&output)?)
}
