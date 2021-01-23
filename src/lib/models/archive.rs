use std::path::Path;
use crate::schema::archives;
use super::*;

#[derive(Insertable)]
#[table_name="archives"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Archive {
    pub arch_name: String,
    pub arch_home: String,
    pub arch_size: i64,
    pub arch_uuid: String,
}
impl Archive {
    pub fn new(archive: &Path, uuid: String) -> Self {
        use std::fs;
        Self {
            arch_name: archive.file_name()
                .expect(&format!("Failed to get file name of '{}'", archive.to_string_lossy()))
                .to_string_lossy().to_string(),
            arch_home: archive.parent().unwrap_or(Path::new(".")).canonicalize()
                .expect(&format!("Failed to build absolute path of the '{}'", archive.to_string_lossy()))
                .to_string_lossy().to_string(),
            arch_size: fs::metadata(archive).map(|meta| meta.len()).unwrap_or_default() as i64,
            arch_uuid: uuid,
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="archives"]
pub struct ArchiveRecord {
    pub id: Id,
    pub arch_name: String,
    pub arch_home: String,
    pub arch_size: i64,
    pub arch_uuid: String,
    pub arch_done: bool,
}
impl Record {
    pub fn find_uniq(conn: &SqliteConnection, uid: &String) -> Option<Id> {
        use crate::schema::archives::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        archives.filter(arch_uuid.eq(uid)).select(id).first(conn).ok()
    }
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
            .filter(arch_name.eq(&value.arch_name))
            .filter(arch_home.eq(&value.arch_home))
            .filter(arch_size.eq(&value.arch_size))
            .filter(arch_uuid.eq(&value.arch_uuid))
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
