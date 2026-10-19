#![allow(clippy::unwrap_used)]
use super::*;
use rust_i18n::t;
use std::collections::BTreeSet;

fn tags(tags: &[&str]) -> Vec<String> {
    tags.iter().map(|tag| (*tag).to_owned()).collect()
}

#[test]
fn a_named_language_ignores_the_system() {
    assert_eq!(locale(Language::English, tags(&["zh-CN"])), "en");
    assert_eq!(
        locale(Language::SimplifiedChinese, tags(&["en-US"])),
        "zh-CN"
    );
}

#[test]
fn auto_takes_the_first_preferred_language_with_text() {
    for (preferred, expected) in [
        (&["zh-CN"][..], "zh-CN"),
        (&["zh-Hans-CN"], "zh-CN"),
        (&["zh_CN"], "zh-CN"),
        (&["zh"], "zh-CN"),
        (&["zh-SG"], "zh-CN"),
        (&["zh-Hans-TW"], "zh-CN"),
        (&["en-GB", "zh-CN"], "en"),
        (&["fr-FR", "zh-CN"], "zh-CN"),
        (&["zh-TW", "zh-CN"], "zh-CN"),
        (&["zh-Hant-TW"], "en"),
        (&["zh-HK"], "en"),
        (&["fr-FR", "de-DE"], "en"),
        (&["C"], "en"),
        (&[], "en"),
    ] {
        assert_eq!(
            locale(Language::Auto, tags(preferred)),
            expected,
            "{preferred:?}"
        );
    }
}

#[test]
fn every_language_settings_offers_has_text() {
    let available = rust_i18n::available_locales!();
    for &language in Language::ALL {
        match language {
            Language::Auto | Language::English => {}
            Language::SimplifiedChinese => {
                assert!(available.iter().any(|locale| locale == language.as_str()));
            }
        }
    }
}

#[test]
fn an_untranslated_message_shows_its_english() {
    assert_eq!(t!("Settings", locale = "zh-CN"), "设置");
    assert_eq!(t!("Settings", locale = "en"), "Settings");
    assert_eq!(
        t!("A message no locale file has", locale = "zh-CN"),
        "A message no locale file has"
    );
}

/// `%{name}` placeholders a message fills in.
fn placeholders(text: &str) -> BTreeSet<&str> {
    text.match_indices("%{")
        .filter_map(|(start, _)| {
            let name = &text[start + 2..];
            name.find('}').map(|end| &name[..end])
        })
        .collect()
}

#[test]
fn every_translation_fills_in_what_its_english_does() {
    let messages = crate::_rust_i18n_backend()
        .messages_for_locale("zh-CN")
        .unwrap_or_default();
    assert!(!messages.is_empty());
    for (english, translated) in messages {
        assert!(!translated.trim().is_empty(), "{english:?} is empty");
        assert_eq!(
            placeholders(&english),
            placeholders(&translated),
            "{english:?} => {translated:?}"
        );
    }
}

/// rust-i18n merges every file under `locales/` and names each file's
/// language after the last dot of its stem, so a misnamed file would quietly
/// become a language of its own and a message in two files would take either
/// translation.
#[test]
fn locale_files_are_flat_named_for_their_language_and_hold_a_message_once() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("locales");
    let mut seen = std::collections::BTreeMap::new();
    for language in std::fs::read_dir(&root).unwrap() {
        let language = language.unwrap().path();
        if !language.is_dir() {
            continue;
        }
        let locale = language.file_name().unwrap().to_str().unwrap().to_owned();
        for file in std::fs::read_dir(&language).unwrap() {
            let file = file.unwrap().path();
            let name = file.file_name().unwrap().to_str().unwrap().to_owned();
            if file.extension().is_none_or(|extension| extension != "yml") {
                continue;
            }
            assert!(
                name.ends_with(&format!(".{locale}.yml")),
                "{name} in {locale}"
            );
            let text = std::fs::read_to_string(&file).unwrap();
            for line in text.lines() {
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }
                let key = serde_json::Deserializer::from_str(line)
                    .into_iter::<String>()
                    .next()
                    .and_then(Result::ok);
                let key = key.unwrap_or_else(|| panic!("{name}: not a quoted message: {line}"));
                if let Some(first) = seen.insert((locale.clone(), key.clone()), name.clone()) {
                    panic!("{key:?} is in both {first} and {name}");
                }
            }
        }
    }
    assert!(!seen.is_empty());
}
