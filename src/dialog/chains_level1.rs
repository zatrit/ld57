use crate::level::level1::InteractAction;

use super::{DialogChain, DialogNode, NEXT, finish, next};

// Vibe coded some dialog messages, because it's faster to use ChatGPT for this

pub const FRIDGE_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "This is your fridge. There's a note on it.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "Read it?",
            options: &[next("Yes", 2), next("No", 3)],
        },
        DialogNode {
            text: "'Dude, go touch some grass' - the note says.",
            options: &[finish(InteractAction::None)],
        },
        DialogNode {
            text: "You decided NOT to read the note.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const BLUE_BED_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "This is your bed.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "For some reason, it's blue.",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "You can go to sleep.",
            options: &[
                next("Go to sleep", 3),
                next("Look for different bedding", 6),
            ],
        },
        DialogNode {
            text: "You just woke up and haven't done much.",
            options: &[next(NEXT, 4)],
        },
        DialogNode {
            text: "Are you SURE you want to sleep?",
            options: &[next("Yes", 5), next("No", 6)],
        },
        DialogNode {
            text: "You started drifting off.",
            options: &[finish(InteractAction::Sleep)],
        },
        DialogNode {
            text: "You decided NOT to sleep for now.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const RED_BED_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "Your bed has somehow turned red.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "That kind of situation definitely suits you.",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "Go to sleep?",
            options: &[next("Yes", 3), next("No", 4)],
        },
        DialogNode {
            text: "Sleep quickly engulfs you.",
            options: &[finish(InteractAction::Sleep)],
        },
        DialogNode {
            text: "Although you're tired from your little adventure...",
            options: &[next(NEXT, 5)],
        },
        DialogNode {
            text: "You decided NOT to sleep for now.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const CARPET_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "This is your favorite carpet.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "In your favorite RED color.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const GRASS_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "This is green grass.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "There are still drops of morning dew on it.",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "Touch it?",
            options: &[next("Yes", 3), next("No", 5)],
        },
        DialogNode {
            text: "You touched the grass.",
            options: &[next(NEXT, 4)],
        },
        DialogNode {
            text: "A feeling of satisfaction fills you.",
            options: &[finish(InteractAction::Touch)],
        },
        DialogNode {
            text: "You decided not to touch the grass.",
            options: &[finish(InteractAction::None)],
        },
    ],
};

pub const FLOWERS_CHAIN: DialogChain<InteractAction> = DialogChain {
    start: 0,
    nodes: &[DialogNode {
        text: "You REALLY don't want to step on these flowers.",
        options: &[finish(InteractAction::None)],
    }],
};
