use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub content_warnings: Option<Vec<String>>,
    pub scenes: Option<Vec<String>>,

    /// The shortened title of the story, used in the manuscript header.
    pub short_title: String,

    /// The shortened name of the author, used in the manuscript header.
    pub short_author: String,
    pub title: String,
    pub author: String,
}
