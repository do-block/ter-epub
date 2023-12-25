use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

use quick_xml::de::from_str;
use quick_xml::{self, Reader};

use crate::book::{Anchor, Toc};

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Ncx {
    #[serde(rename = "navMap")]
    pub nav_map: NavMap,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct NavMap {
    #[serde(rename = "navPoint")]
    pub nav_point: Vec<NavPoint>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct NavPoint {
    #[serde(rename = "navLabel")]
    pub nav_label: NavLabel,
    pub content: Content,
    #[serde(rename = "navPoint", default)]
    pub nav_point: Option<Vec<NavPoint>>,
}

impl NavPoint {
    pub fn get_toc(&self) -> Toc {
        let children = match &self.nav_point {
            Some(child_nav_point) => child_nav_point
                .iter()
                .map(|nav_point| Toc {
                    path: self.content.get_pure_path(),
                    title: nav_point.nav_label.text.clone(),
                    anchor: Anchor {
                        id: nav_point.content.get_anchor_id(),
                        start_pos: 0,
                        end_pos: 0,
                    },
                    children: vec![],
                })
                .collect(),
            None => vec![],
        };

        Toc {
            path: self.content.get_pure_path(),
            title: self.nav_label.text.clone(),
            anchor: Anchor {
                id: self.content.get_anchor_id(),
                start_pos: 0,
                end_pos: 0,
            },
            children,
        }
    }
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Content {
    #[serde(rename = "@src")]
    pub src: String,
}

impl Content {
    pub fn get_anchor_id(&self) -> String {
        match self.src.find('#') {
            Some(index) => self.src[index + 1..].to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_pure_path(&self) -> String {
        match self.src.find('#') {
            Some(index) => self.src[..index].to_string(),
            None => self.src.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct NavLabel {
    pub text: String,
}

impl Ncx {
    pub fn parse(xml_text: &str) -> Self {
        let mut reader = Reader::from_str(xml_text);
        reader.trim_text(true);

        from_str(xml_text).unwrap()
    }

    pub fn read_text(path: &PathBuf) -> Self {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        Ncx::parse(&content)
    }
}
