use serde::{Deserialize, Serialize};

use crate::theme::Theme;

#[derive(Clone)]
pub struct Conf {
    pub theme: Theme,
    pub banner: String,
}

#[derive(Serialize, Deserialize)]
struct ConfToml {
    theme: String,
    banner: String,
    port: u32,
}

impl Conf {
    pub fn get() -> Self {
        Self {
            theme: Theme::new("colors"),
            banner: r#"
   __________ __         __  
  / __/ __/ // /__ _____/ /__
 _\ \_\ \/ _  / _ `/ __/  '_/
/___/___/_//_/\_,_/\__/_/\_\
"#
            .to_string(),
        }
    }
}
