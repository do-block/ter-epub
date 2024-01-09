use container::Container;
use dirs_next::home_dir;
use quick_xml::de::from_str;
use quick_xml::{self, Reader};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::{env, fs};

mod book;
mod cache;
mod container;
mod explorer;
mod opf;
mod toc;
mod ui;

use book::Book;
use explorer::index::Explorer;
use opf::Opf;
use toc::Ncx;

const EPUB_MIME_TYPE: &str = "application/epub+zip";
const TEMP_DIR_NAME: &str = ".epub_reader_temp";
// Save the file corresponding to the chapter and anchor
const TOC_FILE_NAME: &str = ".dtoc";

fn parse_container_xml(xml: &str) -> Container {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    from_str(xml).expect("Failed to parse container.xml")
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path-to-epub-file>", args[0]);
        return Ok(());
    }
    let epub_path = &args[1];
    if !PathBuf::from(epub_path).exists() {
        println!("File not found: {}", epub_path);
        return Ok(());
    }

    let is_reindex = args.len() > 2 && args[2] == "reindex";
    let releate_path = epub_path.split("/").last().unwrap();

    let dir = create_temp_dir(releate_path, is_reindex).expect("Failed to create temp directory");
    let mut book = parse_epub_structure(epub_path, dir)?;

    let _ = ui::show::start(&mut book);

    Ok(())
}

fn parse_epub_structure(book_path: &str, dir: (PathBuf, bool)) -> io::Result<Book> {
    let (dest_dir, is_cache) = dir;

    if !is_cache {
        cache::dir(book_path, &dest_dir)?;
    }

    println!("开始读取文件...");
    let mut book = Book::default();

    // 判断是否是文件夹， 读取本地缓存文件夹，文件夹的名称是书籍的名称
    if dest_dir.is_dir() {
        let mut opf: Opf = Opf::default();
        let mut ncx: Ncx = Ncx::default();
        // 遍历文件夹，读取文件 // mimitype, META_INF, OEBPS
        for entry in fs::read_dir(dest_dir.clone()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            // 读取 mimetype 文件，判断是否是epub文件
            if path.is_file() && path.ends_with("mimetype") {
                let mut file = File::open(&path).unwrap();
                let mut content = String::new();

                file.read_to_string(&mut content).unwrap();

                if content != EPUB_MIME_TYPE {
                    println!("mimetype 文件不是 epub 文件,解析可能会出错");
                } else {
                    println!("mimetype 文件读取成功: {}", content);

                    let root_file = read_meta_inf(&path);

                    // 判断是否存在 root_file 文件
                    let opf_file_path = path.parent().unwrap().join(root_file);

                    if opf_file_path.exists() {
                        // 读取 opf 文件
                        opf = Opf::read_text(&opf_file_path);
                    }

                    let ncx_file_path = opf_file_path.parent().unwrap().join("toc.ncx");

                    if ncx_file_path.exists() {
                        // 生成大纲
                        ncx = Ncx::read_text(&ncx_file_path);
                        // println!("ncx: {:#?}", ncx);
                    }
                }
            }
        }

        let metadata = opf.package.metadata;

        let toc = ncx
            .nav_map
            .nav_point
            .iter()
            .map(|nav_point| nav_point.get_toc())
            .collect();

        book = Book {
            path: dest_dir.clone(),
            title: metadata.title,
            toc,
            language: metadata.language,
            date: metadata.date,
            author: metadata.creator,
            selected: 1,
            flat_toc: vec![],
            context: "empty".to_string(),
        };

        //  写入文件到TOC_FILE_NAME
        let toc_file_path = dest_dir.join(TOC_FILE_NAME);

        // TODO: if file exist, prompt update index? 30 day
        if !toc_file_path.exists() {
            book.generate_anchor_positions()?;
            book.flatten_toc();

            let explorer = Explorer {
                book: book.clone(),
                position: 0,
                selected: 0,
            };
            let mut toc_file = File::create(&toc_file_path)?;
            let toc_file_content = serde_json::to_string(&explorer)?;
            toc_file.write_all(toc_file_content.as_bytes())?;
            Ok(book)
        } else {
            let mut book = File::open(&toc_file_path)?;
            let mut toc_file_content = String::new();
            book.read_to_string(&mut toc_file_content)?;
            let book_json: Explorer = serde_json::from_str(&toc_file_content)?;
            Ok(book_json.book)
        }
    } else {
        Ok(book)
    }
}

fn create_temp_dir(file_name: &str, is_reindex: bool) -> Result<(PathBuf, bool), std::io::Error> {
    let mut is_exist = false;

    let mut temp_dir = home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Home directory not found",
    ))?;

    temp_dir.push(TEMP_DIR_NAME);

    if !temp_dir.exists() {
        println!("enter create_dir_all");
        fs::create_dir_all(&temp_dir)?;
    }

    temp_dir.push(file_name);

    if !temp_dir.exists() || is_reindex {
        fs::create_dir_all(&temp_dir)?;
    } else {
        is_exist = true;
    }

    Ok((temp_dir, is_exist))
}

fn read_meta_inf(path: &PathBuf) -> String {
    // 读取 META_INF/container.xml 文件,获取 OEBPS 文件夹的路径
    let container_xml_path = path.parent().unwrap().join("META-INF/container.xml");

    let mut container_xml_file = File::open(&container_xml_path).unwrap();

    let mut container_xml_content = String::new();

    container_xml_file
        .read_to_string(&mut container_xml_content)
        .unwrap();

    let container_content = parse_container_xml(&container_xml_content);

    container_content
        .rootfiles
        .rootfile
        .iter()
        .find(|rootfile| rootfile.full_path.ends_with(".opf"))
        .expect("Failed to find rootfile, read opf process failed")
        .full_path
        .clone()
}
