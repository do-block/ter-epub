use html2text::from_read_with_decorator;
use html2text::render::text_renderer::TrivialDecorator;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Book {
    pub path: PathBuf,
    pub title: String,
    pub author: String,
    pub language: String,
    pub date: String,
    pub toc: Vec<Toc>,
    pub selected: usize,
    pub context: String,
    pub flat_toc: Vec<FlatToc>,
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
    pub start_pos: usize,
    pub end_pos: usize,
}

impl Book {
    // Read the chapter file
    pub fn read_and_show_text(&mut self) {
        if self.selected >= self.flat_toc.len() {
            return;
        }

        let toc = &self.flat_toc[self.selected];
        let file_path = self.path.join("OEBPS").join(&toc.path);

        if file_path.exists() {
            let file = File::open(&file_path).unwrap();
            let reader = BufReader::new(file);
            let start = toc.anchor.start_pos;
            let end = toc.anchor.end_pos;
            let content: String;

            if start == 0 && end == 0 {
                // If start and end are 0, read the entire file
                content = reader
                    .lines()
                    .map(|line| line.unwrap())
                    .collect::<Vec<String>>()
                    .join("\n");
            } else {
                // Only read the lines in the specified range
                content = reader
                    .lines()
                    .enumerate()
                    .skip(start)
                    .take(end - start)
                    .map(|(_, line)| line.unwrap())
                    .collect::<Vec<String>>()
                    .join("\n");
            }

            self.context =
                from_read_with_decorator(content.as_bytes(), 1400, TrivialDecorator::new());
        }
    }

    pub fn generate_anchor_positions(&mut self) -> Result<(), std::io::Error> {
        let mut anchor_positions = Vec::new();

        // Collect the positions of all anchors
        for toc in &self.toc {
            self.collect_anchor_positions(&toc, &mut anchor_positions)?;
        }

        let mut temp_toc = std::mem::take(&mut self.toc);

        // 为每个 TOC 项设置 start_pos 和 end_pos
        for toc in &mut temp_toc {
            self.process_toc_item(toc, &anchor_positions)?;
        }

        self.toc = temp_toc;

        Ok(())
    }

    fn collect_anchor_positions(
        &self,
        toc: &Toc,
        positions: &mut Vec<(String, usize)>,
    ) -> Result<(), std::io::Error> {
        let path = self.path.join("OEBPS").join(&toc.path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let re = Regex::new(r#"id="([^"]*)""#).unwrap();

        for (num, line) in reader.lines().enumerate() {
            let line = line?;
            if let Some(caps) = re.captures(&line) {
                if let Some(id) = caps.get(1) {
                    positions.push((id.as_str().to_string(), num));
                }
            }
        }

        Ok(())
    }

    fn process_toc_item(
        &mut self,
        toc: &mut Toc,
        positions: &[(String, usize)],
    ) -> Result<(), std::io::Error> {
        if let Some(pos) = positions.iter().find(|(id, _)| *id == toc.anchor.id) {
            toc.anchor.start_pos = pos.1;

            if let Some((_, next_pos)) = positions
                .iter()
                .skip_while(|&(id, _)| *id != toc.anchor.id)
                .nth(1)
            {
                toc.anchor.end_pos = *next_pos;
            } else {
                toc.anchor.end_pos = self.context.lines().count();
            }
        }

        // 递归地处理子 TOC 项
        for child in &mut toc.children {
            self.process_toc_item(child, positions)?;
        }

        Ok(())
    }

    // Flat Toc
    pub fn flatten_toc(&mut self) {
        let mut flat_toc = Vec::new();
        for toc in &self.toc {
            self.flatten_toc_recursive(toc, &mut flat_toc)
        }
        self.flat_toc = flat_toc;
    }

    fn flatten_toc_recursive(&self, toc: &Toc, flat_tocs: &mut Vec<FlatToc>) {
        flat_tocs.push(FlatToc {
            title: toc.title.clone(),
            path: toc.path.clone(),
            anchor: toc.anchor.clone(),
        });

        for child in &toc.children {
            self.flatten_toc_recursive(child, flat_tocs);
        }
    }
}

#[derive(Debug, Default)]
pub struct TocPosition {
    pub start: usize,
    pub path: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlatToc {
    pub title: String,
    pub path: String,
    pub anchor: Anchor,
}
