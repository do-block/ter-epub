use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    mem,
    path::PathBuf,
};

use regex::Regex;
use serde::{Serialize,Deserialize};

#[derive(Debug, Default, Clone,Serialize, Deserialize)]
pub struct Book {
    pub path: PathBuf,
    pub title: String,
    pub author: String,
    pub language: String,
    pub date: String,
    pub toc: Vec<Toc>,
    pub selected: usize,
    pub context: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Toc {
    pub title: String,
    pub path: String,
    pub anchor: Anchor,
    pub children: Vec<Toc>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub id: String,
    pub position: usize,
}

impl Book {
    // 读取章节文件
    pub fn read_text(&mut self) {
        if self.selected >= self.toc.len() {
            return;
        }

        let toc = &self.toc[self.selected];
        let file = self.path.join("OEBPS").join(&toc.path);

        if file.exists() {
            let mut content = String::new();
            let mut file = File::open(&file).unwrap();

            file.read_to_string(&mut content).unwrap();

            self.context = content.clone();
        }
    }

    pub fn generate_anchor_positions(&mut self) -> Result<(), std::io::Error> {
        // TODO: 查找位置的代码性能上需要优化
        let mut temp_toc = mem::replace(&mut self.toc, Vec::new());

        for toc in &mut temp_toc {
            self.process_toc_item(toc)?;
        }

        self.toc = temp_toc;
        Ok(())
    }

    fn process_toc_item(&mut self, toc: &mut Toc) -> Result<(), std::io::Error> {
        if !toc.anchor.id.is_empty() {
            let path = self.path.join("OEBPS").join(&toc.path);
            let file = File::open(&path)?;
            let reader = BufReader::new(file);

            let re = Regex::new(&format!(r#"id="{}""#, toc.anchor.id)).unwrap();

            for (num, line) in reader.lines().enumerate() {
                let line = line?;
                if re.is_match(&line) {
                    toc.anchor.position = num;
                    break;
                }
            }
        }

        for child in &mut toc.children {
            self.process_toc_item(child)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct TocPosition {
    pub start: usize,
    pub path: String,
}
