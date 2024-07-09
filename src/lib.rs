#[derive(Debug)]
pub enum LimitHands {
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    KazoeYakuman,
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
