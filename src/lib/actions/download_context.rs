use std::io;
use std::fs;
use std::path::Path;
use actix_files::NamedFile;
use std::ffi::OsStr;

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

    fn unzip(arch: &Path, book: &String, outfile: &Path) -> io::Result<()> {
        let zip_file = fs::File::open(arch)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;
        let mut file = archive.by_name(book)?;
        let mut out = fs::File::create(&outfile)?;
        io::copy(&mut file, &mut out)?;
        Ok(())
    }

    fn zip(infile: &Path, outfile: &Path) -> io::Result<()> {
        let arch = fs::File::create(outfile)?;
        let mut archive = zip::ZipWriter::new(arch);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        let book: &str = infile.file_name().and_then(OsStr::to_str).unwrap_or("");

        archive.start_file(book, options)?;
        let mut file = fs::File::open(infile)?;
        io::copy(&mut file, &mut archive)?;
        archive.finish()?;
        Ok(())
    }

    pub fn get_unzipped_stream(&self) -> io::Result<NamedFile> {
        let arch = Path::new(&self.book.arch_home).join(&self.book.arch_name);
        let outfile = Path::new(&self.workdir).join(&self.book.book_file);
        Self::unzip(&arch, &self.book.book_file, &outfile)?;
        NamedFile::open(outfile)
    }

    pub fn get_zipped_stream(&self) -> io::Result<NamedFile> {
        let arch = Path::new(&self.book.arch_home).join(&self.book.arch_name);
        let unzipped = Path::new(&self.workdir).join(&self.book.book_file);
        Self::unzip(&arch, &self.book.book_file, &unzipped)?;

        let ext = unzipped.extension().and_then(OsStr::to_str).unwrap_or("");
        let new_ext = format!("{}.zip", ext);
        let zipped = unzipped.with_extension(new_ext);
        Self::zip(&unzipped, &zipped)?;
        NamedFile::open(zipped)
    }
}