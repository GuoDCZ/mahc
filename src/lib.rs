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

    pub fn calculate_fu(&self, tsumo: bool) -> u32 {
        //TODO REMOVE THESE PRINTS (and make this calculation less fucky)
        let mut totalfu = 20;
        println!("base : {:?}", totalfu);
        if tsumo {
            totalfu += 2;
            println!("tsumo : {:?}", 2);
        }
        if !self.is_open() {
            totalfu += 10;
            println!("calosed : {:?}", 10);
        }
        //meld fu cal
        for i in &self.triplets() {
            let mut fu = 2;
            if i == self.groups.last().unwrap() {
                if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    fu *= 2;
                }
                if tsumo {
                    fu *= 2;
                }
                totalfu += fu;
                println!("triplet {:?}", fu);
                continue;
            }
            let mut fu = 2;
            if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                fu *= 2;
            }
            if !i.isopen {
                fu *= 2;
            }
            println!("triplet {:?}", fu);
            totalfu += fu;
        }
        for i in &self.kans() {
            let mut fu = 8;
            if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                fu *= 2;
            }
            if !i.isopen {
                fu *= 2;
            }
            totalfu += fu;
            println!("kan{:?}", fu);
        }
        for i in self.pairs() {
            let mut fu = 0;
            if i.value == self.prev_tile.value
                || i.value == self.seat_tile.value
                || i.suit == Suit::Dragon
            {
                fu = 2;
            }
            println!("pair{:?}", fu);
            totalfu += fu;
        }
        //fu wait cal
        if self.groups.last().unwrap().group_type == GroupType::Pair {
            totalfu += 2;
            println!("pairwait{:?}", 2);
        }
        if self.groups.last().unwrap().group_type == GroupType::Sequence {
            let midtile = self.groups.last().unwrap().value.parse::<u8>().unwrap() + 1;
            if self.win_tile().value == midtile.to_string() {
                println!("midwait{:?}", 2);
                totalfu += 2;
            }
            if !(self.win_tile().value == "1" || self.win_tile().value == "9")
                && self.groups.last().unwrap().isterminal
            {
                println!("single terminal{:?}", 2);
                totalfu += 2;
            }
        }
        println!("ttal:fu{:?}", totalfu);
        assert_eq!(true, false);
        //works cuz ints
        return ((totalfu + 9) / 10) * 10;
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
