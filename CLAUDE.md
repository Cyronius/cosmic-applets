# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
just build-release      # Build all applets (release mode, default)
just build-debug        # Build all applets (debug mode)
just install            # Install to system (prefix=/usr by default)
just vendor             # Create vendored dependency tarball
```

## Linting and Formatting

```bash
cargo clippy --all --all-targets --all-features   # Run clippy lints
cargo +nightly fmt --all -- --check               # Check formatting
cargo +nightly fmt --all                          # Apply formatting
```

## Rust Toolchain

Version 1.90.0 (specified in rust-toolchain.toml). Nightly required for formatting.

## Architecture

This is a Cargo workspace containing panel applets for the COSMIC desktop environment. A single `cosmic-applets` binary is built and symlinked to individual applet names at install time.

### Core Framework

All applets use **libcosmic** (an Iced-based, Wayland-native GUI framework). Each applet implements the `cosmic::Application` trait with:
- `init()` - Initialize state and startup tasks
- `update()` - Handle messages/events
- `view()` - Render main applet view
- `view_window()` - Render popup windows
- `subscription()` - Subscribe to async events (D-Bus, config changes, etc.)

### Standard Applet Pattern

Entry point flow: `main.rs` → `lib.rs::run()` → `cosmic::applet::run::<AppState>()`

Each applet directory contains:
- `src/lib.rs` - Exports `run()` function
- `src/main.rs` - Calls `run()`
- `src/app.rs` - Main `Application` impl with `Message` enum
- `data/*.desktop` - Desktop entry with `X-CosmicApplet=true`
- `data/icons/` - SVG/PNG icon assets
- `i18n/` - Fluent translation files (.ftl)
- `i18n.toml` - i18n configuration

### System Integration

- **D-Bus**: Uses `zbus` for system service communication (NetworkManager, UPower, BlueZ, etc.)
- **Configuration**: `cosmic-config` with `CosmicConfigEntry` derive macro for auto-persisted settings
- **Wayland**: Native integration via `cosmic-protocols` and `smithay-client-toolkit`

### Workspace Dependencies

Common dependencies are specified in the root `Cargo.toml` `[workspace.dependencies]` section. Applets reference these with `workspace = true`.

## System Dependencies

Building requires: libxkbcommon-dev, libwayland-dev, libdbus-1-dev, libpulse-dev, libpipewire-0.3-dev, libinput-dev, libudev-dev, libegl-dev, libclang-dev
