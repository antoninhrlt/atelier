// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Constant strings, and localised strings using [lingo]. 

use crate::lingo;

// Macros
use lingo::{ lingo, locale, strings, s };

use lingo::locales::Locale;
use lingo::Lingo;

/// Generates all the localised strings for the project
/// 
/// The returned instance of [Lingo] is used in the [crate::app::atelier::Atelier] app.
pub fn gen() -> Lingo {
    lingo![
        (
            "app_title",
            strings![
                s!("en_GB", "Atelier - Workshop for embedded systems"),
                s!("fr", "Atelier - pour les systèmes embarqués")
            ]
        )
    ]
}
