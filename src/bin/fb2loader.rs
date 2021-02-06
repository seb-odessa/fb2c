extern crate fb2parser;
extern crate clap;
extern crate zip;

use clap::{Arg, App, AppSettings};
use fb2parser::FictionBook;
use std::{fs, path};
use std::convert::TryFrom;
use std::collections::HashSet;
use lib::database;
use lib::models::Archive;
use lib::parser;


fn main() {
    let selfname: String = std::env::args().nth(0).unwrap_or_default();

    let app = App::new(selfname)
        .version("1.0.0")
        .author("seb <seb@ukr.net>")
        .about("FictionBook Library database loader")
        .arg(Arg::with_name("ARCHIVE.ZIP")
            .help("Sets the input file to use")
            .required(true)
            .index(1)
        )
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();
    let filename = matches.value_of("ARCHIVE.ZIP").unwrap();
    println!("Using input file: {}", filename);
    let path = path::Path::new(filename);
    let file = fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut error_counter = 0;
    let mut skip_counter = 0;
    let mut manager = database::Manager::new();
    let arch_id = match manager.save_archive(Archive::new(&path, md5sum(&path, false))) {
        database::SaveResult::CacheHit(id) => {
            println!("Archive already loaded into DB record id is {}", id);
            id
        },
        database::SaveResult::Quered(id) => {
            println!("Archive already loaded into DB record id is {}", id);
            id
        },
        database::SaveResult::Added(id) => {
            id
        }
    };

    let russian: HashSet<String> = vec!["ru", "rus", "russian", "ru-ru"]
                .into_iter()
                .map(|s| String::from(s))
                .collect();

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        if let Some(_) = manager.find_book(arch_id, zip_file.name(), zip_file.crc32() as i64) {
            continue
        }
        if let Some(header) = parser::load_header(&mut zip_file)
        {
            match FictionBook::try_from(header.as_bytes()) {
                Ok(fb) => {
                    let lang = if let Some(ref el) = fb.description.title_info.lang {
                        &el.text
                    } else {
                        "ru"
                    }.to_lowercase();

                    if russian.contains(&lang) {
                        let book_id = manager.save_book(arch_id, &zip_file).get_id();
                        manager.save_content(book_id, &fb);
                    } else {
                        skip_counter += 1;
                    }
                },
                Err(err) =>  {
                    println!("{} : {:?} '{}'", zip_file.name(), err, header);
                    error_counter += 1;
                }
            }
        }
    }
    println!("Total books in archive: {} ", archive.len());
    println!("Broken books found: {} ", error_counter);
    println!("Skipped by language filter: {} ", skip_counter);
}

fn md5sum(path: &path::Path, complete: bool) -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut file = File::open(path).expect(&format!("Can't open file '{}'", path.to_string_lossy()));
    let mut buffer = [0; 1024*1024];
    let mut ctx = md5::Context::new();
    while let Some(readed) = file.read(&mut buffer).ok() {
        if readed > 0 {
            ctx.write(&buffer[0..readed]).expect("Failed to calculate md5");
            if !complete {
                break;
            }
        } else {
            break;
        }
    }
    format!("{:X}", ctx.compute())
}

