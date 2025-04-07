use crate::level::level22::InteractAction;

use super::{DialogChain, DialogNode, NEXT, finish, next};

pub const BLUE_BED1_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "There's a perfect copy of blue bed...",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "It's drawn on the floor...",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "You don't feel like trying to sleep on it.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const BLUE_BED2_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "This bed looks like yours, only... a different color.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "You wonder if your real bed is somewhere around here...",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "Try to fall asleep?",
            options: &[next("Yes", 3), next("No", 4)],
        },
        DialogNode {
            text: "Sleep slowly engulfs you.",
            options: &[finish(InteractAction::SleepBlue)],
        },
        DialogNode {
            text: "You decided NOT to sleep on any blue beds, ever again.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const STAIRS_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[DialogNode {
        text: "sorry, this staircase only works *downwards* :(",
        options: &[finish(InteractAction::None)],
    }],
};

pub const BLACK_CAT_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "There's a black cat sitting here.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "He's staring at a bird's nest...",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "On a floating chunk of marble?",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const TENT_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "There's a tent here.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "The campfire next to it seems long extinguished.",
            options: &[finish(InteractAction::None)],
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
            options: &[finish(InteractAction::SleepRed)],
        },
        DialogNode {
            text: "You decided not to sleep yet, though you're tired.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const FLAG_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "A red flag.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "Whoever planted it, you admire their spirit.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const FRIDGE_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "Just a regular fridge.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "You've seen it before.",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "There was a note stuck to it.",
            options: &[finish(InteractAction::None)],
        },
    ],
};
