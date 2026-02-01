// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{
    self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub const APP_ID: &str = "com.system76.CosmicAppList";

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub enum ToplevelFilter {
    #[default]
    ActiveWorkspace,
    ConfiguredOutput,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, CosmicConfigEntry)]
#[version = 2]
pub struct AppListConfig {
    pub filter_top_levels: Option<ToplevelFilter>,
    pub favorites: Vec<String>,
    pub enable_drag_source: bool,
    /// When true, show each window individually instead of grouping by application
    pub ungrouped_windows: bool,
}

impl Default for AppListConfig {
    fn default() -> Self {
        Self {
            filter_top_levels: None,
            favorites: Vec::new(),
            enable_drag_source: true,
            ungrouped_windows: false,
        }
    }
}

impl AppListConfig {
    pub fn add_pinned(&mut self, id: String, config: &Config) {
        if !self.favorites.contains(&id) {
            self.favorites.push(id);
            let _ = self.write_entry(config);
        }
    }

    pub fn remove_pinned(&mut self, id: &str, config: &Config) {
        if let Some(pos) = self.favorites.iter().position(|e| e == id) {
            self.favorites.remove(pos);
            let _ = self.write_entry(config);
        }
    }

    pub fn update_pinned(&mut self, favorites: Vec<String>, config: &Config) {
        self.favorites = favorites;
        let _ = self.write_entry(config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_ungrouped_windows_disabled() {
        let config = AppListConfig::default();
        assert!(!config.ungrouped_windows, "ungrouped_windows should default to false");
    }

    #[test]
    fn test_ungrouped_windows_can_be_enabled() {
        let mut config = AppListConfig::default();
        config.ungrouped_windows = true;
        assert!(config.ungrouped_windows);
    }

    #[test]
    fn test_config_serialization_with_ungrouped_windows() {
        let mut config = AppListConfig::default();
        config.ungrouped_windows = true;
        config.favorites = vec!["firefox".to_string()];

        // Test that it can be serialized and deserialized
        let serialized = serde_json::to_string(&config).expect("should serialize");
        let deserialized: AppListConfig =
            serde_json::from_str(&serialized).expect("should deserialize");

        assert_eq!(config, deserialized);
    }
}
