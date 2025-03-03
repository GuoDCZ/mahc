use crate::hand::error::HandErr;

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
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
    /// use mahc::suit::Suit;
    ///
    /// let tile_string = "9m";
    /// let actual_suit = Suit::suit_from_string(&tile_string.chars().nth(1).unwrap().to_string(), &tile_string.chars().nth(0).unwrap().to_string());
    /// let expected = Ok(Suit::Manzu);
    ///
    /// assert_eq!(actual_suit, expected);
    ///
    /// let tile_string = "6z";
    /// let actual_suit = Suit::suit_from_string(&tile_string.chars().nth(1).unwrap().to_string(), &tile_string.chars().nth(0).unwrap().to_string());
    /// let expected = Ok(Suit::Dragon);
    ///
    /// assert_eq!(actual_suit, expected);
    /// ```
    pub fn suit_from_string(suit: &String, value: &String) -> Result<Self, HandErr> {
        if vec!["s", "p", "m"].contains(&suit.as_str())
            && !vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&value.as_str())
        {
            return Err(HandErr::InvalidGroup);
        }
        match suit.as_str() {
            "s" => Ok(Self::Souzu),
            "p" => Ok(Self::Pinzu),
            "m" => Ok(Self::Manzu),
            "w" => {
                if !vec!["E", "S", "W", "N"].contains(&value.as_str()) {
                    Err(HandErr::InvalidGroup)
                } else {
                    Ok(Self::Wind)
                }
            }
            "d" => {
                if !vec!["r", "g", "w"].contains(&value.as_str()) {
                    Err(HandErr::InvalidGroup)
                } else {
                    Ok(Self::Dragon)
                }
            }
            "z" => {
                if vec!["1", "2", "3", "4"].contains(&value.as_str()) {
                    Ok(Self::Wind)
                } else if vec!["5", "6", "7"].contains(&value.as_str()) {
                    Ok(Self::Dragon)
                } else {
                    Err(HandErr::InvalidGroup)
                }
            }
            _ => Err(HandErr::InvalidSuit),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn souzu_suit_from_string() {
        let suit = "s".to_string();
        let value = "1".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Souzu);

        assert_eq!(actual, expected);
    }

    #[test]
    fn manzu_suit_from_string() {
        let suit = "m".to_string();
        let value = "1".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Manzu);

        assert_eq!(actual, expected);
    }
    #[test]
    fn pinzu_suit_from_string() {
        let suit = "p".to_string();
        let value = "1".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Pinzu);

        assert_eq!(actual, expected);
    }
    #[test]
    fn wind_suit_from_string() {
        let suit = "z".to_string();
        let value = "1".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Wind);

        assert_eq!(actual, expected);
        let suit = "z".to_string();
        let value = "4".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Wind);

        assert_eq!(actual, expected);
        let suit = "w".to_string();
        let value = "W".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Wind);

        assert_eq!(actual, expected);
    }
    #[test]
    fn akadora_suit_from_string() {
        let value = "0".to_string();
        let suit = "m".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Ok(Suit::Manzu);
        assert_eq!(actual, expected);

        let value = "0".to_string();
        let suit = "z".to_string();
        let actual = Suit::suit_from_string(&suit, &value);
        let expected = Err(HandErr::InvalidGroup); 
        assert_eq!(actual, expected);

    }
}
