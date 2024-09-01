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
    prev_tile: TileGroup,
    isopen: bool,
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
            HandErr::InvalidGroup => write!(f, "Invalid Group found"),
            HandErr::InvalidSuit => write!(f, "Invalid Suit found"),
            HandErr::InvalidShape => write!(f, "Invalid Hand Shape found"),
            HandErr::NoYaku => write!(f, "No Yaku"),
            HandErr::NoHandTiles => write!(f, "No Hand Tiles given"),
            HandErr::NoWinTile => write!(f, "No Win Tile given"),
            HandErr::DuplicateRiichi => write!(f, "Cant Riichi and Double Riichi Simulatinously"),
            HandErr::IppatsuWithoutRiichi => write!(f, "Cant Ippatsu without Riichi"),
            HandErr::ChankanTsumo => write!(f, "Cant Tsumo and Chankan"),
            HandErr::RinshanKanWithoutKan => write!(f, "Cant Rinshan without Kan"),
            HandErr::RinshanWithoutTsumo => write!(f, "Cant Rinshan without tsumo"),
            HandErr::RinshanIppatsu => write!(f, "Cant Rinshan and Ippatsu"),
            HandErr::DoubleRiichiHaiteiIppatsu => {
                write!(f, "Cant Double Riichi, Ippatsu and haitei")
            }
            HandErr::DoubleRiichiHaiteiChankan => {
                write!(f, "Cant Double Riichi, Ippatsu and haitei")
            }
            HandErr::NoHan => write!(f, "No han provided!"),
            HandErr::NoFu => write!(f, "No fu provided!"),
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
            Fu::BasePoints => write!(f, "BasePoints: 20"),
            Fu::BasePointsChitoi => write!(f, "BasePoints: 25"),
            Fu::ClosedRon => write!(f, "ClosedRon: 10"),
            Fu::Tsumo => write!(f, "Tsumo: 2"),
            Fu::NonSimpleClosedTriplet => write!(f, "NonSimpleClosedTriplet: 8"),
            Fu::SimpleClosedTriplet => write!(f, "ClosedTriplet: 4"),
            Fu::NonSimpleOpenTriplet => write!(f, "NonSimpleOpenTriplet: 4"),
            Fu::SimpleOpenTriplet => write!(f, "OpenTriplet: 2"),
            Fu::NonSimpleClosedKan => write!(f, "NonSimpleClosedKan: 32"),
            Fu::SimpleClosedKan => write!(f, "ClosedKan: 16"),
            Fu::NonSimpleOpenKan => write!(f, "NonSimpleOpenKan: 16"),
            Fu::SimpleOpenKan => write!(f, "OpenKan: 8"),
            Fu::Toitsu => write!(f, "Toitsu: 2"),
            Fu::SingleWait => write!(f, "SingleWait: 2"),
        }
    }
}

impl Fu {
    /// Get the minipoint value.
    pub fn value(&self) -> u16 {
        match self {
            Fu::BasePoints => 20,
            Fu::BasePointsChitoi => 25,
            Fu::ClosedRon => 10,
            Fu::Tsumo => 2,
            Fu::NonSimpleClosedTriplet => 8,
            Fu::SimpleClosedTriplet => 4,
            Fu::NonSimpleOpenTriplet => 4,
            Fu::SimpleOpenTriplet => 2,
            Fu::NonSimpleClosedKan => 32,
            Fu::SimpleClosedKan => 16,
            Fu::NonSimpleOpenKan => 16,
            Fu::SimpleOpenKan => 8,
            Fu::Toitsu => 2,
            Fu::SingleWait => 2,
        }
    }
}

