use ratatui::style::Color;
use serde::Deserialize;

#[derive(Clone)]
pub struct Theme {
    pub base00: Color,
    pub base01: Color,
    pub base02: Color,
    pub base03: Color,
    pub base04: Color,
    pub base05: Color,
    pub base06: Color,
    pub base07: Color,
    pub base08: Color,
    pub base09: Color,
    pub base0a: Color,
    pub base0b: Color,
    pub base0c: Color,
    pub base0d: Color,
    pub base0e: Color,
    pub base0f: Color,
}

#[derive(Deserialize)]
struct ThemeStringRepresentation {
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    pub base0A: String,
    pub base0B: String,
    pub base0C: String,
    pub base0D: String,
    pub base0E: String,
    pub base0F: String,
}
impl ThemeStringRepresentation {
    fn theme(&self) -> Theme {
        Theme {
            base00: Color::from_u32(u32::from_str_radix(&self.base00, 16).unwrap()),
            base01: Color::from_u32(u32::from_str_radix(&self.base01, 16).unwrap()),
            base02: Color::from_u32(u32::from_str_radix(&self.base02, 16).unwrap()),
            base03: Color::from_u32(u32::from_str_radix(&self.base03, 16).unwrap()),
            base04: Color::from_u32(u32::from_str_radix(&self.base04, 16).unwrap()),
            base05: Color::from_u32(u32::from_str_radix(&self.base05, 16).unwrap()),
            base06: Color::from_u32(u32::from_str_radix(&self.base06, 16).unwrap()),
            base07: Color::from_u32(u32::from_str_radix(&self.base07, 16).unwrap()),
            base08: Color::from_u32(u32::from_str_radix(&self.base08, 16).unwrap()),
            base09: Color::from_u32(u32::from_str_radix(&self.base09, 16).unwrap()),
            base0a: Color::from_u32(u32::from_str_radix(&self.base0A, 16).unwrap()),
            base0b: Color::from_u32(u32::from_str_radix(&self.base0B, 16).unwrap()),
            base0c: Color::from_u32(u32::from_str_radix(&self.base0C, 16).unwrap()),
            base0d: Color::from_u32(u32::from_str_radix(&self.base0D, 16).unwrap()),
            base0e: Color::from_u32(u32::from_str_radix(&self.base0E, 16).unwrap()),
            base0f: Color::from_u32(u32::from_str_radix(&self.base0F, 16).unwrap()),
        }
    }
}

impl Theme {
    pub fn new(theme: &str) -> Self {
        match theme {
            "nebula" => StandardThemes::Nebula.value(),
            "onelight" => StandardThemes::Onelight.value(),
            "gruvbox-dark-hard" => StandardThemes::GruvboxDarkHard.value(),
            "gruvbox-dark-medium" => StandardThemes::GruvboxDarkMedium.value(),
            "gruvbox-dark-pale" => StandardThemes::GruvboxDarkPale.value(),
            "gruvbox-dark-soft" => StandardThemes::GruvboxDarkSoft.value(),
            "gruvbox-light-hard" => StandardThemes::GruvboxLightHard.value(),
            "gruvbox-light-medium" => StandardThemes::GruvboxLightMedium.value(),
            "gruvbox-light-soft" => StandardThemes::GruvboxLightSoft.value(),
            "nord" => StandardThemes::Nord.value(),
            "onedark" => StandardThemes::Onedark.value(),
            "colors" => StandardThemes::Colors.value(),
            _ => todo!(),
        }
    }
}

pub enum StandardThemes {
    Nebula,
    Onelight,
    GruvboxDarkHard,
    GruvboxDarkMedium,
    GruvboxDarkPale,
    GruvboxDarkSoft,
    GruvboxLightHard,
    GruvboxLightMedium,
    GruvboxLightSoft,
    Nord,
    Onedark,
    Colors,
}

impl StandardThemes {
    fn value(&self) -> Theme {
        match self {
            StandardThemes::Nebula => {
                let string = include_str!("../themes/nebula.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::Onelight => {
                let string = include_str!("../themes/one-light.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxDarkHard => {
                let string = include_str!("../themes/gruvbox-dark-hard.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxDarkMedium => {
                let string = include_str!("../themes/gruvbox-dark-medium.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxDarkPale => {
                let string = include_str!("../themes/gruvbox-dark-pale.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxDarkSoft => {
                let string = include_str!("../themes/gruvbox-dark-soft.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxLightHard => {
                let string = include_str!("../themes/gruvbox-light-hard.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxLightMedium => {
                let string = include_str!("../themes/gruvbox-light-medium.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::GruvboxLightSoft => {
                let string = include_str!("../themes/gruvbox-light-soft.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::Nord => {
                let string = include_str!("../themes/nord.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::Onedark => {
                let string = include_str!("../themes/onedark.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }
            StandardThemes::Colors => {
                let string = include_str!("../themes/colors.yaml");
                let theme_string_repr: ThemeStringRepresentation =
                    serde_yaml_ng::from_str(string).unwrap();
                theme_string_repr.theme()
            }

            _ => todo!(),
        }
    }
}
