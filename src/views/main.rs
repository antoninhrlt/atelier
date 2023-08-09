// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use iced::widget::{column, text};

use crate::app::Message;

pub fn view<'a>() -> iced::Element<'a, Message> {
    column![
        text("test")
    ].into()
}
