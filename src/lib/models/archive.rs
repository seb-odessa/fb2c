use std::path::Path;
use crate::md5;
use crate::schema::archives;
use super::*;

#[derive(Insertable)]
#[table_name="archives"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Archive {
    pub name: String,
    pub path: String,
    pub size: i64,
    pub md5: String,
}
impl Archive {
    pub fn new(archive: &Path) -> Self {
        use std::fs;
        Self {
            name: archive.file_name()
                    .expect(&format!("Failed to get file name of '{}'", archive.to_string_lossy()))
                    .to_string_lossy().to_string(),
                path: archive.canonicalize()
                    .expect(&format!("Failed to build absolute path of the '{}'", archive.to_string_lossy()))
                    .to_string_lossy().to_string(),
                size: fs::metadata(archive).map(|meta| meta.len()).unwrap_or_default() as i64,
                md5: calc_md5(archive),
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="archives"]
pub struct ArchiveRecord {
    pub id: Id,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub md5: String,
    pub done: bool,
}

type Base = Archive;
type Record = ArchiveRecord;
impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::archives::dsl::archives;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        archives.find(id).first(conn)
    }
}
impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::archives::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        archives
            .filter(name.eq(&value.name))
            .filter(path.eq(&value.path))
            .filter(size.eq(&value.size))
            .filter(md5.eq(&value.md5))
            .select(id)
            .first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(archives::table).values(value).execute(conn)
    }
}


fn calc_md5(archive: &Path) -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut ctx = md5::Context::new();
    let mut file = File::open(archive).expect(&format!("Can't open file '{}'", archive.to_string_lossy()));
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