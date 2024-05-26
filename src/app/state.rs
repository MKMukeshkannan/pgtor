pub struct App {
    is_logged: bool,
    database_credentials: Option<DBCred>,
}

impl App {
    pub fn new() -> App {
        App {
            is_logged: false,
            database_credentials: None,
        }
    }

    pub fn login(&mut self, db: DBCred) {
        self.is_logged = true;
        self.database_credentials = Some(db);
    }
}

pub struct DBCred {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub port: u8,
}
