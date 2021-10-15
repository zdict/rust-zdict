pub struct Cache {
    conn: rusqlite::Connection,
}

impl Cache {
    pub fn new() -> Option<Self> {
        let home = match std::env::var_os("HOME") {
            None => {
                log::error!("missing env var $HOME");
                return None;
            }
            Some(home) => home,
        };

        log::info!("$HOME = {:?}", home);
        let home = std::path::Path::new(home.as_os_str());
        let db_dir = &home.join(".zd/");
        let db_file = &db_dir.join("zd.db");

        log::info!("create {:?} if not exists", db_dir);
        if let Err(err) = std::fs::create_dir_all(db_dir) {
            log::error!("{}", err);
            return None;
        }

        log::info!("create and connect {:?} if not exists", db_file);
        rusqlite::Connection::open(db_file).and_then(|conn| {
            log::info!("create table if not exists");
            conn.execute("\
                CREATE TABLE IF NOT EXISTS \"record\" (\
                    \"word\" TEXT NOT NULL, \
                    \"source\" VARCHAR(255) NOT NULL, \
                    \"content\" TEXT NOT NULL, \
                    PRIMARY KEY (\"word\", \"source\")\
                );\
            ", [])
            .and(Ok(conn))
        }).map_or_else(
            |err| {
                log::error!("{}", err);
                None
            },
            |conn| Some(Cache { conn }),
        )
    }

    pub fn query(&self, word: &str, source: &str) -> rusqlite::Result<Option<String>> {
        log::info!("select by key {:?} {:?}", word, source);
        self.conn.query_row(
            "SELECT * FROM record WHERE word = $1 AND source = $2",
            [word, source],
            |row| row.get("content"),
        ).map_or_else(
            |err| match err {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                _ => Err(err),
            },
            |v| Ok(Some(v)),
        )
    }

    pub fn save(&self, word: &str, source: &str, content: &str) -> rusqlite::Result<()> {
        log::info!("save by key: {:?} {:?}", word, source);
        self.conn.execute(
            "INSERT OR REPLACE INTO record (word,source,content) VALUES ($1,$2,$3)",
            [word, source, content],
        ).map(|_ /* number of rows been updated, always 1 */|())
    }
}
