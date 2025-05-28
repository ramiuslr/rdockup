use oci_client::client::ClientConfig;
use oci_client::{Client, Reference, secrets::RegistryAuth};
use serde::Serialize;

#[derive(Serialize)]
struct SerializableTags<'a> {
    name: &'a str,
    tags: &'a [String],
}

pub async fn get_tags(image: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new(ClientConfig::default());
    let img_ref: Reference = image.parse()?;
    let auth = RegistryAuth::Anonymous;
    let tags = client
        .list_tags(&img_ref, &auth, Some(100), Some("17"))
        .await?;
    let serialized_tags = SerializableTags {
        name: &tags.name,
        tags: &tags.tags,
    };
    Ok(serde_json::to_string(&serialized_tags)?)
}
