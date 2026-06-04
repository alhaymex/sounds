pub enum Screen {
    Library,
    Playlist,
}

pub struct App {
    screen: Screen,
}

impl App {
    pub fn new(&self) -> Self {
        Self {
            screen: Screen::Library,
        }
    }
}
