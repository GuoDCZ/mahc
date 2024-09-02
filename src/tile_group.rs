use crate::hand::error::HandErr;
use crate::Suit;

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
    pub fn is_honor(&self) -> bool {
        matches!(self.suit, Suit::Wind | Suit::Dragon)
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, std::num::ParseIntError> {
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
    /// use mahc::tile_group::GroupType;
    ///
    /// let input = "789s".to_string();
    /// let actual = GroupType::group_type_from_string(input);
    /// let expected = Ok(GroupType::Sequence);
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
