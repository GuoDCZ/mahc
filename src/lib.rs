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
    sequences: Vec<TileGroup>,
    triplets: Vec<TileGroup>,
    kans: Vec<TileGroup>,
    pairs: Vec<TileGroup>,
    win_tile: TileGroup,
    isopen: bool,
}
#[derive(Debug, PartialEq)]
pub enum HandErr {
    InvalidGroup,
    InvalidSuit,
    InvalidShape,
}

impl Hand {
    pub fn new(tiles: Vec<String>, win: String) -> Result<Self, HandErr> {
        let mut sequences: Vec<TileGroup> = Vec::new();
        let mut triplets: Vec<TileGroup> = Vec::new();
        let mut kans: Vec<TileGroup> = Vec::new();
        let mut pairs: Vec<TileGroup> = Vec::new();
        let mut ishandopen = false;
        for i in &tiles {
            let tile = TileGroup::new(i.to_string())?;
            if tile.isopen {
                ishandopen = true;
            }
            match tile.group_type {
                GroupType::Sequence => sequences.push(tile),
                GroupType::Triplet => triplets.push(tile),
                GroupType::Kan => kans.push(tile),
                GroupType::Pair => pairs.push(tile),
                GroupType::None => (),
            }
        }

        //TODO: standard hand ONLY CHECK MUST FIX FOR CHITOIT AND KOKUSHI
        if !(sequences.len() + triplets.len() + kans.len() == 4 && pairs.len() == 1) {
            return Err(HandErr::InvalidShape);
        }

        let win_tile = TileGroup {
            value: win.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(win.chars().nth(1).unwrap().to_string()).unwrap(),
            isopen: false,
            group_type: GroupType::None,
        };

        let hand = Hand {
            sequences,
            triplets,
            kans,
            pairs,
            win_tile,
            isopen: ishandopen,
        };

        return Ok(hand);
    }

    pub fn sequences(&self) -> Vec<TileGroup> {
        self.sequences.clone()
    }
    pub fn triplets(&self) -> Vec<TileGroup> {
        self.triplets.clone()
    }
    pub fn kans(&self) -> Vec<TileGroup> {
        self.kans.clone()
    }
    pub fn pairs(&self) -> Vec<TileGroup> {
        self.pairs.clone()
    }
    pub fn win_tile(&self) -> TileGroup {
        self.win_tile.clone()
    }
    pub fn is_open(&self) -> bool {
        self.isopen
    }
}

#[derive(Debug, Clone)]
pub struct TileGroup {
    pub value: String,
    pub suit: Suit,
    pub isopen: bool,
    pub group_type: GroupType,
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

        let tile = TileGroup {
            value,
            suit,
            isopen,
            group_type,
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
