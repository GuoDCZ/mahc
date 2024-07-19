#[derive(Debug, PartialEq)]
pub enum Yaku {
    // One Han Yaku
    Tanyao,   //done
    Iipeikou, //done
    Yakuhai,  //done
    MenzenTsumo,
    Pinfu,
    Riichi,  //(bascially done by default)
    Ippatsu, //(bascially done by default)
    HaiteiRaoyue,
    HouteiRaoyui,
    RinshanKaihou,
    Chankan,

    // Two Han Yaku
    DoubleRiichi,
    Toitoi,
    Ittsuu,
    SanshokuDoujun,
    Chantaiyao,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Honroutou,
    Shousangen,
    Chiitoitsu,

    // Three Han Yaku
    Honitsu,
    JunchanTaiyao,
    Ryanpeikou, //done

    // Six Han Yaku
    Chinitsu,

    // Yakuman
    KazoeYakuman,
    KokushiMusou,
    Suuankou,
    Daisangen,
    Shousuushii,
    Daisuushii,
    Tsuuiisou,
    Chinroutou,
    Ryuuiisou,
    ChuurenPoutou,
    Suukantsu,
    Tenhou,
    Chiihou,
}

impl Yaku {
    pub fn to_string(&self, is_open: bool) -> String {
        match self {
            Yaku::Tanyao => "Tanyao: 1".to_string(),
            Yaku::Iipeikou => "Iipeikou: 1".to_string(),
            Yaku::Yakuhai => "Yakuhai: 1".to_string(),
            Yaku::MenzenTsumo => "MenzenTsumo: 1".to_string(),
            Yaku::Pinfu => "Pinfu: 1".to_string(),
            Yaku::Riichi => "Riichi: 1".to_string(),
            Yaku::Ippatsu => "Ippatsu: 1".to_string(),
            Yaku::HaiteiRaoyue => "HaiteiRaoyue: 1".to_string(),
            Yaku::HouteiRaoyui => "HouteiRaoyui: 1".to_string(),
            Yaku::RinshanKaihou => "RinshanKaihou: 1".to_string(),
            Yaku::Chankan => "Chankan: 1".to_string(),

            Yaku::DoubleRiichi => "DoubleRiichi: 2".to_string(),
            Yaku::Toitoi => "Toitoi: 2".to_string(),
            Yaku::Ittsuu => {
                if is_open {
                    return "Ittsuu: 1".to_string();
                }
                return "Ittsuu: 2".to_string();
            }
            Yaku::SanshokuDoujun => {
                if is_open {
                    "Sanshoku Doujun: 1".to_string()
                } else {
                    "Sanshoku Doujun: 2".to_string()
                }
            }
            Yaku::Chantaiyao => {
                if is_open {
                    return "Chantaiyao: 1".to_string()
                }
                    return "Chantaiyao: 2".to_string()
            }
            Yaku::Sanankou => "Sanankou: 2".to_string(),
            Yaku::SanshokuDoukou => "SanshokuDoukou: 2".to_string(),
            Yaku::Sankantsu => "Sankantsu: 2".to_string(),
            Yaku::Honroutou => "Honroutou: 2".to_string(),
            Yaku::Shousangen => "Shousangen: 2".to_string(),
            Yaku::Chiitoitsu => "Chiitoitsu: 2".to_string(),

            Yaku::Honitsu => {
                if is_open {
                    return "Honitsu: 2".to_string();
                }
                return "Honitsu: 3".to_string();
            }
            Yaku::JunchanTaiyao => {
                if is_open {
                    "JunchanTaiyao: 2".to_string()
                } else {
                    "JunchanTaiyao: 3".to_string()
                }
            }
            Yaku::Ryanpeikou => "Ryanpeikou: 3".to_string(),

            Yaku::Chinitsu => "Chinitsu: 6".to_string(),

            //TODO gota be a better way of doing this
            Yaku::KazoeYakuman => "KazoeYakuman: ".to_string(),
            Yaku::KokushiMusou => "KokushiMusou: ".to_string(),
            Yaku::Suuankou => "Suuankou: ".to_string(),
            Yaku::Daisangen => "Daisangen: ".to_string(),
            Yaku::Shousuushii => "Shousuushii: ".to_string(),
            Yaku::Daisuushii => "Daisuushii: ".to_string(),
            Yaku::Tsuuiisou => "Tsuuiisou: ".to_string(),
            Yaku::Chinroutou => "Chinroutou: ".to_string(),
            Yaku::Ryuuiisou => "Ryuuiisou: ".to_string(),
            Yaku::ChuurenPoutou => "ChuurenPoutou: ".to_string(),
            Yaku::Suukantsu => "Suukantsu: ".to_string(),
            Yaku::Tenhou => "Tenhou: ".to_string(),
            Yaku::Chiihou => "Chiihou: ".to_string(),
        }
    }
    //TODO adjust for open or closed !!!!
    pub fn get_han(&self, is_open: bool) -> u16 {
        match self {
            Yaku::Tanyao => 1,
            Yaku::Iipeikou => 1,
            Yaku::Yakuhai => 1,
            Yaku::MenzenTsumo => 1,
            Yaku::Pinfu => 1,
            Yaku::Riichi => 1,
            Yaku::Ippatsu => 1,
            Yaku::HaiteiRaoyue => 1,
            Yaku::HouteiRaoyui => 1,
            Yaku::RinshanKaihou => 1,
            Yaku::Chankan => 1,

            Yaku::DoubleRiichi => 2,
            Yaku::Toitoi => 2,
            Yaku::Ittsuu => {
                if is_open {
                    return 1;
                }
                return 2;
            }
            Yaku::SanshokuDoujun => {
                if is_open {
                    return 1;
                }
                return 2;
            }
            Yaku::Chantaiyao => 2,
            Yaku::Sanankou => 2,
            Yaku::SanshokuDoukou => 2,
            Yaku::Sankantsu => 2,
            Yaku::Honroutou => 2,
            Yaku::Shousangen => 2,
            Yaku::Chiitoitsu => 2,

            Yaku::Honitsu => {
                if is_open {
                    return 2;
                }
                return 3;
            }
            Yaku::JunchanTaiyao => {
                if is_open {
                    return 2;
                }
                return 3;
            }
            Yaku::Ryanpeikou => 3,

            Yaku::Chinitsu => 6,

            //TODO gota be a better way of doing this
            Yaku::KazoeYakuman => 0,
            Yaku::KokushiMusou => 0,
            Yaku::Suuankou => 0,
            Yaku::Daisangen => 0,
            Yaku::Shousuushii => 0,
            Yaku::Daisuushii => 0,
            Yaku::Tsuuiisou => 0,
            Yaku::Chinroutou => 0,
            Yaku::Ryuuiisou => 0,
            Yaku::ChuurenPoutou => 0,
            Yaku::Suukantsu => 0,
            Yaku::Tenhou => 0,
            Yaku::Chiihou => 0,
        }
    }
}
