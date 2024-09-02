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

                    let win_int = win_tile.into_u8().unwrap();
                    let last_int = last_group.into_u8().unwrap();

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
                    let mid_tile = group.into_u8().unwrap() + 1;
                    if self.win_tile().into_u8().unwrap() == mid_tile {
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
                if &group.suit != s && group.is_honor() {
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
            if group.is_honor() {
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

#[derive(Debug, PartialEq)]
pub enum HandErr {
    InvalidGroup,
    InvalidSuit,
    InvalidShape,
    NoYaku,
    NoHandTiles,
    NoWinTile,
    DuplicateRiichi,
    IppatsuWithoutRiichi,
    DoubleRiichiHaiteiIppatsu,
    DoubleRiichiHaiteiChankan,
    ChankanTsumo,
    RinshanKanWithoutKan,
    RinshanWithoutTsumo,
    RinshanIppatsu,
    NoHan,
    NoFu,
}
impl std::fmt::Display for HandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGroup => write!(f, "Invalid Group found"),
            Self::InvalidSuit => write!(f, "Invalid Suit found"),
            Self::InvalidShape => write!(f, "Invalid Hand Shape found"),
            Self::NoYaku => write!(f, "No Yaku"),
            Self::NoHandTiles => write!(f, "No Hand Tiles given"),
            Self::NoWinTile => write!(f, "No Win Tile given"),
            Self::DuplicateRiichi => write!(f, "Cant Riichi and Double Riichi Simultaneously"),
            Self::IppatsuWithoutRiichi => write!(f, "Cant Ippatsu without Riichi"),
            Self::ChankanTsumo => write!(f, "Cant Tsumo and Chankan"),
            Self::RinshanKanWithoutKan => write!(f, "Cant Rinshan without Kan"),
            Self::RinshanWithoutTsumo => write!(f, "Cant Rinshan without Tsumo"),
            Self::RinshanIppatsu => write!(f, "Cant Rinshan and Ippatsu"),
            Self::DoubleRiichiHaiteiIppatsu => {
                write!(f, "Cant Double Riichi, Ippatsu and Haitei")
            }
            Self::DoubleRiichiHaiteiChankan => {
                write!(f, "Cant Double Riichi, Ippatsu and Haitei")
            }
            Self::NoHan => write!(f, "No Han provided!"),
            Self::NoFu => write!(f, "No Fu provided!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Fu {
    BasePoints,
    BasePointsChitoi,
    ClosedRon,
    Tsumo,
    NonSimpleClosedTriplet,
    SimpleClosedTriplet,
    NonSimpleOpenTriplet,
    SimpleOpenTriplet,
    NonSimpleClosedKan,
    SimpleClosedKan,
    NonSimpleOpenKan,
    SimpleOpenKan,
    Toitsu,
    SingleWait,
}

impl std::fmt::Display for Fu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BasePoints => write!(f, "BasePoints: 20"),
            Self::BasePointsChitoi => write!(f, "BasePoints: 25"),
            Self::ClosedRon => write!(f, "ClosedRon: 10"),
            Self::Tsumo => write!(f, "Tsumo: 2"),
            Self::NonSimpleClosedTriplet => write!(f, "NonSimpleClosedTriplet: 8"),
            Self::SimpleClosedTriplet => write!(f, "ClosedTriplet: 4"),
            Self::NonSimpleOpenTriplet => write!(f, "NonSimpleOpenTriplet: 4"),
            Self::SimpleOpenTriplet => write!(f, "OpenTriplet: 2"),
            Self::NonSimpleClosedKan => write!(f, "NonSimpleClosedKan: 32"),
            Self::SimpleClosedKan => write!(f, "ClosedKan: 16"),
            Self::NonSimpleOpenKan => write!(f, "NonSimpleOpenKan: 16"),
            Self::SimpleOpenKan => write!(f, "OpenKan: 8"),
            Self::Toitsu => write!(f, "Toitsu: 2"),
            Self::SingleWait => write!(f, "SingleWait: 2"),
        }
    }
}

