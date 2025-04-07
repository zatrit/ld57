use crate::level::level32::EndingChoice;

use super::{DialogChain, DialogNode, NEXT, finish, next};

pub const WHEAT_ENDING_CHAIN: DialogChain<EndingChoice> = DialogChain {
    start: 0,
    nodes: &[
        DialogNode {
            text: "You've reached the end of your journey.",
            options: &[next(NEXT, 1)],
        },
        DialogNode {
            text: "Would you like to start over?",
            options: &[next("Return", 2), next("Enjoy the moment", 3)],
        },
        DialogNode {
            text: "You feel this dream should've ended differently.",
            options: &[finish(EndingChoice::Restart)],
        },
        DialogNode {
            text: "You take in the golden field.",
            options: &[next(NEXT, 4)],
        },
        DialogNode {
            text: "Wasn't this what you wanted?",
            options: &[finish(EndingChoice::Finish)],
        },
    ],
};
