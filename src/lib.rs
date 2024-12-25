/// The number of times in a row an item was recalled correctly.
pub type Repetitions = u32;

/// The Ease Factor (EF) of an item.
pub type Ease = f32;

/// The initial EF of an item.
pub const INITIAL_EF: Ease = 2.5;

/// The minimum EF value.
const MIN_EF: Ease = 1.3;

/// If the given EF is below the minimum, return the minimum.
fn min(ef: Ease) -> Ease {
    if ef < MIN_EF {
        MIN_EF
    } else {
        ef
    }
}

/// The number of days until the next review.
pub type Interval = u32;

/// Recall quality.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Quality {
    /// Complete blackout.
    Blackout = 0,
    /// Incorrect response; the correct one remembered.
    Incorrect = 1,
    /// Incorrect response; where the correct one seemed easy to recall.
    IncorrectEasy = 2,
    /// Correct response recalled with serious difficulty.
    Hard = 3,
    /// Correct response after a hesitation.
    Good = 4,
    /// Perfect response.
    Perfect = 5,
}

impl Quality {
    /// True for quality levels representing failure.
    pub fn forgot(self) -> bool {
        match self {
            Self::Blackout
            | Self::Incorrect
            | Self::IncorrectEasy => true,
            Self::Hard | Self::Good | Self::Perfect => {
                false
            }
        }
    }

    /// Should the item be repeated at the end of the session?
    pub fn repeat(self) -> bool {
        match self {
            Self::Blackout
            | Self::Incorrect
            | Self::IncorrectEasy
            | Self::Hard => true,
            Self::Good | Self::Perfect => false,
        }
    }
}

/// An item of knowledge.
#[derive(Debug, Copy, Clone)]
pub struct Item {
    n: Repetitions,
    ef: Ease,
}

impl Item {
    /// Construct an item from a repetition count and an EF.
    pub fn new(n: Repetitions, ef: Ease) -> Self {
        Self { n, ef }
    }

    /// The item's number of repetitions.
    pub fn n(&self) -> Repetitions {
        self.n
    }

    /// The item's easiness factor.
    pub fn ef(&self) -> Ease {
        self.ef
    }

    #[must_use = "Item::review returns a new Item"]
    pub fn review(self, q: Quality) -> Self {
        Self {
            n: np(self.n, q),
            ef: efp(self.ef, q),
        }
    }

    /// The interval when the item will be reviewed next.
    pub fn interval(&self) -> Interval {
        let r = self.n;
        let ef = self.ef;
        match self.n {
            0 => 0,
            1 => 1,
            2 => 6,
            _ => {
                let r = r as f32;
                let i = 6.0 * ef.powf(r - 2.0);
                let i = i.ceil();
                i as u32
            }
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            n: 0,
            ef: INITIAL_EF,
        }
    }
}

/// Update the repetitions after a review.
fn np(rep: Repetitions, q: Quality) -> Repetitions {
    if q.forgot() {
        0
    } else {
        rep + 1
    }
}

/// Update EF after a review.
fn efp(ef: Ease, q: Quality) -> Ease {
    let ef = min(ef);
    let q = (q as u8) as f32;
    let ef = ef - 0.8 + 0.28 * q - 0.02 * q * q;
    min(ef)
}
