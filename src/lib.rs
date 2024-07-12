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
}

#[derive(Debug, PartialEq)]
pub enum Fu {
    BasePoints,
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

impl Fu {
    pub fn to_string(&self) -> String {
        match self{
            Fu::BasePoints =>"BasePoints: 20".to_string(), 
            Fu::ClosedRon =>"ClosedRon: 10".to_string(), 
            Fu::Tsumo =>"Tsumo: 2".to_string(), 
            Fu::NonSimpleClosedTriplet =>"NonSimpleClosedTriplet: 8".to_string(), 
            Fu::SimpleClosedTriplet =>"ClosedTriplet: 4".to_string(), 
            Fu::NonSimpleOpenTriplet =>"NonSimpleOpenTriplet: 4".to_string(), 
            Fu::SimpleOpenTriplet =>"OpenTriplet: 2".to_string(), 
            Fu::NonSimpleClosedKan =>"NonSimpleClosedKan: 32".to_string(), 
            Fu::SimpleClosedKan =>"ClosedKan: 16".to_string(), 
            Fu::NonSimpleOpenKan =>"NonSimpleOpenKan: 16".to_string(), 
            Fu::SimpleOpenKan =>"OpenKan: 8".to_string(), 
            Fu::Toitsu =>"Toitsu: 2".to_string(), 
            Fu::SingleWait =>"SingleWait: 2".to_string(), 
        }
    }

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
        for i in &tiles {
            let tile = TileGroup::new(i.to_string())?;
            if tile.isopen {
                ishandopen = true;
            }
            tile_groups.push(tile);
        }

        //TODO: standard hand ONLY CHECK MUST FIX FOR CHITOIT AND KOKUSHI
        //TODO: this can FORSURE be shorter
        let (mut tripcount, mut seqcount, mut paircount, mut kancount) = (0, 0, 0, 0);
        for i in &tile_groups {
            if i.group_type == GroupType::Triplet {
                tripcount += 1;
            } else if i.group_type == GroupType::Sequence {
                seqcount += 1;
            } else if i.group_type == GroupType::Pair {
                paircount += 1;
            } else if i.group_type == GroupType::Kan {
                kancount += 1;
            }
        }

        if !(tripcount + seqcount + kancount == 4 && paircount == 1) {
            return Err(HandErr::InvalidShape);
        }

