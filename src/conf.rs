use std::io::Read;

use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::theme::{Theme, STANDARD_THEME};

#[derive(Clone)]
pub struct Conf {
    pub theme: Theme,
    pub banner: String,
}

#[derive(Serialize, Deserialize)]
struct ConfToml {
    theme: String,
    banner: String,
}
impl ConfToml {
    fn conf(self) -> Option<Conf> {
        Some(Conf {
            theme: Theme::new(&self.theme)?,
            banner: self.banner,
        })
    }
}

impl Conf {
    pub fn get() -> Self {
        Self::get_from_file().unwrap_or(Self {
            theme: Theme::new("colors").unwrap_or(STANDARD_THEME),
            banner: r#"
   __________ __         __  
  / __/ __/ // /__ _____/ /__
 _\ \_\ \/ _  / _ `/ __/  '_/
/___/___/_//_/\_,_/\__/_/\_\
"#
            .to_string(),
        })
    }

    fn get_from_file() -> Option<Self> {
        let mut file = std::env::home_dir()?;
        file.push(".config");
        file.push("sshack");
        file.push("config.toml");
        let mut f = std::fs::File::open(file).ok()?;
        let mut content = String::new();
        f.read_to_string(&mut content).ok()?;
        let conf_str: ConfToml = from_str(&content).ok()?;
        Some(conf_str.conf()?)
    }
}
