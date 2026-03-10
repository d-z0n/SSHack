use crate::theme::Theme;

#[derive(Clone)]
pub struct Conf {
    pub theme: Theme,
}

impl Conf {
    pub fn get() -> Self {
        Self {
            theme: Theme::new("colors"),
        }
    }
}
