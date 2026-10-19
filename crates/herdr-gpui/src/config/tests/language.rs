use super::*;
use crate::config::{Language, preferences::Preference};

#[test]
fn language_is_named_and_defaults_to_the_system() -> anyhow::Result<()> {
    for config in [
        Config::default(),
        Config::parse("")?,
        Config::parse(DEFAULT_CONFIG)?,
    ] {
        assert_eq!(config.language, Language::Auto);
    }
    for &language in Language::ALL {
        let text = format!("language = \"{}\"", language.as_str());
        assert_eq!(Config::parse(&text)?.language, language);
    }
    for bad in ["\"zh\"", "\"zh-cn\"", "\"English\"", "true"] {
        assert!(Config::parse(&format!("language = {bad}")).is_err());
    }
    Ok(())
}

#[test]
fn language_saves_in_place_and_keeps_other_settings() -> anyhow::Result<()> {
    let temp = TempDirectory::new()?;
    let path = temp.0.join("config.toml");
    let original = "theme = 'Nord' # keep\nlanguage = \"auto\" # mine\n";
    fs::write(&path, original)?;
    Config::save_preference_path(Preference::Language(Language::SimplifiedChinese), &path)?;
    let text = fs::read_to_string(&path)?;
    assert!(text.contains("theme = 'Nord' # keep"));
    assert!(text.contains("language = \"zh-CN\" # mine"));
    assert_eq!(Config::parse(&text)?.language, Language::SimplifiedChinese);
    Config::save_preference_path(Preference::Language(Language::Auto), &path)?;
    assert_eq!(fs::read_to_string(&path)?, original);
    Ok(())
}
