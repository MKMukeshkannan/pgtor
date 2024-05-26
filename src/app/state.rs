pub enum CurrentPage {
    Login,
    Loading
}

pub struct App {
    pub current_page:  CurrentPage,
    pub is_logged: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_page: CurrentPage::Login,
            is_logged: false,
        }
    }

    pub fn login(&mut self) {
        self.current_page = CurrentPage::Loading;
        self.is_logged = true;
    }
}