impl Fu {
    /// Get the minipoint value.
    pub fn value(&self) -> u16 {
        match self {
            Self::BasePoints => 20,
            Self::BasePointsChitoi => 25,
            Self::ClosedRon => 10,
            Self::Tsumo => 2,
            Self::NonSimpleClosedTriplet => 8,
            Self::SimpleClosedTriplet => 4,
            Self::NonSimpleOpenTriplet => 4,
            Self::SimpleOpenTriplet => 2,
            Self::NonSimpleClosedKan => 32,
            Self::SimpleClosedKan => 16,
            Self::NonSimpleOpenKan => 16,
            Self::SimpleOpenKan => 8,
            Self::Toitsu => 2,
            Self::SingleWait => 2,
        }
    }
}

/// Sum up all of the fu, rounding to the nearest 10.
pub fn calculate_total_fu_value(fu: &[Fu]) -> u16 {
    ((fu.iter().map(|f| f.value()).sum::<u16>() + 9) / 10) * 10
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileGroup {
    pub value: String,
    pub suit: Suit,
    pub isopen: bool,
    pub group_type: GroupType,
    pub isterminal: bool,
}

impl TileGroup {
    pub fn new(group: String) -> Result<Self, HandErr> {
        let isopen = group.chars().last().unwrap().to_string() == "o";
        let value = group.chars().nth(0).unwrap().to_string();
        let suit = if !isopen {
            group.chars().last().unwrap().to_string()
        } else {
            group.chars().nth(group.len() - 2).unwrap().to_string()
        };

        let suit = Suit::suit_from_string(suit)?;
        let group_type = GroupType::group_type_from_string(group.to_string())?;
        let mut isterminal = false;
        if group_type == GroupType::Sequence {
            if value == "1" || value == "7" {
                isterminal = true;
            }
        } else if value == "1" || value == "9" {
            isterminal = true;
        }
        let tile = Self {
            value,
            suit,
            isopen,
            group_type,
            isterminal,
        };
        Ok(tile)
    }

    /// Check if the group is an honor.
    pub fn honor(&self) -> bool {
        matches!(self.suit, Suit::Wind | Suit::Dragon)
    }

    /// Parse the group value into an integer.
    pub fn into_u8(&self) -> Result<u8, std::num::ParseIntError> {
        self.value.parse()
    }
}

//AHAHAHAHAHAHAHAH I DONT NEED THIS
//turns our i did need this :)
#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    Sequence,
    Triplet,
    Kan,
    Pair,
    None,
}

impl GroupType {
    /// Parse the group type from the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::GroupType;
    ///
    /// let input = "789s".to_string();
    /// let actual = GroupType::group_type_from_string(input);
    /// let expected = GroupType::Sequence;
    ///
    /// assert_eq!(actual, expected);
    /// ```
    pub fn group_type_from_string(group: String) -> Result<Self, HandErr> {
        let count = if group.contains('o') {
            group.len() - 2
        } else {
            group.len() - 1
        };

        if let Some(sub_group) = group.get(0..count) {
            for i in sub_group.chars() {
                if !"123456789ESWNrgw".contains(i) {
                    return Err(HandErr::InvalidGroup);
                }
            }
        } else {
            return Err(HandErr::InvalidGroup);
        }

        match count {
            2 => Ok(Self::Pair),
            3 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                {
                    Ok(Self::Triplet)
                } else if ["123", "234", "345", "456", "567", "678", "789"]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<&str>>()
                    .contains(group.get(0..count).unwrap())
                {
                    return Ok(Self::Sequence);
                } else {
                    return Err(HandErr::InvalidGroup);
                }
            }
            4 => Ok(Self::Kan),
            1 => Ok(Self::None),
            _ => Err(HandErr::InvalidGroup),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Wind,
    Dragon,
}

impl Suit {
    /// Parse the suit from the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let tile_string = "9m";
    /// let actual_suit = Suit::suit_from_string(tile_string.chars().nth(1).unwrap().to_string());
    /// let expected = Suit::Manzu;
    ///
    /// assert_eq!(actual_suit, expected);
    /// ```
    pub fn suit_from_string(suit: String) -> Result<Self, HandErr> {
        match suit.as_str() {
            "s" => Ok(Self::Souzu),
            "p" => Ok(Self::Pinzu),
            "m" => Ok(Self::Manzu),
            "w" => Ok(Self::Wind),
            "d" => Ok(Self::Dragon),
            _ => Err(HandErr::InvalidSuit),
        }
    }
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
