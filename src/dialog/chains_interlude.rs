use crate::level::interlude::InterludeAction;

use super::{DialogChain, DialogNode, NEXT, finish, next};

pub const INTERLUDE_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        // Node 0
        DialogNode {
            text: "You are in a deep slumber. What will you do?",
            options: &[next("Go deeper", 1), next("Awake a bit", 2)],
        },
        // Node 1 (Go deeper)
        DialogNode {
            text: "You drift deeper into your sleep, the world fading away.",
            options: &[finish(InterludeAction::Deeper)],
        },
        // Node 2 (Awake a bit)
        DialogNode {
            text: "You stir awake slightly, feeling pull of consciousness.",
            options: &[finish(InterludeAction::Awake)],
        },
    ],
};

pub const FOREST_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You are sleeping...",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "You see your dreams...",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "You feel the dampness of the forest through your dream...",
            options: &[finish(InterludeAction::None)],
        },
    ],
};

pub const HOME_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You feel the warmth of home...",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "Maybe you haven't finished your chores yet...",
            options: &[finish(InterludeAction::None)],
        },
    ],
};

pub const MAZE_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[DialogNode {
        text: "Through the dream, you feel lost...",
        options: &[finish(InterludeAction::None)],
    }],
};

pub const SKY_ISLES_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You feel yourself rising toward reality.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "You see the endless blue sky.",
            options: &[finish(InterludeAction::None)],
        },
    ],
};

pub const TOO_DEEP_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You've drifted too far into sleep.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "You see... something blue.",
            options: &[finish(InterludeAction::None)],
        },
    ],
};

pub const WHEAT_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "Reaching the end, you chose to stay in the middle.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "You feel the wind brushing through your hair.",
            options: &[next(NEXT, 2)],
        },
        DialogNode {
            text: "A wheat field surrounds you.",
            options: &[finish(InterludeAction::None)],
        },
    ],
};

pub const WAKEUP_CHAIN: DialogChain<InterludeAction> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "Suddenly, you feel like it’s time to wake up.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "That dream still lingers in your mind...",
            options: &[finish(InterludeAction::None)],
        },
    ],
};
