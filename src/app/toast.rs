use super::App;
use crate::tui::toast::Toast;

impl App {
    pub fn push_toast(&mut self, mut toast: Toast) -> u64 {
        self.toast_id_counter += 1;
        toast.id = self.toast_id_counter;
        self.toasts.push(toast);
        self.toast_id_counter
    }

    pub fn tick_toasts(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }

    pub fn dismiss_toast(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }
}
