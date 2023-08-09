// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::thread::Builder;

use crate::lingo;

// Macros
use lingo::locale;

use lingo::locales::Locale;
use lingo::Lingo;

use super::Message;
use crate::strings;

/// The application powered by [`iced`].
pub struct Atelier<'a> {
    /// Manager for localised strings.
    lingo: Lingo,

    /// Stores the function that builds the different views.
    ///
    /// ## Note
    /// This array does not store [`iced::Element`] to avoid building the views
    /// before using them and to rebuild them when needed.
    views: [fn() -> iced::Element<'a, Message>; 1],

    /// Index of the current view rendered by [`view`](iced::Application::view).
    current_view: usize,
}

impl<'a> iced::Application for Atelier<'a> {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    /// Creates a new [Atelier] app.
    ///
    /// Initialises the [Lingo] object by generating the strings.
    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut lingo: Lingo = strings::gen();
        lingo.set_default_locale(locale!("en_GB"));

        (
            Atelier {
                lingo: lingo,
                views: [crate::views::main::view],
                current_view: 0,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.lingo.string("app_title").unwrap()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        if let Message::ChangeView(view_index) = message {
            self.current_view = view_index;
            return iced::Command::none();
        }

        match message {
            _ => {}
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        // Gets the function builder for the current view and builds it.
        self.views[self.current_view]()
    }
}
