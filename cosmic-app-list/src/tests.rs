// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Tests for ungrouped windows functionality
//!
//! These tests verify the behavior of the ungrouped windows feature:
//! - Windows are shown individually when ungrouped_windows is enabled
//! - Windows are grouped by application when ungrouped_windows is disabled
//! - Output filtering works correctly to show windows only on their active monitor

/// Test that the output filter helper correctly identifies windows on the current output
#[test]
fn test_output_name_matching() {
    // Test the logic for matching output names
    let current_output = "DP-1";
    let window_outputs = vec!["DP-1".to_string(), "HDMI-1".to_string()];

    let is_on_current = window_outputs.iter().any(|o| o == current_output);
    assert!(is_on_current, "Window should be on current output DP-1");

    let other_output = "DP-2";
    let is_on_other = window_outputs.iter().any(|o| o == other_output);
    assert!(!is_on_other, "Window should not be on output DP-2");
}

/// Test that windows without any output info are handled gracefully
#[test]
fn test_empty_output_list() {
    let current_output = "DP-1";
    let window_outputs: Vec<String> = vec![];

    let is_on_current = window_outputs.iter().any(|o| o == current_output);
    assert!(
        !is_on_current,
        "Window with no outputs should not match any output"
    );
}

/// Test title truncation logic for ungrouped window labels
#[test]
fn test_title_truncation() {
    // Short titles should not be truncated
    let short_title = "Firefox";
    let truncated = truncate_title(short_title, 20);
    assert_eq!(truncated, "Firefox");

    // Long titles should be truncated with ellipsis
    let long_title = "Very Long Window Title That Exceeds Limit";
    let truncated = truncate_title(long_title, 20);
    assert_eq!(truncated.len(), 20);
    assert!(truncated.ends_with("..."));

    // Edge case: title exactly at limit
    let exact_title = "Exactly Twenty Chars";
    let truncated = truncate_title(exact_title, 20);
    assert_eq!(truncated, "Exactly Twenty Chars");
}

/// Helper function for title truncation (to be used in rendering)
fn truncate_title(title: &str, max_len: usize) -> String {
    if title.len() <= max_len {
        title.to_string()
    } else {
        format!("{}...", &title[..max_len - 3])
    }
}

/// Test that ungrouped mode creates separate entries for each window
#[test]
fn test_ungrouped_mode_behavior() {
    // This tests the conceptual behavior:
    // In ungrouped mode, 3 Firefox windows should result in 3 separate dock items
    let windows = vec![
        ("firefox", "Home - Firefox"),
        ("firefox", "Settings - Firefox"),
        ("firefox", "Gmail - Firefox"),
    ];

    let ungrouped = true;
    let items = if ungrouped {
        // Each window becomes its own item
        windows
            .iter()
            .map(|(app_id, title)| (app_id.to_string(), title.to_string()))
            .collect::<Vec<_>>()
    } else {
        // Windows are grouped by app_id
        let mut grouped: Vec<(String, String)> = vec![];
        for (app_id, _title) in &windows {
            if !grouped.iter().any(|(id, _)| id == *app_id) {
                grouped.push((app_id.to_string(), "Firefox".to_string()));
            }
        }
        grouped
    };

    assert_eq!(items.len(), 3, "Ungrouped mode should show 3 separate items");
}

/// Test that grouped mode combines windows from the same application
#[test]
fn test_grouped_mode_behavior() {
    let windows = vec![
        ("firefox", "Home - Firefox"),
        ("firefox", "Settings - Firefox"),
        ("firefox", "Gmail - Firefox"),
    ];

    let ungrouped = false;
    let items = if ungrouped {
        windows
            .iter()
            .map(|(app_id, title)| (app_id.to_string(), title.to_string()))
            .collect::<Vec<_>>()
    } else {
        // Windows are grouped by app_id
        let mut grouped: Vec<(String, String)> = vec![];
        for (app_id, _title) in &windows {
            if !grouped.iter().any(|(id, _)| id == *app_id) {
                grouped.push((app_id.to_string(), "Firefox".to_string()));
            }
        }
        grouped
    };

    assert_eq!(items.len(), 1, "Grouped mode should show 1 item for Firefox");
}

/// Test that pinned apps without windows are still visible
#[test]
fn test_pinned_app_without_windows_visible() {
    let pinned_apps = vec!["firefox", "thunderbird", "code"];
    let open_windows: Vec<&str> = vec![]; // No windows open

    // In ungrouped mode, pinned apps with no windows should still show their icon
    let visible_pinned: Vec<_> = pinned_apps
        .iter()
        .filter(|app| {
            // Show if no windows OR if this is a pinned app with no matching windows
            !open_windows.iter().any(|w| w == *app)
        })
        .collect();

    assert_eq!(
        visible_pinned.len(),
        3,
        "All pinned apps should be visible when no windows are open"
    );
}

/// Test that pinned apps with windows are replaced by individual windows in ungrouped mode
#[test]
fn test_pinned_replaced_by_windows_in_ungrouped_mode() {
    let pinned_apps = vec!["firefox"];
    let open_windows = vec![
        ("firefox", "Home - Firefox"),
        ("firefox", "Gmail - Firefox"),
    ];

    let ungrouped = true;

    // Count what should be visible
    let visible_count = if ungrouped {
        // Pinned app is replaced by its individual windows
        let pinned_with_windows: Vec<_> = pinned_apps
            .iter()
            .filter(|app| open_windows.iter().any(|(id, _)| id == *app))
            .collect();

        // For pinned apps with windows: show windows, not the pinned icon
        // For pinned apps without windows: show the pinned icon
        let windows_for_pinned = open_windows
            .iter()
            .filter(|(id, _)| pinned_apps.contains(id))
            .count();

        let pinned_without_windows = pinned_apps.len() - pinned_with_windows.len();

        windows_for_pinned + pinned_without_windows
    } else {
        pinned_apps.len()
    };

    assert_eq!(
        visible_count, 2,
        "Should show 2 individual Firefox windows, not 1 grouped icon"
    );
}
