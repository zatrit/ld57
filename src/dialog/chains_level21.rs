use crate::level::level1::InteractAction;

use super::{DialogChain, DialogNode, NEXT, finish, next};

pub const WHITE_CAT_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "Looks like a white cat is sitting here.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "She looks like she knows the way out of this labyrinth...",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "But she will absolutely NOT tell you anything.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const MIRROR_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You approached the mirror.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "In it, you saw yourself... but in a blue t-shirt?",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "You touched the reflection.",
            options: &[next(NEXT, 3)],
        },
        DialogNode {
            text: ".................",
            options: &[next(NEXT, 4)],
        },
        DialogNode {
            text: "While you were recovering from the shock...",
            options: &[next(NEXT, 5)],
        },
        DialogNode {
            text: "Something seems to have changed.",
            options: &[finish(InteractAction::Touch)],
        },
    ],
};

pub const FAVORITE_BED_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You've found a bed in a color you actually like.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "Want to take a nap?",
            options: &[next("Yes", 2), next("No", 3)],
        },
        DialogNode {
            text: "You lie down. It feels... just right.",
            options: &[finish(InteractAction::Sleep)],
        },
        DialogNode {
            text: "You decided not to sleep yet, though you're tired.",
            options: &[finish(InteractAction::None)],
        },
    ],
};
