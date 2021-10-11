use rusqlite::{Connection, Result};

const TABLE_NAME: &str = "record";

#[derive(Debug)]
pub struct Record {
    pub word: String,
    pub source: String,
    pub content: String,
}

pub fn get_conn(pathbuf: std::path::PathBuf) -> Result<Connection> {
    // below create a new db if non-existing
    let conn = Connection::open(pathbuf)?;
    Ok(conn)
}

// generate path
// create a new db if non-existing
// create a new table if non-existing

fn get(conn: &Connection, word: &str, source: &str) -> Result<Record> {
    conn.query_row (
        "SELECT * FROM record WHERE word = $1 AND source = $2",
        [word, source],
        |row| Ok(Record {
            word: row.get("word")?,
            source: row.get("source")?,
            content: row.get("content")?,
        })
    )
}

fn set(conn: &Connection, record: &Record) -> Result<()> {
    match conn.execute(
        "INSERT OR REPLACE INTO record (word,source,content) VALUES ($1,$2,$3)",
        [&record.word, &record.source, &record.content],
    ) {
        Ok(_ /* number of rows been updated, always 1 */) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn main() {
    let home = std::env::var_os("HOME").unwrap();
    let db_path = std::path::Path::new(&home).join(".zdict/zdict.db");
    let conn = get_conn(db_path).expect("IMO, if db conn failed, just fallback");
    //let record = get(&conn, "love", "yahoo");
    //record.unwrap();
    //if let Ok(record) = record { dbg!(record); } else { println!("Nothing"); }
    //set(&conn, &Record{word:"1".to_string(),source:"3".to_string(),content:"9".to_string()});
}



pub struct Cache;
impl Cache {
    pub fn new(_disable: bool) -> Self { Cache }
    pub fn query(&self, word: &str, info_name: &str) -> Option<Record> {
        Some(Record {
            word: word.into(),
            source: info_name.into(),
            content: "content".into(),
        })
    }
    pub fn save(&self, _word: &str, _info_name: &str, _content: &str) {}
}