        // AHAHAHAHAHAHAHAHAh (these are special cases for singular tiles)
        let win_tile = TileGroup {
            value: win.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(win.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: "19ESWNrgw".contains(win.chars().nth(0).unwrap()),
        };

        let seat_tile = TileGroup {
            value: seat.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(seat.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: "19ESWNrgw".contains(seat.chars().nth(0).unwrap()),
        };

        let prev_tile = TileGroup {
            value: prev.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(prev.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: "19ESWNrgw".contains(prev.chars().nth(0).unwrap()),
        };

        let hand = Hand {
            groups: tile_groups,
            win_tile,
            seat_tile,
            prev_tile,
            isopen: ishandopen,
        };

        return Ok(hand);
    }

    pub fn calculate_fu(&self, tsumo: bool) -> (u16, Vec<Fu>) {
        //TODO REMOVE THESE PRINTS (and make this calculation less fucky)
        let mut fu_types: Vec<Fu> = vec![];
        let mut totalfu = 20;
        fu_types.push(Fu::BasePoints);
        if tsumo {
            totalfu += 2;
            fu_types.push(Fu::Tsumo);
        }
        if !self.is_open() {
            totalfu += 10;
            fu_types.push(Fu::ClosedRon);
        }
        //meld fu cal
        for i in &self.triplets() {
            if i == self.groups.last().unwrap() {
                if tsumo {
                    if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                        fu_types.push(Fu::NonSimpleClosedTriplet);
                        totalfu += 8;
                    } else {
                        fu_types.push(Fu::SimpleClosedTriplet);
                        totalfu += 4;
                    }
                } else {
                    if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                        totalfu += 4;
                        fu_types.push(Fu::NonSimpleOpenTriplet);
                    } else {
                        totalfu += 2;
                        fu_types.push(Fu::SimpleOpenTriplet);
                    }
                }
                continue;
            }
            if !(i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal) && i.isopen {
                totalfu += 2;
                fu_types.push(Fu::SimpleOpenTriplet);
            }
            if !i.isopen {
                if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    totalfu += 8;
                    fu_types.push(Fu::NonSimpleClosedTriplet);
                } else {
                    totalfu += 4;
                    fu_types.push(Fu::SimpleClosedTriplet);
                }
            } else {
                if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    totalfu += 4;
                    fu_types.push(Fu::NonSimpleOpenTriplet);
                }
            }
        }
        for i in &self.kans() {
            if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                if !i.isopen {
                    totalfu += 32;
                    fu_types.push(Fu::NonSimpleClosedKan);
                } else {
                    fu_types.push(Fu::NonSimpleOpenKan);
                    totalfu += 16;
                }
            } else {
                if !i.isopen {
                    fu_types.push(Fu::SimpleClosedKan);
                    totalfu += 16;
                } else {
                    fu_types.push(Fu::SimpleOpenKan);
                    totalfu += 8;
                }
            }
        }
        for i in self.pairs() {
            if i.value == self.prev_tile.value
                || i.value == self.seat_tile.value
                || i.suit == Suit::Dragon
            {
                fu_types.push(Fu::Toitsu);
                totalfu += 2;
            }
        }
        //fu wait cal
        if self.groups.last().unwrap().group_type == GroupType::Pair {
            fu_types.push(Fu::SingleWait);
            totalfu += 2;
        }
        if self.groups.last().unwrap().group_type == GroupType::Sequence {
            let midtile = self.groups.last().unwrap().value.parse::<u8>().unwrap() + 1;
            if self.win_tile().value == midtile.to_string() {
                fu_types.push(Fu::SingleWait);
                totalfu += 2;
            }
            if !(self.win_tile().value == "1" || self.win_tile().value == "9")
                && self.groups.last().unwrap().isterminal
            {
                fu_types.push(Fu::SingleWait);
                totalfu += 2;
            }
        }
        //works cuz ints
        return (((totalfu + 9) / 10) * 10, fu_types);
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
        for i in self.groups.clone() {
            if i.isterminal || i.suit == Suit::Dragon || i.suit == Suit::Wind {
                return false;
            }
        }
        return true;
    }
    pub fn is_ryanpeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();
        if seqs.len() != 4 {
            return false;
        }
        seqs.dedup();
        if seqs.len() != 2 {
            return false;
        }
        return true;
    }
    pub fn is_iipeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();
        seqs.dedup();
        if self.sequences().len() == seqs.len() || self.is_open() || self.is_ryanpeikou(){
            return false;
        }
        return true;
    }
    pub fn is_yakuhai(&self) -> u16 {
        let mut count = 0;
        for i in self.triplets() {
            if i.value == self.prev_tile.value
                || i.value == self.seat_tile.value
                || i.suit == Suit::Dragon
            {
                count += 1;
            }
        }
        for i in self.kans() {
            if i.value == self.prev_tile.value
                || i.value == self.seat_tile.value
                || i.suit == Suit::Dragon
            {
                count += 1;
            }
        }
        return count;
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
        let mut suit: String = Default::default();

        if !isopen {
            suit = group.chars().last().unwrap().to_string();
        } else {
            suit = group.chars().nth(group.len() - 2).unwrap().to_string();
        }

        let suit = Suit::suit_from_string(suit)?;
        let group_type = GroupType::group_type_from_string(group.to_string())?;
        let mut isterminal = false;
        if group_type == GroupType::Sequence {
            if value == "1" || value == "7" {
                isterminal = true;
            }
        } else {
            if value == "1" || value == "9" {
                isterminal = true;
            }
        }
        let tile = TileGroup {
            value,
            suit,
            isopen,
            group_type,
            isterminal,
        };
        return Ok(tile);
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
        let mut count = 0;
        if group.contains('o') {
            count = group.len() - 2;
        } else {
            count = group.len() - 1;
        }

        for i in group.get(0..count).unwrap().chars() {
            if "123456789ESWNrgw".contains(i) {
                continue;
            } else {
                return Err(HandErr::InvalidGroup);
            }
        }

        match count {
            2 => return Ok(GroupType::Pair),
            3 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                {
                    return Ok(GroupType::Triplet);
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
            4 => return Ok(GroupType::Kan),
            _ => return Err(HandErr::InvalidGroup),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
            "s" => return Ok(Suit::Souzu),
            "p" => return Ok(Suit::Pinzu),
            "m" => return Ok(Suit::Manzu),
            "w" => return Ok(Suit::Wind),
            "d" => return Ok(Suit::Dragon),
            _ => return Err(HandErr::InvalidSuit),
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
    return false;
}

impl LimitHands {
    //TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
    pub fn get_limit_hand(han: u16, fu: u16) -> Option<LimitHands> {
        if !is_limit_hand(han, fu) {
            return None;
        }
        if han <= 5 {
            return Some(LimitHands::Mangan);
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
