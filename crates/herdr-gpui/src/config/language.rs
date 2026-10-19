//! The language the app's own text is shown in; terminal output never changes.
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
pub enum Language {
    /// The first of the system's preferred languages the app has a
    /// translation for, or English.
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "zh-CN")]
    SimplifiedChinese,
}

impl Language {
    /// Settings offers them in this order.
    pub(crate) const ALL: &[Self] = &[Self::Auto, Self::English, Self::SimplifiedChinese];

    /// How Settings names it: each language in its own words, so a reader
    /// finds theirs whatever the app is showing now.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Auto => "System",
            Self::English => "English",
            Self::SimplifiedChinese => "简体中文",
        }
    }

    /// The value written to the config file; matches the serde name.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::English => "en",
            Self::SimplifiedChinese => "zh-CN",
        }
    }
}
