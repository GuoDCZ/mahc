pub mod error;

use crate::fu::Fu;
use crate::suit::Suit;
use crate::tile_group::{GroupType, TileGroup};
use crate::TERMINAL_CHARS;
use error::HandErr;

#[derive(Debug)]
pub struct Hand {
    groups: Vec<TileGroup>,
    win_tile: TileGroup,
    seat_tile: TileGroup,
    /// Prevalent or round wind.
    prev_tile: TileGroup,
    isopen: bool,
}

impl Hand {
    pub fn new(
        tiles: Vec<String>,
        win: String,
        prev: String,
        seat: String,
    ) -> Result<Self, HandErr> {
        let mut tile_groups: Vec<TileGroup> = Vec::new();
        let mut ishandopen = false;

        // NOTE: Strings are complicated in Rust and needs evaluation about how to iterate over one. Because the string is expected to contain ASCII characters, `.chars()` should be okay.
        for i in &tiles {
            let tile = TileGroup::new(i.to_string())?;
            if tile.isopen {
                ishandopen = true;
            }
            tile_groups.push(tile);
        }

        //TODO: standard hand ONLY CHECK MUST FIX FOR KOKUSHI
        let mut full_shape_count = 0;
        let mut pair_count = 0;
        let mut no_shape_count = 0;
        for group in &tile_groups {
            match group.group_type {
                GroupType::Triplet | GroupType::Sequence | GroupType::Kan => full_shape_count += 1,
                GroupType::Pair => pair_count += 1,
                GroupType::None => no_shape_count += 1,
            }
        }

        if !(full_shape_count == 4 && pair_count == 1)
            && pair_count != 7
            && !(no_shape_count == 12 && pair_count == 1)
        {
            return Err(HandErr::InvalidShape);
        }

        // AHAHAHAHAHAHAHAHAh (these are special cases for singular tiles)
        let win_tile = TileGroup {
            value: win.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(win.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: TERMINAL_CHARS.contains(&win.chars().nth(0).unwrap()),
        };

        // check if last group contains the winning tile
        // FUCK handling kokuushi
        if tiles.len() != 13 {
            let last_group = tile_groups.last().unwrap();
            match last_group.group_type {
                GroupType::Sequence => {
                    if win_tile.suit != last_group.suit {
                        return Err(HandErr::InvalidShape);
                    }

                    let win_int = win_tile.parse_u8().unwrap();
                    let last_int = last_group.parse_u8().unwrap();

                    if win_int != last_int && win_int != last_int + 1 && win_int != last_int + 2 {
                        return Err(HandErr::InvalidShape);
                    }
                }
                GroupType::Triplet | GroupType::Pair => {
                    if last_group.value != win_tile.value || last_group.suit != win_tile.suit {
                        return Err(HandErr::InvalidShape);
                    }
                }
                GroupType::Kan | GroupType::None => return Err(HandErr::InvalidShape),
            }
        }

        let seat_tile = TileGroup {
            value: seat.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(seat.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: TERMINAL_CHARS.contains(&seat.chars().nth(0).unwrap()),
        };

        let prev_tile = TileGroup {
            value: prev.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(prev.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: TERMINAL_CHARS.contains(&prev.chars().nth(0).unwrap()),
        };

        let hand = Self {
            groups: tile_groups,
            win_tile,
            seat_tile,
            prev_tile,
            isopen: ishandopen,
        };

        Ok(hand)
    }

    /// Calculate the fu types in the hand.
    pub fn calculate_fu(&self, tsumo: bool) -> Vec<Fu> {
        let mut fu_types: Vec<Fu> = vec![];

        fu_types.push(Fu::BasePoints);

        if tsumo {
            fu_types.push(Fu::Tsumo);
        }

        if !self.is_open() {
            fu_types.push(Fu::ClosedRon);
        }

        //meld fu cal
        for tile_group in &self.triplets() {
            let group_is_terminal_or_honor = tile_group.is_honor() || tile_group.isterminal;

            if tile_group == self.groups.last().unwrap() {
                if tsumo {
                    if group_is_terminal_or_honor {
                        fu_types.push(Fu::NonSimpleClosedTriplet);
                    } else {
                        fu_types.push(Fu::SimpleClosedTriplet);
                    }
                } else if group_is_terminal_or_honor {
                    fu_types.push(Fu::NonSimpleOpenTriplet);
                } else {
                    fu_types.push(Fu::SimpleOpenTriplet);
                }
                continue;
            }

            if !group_is_terminal_or_honor && tile_group.isopen {
                fu_types.push(Fu::SimpleOpenTriplet);
            }

            if !tile_group.isopen {
                if group_is_terminal_or_honor {
                    fu_types.push(Fu::NonSimpleClosedTriplet);
                } else {
                    fu_types.push(Fu::SimpleClosedTriplet);
                }
            } else if group_is_terminal_or_honor {
                fu_types.push(Fu::NonSimpleOpenTriplet);
            }
        }

        for kan in &self.kans() {
            let group_is_terminal_or_honor = kan.is_honor() || kan.isterminal;

            if group_is_terminal_or_honor {
                if !kan.isopen {
                    fu_types.push(Fu::NonSimpleClosedKan);
                } else {
                    fu_types.push(Fu::NonSimpleOpenKan);
                }
            } else if !kan.isopen {
                fu_types.push(Fu::SimpleClosedKan);
            } else {
                fu_types.push(Fu::SimpleOpenKan);
            }
        }

        for pair in self.pairs() {
            if pair.value == self.prev_tile.value
                || pair.value == self.seat_tile.value
                || pair.suit == Suit::Dragon
            {
                fu_types.push(Fu::Toitsu);
            }
        }

        //fu wait cal
        if let Some(group) = self.groups.last() {
            match group.group_type {
                GroupType::Pair => fu_types.push(Fu::SingleWait),
                GroupType::Sequence => {
                    let mid_tile = group.parse_u8().unwrap() + 1;
                    if self.win_tile().parse_u8().unwrap() == mid_tile {
                        fu_types.push(Fu::SingleWait);
                    }

                    if !self.win_tile().isterminal && group.isterminal {
                        fu_types.push(Fu::SingleWait);
                    }
                }
                _ => {}
            }
        }

        fu_types
    }

    /// Get the sequence groups in the hand.
    pub fn sequences(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type, GroupType::Sequence))
            .collect()
    }

    /// Get the triplet groups in the hand.
    pub fn triplets(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type, GroupType::Triplet))
            .collect()
    }

    /// Get the kan groups in the hand.
    pub fn kans(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type, GroupType::Kan))
            .collect()
    }

    /// Get the pair groups in the hand.
    pub fn pairs(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type, GroupType::Pair))
            .collect()
    }

    /// Get the groups with no shape in the hand.
    ///
    /// This can be used to check for kokushi musou (thirteen orphans).
    pub fn singles(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type, GroupType::None))
            .collect()
    }

    /// Get the winning tile the completes the hand.
    pub fn win_tile(&self) -> TileGroup {
        self.win_tile.clone()
    }

    /// Get the seat wind.
    pub fn seat_tile(&self) -> TileGroup {
        self.seat_tile.clone()
    }

    /// Get the prevalent wind.
    pub fn prev_tile(&self) -> TileGroup {
        self.prev_tile.clone()
    }

    /// Get the state of whether or not the hand has been opened.
    pub fn is_open(&self) -> bool {
        self.isopen
    }

    //yaku validation

    /// Check if the hand only contains simple tiles -- no terminal or honor tiles.
    pub fn is_tanyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        for group in self.groups.clone() {
            if group.isterminal || group.is_honor() {
                return false;
            }
        }

        true
    }

    /// Check if the hand contains two unique identical sequences.
    pub fn is_ryanpeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();

        if seqs.len() != 4 {
            return false;
        }

        seqs.dedup();
        seqs.len() == 2
    }

    /// Check if the hand contains two identical sequences.
    pub fn is_iipeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();

        seqs.dedup();
        !(self.sequences().len() == seqs.len() || self.is_open() || self.is_ryanpeikou())
    }

    /// Check if the hand contains value honors.
    pub fn is_yakuhai(&self) -> u16 {
        // i do it like this because a single group can have multiple yakuhai
        let mut count = 0;

        for triplet_group in self.triplets() {
            if triplet_group.value == self.prev_tile.value {
                count += 1;
            }
            if triplet_group.value == self.seat_tile.value {
                count += 1;
            }
            if triplet_group.suit == Suit::Dragon {
                count += 1;
            }
        }

        for kan_group in self.kans() {
            if kan_group.value == self.prev_tile.value {
                count += 1;
            }
            if kan_group.value == self.seat_tile.value {
                count += 1;
            }
            if kan_group.suit == Suit::Dragon {
                count += 1;
            }
        }

        count
    }

    /// Check if the hand contains all triplets and kans.
    pub fn is_toitoi(&self) -> bool {
        self.triplets().len() + self.kans().len() == 4
    }

    /// Check if the hand contains three concealed triplets (including kans).
    ///
    /// If the last necessary triplet is formed from a ron, it is not considered concealed and sanankou is not granted.
    pub fn is_sanankou(&self, tsumo: bool) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let mut closed_triplet_count = 0;

        for triplet_group in self.triplets() {
            if !triplet_group.isopen {
                closed_triplet_count += 1;
            }
        }

        for kan_group in self.kans() {
            if !kan_group.isopen {
                closed_triplet_count += 1;
            }
        }

        if !tsumo && self.groups.last().unwrap().group_type == GroupType::Triplet {
            closed_triplet_count -= 1;
        }

        closed_triplet_count == 3
    }

    /// Check if the hand contains a mixed triple sequence (ex: `123m 123p 123s`).
    pub fn is_sanshokudoujun(&self) -> bool {
        if self.sequences().len() < 3 {
            return false;
        }

        let mut list_of_vals: Vec<String> = vec![];
        for sequence_group in self.sequences() {
            list_of_vals.push(sequence_group.value.clone());
        }
        list_of_vals.dedup();

        if self.sequences().len() == 3 {
            if list_of_vals.len() == 1 {
                return true;
            }
        } else if list_of_vals.len() == 2 {
            return true;
        }

        false
    }

    /// Check if the hand only contains tiles of one suit and any honor tiles.
    ///
    /// This is commonly referred to as a "half flush".
    pub fn is_honitsu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let mut has_honor = false;
        let mut has_normal = false;
        let mut suit: Option<&Suit> = None;
        for group in &self.groups {
            if group.is_honor() {
                has_honor = true;
            } else {
                has_normal = true;
                suit = Some(&group.suit);
            }
        }

        if !has_normal || !has_honor {
            return false;
        }

        if let Some(s) = suit {
            for group in &self.groups {
                if &group.suit != s && !group.is_honor() {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }

    /// Check if the hand has two dragon triplets or quads and a pair of dragon tiles.
    pub fn is_shousangen(&self) -> bool {
        let dragon_count = self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|group| group.suit == Suit::Dragon)
            .count();

        dragon_count == 2 && self.pairs()[0].suit == Suit::Dragon
    }

    /// Check if the hand only contains groups with at least one terminal tile.
    pub fn is_junchantaiyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self
            .groups
            .iter()
            .any(|group| group.is_honor() || !group.isterminal)
        {
            return false;
        }

        !self.sequences().is_empty()
    }

    /// Check if the hand only contains terminal and honor tiles.
    pub fn is_honroutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if !self.sequences().is_empty() {
            return false;
        }

        let mut has_terminal: bool = false;
        let mut has_honor: bool = false;
        for group in self.groups.clone() {
            if group.isterminal {
                has_terminal = true;
            } else if group.is_honor() {
                has_honor = true;
            } else {
                return false;
            }
        }

        has_terminal && has_honor
    }

    /// Check if the hand has three quads.
    pub fn is_sankantsu(&self) -> bool {
        self.kans().len() == 3
    }

    /// Check if the hand has three exact sequences of 1-2-3, 4-5-6, and 7-8-9 in the same suit.
    pub fn is_ittsuu(&self) -> bool {
        //there has GOTTO be a better way to do this
        let suits = [Suit::Pinzu, Suit::Manzu, Suit::Souzu];
        suits.iter().any(|suit| {
            let values: Vec<String> = self
                .sequences()
                .iter()
                .filter(|&x| x.suit == *suit)
                .map(|x| x.value.clone())
                .collect();

            values.contains(&"1".to_string())
                && values.contains(&"4".to_string())
                && values.contains(&"7".to_string())
        })
    }

    /// Check if the hand only contains groups with terminal or honor tiles; sequences are permitted as long as they contain a terminal.
    ///
    /// There must be at least one sequence, otherwise the hand would either be junchan or tsuuiisou.
    pub fn is_chantaiyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.sequences().is_empty() {
            return false;
        }

        let mut has_terminal = false;
        let mut has_honor = false;
        for group in self.groups.clone() {
            if group.isterminal {
                has_terminal = true;
            } else if group.is_honor() {
                has_honor = true;
            } else {
                return false;
            }
        }

        has_terminal && has_honor
    }

    /// Check if the hand consists of 7 unique pairs.
    pub fn is_chiitoitsu(&self) -> bool {
        self.pairs().len() == 7
    }

    /// Check if the hand has won with a self-draw.
    ///
    /// The hand must be closed.
    pub fn is_menzentsumo(&self, tsumo: bool) -> bool {
        !self.isopen && tsumo
    }

    /// Check if the hand has won with all sequences, with the last shape waiting to complete a sequence.
    ///
    /// The hand must be closed.
    pub fn is_pinfu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.isopen {
            return false;
        }

        self.calculate_fu(false)
            .iter()
            .all(|fu| matches!(fu, Fu::BasePoints | Fu::ClosedRon))
    }

    /// Check if the hand contains three triplets (or quads) of the same value across the three numerical suits (manzu, pinzu, and souzu).
    pub fn is_sanshokudoukou(&self) -> bool {
        if self.triplets().len() + self.kans().len() < 3 {
            return false;
        }

        let mut list_of_vals: Vec<String> = vec![];
        for group in self.triplets().iter().chain(self.kans().iter()) {
            list_of_vals.push(group.value.clone());
        }
        list_of_vals.dedup();

        if self.triplets().len() + self.kans().len() == 3 {
            if list_of_vals.len() == 1 {
                return true;
            }
        } else if list_of_vals.len() == 2 {
            return true;
        }

        false
    }

    /// Check if the hand only contains tiles of a single suit.
    pub fn is_chinitsu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let mut suits: Vec<Suit> = self.groups.iter().map(|x| x.suit.clone()).collect();
        suits.dedup();

        suits.len() == 1
    }

    //yakuman

    /// Check if the hand contains three dragon triplets (or quads).
    pub fn is_daisangen(&self) -> bool {
        let trips: Vec<String> = self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .map(|x| x.value.clone())
            .collect();

        trips.contains(&"r".to_string())
            && trips.contains(&"g".to_string())
            && trips.contains(&"w".to_string())
    }

    /// Check if the hand contains four concealed triplets.
    ///
    /// If winning by ron, the last completed shape cannot be the triplet (e.g. only the pair).
    pub fn is_suuankou(&self, tsumo: bool) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.triplets().len() + self.kans().len() != 4 || self.isopen {
            return false;
        }

        if !tsumo && self.groups.last().unwrap().group_type == GroupType::Triplet {
            return false;
        }

        true
    }

    /// Check if the hand contains four concealed triplets, with the last completed shape being the pair.
    ///
    /// This yaku can be awarded on a ron with a tanki (single pair) wait.
    pub fn is_suuankoutankiwait(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.triplets().len() + self.kans().len() != 4 || self.isopen {
            return false;
        }

        if self.groups.last().unwrap().group_type == GroupType::Pair {
            return true;
        }

        false
    }

    /// Check if the hand only contains terminal tiles.
    pub fn is_chinroutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.kans().len() + self.triplets().len() != 4 {
            return false;
        }

        for group in self.groups.clone() {
            if !group.isterminal {
                return false;
            }
        }

        true
    }

    /// Check if the hand only contains the 2-3-4-6-8 sou (bamboo) tiles and green dragon tile.
    ///
    /// This is commonly known as "all greens".
    pub fn is_ryuuiisou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if !self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .chain(self.pairs().iter())
            .all(|group| ["2", "3", "4", "6", "8", "g"].contains(&group.value.as_str()))
        {
            return false;
        }

        for group in self.sequences() {
            if group.value != "2" {
                return false;
            }
        }

        true
    }

    /// Check if the hand consists of 1112345678999 in the same suit, plus one additional tile of that suit.
    pub fn is_chuurenpoutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let suit: Suit = self.groups[0].suit.clone();
        if self.triplets().len() != 2 || self.sequences().len() != 2 || self.pairs().len() != 1 {
            return false;
        }

        for group in self.groups.clone() {
            if group.suit != suit {
                return false;
            }
        }

        let has_1 = self.triplets().clone().iter().any(|i| i.value == "1");
        let has_9 = self.triplets().clone().iter().any(|i| i.value == "9");
        if !has_1 || !has_9 {
            return false;
        }

        let mut vals: Vec<u8> = vec![];
        for sequence_group in self.sequences() {
            let int = sequence_group.value.parse::<u8>().unwrap();
            vals.push(int);
            vals.push(int + 1);
            vals.push(int + 2);
        }

        for pair_group in self.pairs() {
            let int = pair_group.value.parse::<u8>().unwrap();
            vals.push(int);
        }

        vals.sort();
        if vals != [2, 3, 4, 5, 6, 7, 8] {
            return false;
        }

        true
    }

    /// Check if the hand consists of 1112345678999 in the same suit, plus one additional tile of that suit.
    ///
    /// This variant checks that the hand was completed with a 9-sided wait.
    pub fn is_chuurenpoutou9sided(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if !self.is_chuurenpoutou() {
            return false;
        }

        if self.groups.last().unwrap().group_type != GroupType::Pair {
            return false;
        }

        true
    }

    /// Check if the hand only consists of honor tiles.
    pub fn is_tsuuiisou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        for group in self.groups.clone() {
            if !group.is_honor() {
                return false;
            }
        }

        true
    }

    /// Check if the hand only consists of honor tiles as seven pairs.
    pub fn is_daichiishin(&self) -> bool {
        self.is_tsuuiisou() && self.pairs().len() == 7
    }

    /// Check if the hand has four quads.
    pub fn is_suukantsu(&self) -> bool {
        self.kans().len() == 4
    }

    /// Check if the hand has three wind triplets (or quads) and a wind pair.
    pub fn is_shousuushii(&self) -> bool {
        self.groups
            .iter()
            .filter(|i| i.suit == Suit::Wind && i.group_type != GroupType::None)
            .count()
            == 4
    }

    /// Check if the hand has four wind triplets (or quads).
    pub fn is_daisuushii(&self) -> bool {
        self.triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|i| i.suit == Suit::Wind)
            .count()
            == 4
    }

    /// Check if the hand has one of each type of terminal and honor tile and one additional terminal or honor tile.
    pub fn is_kokushi(&self) -> bool {
        if self.singles().len() != 12 || self.pairs().len() != 1 {
            return false;
        }

        let singles = self.groups.clone();
        let mut vals: Vec<String> = vec![];
        for i in singles {
            vals.push(i.value.clone());
        }
        vals.dedup();

        vals.len() == 13
    }

    /// Check if the hand has one of each type of terminal and honor tile and one additional terminal or honor tile, on a 13-sided wait.
    pub fn is_kokushi13sided(&self) -> bool {
        self.is_kokushi() && self.groups.last().unwrap().group_type == GroupType::Pair
    }

    /// Check if the player is the dealer and has a winning hand in the uninterrupted first turn.
    ///
    /// Calling a kan counts as interrupting the turn order.
    pub fn is_tenhou(&self, tenhou: bool) -> bool {
        if tenhou && self.seat_tile().value == "E" {
            return true;
        }
        false
    }

    /// Check if the player is in a non-dealer seat and has a winning hand in the first non-interrupted turn.
    ///
    /// Calling a kan counts as interrupting the turn order.
    pub fn is_chiihou(&self, tenhou: bool) -> bool {
        if tenhou && self.seat_tile().value != "E" {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Hand;
    use crate::hand::error::HandErr;

    #[test]
    fn yaku_kokushi() {
        let out = Hand::new(
            vec![
                "1s".to_string(),
                "9s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "rd".to_string(),
                "wwd".to_string(),
            ],
            "wd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_kokushi());
        assert!(out.is_kokushi13sided());
        let out = Hand::new(
            vec![
                "1s".to_string(),
                "9s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "wwd".to_string(),
                "rd".to_string(),
            ],
            "rd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_kokushi());
        assert!(!out.is_kokushi13sided());
        let out = Hand::new(
            vec![
                "9s".to_string(),
                "9s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "rd".to_string(),
                "wwd".to_string(),
            ],
            "wd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_kokushi());
    }

    #[test]
    fn yaku_daisuushii() {
        let out = Hand::new(
            vec![
                "EEEEw".to_string(),
                "SSSw".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daisuushii());
        let out = Hand::new(
            vec![
                "EEEEw".to_string(),
                "SSw".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_daisuushii());
    }

    #[test]
    fn yaku_shousuushi() {
        let out = Hand::new(
            vec![
                "EEEEw".to_string(),
                "SSw".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_shousuushii());
        let out = Hand::new(
            vec![
                "EEEEw".to_string(),
                "22s".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_shousuushii());
    }

    #[test]
    fn yaku_suukantsu() {
        let out = Hand::new(
            vec![
                "EEEEw".to_string(),
                "2222p".to_string(),
                "1111mo".to_string(),
                "7777s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suukantsu());
        let out = Hand::new(
            vec![
                "EEEw".to_string(),
                "2222p".to_string(),
                "1111mo".to_string(),
                "7777s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_suukantsu());
    }

    #[test]
    fn yaku_daichiishin() {
        let out = Hand::new(
            vec![
                "SSw".to_string(),
                "rrd".to_string(),
                "ggd".to_string(),
                "wwd".to_string(),
                "NNw".to_string(),
                "SSw".to_string(),
                "EEw".to_string(),
            ],
            "Ew".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daichiishin());
        let out = Hand::new(
            vec![
                "WWw".to_string(),
                "NNNw".to_string(),
                "gggd".to_string(),
                "rrrd".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_daichiishin());
    }

    #[test]
    fn yaku_tsuuiisou() {
        let out = Hand::new(
            vec![
                "SSw".to_string(),
                "NNNw".to_string(),
                "gggd".to_string(),
                "rrrd".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_tsuuiisou());
        let out = Hand::new(
            vec![
                "11s".to_string(),
                "NNNw".to_string(),
                "gggd".to_string(),
                "rrrd".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_tsuuiisou());
    }

    #[test]
    fn yaku_cuurenpoutou() {
        let out = Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "55s".to_string(),
                "678s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chuurenpoutou());
        let out = Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "55m".to_string(),
                "678s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chuurenpoutou());
        let out = Hand::new(
            vec![
                "123s".to_string(),
                "234s".to_string(),
                "555s".to_string(),
                "678s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chuurenpoutou());
        let out = Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "678s".to_string(),
                "999s".to_string(),
                "55s".to_string(),
            ],
            "5s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chuurenpoutou9sided());
        let out = Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "678s".to_string(),
                "55s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chuurenpoutou9sided());
    }

    #[test]
    fn yaku_ryuuiisou() {
        let out = Hand::new(
            vec![
                "234s".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ryuuiisou());
        let out = Hand::new(
            vec![
                "345s".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ryuuiisou());

        let out = Hand::new(
            vec![
                "234s".to_string(),
                "234s".to_string(),
                "666s".to_string(),
                "gggd".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ryuuiisou());
    }

    #[test]
    fn yaku_chinroutou() {
        let out = Hand::new(
            vec![
                "111so".to_string(),
                "1111m".to_string(),
                "999s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chinroutou());
        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "1111m".to_string(),
                "999s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chinroutou());
        let out = Hand::new(
            vec![
                "111s".to_string(),
                "1111m".to_string(),
                "789s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chinroutou());
    }

    #[test]
    fn yaku_suuankoutankiwait() {
        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "111s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suuankoutankiwait());
        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_suuankoutankiwait());
    }

    #[test]
    fn yaku_suuankou() {
        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "111s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suuankou(false));

        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suuankou(true));
        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_suuankou(false));
    }

    #[test]
    fn yaku_daisangen() {
        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "gggd".to_string(),
                "wwwwd".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daisangen());
        let out = Hand::new(
            vec![
                "rrrrd".to_string(),
                "ggggd".to_string(),
                "wwwwd".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daisangen());

        let out = Hand::new(
            vec![
                "rrrrd".to_string(),
                "gggd".to_string(),
                "wwd".to_string(),
                "888p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_daisangen());
    }

    #[test]
    fn yaku_chinitsu() {
        let out = Hand::new(
            vec![
                "222p".to_string(),
                "123p".to_string(),
                "345p".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chinitsu());
        let out = Hand::new(
            vec![
                "222p".to_string(),
                "123p".to_string(),
                "345p".to_string(),
                "88s".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chinitsu());
    }

    #[test]
    fn yaku_sanshokudoukou() {
        let out = Hand::new(
            vec![
                "222p".to_string(),
                "222m".to_string(),
                "222s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "6m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoukou());
        let out = Hand::new(
            vec![
                "222p".to_string(),
                "2222m".to_string(),
                "222s".to_string(),
                "3333s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoukou());
        let out = Hand::new(
            vec![
                "222p".to_string(),
                "333m".to_string(),
                "222s".to_string(),
                "11s".to_string(),
                "333s".to_string(),
            ],
            "3s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanshokudoukou());
    }

    #[test]
    fn yaku_pinfu() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "6m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_pinfu());

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "678po".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "5m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_pinfu());
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "5m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_pinfu());
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_pinfu());
    }

    #[test]
    fn yaku_menzentsumo() {
        let out = Hand::new(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_menzentsumo(true));

        let out = Hand::new(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_menzentsumo(false));

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_menzentsumo(false));
    }

    #[test]
    fn yaku_chiitoitsu() {
        let out = Hand::new(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chiitoitsu());
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chiitoitsu());
    }

    #[test]
    fn yaku_chantaiyao() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chantaiyao());

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "999m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chantaiyao());
        let out = Hand::new(
            vec![
                "111s".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "999m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chantaiyao());
    }

    #[test]
    fn yaku_ittsuu() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "456p".to_string(),
                "789p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ittsuu());

        let out = Hand::new(
            vec![
                "789m".to_string(),
                "456m".to_string(),
                "123m".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ittsuu());
        let out = Hand::new(
            vec![
                "123s".to_string(),
                "789s".to_string(),
                "456s".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ittsuu());
        let out = Hand::new(
            vec![
                "123m".to_string(),
                "456m".to_string(),
                "678m".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ittsuu());
    }

    #[test]
    fn yaku_sankantsu() {
        let out = Hand::new(
            vec![
                "9999so".to_string(),
                "123p".to_string(),
                "SSSSw".to_string(),
                "EEEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sankantsu());
        let out = Hand::new(
            vec![
                "9999so".to_string(),
                "123p".to_string(),
                "SSSw".to_string(),
                "EEEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sankantsu());
    }

    #[test]
    fn yaku_honroutou() {
        let out = Hand::new(
            vec![
                "999s".to_string(),
                "111p".to_string(),
                "SSSw".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_honroutou());

        let out = Hand::new(
            vec![
                "999s".to_string(),
                "123p".to_string(),
                "SSSw".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_honroutou());

        let out = Hand::new(
            vec![
                "999s".to_string(),
                "111p".to_string(),
                "111s".to_string(),
                "999m".to_string(),
                "99p".to_string(),
            ],
            "9p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_honroutou());
    }

    #[test]
    fn yaku_junchantaiyao() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "123p".to_string(),
                "999m".to_string(),
                "789s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_junchantaiyao());

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "123p".to_string(),
                "999m".to_string(),
                "789s".to_string(),
                "ggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_junchantaiyao());

        let out = Hand::new(
            vec![
                "111s".to_string(),
                "111m".to_string(),
                "999m".to_string(),
                "999s".to_string(),
                "11p".to_string(),
            ],
            "1p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_junchantaiyao());
    }

    #[test]
    fn yaku_shousangen() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "wwwwd".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_shousangen());
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "wwwwd".to_string(),
                "rrd".to_string(),
                "234p".to_string(),
            ],
            "4p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_shousangen());
    }

    #[test]
    fn yaku_honitsu() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567p".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_honitsu());

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567m".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_honitsu());

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567p".to_string(),
                "111p".to_string(),
                "33p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //should be chinitsu not honitsu
        assert!(!out.is_honitsu());
    }

    #[test]
    fn yaku_sanankou() {
        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanankou(false));

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanankou(true));

        let out = Hand::new(
            vec![
                "123p".to_string(),
                "123s".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanankou(true));
    }

    #[test]
    fn yaku_sanshokudoujun() {
        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "234s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoujun());

        let out = Hand::new(
            vec![
                "678s".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "234s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoujun());

        let out = Hand::new(
            vec![
                "111p".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanshokudoujun());
    }

    #[test]
    fn yaku_toitoi() {
        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "EEEw".to_string(),
                "gggd".to_string(),
                "222p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_toitoi());

        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "2222m".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_toitoi());

        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "2222m".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
                "456so".to_string(),
            ],
            "5s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_toitoi());
    }

    #[test]
    fn yaku_yakuhai() {
        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = Hand::new(
            vec![
                "EEEEwo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 2);
        let out = Hand::new(
            vec![
                "3333mo".to_string(),
                "WWWm".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = Hand::new(
            vec![
                "3333mo".to_string(),
                "222m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 0);
    }

    #[test]
    fn yaku_ryanpeikou() {
        let out = Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "es".to_string(),
            "ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(out.is_ryanpeikou());

        let out = Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "678m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "es".to_string(),
            "ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(!out.is_ryanpeikou());
    }

    #[test]
    fn yaku_iipeko() {
        let out = Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(!out.is_iipeikou());

        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(!out.is_iipeikou());

        let out = Hand::new(
            vec![
                "rrrd".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_iipeikou());
    }

    #[test]
    fn yaku_tanyao() {
        let out = Hand::new(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "22s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_tanyao());
        let out = Hand::new(
            vec![
                "333mo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_tanyao());
        let out = Hand::new(
            vec![
                "555mo".to_string(),
                "678p".to_string(),
                "22s".to_string(),
                "333s".to_string(),
                "345m".to_string(),
            ],
            "4m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_tanyao());
    }

    #[test]
    fn invalid_group_sequence_not_in_order() {
        let out = Hand::new(
            vec![
                "135m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_group_size_too_small() {
        let out = Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidSuit);
    }

    #[test]
    fn invalid_group_size_too_big() {
        let out = Hand::new(
            vec![
                "SSSSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_suit() {
        let out = Hand::new(
            vec![
                "hhhjo".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidSuit);
    }

    #[test]
    fn hand_too_small() {
        let out = Hand::new(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidShape);
        let out = Hand::new(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidShape);
    }
    #[test]
    fn hand_too_big() {
        let out = Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidShape);
    }
}
