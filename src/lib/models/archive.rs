use std::path::Path;
use crate::md5;
use crate::schema::archives;
use super::*;

#[derive(Insertable, Queryable)]
#[table_name="archives"]
pub struct ArchiveRecord {
    pub id: Id,
    pub zip_name: String,
    pub zip_path: String,
    pub zip_md5: String,
}
impl ArchiveRecord {
    pub fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::archives::dsl::archives;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        archives.find(id).first(conn)
    }

    pub fn find(conn: &SqliteConnection, value: &ArchiveName) -> QueryResult<Id> {
        use crate::schema::archives::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        archives
            .filter(zip_name.eq(&value.zip_name))
            .filter(zip_path.eq(&value.zip_path))
            .filter(zip_md5.eq(&value.zip_md5))
            .select(id)
            .first(conn)
    }

    pub fn find_by_md5(conn: &SqliteConnection, md5sum: &str) -> QueryResult<Id> {
        use crate::schema::archives::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        archives.filter(zip_md5.eq(md5sum)).select(id).first(conn)
    }

    pub fn save(conn: &SqliteConnection, value: &ArchiveName) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(archives::table).values(value).execute(conn)
    }
}

#[derive(Insertable)]
#[table_name="archives"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ArchiveName {
    pub zip_name: String,
    pub zip_path: String,
    pub zip_md5: String,
}
impl ArchiveName {
    pub fn new(path: &Path) -> Self {
        Self {
            zip_name: path.file_stem()
                .expect(&format!("Failed to get file name of '{}'", path.to_string_lossy()))
                .to_string_lossy().to_string(),
            zip_path: path.canonicalize()
                .expect(&format!("Failed to build absolute path of the '{}'", path.to_string_lossy()))
                .to_string_lossy().to_string(),
            zip_md5: Self::md5(path),
        }
    }

    pub fn md5(path: &Path) -> String {
        use std::io::prelude::*;
        use std::fs::File;

        let mut ctx = md5::Context::new();
        let mut file = File::open(path).expect(&format!("Can't open file '{}'", path.to_string_lossy()));
        let mut buffer = [0; 1024*1024];
        while let Some(readed) = file.read(&mut buffer).ok() {
            if readed > 0 {
                ctx.write(&buffer[0..readed]).expect("Failed to calculate md5");
            } else {
                break;
            }
        }
        format!("{:X}", ctx.compute())
    }
}


