use database::ConfigDatabase;

mod database;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) database: ConfigDatabase,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub(crate) fn new() -> Self {
        let database: ConfigDatabase = ConfigDatabase::new();

        Config {
            database,
        }
    }

    pub(crate) fn url_database(&self) -> String {
        self.database.url()
    }
}
