// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> cosmic::iced::Result {
    // Install panic hook for better crash diagnostics
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {:?}", info);
        eprintln!("Backtrace: {:?}", std::backtrace::Backtrace::capture());
    }));

    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    tracing::info!("Starting cosmic-app-list with version {VERSION}");

    cosmic_app_list::run()
}
