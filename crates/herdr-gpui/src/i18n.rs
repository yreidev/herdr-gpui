//! The app's own text in the configured language. Each message's English text
//! in the code is its key: `t!("Settings")` looks it up in the files under
//! `locales/<locale>/` and shows the English when none has an entry, so a
//! message nobody has translated yet still reads correctly.
//!
//! The language is process-wide, as the menu bar is. Tests never set it, so
//! they see the English whatever the machine's own language; a test of a
//! translation names its locale (`t!("Settings", locale = "zh-CN")`).
use crate::config::Language;

/// Shows the app's text in `language` from the next render on. Windows
/// re-render after the config reload that brings a new language, and the menu
/// bar must be installed again.
pub(crate) fn apply(language: Language) {
    rust_i18n::set_locale(locale(language, sys_locale::get_locales()));
}

/// The locale `language` names, or for `auto` the first of `preferred` (the
/// system's languages as BCP 47 tags, most preferred first) that the app has
/// text in. English when none has.
fn locale(language: Language, preferred: impl IntoIterator<Item = String>) -> &'static str {
    match language {
        Language::English => Language::English.as_str(),
        Language::SimplifiedChinese => Language::SimplifiedChinese.as_str(),
        Language::Auto => preferred
            .into_iter()
            .find_map(|tag| supported(&tag))
            .unwrap_or(Language::English)
            .as_str(),
    }
}

/// The language a system tag such as `zh-Hans-CN` or `en-GB` asks for, when
/// the app has text in it. Traditional Chinese is not Simplified Chinese:
/// those readers get the next language they listed, or English. A script
/// subtag decides before a region does, so `zh-Hans-TW` is Simplified.
fn supported(tag: &str) -> Option<Language> {
    let subtags: Vec<String> = tag.split(['-', '_']).map(str::to_ascii_lowercase).collect();
    let has = |code: &str| subtags.iter().skip(1).any(|subtag| subtag == code);
    match subtags.first()?.as_str() {
        "en" => Some(Language::English),
        "zh" => {
            let traditional =
                has("hant") || (!has("hans") && ["tw", "hk", "mo"].into_iter().any(has));
            (!traditional).then_some(Language::SimplifiedChinese)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
