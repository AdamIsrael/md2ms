use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Metadata {
    pub content_warnings: Option<Vec<String>>,
    pub include: Option<Vec<String>>,

    /// The shortened title of the story, used in the manuscript header.
    pub short_title: Option<String>,

    /// The shortened name of the author, used in the manuscript header.
    pub short_author: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,

    /// The heading to use when rendering the child documents
    pub heading: Option<String>,
}
