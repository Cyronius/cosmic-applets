# cosmic-app-list

> **Fork Notice**: This is a fork of [pop-os/cosmic-applets](https://github.com/pop-os/cosmic-applets) that adds an **ungrouped windows** feature to the app list. This allows each window to appear individually in the taskbar (similar to Windows' "never combine" option) rather than being grouped by application.

The app list/taskbar applet for the COSMIC desktop environment. Displays pinned applications and running windows in the panel.

## Features

### Ungrouped Windows Mode

Similar to Windows taskbar's "never combine" option, this feature shows each window individually instead of grouping them by application.

**Behavior:**
- **Pinned apps with 0 windows**: Shows the app icon for launching
- **Apps with 1+ windows**: Shows each window separately with its title
- **Per-monitor filtering**: Windows only appear on the monitor where they're active
- **Visible title labels**: Truncated window titles displayed below/beside icons

### Configuration

The applet is configured via `cosmic-config`. Configuration files are stored at:
```
~/.config/cosmic/com.system76.CosmicAppList/v2/
```

#### Config Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `filter_top_levels` | `Option<ToplevelFilter>` | `None` | Filter windows by workspace or output |
| `favorites` | `Vec<String>` | `[]` | List of pinned application IDs |
| `enable_drag_source` | `bool` | `true` | Allow drag-and-drop reordering |
| `ungrouped_windows` | `bool` | `false` | Show windows individually instead of grouped |

#### Enabling Ungrouped Windows

To enable ungrouped windows mode, create or edit the config file:
```bash
mkdir -p ~/.config/cosmic/com.system76.CosmicAppList/v2
echo "true" > ~/.config/cosmic/com.system76.CosmicAppList/v2/ungrouped_windows
```

Or set it programmatically via the COSMIC Settings API.

## Building

### Dependencies

```bash
# Fedora/RHEL
sudo dnf install libxkbcommon-devel libwayland-dev libdbus-1-dev libpulse-dev libpipewire-0.3-dev libinput-dev libudev-dev libegl-dev libclang-dev

# Debian/Ubuntu
sudo apt install libxkbcommon-dev libwayland-dev libdbus-1-dev libpulse-dev libpipewire-0.3-dev libinput-dev libudev-dev libegl-dev libclang-dev
```

### Build Commands

```bash
# From repository root
just build-release      # Build all applets (release mode)
just build-debug        # Build all applets (debug mode)

# Or build just this applet
cargo build --release -p cosmic-app-list
```

## Installation

After building, install all applets system-wide:

```bash
# From repository root - installs to /usr by default
sudo just install

# Or specify a custom prefix
sudo just prefix=/usr/local install
```

This will:
1. Install the `cosmic-applets` binary to `/usr/bin/`
2. Create symlinks for each applet (e.g., `cosmic-app-list` -> `cosmic-applets`)
3. Install desktop entries to `/usr/share/applications/`
4. Install icons to `/usr/share/icons/hicolor/`
5. Install default config schemas to `/usr/share/cosmic/`

### Restart the Panel

After installation, restart the COSMIC panel to load the updated applet:

```bash
# Option 1: Log out and back in

# Option 2: Restart cosmic-panel
systemctl --user restart cosmic-panel
```

### Development Installation

For development/testing without system-wide install, you can run the applet directly:

```bash
# Build debug version
cargo build -p cosmic-app-list

# Run directly (must be run within a COSMIC session)
./target/debug/cosmic-app-list
```

## Testing

```bash
# Run tests for the config crate
cargo test -p cosmic-app-list-config

# Run tests for the main applet
cargo test -p cosmic-app-list
```

## Architecture

### Key Files

| File | Purpose |
|------|---------|
| `src/app.rs` | Main application logic, view rendering, message handling |
| `src/wayland_handler.rs` | Low-level Wayland event handling |
| `src/wayland_subscription.rs` | Async Wayland subscription for toplevel events |
| `cosmic-app-list-config/src/lib.rs` | Configuration schema and persistence |
| `i18n/en/cosmic_app_list.ftl` | English translations |

### Data Structures

**DockItem**: Represents an application in the dock
```rust
struct DockItem {
    id: u32,
    toplevels: Vec<(ToplevelInfo, Option<WaylandImage>)>,
    desktop_info: DesktopEntry,
    original_app_id: String,
}
```

**Key Methods:**
- `as_icon()` - Renders grouped application icon with window indicators
- `as_ungrouped_window()` - Renders individual window with title label

### Wayland Integration

The applet uses the following Wayland protocols:
- `ext_foreign_toplevel_list` - Track toplevel windows
- `zcosmic_toplevel_info` - COSMIC-specific window information
- `ext_workspace` - Workspace management
- `wl_output` - Monitor/display information

## License

GPL-3.0-only - Copyright 2023 System76
