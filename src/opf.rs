use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::path::PathBuf;

use serde::Deserialize;

use quick_xml::de::from_str;
use quick_xml::{self, Reader};

// use crate::htmltotext;

const CHUNK_SIZE: usize = 110242; // 块大小，例如4KB

#[derive(Debug, Default)]
pub struct Opf {
    pub package: Package,
    pub spine_items: Vec<SpineItems>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Package {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "metadata")]
    pub metadata: OpfMetaData,
    #[serde(rename = "manifest")]
    pub manifest: Manifest,
    #[serde(rename = "spine")]
    pub spine: Spine,
    #[serde(rename = "@xmlns")]
    xmlns: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct OpfMetaData {
    #[serde(rename = "@xmlns:dc")]
    pub xmlns_dc: String,
    #[serde(rename = "@xmlns:opf")]
    pub xmlns_opf: String,
    // #[serde(rename = "dc:title")]
    pub title: String,
    // #[serde(rename = "dc:language")]
    pub language: String,
    // #[serde(rename = "dc:creator")]
    pub creator: String,
    // #[serde(rename = "dc:publisher")]
    pub publisher: String,
    // #[serde(rename = "dc:date")]
    pub date: String,
    pub meta: Vec<OpfMeta>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct OpfMeta {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@content")]
    pub content: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct Spine {
    pub itemref: Vec<ItemRef>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct ItemRef {
    #[serde(rename = "@idref")]
    pub idref: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Manifest {
    pub item: Vec<Item>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Item {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@media-type")]
    pub media_type: String,
}

impl Opf {
    pub fn read_text(path: &PathBuf) -> Self {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content)
            .expect("Failed to read opf file");

        let package = Package::parse(&content);

        // println!("xml_parser_result: {:?}\n", package);

        let spine_items = Opf::get_spines(&package.spine, &package.manifest);

        println!("spine_items: {:#?}\n", spine_items);

        // 这里要做处理,多个章节
        // let selected_chapter = &spine_items[1];
        // let selected_chapter_path = path.parent().unwrap().join(&selected_chapter.href);
        // let file_path = selected_chapter_path.to_str().unwrap();
        // let mut current_pos = 0;
        // let opf_content = Opf::_load_more(file_path, &mut current_pos).unwrap();
        // println!("opf_content: {}", opf_content);
        // let text = htmltotext::run(&all_content);

        Self {
            package,
            spine_items,
        }
    }

    pub fn get_spines(spine: &Spine, manifest: &Manifest) -> Vec<SpineItems> {
        let mut spine_items = vec![];

        let spine_value = &spine
            .itemref
            .iter()
            .map(|itemref| {
                let idref = &itemref.idref;
                idref
            })
            .collect::<Vec<&String>>();

        manifest.item.iter().for_each(|item| {
            let id = &item.id;
            let href = &item.href;
            let media_type = &item.media_type;

            if spine_value.contains(&id) {
                spine_items.push(SpineItems {
                    id: id.clone(),
                    href: href.clone(),
                    media_type: media_type.clone(),
                });
            }
        });

        spine_items
    }

    pub fn _load_more(file_path: &str, current_pos: &mut u64) -> io::Result<String> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        reader.seek(std::io::SeekFrom::Start(*current_pos))?;

        let mut buffer = [0; CHUNK_SIZE];
        let bytes_read = reader.read(&mut buffer)?;
        *current_pos += bytes_read as u64;

        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }
}

impl Package {
    pub fn parse(xml_text: &str) -> Self {
        let new_xml = xml_text.replace("dc:", "dc_");
        let mut reader = Reader::from_str(&new_xml);

        reader.trim_text(true);

        from_str(&new_xml).unwrap()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SpineItems {
    pub id: String,
    pub href: String,
    pub media_type: String,
}
