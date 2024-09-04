use crate::fu::Fu;
use crate::yaku::Yaku;

/// Number of points players pay to the winner.
// NOTE: `u64` allows for scoring with aotenjou (no limits).
pub type Points = u64;
/// Han value.
pub type HanValue = u32;
/// Fu (minipoints) value.
pub type FuValue = u64;
/// Number of honba (repeat counts).
pub type HonbaCounter = u64;

pub(crate) const DEALER_RON_MULTIPLIER: u64 = 6;
pub(crate) const DEALER_TSUMO_MULTIPLIER: u64 = 2;
pub(crate) const NON_DEALER_RON_MULTIPLIER: u64 = 4;
pub(crate) const NON_DEALER_TSUMO_TO_NON_DEALER_MULTIPLIER: u64 = 1;
pub(crate) const NON_DEALER_TSUMO_TO_DEALER_MULTIPLIER: u64 = 2;

/// Detailed breakdown of the winning hand's score.
#[derive(Debug)]
pub struct Score {
    /// Breakdown of payment amounts.
    payment: Payment,
    /// List of yaku that were awarded.
    yaku: Vec<Yaku>,
    /// List of fu that were awarded.
    fu: Vec<Fu>,
    /// Total score of the yaku that were awarded including dora.
    han: HanValue,
    /// Total score of the fu that were awarded.
    fu_score: FuValue,
    /// Is the hand open when it scored?
    is_open: bool,
}

impl Score {
    /// Create a new [`Score`].
    pub fn new(
        payment: Payment,
        yaku: Vec<Yaku>,
        fu: Vec<Fu>,
        han: HanValue,
        fu_score: FuValue,
        is_open: bool,
    ) -> Self {
        Self {
            payment,
            yaku,
            fu,
            han,
            fu_score,
            is_open,
        }
    }

    /// Get the payment breakdown.
    pub fn payment(&self) -> &Payment {
        &self.payment
    }

    /// Get the list of yaku that were awarded.
    pub fn yaku(&self) -> &[Yaku] {
        &self.yaku
    }

    /// Get the list of fu that were awarded.
    pub fn fu(&self) -> &[Fu] {
        &self.fu
    }

    /// Get the total han value of the hand.
    pub fn han(&self) -> HanValue {
        self.han
    }

    /// Get the total fu (minipoints) value of the hand.
    pub fn fu_score(&self) -> FuValue {
        self.fu_score
    }

    /// Get the state of whether or not the hand was opened.
    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

/// Breakdown of payment amounts players will pay.
#[derive(Debug)]
pub struct Payment {
    /// The amount the player that dealt-in pays to the dealer.
    dealer_ron: Points,
    /// The amount each player pays when the dealer tsumos.
    dealer_tsumo: Points,
    /// The amount the player that dealt-in pays to a non-dealer.
    non_dealer_ron: Points,
    /// The amount non-dealer players pay when a non-dealer wins by tsumo.
    non_dealer_tsumo_to_non_dealer: Points,
    /// The amount the dealer pays when a non-dealer wins by tsumo.
    non_dealer_tsumo_to_dealer: Points,
}

impl Payment {
    /// Create a new [`Payment`].
    pub fn new(
        dealer_ron: Points,
        dealer_tsumo: Points,
        non_dealer_ron: Points,
        non_dealer_tsumo_to_non_dealer: Points,
        non_dealer_tsumo_to_dealer: Points,
    ) -> Self {
        Self {
            dealer_ron,
            dealer_tsumo,
            non_dealer_ron,
            non_dealer_tsumo_to_non_dealer,
            non_dealer_tsumo_to_dealer,
        }
    }

    /// Get the amount of points the player that dealt-in has to pay to a dealer.
    pub fn dealer_ron(&self) -> Points {
        self.dealer_ron
    }

    /// Set the amount of points the player that dealt-in has to pay to a dealer.
    pub fn set_dealer_ron(&mut self, dealer_ron: Points) {
        self.dealer_ron = dealer_ron;
    }

    /// Get the amount of points each player pays when the dealer tsumos.
    pub fn dealer_tsumo(&self) -> Points {
        self.dealer_tsumo
    }

    /// Set the amount of points each player pays when the dealer tsumos.
    pub fn set_dealer_tsumo(&mut self, dealer_tsumo: Points) {
        self.dealer_tsumo = dealer_tsumo;
    }

    /// Get the amount the player that dealt-in pays to a non-dealer.
    pub fn non_dealer_ron(&self) -> Points {
        self.non_dealer_ron
    }

    /// Set the amount the player that dealt-in pays to a non-dealer.
    pub fn set_non_dealer_ron(&mut self, non_dealer_ron: Points) {
        self.non_dealer_ron = non_dealer_ron;
    }

    /// Get the amount non-dealer players pay when a non-dealer wins by tsumo.
    pub fn non_dealer_tsumo_to_non_dealer(&self) -> Points {
        self.non_dealer_tsumo_to_non_dealer
    }

    /// Set the amount non-dealer players pay when a non-dealer wins by tsumo.
    pub fn set_non_dealer_tsumo_to_non_dealer(&mut self, non_dealer_tsumo_to_non_dealer: Points) {
        self.non_dealer_tsumo_to_non_dealer = non_dealer_tsumo_to_non_dealer;
    }

    /// Get the amount the dealer pays when a non-dealer wins by tsumo.
    pub fn non_dealer_tsumo_to_dealer(&self) -> Points {
        self.non_dealer_tsumo_to_dealer
    }

    /// Set the amount the dealer pays when a non-dealer wins by tsumo.
    pub fn set_non_dealer_tsumo_to_dealer(&mut self, non_dealer_tsumo_to_dealer: Points) {
        self.non_dealer_tsumo_to_dealer = non_dealer_tsumo_to_dealer;
    }
}
