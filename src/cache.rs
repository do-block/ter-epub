use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};

use zip::ZipArchive;

pub fn dir(book_path: &str, dest_dir: &PathBuf) -> io::Result<()> {
    let mut archive = open_epub(book_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let file_name = file.name().to_owned();
        let file_path = dest_dir.join(file_name);

        if file.is_dir() {
            fs::create_dir_all(&file_path).expect("Failed to create directory");
        } else {
            if let Some(p) = file_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).expect("Failed to create directory");
                }
            }

            let mut dest_file = File::create(&file_path).expect("Failed to create file");

            std::io::copy(&mut file, &mut dest_file).expect("Failed to copy file");
        }
    }
    println!("解压完成, 文件缓存成功!");

    Ok(())
}

fn open_epub(file_path: &str) -> ZipArchive<BufReader<File>> {
    let file = File::open(file_path).expect("Failed to open EPUB file");
    let reader = BufReader::new(file);
    ZipArchive::new(reader).expect("Failed to read ZIP archive")
}