pub fn calculate_total_fu_value(fu: &[Fu]) -> u16 {
    ((fu.iter().map(|f| f.value()).sum::<u16>() + 9) / 10) * 10
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
        for i in &tile_groups {
            match i.group_type {
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
            match tile_groups.last().unwrap().group_type {
                GroupType::Sequence => {
                    if win_tile.suit != tile_groups.last().unwrap().suit {
                        return Err(HandErr::InvalidShape);
                    }
                    let win_int = win_tile.value.parse::<u8>().unwrap();
                    let last_int = tile_groups.last().unwrap().value.parse::<u8>().unwrap();
                    if win_int != last_int && win_int != last_int + 1 && win_int != last_int + 2 {
                        return Err(HandErr::InvalidShape);
                    }
                }
                GroupType::Triplet | GroupType::Pair => {
                    if tile_groups.last().unwrap().value != win_tile.value
                        || tile_groups.last().unwrap().suit != win_tile.suit
                    {
                        return Err(HandErr::InvalidShape);
                    }
                }
                GroupType::Kan => return Err(HandErr::InvalidShape),
                GroupType::None => return Err(HandErr::InvalidShape),
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

        let hand = Hand {
            groups: tile_groups,
            win_tile,
            seat_tile,
            prev_tile,
            isopen: ishandopen,
        };

        Ok(hand)
    }

    pub fn calculate_fu(&self, tsumo: bool) -> Vec<Fu> {
        //TODO REMOVE THESE PRINTS (and make this calculation less fucky)
        let mut fu_types: Vec<Fu> = vec![];
        fu_types.push(Fu::BasePoints);

        if tsumo {
            fu_types.push(Fu::Tsumo);
        }
        if !self.is_open() {
            fu_types.push(Fu::ClosedRon);
        }
        //meld fu cal
        for i in &self.triplets() {
            if i == self.groups.last().unwrap() {
                if tsumo {
                    if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                        fu_types.push(Fu::NonSimpleClosedTriplet);
                    } else {
                        fu_types.push(Fu::SimpleClosedTriplet);
                    }
                } else if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    fu_types.push(Fu::NonSimpleOpenTriplet);
                } else {
                    fu_types.push(Fu::SimpleOpenTriplet);
                }
                continue;
            }
            if !(i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal) && i.isopen {
                fu_types.push(Fu::SimpleOpenTriplet);
            }
            if !i.isopen {
                if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    fu_types.push(Fu::NonSimpleClosedTriplet);
                } else {
                    fu_types.push(Fu::SimpleClosedTriplet);
                }
            } else if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                fu_types.push(Fu::NonSimpleOpenTriplet);
            }
        }
        for i in &self.kans() {
            if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                if !i.isopen {
                    fu_types.push(Fu::NonSimpleClosedKan);
                } else {
                    fu_types.push(Fu::NonSimpleOpenKan);
                }
            } else if !i.isopen {
                fu_types.push(Fu::SimpleClosedKan);
            } else {
                fu_types.push(Fu::SimpleOpenKan);
            }
        }
        for i in self.pairs() {
            if i.value == self.prev_tile.value
                || i.value == self.seat_tile.value
                || i.suit == Suit::Dragon
            {
                fu_types.push(Fu::Toitsu);
            }
        }
        //fu wait cal
        if self.groups.last().unwrap().group_type == GroupType::Pair {
            fu_types.push(Fu::SingleWait);
        }
        if self.groups.last().unwrap().group_type == GroupType::Sequence {
            let midtile = self.groups.last().unwrap().value.parse::<u8>().unwrap() + 1;
            if self.win_tile().value == midtile.to_string() {
                fu_types.push(Fu::SingleWait);
            }
            if !(self.win_tile().value == "1" || self.win_tile().value == "9")
                && self.groups.last().unwrap().isterminal
            {
                fu_types.push(Fu::SingleWait);
            }
        }

        fu_types
    }

    pub fn sequences(&self) -> Vec<TileGroup> {
        let mut seqs: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Sequence {
                seqs.push(i);
            }
        }
        seqs
    }
    pub fn triplets(&self) -> Vec<TileGroup> {
        let mut trips: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Triplet {
                trips.push(i);
            }
        }
        trips
    }
    pub fn kans(&self) -> Vec<TileGroup> {
        let mut kans: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Kan {
                kans.push(i);
            }
        }
        kans
    }
    pub fn pairs(&self) -> Vec<TileGroup> {
        let mut pairs: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Pair {
                pairs.push(i);
            }
        }
        pairs
    }
    pub fn singles(&self) -> Vec<TileGroup> {
        let mut singles: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::None {
                singles.push(i);
            }
        }
        singles
    }
    pub fn win_tile(&self) -> TileGroup {
        self.win_tile.clone()
    }
    pub fn seat_tile(&self) -> TileGroup {
        self.seat_tile.clone()
    }
    pub fn prev_tile(&self) -> TileGroup {
        self.prev_tile.clone()
    }
    pub fn is_open(&self) -> bool {
        self.isopen
    }

    //yaku validation
    pub fn is_tanyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        for i in self.groups.clone() {
            if i.isterminal || i.suit == Suit::Dragon || i.suit == Suit::Wind {
                return false;
            }
        }
        true
    }
    pub fn is_ryanpeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();
        if seqs.len() != 4 {
            return false;
        }
        seqs.dedup();
        seqs.len() == 2
    }
    pub fn is_iipeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();
        seqs.dedup();
        !(self.sequences().len() == seqs.len() || self.is_open() || self.is_ryanpeikou())
    }
    pub fn is_yakuhai(&self) -> u16 {
        // i do it like this because a single group can have multiple yakuhai
        let mut count = 0;
        for i in self.triplets() {
            if i.value == self.prev_tile.value {
                count += 1;
            }
            if i.value == self.seat_tile.value {
                count += 1;
            }
            if i.suit == Suit::Dragon {
                count += 1;
            }
        }
        for i in self.kans() {
            if i.value == self.prev_tile.value {
                count += 1;
            }
            if i.value == self.seat_tile.value {
                count += 1;
            }
            if i.suit == Suit::Dragon {
                count += 1;
            }
        }
        count
    }
    pub fn is_toitoi(&self) -> bool {
        self.triplets().len() + self.kans().len() == 4
    }
    pub fn is_sanankou(&self, tsumo: bool) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        let mut closed_triplet_count = 0;
        for i in self.triplets() {
            if !i.isopen {
                closed_triplet_count += 1;
            }
        }
        for i in self.kans() {
            if !i.isopen {
                closed_triplet_count += 1;
            }
        }
        if !tsumo && self.groups.last().unwrap().group_type == GroupType::Triplet {
            closed_triplet_count -= 1;
        }

        closed_triplet_count == 3
    }
    pub fn is_sanshokudoujun(&self) -> bool {
        if self.sequences().len() < 3 {
            return false;
        }
        let mut list_of_vals: Vec<String> = vec![];
        for i in self.sequences() {
            list_of_vals.push(i.value.clone());
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
    pub fn is_honitsu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        let tile_groups = self.groups.clone();
        let mut has_honor = false;
        let mut has_normal = false;
        let mut suit: Option<Suit> = None;
        for i in &tile_groups {
            if i.suit == Suit::Dragon || i.suit == Suit::Wind {
                has_honor = true;
            } else {
                has_normal = true;
                suit = Some(i.suit.clone());
            }
        }

        if !has_normal || !has_honor {
            return false;
        }

        if let Some(s) = suit {
            for i in &tile_groups {
                if i.suit != s && i.suit != Suit::Dragon && i.suit != Suit::Wind {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }
    pub fn is_shousangen(&self) -> bool {
        let dragon_count = self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|i| i.suit == Suit::Dragon)
            .count();
        dragon_count == 2 && self.pairs()[0].suit == Suit::Dragon
    }
    pub fn is_junchantaiyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        for i in self.groups.clone() {
            if i.suit == Suit::Dragon || i.suit == Suit::Wind || !i.isterminal {
                return false;
            }
        }
        !(self.sequences().is_empty())
    }
    pub fn is_honroutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        if !self.sequences().is_empty() {
            return false;
        }
        let mut has_terminal: bool = false;
        let mut has_honor: bool = false;
        for i in self.groups.clone() {
            if i.isterminal {
                has_terminal = true;
            } else if i.suit == Suit::Dragon || i.suit == Suit::Wind {
                has_honor = true;
            } else {
                return false;
            }
        }
        has_terminal && has_honor
    }
    pub fn is_sankantsu(&self) -> bool {
        self.kans().len() == 3
    }
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
    pub fn is_chantaiyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        if self.sequences().is_empty() {
            return false;
        }
        let mut has_terminal: bool = false;
        let mut has_honor: bool = false;
        for i in self.groups.clone() {
            if i.isterminal {
                has_terminal = true;
            } else if i.suit == Suit::Dragon || i.suit == Suit::Wind {
                has_honor = true;
            } else {
                return false;
            }
        }
        has_terminal && has_honor
    }
    pub fn is_chiitoitsu(&self) -> bool {
        self.pairs().len() == 7
    }
    pub fn is_menzentsumo(&self, tsumo: bool) -> bool {
        !self.isopen && tsumo
    }
    pub fn is_pinfu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        if self.isopen {
            return false;
        }
        let fu = self.calculate_fu(false);
        for fu_type in fu {
            if !matches!(fu_type, Fu::ClosedRon | Fu::BasePoints) {
                return false;
            }
        }

        true
    }
    pub fn is_sanshokudoukou(&self) -> bool {
        if self.triplets().len() + self.kans().len() < 3 {
            return false;
        }
        let mut list_of_vals: Vec<String> = vec![];
        for i in self.triplets().iter().chain(self.kans().iter()) {
            list_of_vals.push(i.value.clone());
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
    pub fn is_chinitsu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        let mut suits: Vec<Suit> = self.groups.iter().map(|x| x.suit.clone()).collect();
        suits.dedup();
        suits.len() == 1
    }
    //yakuman
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

    pub fn is_chinroutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        if self.kans().len() + self.triplets().len() != 4 {
            return false;
        }
        for i in self.groups.clone() {
            if !i.isterminal {
                return false;
            }
        }
        true
    }
    pub fn is_ryuuiisou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        for i in self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .chain(self.pairs().iter())
        {
            match i.value.as_str() {
                "2" | "3" | "4" | "6" | "8" | "g" => continue,
                _ => return false,
            }
        }
        for i in self.sequences() {
            if i.value != "2" {
                return false;
            }
        }
        true
    }
    pub fn is_chuurenpoutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        let suit: Suit = self.groups[0].suit.clone();
        if self.triplets().len() != 2 || self.sequences().len() != 2 || self.pairs().len() != 1 {
            return false;
        }
        for i in self.groups.clone() {
            if i.suit != suit {
                return false;
            }
        }
        let has_1 = self.triplets().clone().iter().any(|i| i.value == "1");
        let has_9 = self.triplets().clone().iter().any(|i| i.value == "9");
        if !has_1 || !has_9 {
            return false;
        }
        let mut vals: Vec<u8> = vec![];
        for i in self.sequences() {
            let int = i.value.parse::<u8>().unwrap();
            vals.push(int);
            vals.push(int + 1);
            vals.push(int + 2);
        }
        for i in self.pairs() {
            let int = i.value.parse::<u8>().unwrap();
            vals.push(int);
        }
        vals.sort();
        if vals != [2, 3, 4, 5, 6, 7, 8] {
            return false;
        }
        true
    }
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
    pub fn is_tsuuiisou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }
        for i in self.groups.clone() {
            if i.suit != Suit::Dragon && i.suit != Suit::Wind {
                return false;
            }
        }
        true
    }
    pub fn is_daichiishin(&self) -> bool {
        self.is_tsuuiisou() && self.pairs().len() == 7
    }
    pub fn is_suukantsu(&self) -> bool {
        self.kans().len() == 4
    }
    pub fn is_shousuushii(&self) -> bool {
        self.groups
            .iter()
            .filter(|i| i.suit == Suit::Wind && i.group_type != GroupType::None)
            .count()
            == 4
    }
    pub fn is_daisuushii(&self) -> bool {
        self.triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|i| i.suit == Suit::Wind)
            .count()
            == 4
    }
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
    pub fn is_kokushi13sided(&self) -> bool {
        self.is_kokushi() && self.groups.last().unwrap().group_type == GroupType::Pair
    }
    pub fn is_tenhou(&self, tenhou: bool) -> bool {
        if tenhou && self.seat_tile().value == "E" {
            return true;
        }
        false
    }
    pub fn is_chiihou(&self, tenhou: bool) -> bool {
        if tenhou && self.seat_tile().value != "E" {
            return true;
        }
        false
    }
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
        let tile = TileGroup {
            value,
            suit,
            isopen,
            group_type,
            isterminal,
        };
        Ok(tile)
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
    pub fn group_type_from_string(group: String) -> Result<GroupType, HandErr> {
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
            2 => Ok(GroupType::Pair),
            3 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                {
                    Ok(GroupType::Triplet)
                } else if ["123", "234", "345", "456", "567", "678", "789"]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<&str>>()
                    .contains(group.get(0..count).unwrap())
                {
                    return Ok(GroupType::Sequence);
                } else {
                    return Err(HandErr::InvalidGroup);
                }
            }
            4 => Ok(GroupType::Kan),
            1 => Ok(GroupType::None),
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
    pub fn suit_from_string(suit: String) -> Result<Suit, HandErr> {
        match suit.as_str() {
            "s" => Ok(Suit::Souzu),
            "p" => Ok(Suit::Pinzu),
            "m" => Ok(Suit::Manzu),
            "w" => Ok(Suit::Wind),
            "d" => Ok(Suit::Dragon),
            _ => Err(HandErr::InvalidSuit),
        }
    }
}

//TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
pub fn is_limit_hand(han: u16, fu: u16) -> bool {
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

impl LimitHands {
    //TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
    pub fn get_limit_hand(han: u16, fu: u16) -> Option<LimitHands> {
        if !is_limit_hand(han, fu) {
            return None;
        }
        if han <= 5 {
            Some(LimitHands::Mangan)
        } else if han <= 7 {
            return Some(LimitHands::Haneman);
        } else if han <= 10 {
            return Some(LimitHands::Baiman);
        } else if han <= 12 {
            return Some(LimitHands::Sanbaiman);
        } else {
            return Some(LimitHands::KazoeYakuman);
        }
    }
    pub fn get_score(&self) -> Vec<u16> {
        match self {
            LimitHands::Mangan => {
                vec![12000, 4000, 8000, 2000, 4000]
            }
            LimitHands::Haneman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    let j = i / 2;
                    out.push(i + j)
                }
                out
            }
            LimitHands::Baiman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 2)
                }
                out
            }
            LimitHands::Sanbaiman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 3)
                }
                out
            }
            LimitHands::KazoeYakuman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 4)
                }
                out
            }
        }
    }
}
