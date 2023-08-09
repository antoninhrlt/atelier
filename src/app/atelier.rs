// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::lingo;

// Macros
use lingo::locale;
use iced::widget::column;

use lingo::locales::Locale;
use lingo::Lingo;

use crate::strings;
use super::Message;

pub struct Atelier {
    lingo: Lingo,
}

impl iced::Application for Atelier {
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
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.lingo.string("app_title").unwrap()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            _ => {}
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![].into()
    }
}
