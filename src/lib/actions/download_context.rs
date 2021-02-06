use std::io;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::convert::TryFrom;
use actix_files::NamedFile;
use fb2parser::FictionBook;
use sanitize_filename;

use super::book_record::BookRecord;
use super::super::parser;

#[derive(Debug)]
pub struct DownloadContext {
    pub book: BookRecord,
    pub workdir: String,
    pub files: Vec<String>,
}

impl Drop for DownloadContext {
    fn drop(&mut self) {
        for file in self.files.iter() {
            let path = Path::new(file);
            fs::remove_file(path).unwrap_or(());
        }
    }
}

impl DownloadContext {
    pub fn new(workdir: &String, book: BookRecord) -> Self {
        Self{
            book: book,
            workdir: workdir.clone(),
            files: Vec::new(),
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

    fn make_name(ifile: &Path, book_file: &String) -> io::Result<String> {
        let mut name = String::new();
        let mut file = fs::File::open(ifile)?;
        if let Some(header) = parser::load_header(&mut file)
        {
            if let Some(fb2) = FictionBook::try_from(header.as_bytes()).ok()
            {
                name = format!("{}.fb2", sanitize_filename::sanitize(fb2.get_title()));
            }
        }

        if name.is_empty()
        {
            name = book_file.clone();
        }

        return Ok(name);
    }

    pub fn get_unzipped_stream(&mut self) -> io::Result<NamedFile> {
        let arch = Path::new(&self.book.arch_home).join(&self.book.arch_name);
        let unzipped = Path::new(&self.workdir).join(&self.book.book_file);
        Self::unzip(&arch, &self.book.book_file, &unzipped)?;

        self.files.push(unzipped.to_string_lossy().to_string());

        let book_name = Self::make_name(unzipped.as_path(), &self.book.book_file)?;
        let file = fs::File::open(unzipped)?;
        NamedFile::from_file(file, book_name)
        //NamedFile::open(outfile)
    }

    pub fn get_zipped_stream(&mut self) -> io::Result<NamedFile> {
        let arch = Path::new(&self.book.arch_home).join(&self.book.arch_name);
        let unzipped = Path::new(&self.workdir).join(&self.book.book_file);
        Self::unzip(&arch, &self.book.book_file, &unzipped)?;
        self.files.push(unzipped.to_string_lossy().to_string());

        let ext = unzipped.extension().and_then(OsStr::to_str).unwrap_or("");
        let new_ext = format!("{}.zip", ext);
        let zipped = unzipped.with_extension(new_ext);
        Self::zip(&unzipped, &zipped)?;
        self.files.push(zipped.to_string_lossy().to_string());

        let book_name = Self::make_name(unzipped.as_path(), &self.book.book_file)?;
        let file = fs::File::open(zipped)?;
        NamedFile::from_file(file, format!("{}.zip", book_name))
        //NamedFile::open(zipped)
    }
}