use anyhow::{Context, Result};
use i18n_embed::unic_langid::LanguageIdentifier;
use once_cell::sync::Lazy;
use relm4::adw;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Mutex;
use toml;

use super::localization;
use crate::app::models::{ColorScheme, Preference};

pub static PREFERENCES: Lazy<Mutex<Preference>> = Lazy::new(|| Mutex::new(Preference::default()));

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsToml {
    #[allow(dead_code)]
    pub theme: ColorScheme,
    #[allow(dead_code)]
    pub language: String,
    #[allow(dead_code)]
    pub database_path: String,
}

pub(crate) fn init() -> Result<()> {
    let settings_toml = get_settings()?;

    {
        let mut preference = PREFERENCES.lock().unwrap();
        preference.set_color_scheme(settings_toml.theme);
        preference.set_language(&settings_toml.language);
        preference.set_database_path(&settings_toml.database_path);
    }

    set_localization(settings_toml.language)?;
    set_color_scheme(settings_toml.theme);

    Ok(())
}

fn get_settings() -> Result<SettingsToml> {
    let toml_path = env::current_dir()?.join("settings.toml");
    let toml_str = fs::read_to_string(toml_path).context("Failed to read settings.toml")?;
    let settings_toml: SettingsToml =
        toml::from_str(&toml_str).context("Failed to deserialize settings.toml")?;

    Ok(settings_toml)
}

fn set_localization(language: String) -> Result<()> {
    let localizer = localization::localizer();
    let requested_language: LanguageIdentifier = language
        .parse()
        .context("Failed to parsing language identifier")?;

    if let Err(error) = localizer.select(&[requested_language]) {
        anyhow::bail!("Failed to loading language: {error}");
    }

    Ok(())
}

pub(crate) fn set_color_scheme(color_scheme: ColorScheme) {
    let color_scheme = match color_scheme {
        ColorScheme::Dark => adw::ColorScheme::ForceDark,
        ColorScheme::Light => adw::ColorScheme::ForceLight,
        ColorScheme::Default => adw::ColorScheme::Default,
    };
    adw::StyleManager::default().set_color_scheme(color_scheme);
}

fn set_settings(settings: &SettingsToml) -> Result<()> {
    let toml_path = env::current_dir()?.join("settings.toml");
    let toml_string = toml::to_string(settings)?;
    let mut file = File::create(toml_path)?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

pub(crate) async fn save_preferences(preference: &Preference) -> Result<()> {
    let settings_toml = SettingsToml {
        theme: preference.color_scheme,
        language: preference.language.to_string(),
        database_path: preference
            .database_path
            .to_str()
            .unwrap_or_default()
            .to_string(),
    };
    set_settings(&settings_toml).context("Failed to save preferences.")?;

    let mut preference = PREFERENCES.lock().unwrap();
    preference.set_color_scheme(settings_toml.theme);
    preference.set_language(&settings_toml.language);
    preference.set_database_path(&settings_toml.database_path);

    Ok(())
}
