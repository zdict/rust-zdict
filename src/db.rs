use rusqlite::{Connection, Result};


#[derive(Default)]
pub struct Cache {
    pub conn: Option<Connection>,
}

impl Cache {
    pub fn new() -> Self {
        let home = match std::env::var_os("HOME") {
            None => {
                log::warn!("missing env var $HOME");
                return Default::default();
            }
            Some(v) => v,
        };

        log::debug!("$HOME = {:?}", home);
        let home = std::path::Path::new(home.as_os_str());
        let db_dir = &home.join(".zd/");
        let db_file = &db_dir.join("zd.db");

        log::debug!("create {:?} if not exists", db_dir);
        if let Err(err) = std::fs::create_dir_all(db_dir) {
            log::warn!("unable to create {:?}, error: {}", db_dir, err);
            return Default::default();
        }

        log::debug!("create and connect {:?} if not exists", db_file);
        let conn = Connection::open(db_file).and_then(|conn| {
            log::debug!("create table if not exists");
            conn.execute("\
                CREATE TABLE IF NOT EXISTS \"record\" (\
                    \"word\" TEXT NOT NULL, \
                    \"source\" VARCHAR(255) NOT NULL, \
                    \"content\" TEXT NOT NULL, \
                    PRIMARY KEY (\"word\", \"source\")\
                );\
            ", [])
            .and(Ok(conn))
        });
        if let Err(err) = conn {
            log::warn!("db error occur: {}", err);
            return Default::default();
        }

        Cache { conn: conn.ok() }
    }

    pub fn query(&self, word: &str, source: &str) -> Option<String> {
        self.conn.as_ref().and_then(|conn| {
            conn.query_row (
                "SELECT * FROM record WHERE word = $1 AND source = $2",
                [word, source],
                |row| row.get("content"),
            ).map_err(|err| {
                log::warn!("db error occur: {}", err);
            }).ok()
        })
    }
    fn save(&self, word: &str, source: &str, content: &str) {
        self.conn.as_ref().map(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO record (word,source,content) VALUES ($1,$2,$3)",
                [word, source, content],
            ).map_err(|err| {
                log::warn!("db error occur: {}", err);
            })
        });
    }
}

pub fn main() {
    let db_cache = Cache::new();
    assert!(db_cache.conn.is_some());
    let s = db_cache.query("1", "2");
    dbg!(s);
    db_cache.save("1","2","3");
    let s = db_cache.query("1", "2");
    dbg!(s);
}
