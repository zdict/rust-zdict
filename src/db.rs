use rusqlite::{Connection, Result};

const TABLE_NAME: &str = "record";

#[derive(Debug)]
pub struct Record {
    word: String,
    source: String,
    content: String,
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



pub struct Cache {
    readable: bool,
}

impl Cache {
    pub fn new(disable_db_cache: bool) -> Self {
        let readable = !disable_db_cache;
        log::debug!("set cache `readable` to {:?}", readable);
        Cache { readable }
    }
    pub fn query(&self, word: &str, info_name: &str) -> Option<String> {
        if !self.readable {
            log::info!("bypass query");
            return None;
        }

        log::info!("query by {}-{}", word, info_name);
        // placeholder
        if word == "ground" { // not found
            log::debug!("found record");
            Some("content string".into())
        } else {
            log::debug!("record not found");
            None
        }
    }
    pub fn save(&self, word: &str, info_name: &str, content: &str) {
        log::info!("save record: {}-{}-{}", word, info_name, content);
    }
}
