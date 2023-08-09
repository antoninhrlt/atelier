// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//! ## Atelier
//!

pub use lingo_lib as lingo; // note: crate's name must be updated

pub(crate) mod app;
use app::run_app;

mod views;

pub mod strings;

fn main() -> iced::Result {
    run_app()
}
