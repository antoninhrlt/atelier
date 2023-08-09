// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

mod atelier;
mod message;

pub(crate) use message::Message;

use atelier::Atelier;
use iced::{Application, Settings};

/// Runs the "atelier" application.
///
/// Creates an [Atelier] then runs it.
pub fn run_app() -> iced::Result {
    // Creates the settings for the app.
    let settings = Settings {
        ..Settings::default()
    };

    // Runs the app and returns its result
    Atelier::run(settings)
}
