use serde::Deserialize;

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Container {
    pub rootfiles: Rootfiles,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Rootfiles {
    pub rootfile: Vec<Rootfile>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Rootfile {
    #[serde(rename = "@full-path")]
    pub full_path: String,
    #[serde(rename = "@media-type")]
    pub media_type: String,
}

