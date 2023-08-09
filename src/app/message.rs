// This file is part of "atelier".
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[derive(Debug, Clone, Copy)]
pub enum Message  {
    /// Stores the index of the view to switch on.
    ChangeView(usize),
}
