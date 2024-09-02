pub mod calc;
pub mod fu;
pub mod hand;
pub mod suit;
pub mod tile_group;
pub mod yaku;

use crate::fu::{calculate_total_fu_value, Fu};
use crate::suit::Suit;
use crate::tile_group::{GroupType, TileGroup};

/// Characters that represent terminal or honor tiles.
// Using a fixed array gets stored on the stack rather than a `String` which gets stored on the heap.
const TERMINAL_CHARS: [char; 9] = ['1', '9', 'E', 'S', 'W', 'N', 'r', 'g', 'w'];

#[derive(Debug)]
pub enum LimitHands {
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    KazoeYakuman,
}

impl LimitHands {
    //TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
    /// Check if the score of the hand is limited (no aotenjou).
    fn is_limit_hand(han: u16, fu: u16) -> bool {
        if han >= 5 {
            return true;
        }

        if han == 4 && fu >= 40 {
            return true;
        }

        if han == 3 && fu >= 70 {
            return true;
        }

        false
    }

    /// Calculate the limit hand type from the han and fu scores.
    pub fn get_limit_hand(han: u16, fu: u16) -> Option<Self> {
        if !Self::is_limit_hand(han, fu) {
            return None;
        }

        // TODO: Allow (3 han, 70+ fu) and (4 han, 40+ fu) to be considered manga.
        if han <= 5 {
            Some(Self::Mangan)
        } else if han <= 7 {
            return Some(Self::Haneman);
        } else if han <= 10 {
            return Some(Self::Baiman);
        } else if han <= 12 {
            return Some(Self::Sanbaiman);
        } else {
            return Some(Self::KazoeYakuman);
        }
    }

    /// Get the payment amounts.
    ///
    /// Format is as follows:
    ///
    /// - dealer_ron
    /// - dealer_tsumo
    /// - non_dealer_ron
    /// - non_dealer_tsumo_to_non_dealer
    /// - non_dealer_tsumo_to_dealer
    pub fn get_score(&self) -> Vec<u16> {
        match self {
            Self::Mangan => {
                vec![12000, 4000, 8000, 2000, 4000]
            }
            Self::Haneman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    let j = i / 2;
                    out.push(i + j)
                }
                out
            }
            Self::Baiman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 2)
                }
                out
            }
            Self::Sanbaiman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 3)
                }
                out
            }
            Self::KazoeYakuman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 4)
                }
                out
            }
        }
    }
}
