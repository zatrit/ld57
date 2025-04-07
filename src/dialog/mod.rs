use std::fmt::Debug;

use trait_set::trait_set;

pub mod chains_level1;
pub mod handler;
pub mod chains_level22;
pub mod chains_interlude;
pub mod chains_level21;

pub const NEXT: &str = "";

trait_set! {
     pub trait Action = Clone + Copy + Debug + 'static;
}

#[derive(Clone, Copy, Debug)]
pub enum DialogAction<A: Action> {
    Next(usize),
    Finish(A),
}

#[derive(Clone, Copy, Debug)]
pub struct DialogChain<A: Action> {
    pub start: usize,
    pub nodes: &'static [DialogNode<A>],
}

#[derive(Clone, Copy, Debug)]
pub struct DialogNode<A: Action> {
    pub text: &'static str,
    pub options: &'static [DialogOption<A>],
}

#[derive(Clone, Copy, Debug)]
pub struct DialogOption<A: Action> {
    pub label: &'static str,
    pub action: DialogAction<A>,
}

pub const fn finish<A: Action>(action: A) -> DialogOption<A> {
    DialogOption {
        label: NEXT,
        action: DialogAction::Finish(action),
    }
}

pub const fn next<A: Action>(label: &'static str, to: usize) -> DialogOption<A> {
    DialogOption {
        label,
        action: DialogAction::Next(to),
    }
}
