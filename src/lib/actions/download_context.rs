use std::io;
use std::fs;
use std::path::Path;
use actix_files::NamedFile;


use super::book_record::BookRecord;


#[derive(Debug)]
pub struct DownloadContext {
    pub book: BookRecord,
    pub workdir: String,
}

impl Drop for DownloadContext {
    fn drop(&mut self) {
        let full_path = Path::new(&self.workdir).join(&self.book.book_file);
        fs::remove_file(full_path).unwrap_or(());
    }
}

impl DownloadContext {
    pub fn new(workdir: &String, book: BookRecord) -> Self {
        Self{
            book: book,
            workdir: workdir.clone(),
        }
    }

    pub fn extract(&self) -> io::Result<()> {
        let full_name = format!("{}/{}", self.book.arch_home, self.book.arch_name);
        let zip_path = Path::new(&full_name);
        let zip_file = fs::File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;
        let mut file = archive.by_name(&self.book.book_file)?;
        let full_path = Path::new(&self.workdir).join(&self.book.book_file);
        let mut outfile = fs::File::create(&full_path)?;
        io::copy(&mut file, &mut outfile).unwrap();
        Ok(())
    }

    // pub fn getFileName(&self) -> String {
    //     format!("")
    // }

    pub fn get_stream(&self) -> io::Result<NamedFile> {
        let full_path = Path::new(&self.workdir).join(&self.book.book_file);
        NamedFile::open(full_path)
    }
}