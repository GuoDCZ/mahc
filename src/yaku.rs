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
    Haitei,
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
    KokushiMusou13SidedWait,
    Suuankou,
    SuuankouTankiWait,
    Daisangen,
    Shousuushii,
    Daisuushii,
    Tsuuiisou,
    Daichiishin,
    Chinroutou,
    Ryuuiisou,
    ChuurenPoutou,
    ChuurenPoutou9SidedWait,
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
            Yaku::Haitei => "Haitei: 1".to_string(),
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
                    return "Chantaiyao: 1".to_string();
                }
                return "Chantaiyao: 2".to_string();
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

            Yaku::Chinitsu => {
                if is_open {
                    "Chinitsu: 5".to_string()
                } else {
                    "Chinitsu: 6".to_string()
                }
            }

            //TODO gota be a better way of doing this
            Yaku::KazoeYakuman => "Kazoe Yakuman ".to_string(),
            Yaku::KokushiMusou => "KokushiMusou Yakuman".to_string(),
            Yaku::KokushiMusou13SidedWait => "KokushiMusou Yakuman 13 sided wait".to_string(),
            Yaku::Suuankou => "Suuankou Yakuman".to_string(),
            Yaku::Daisangen => "Daisangen Yakuman".to_string(),
            Yaku::Shousuushii => "Shousuushii Yakuman".to_string(),
            Yaku::Daisuushii => "Daisuushii Yakuman".to_string(),
            Yaku::Tsuuiisou => "Tsuuiisou Yakuman".to_string(),
            Yaku::Chinroutou => "Chinroutou Yakuman".to_string(),
            Yaku::Ryuuiisou => "Ryuuiisou Yakuman".to_string(),
            Yaku::ChuurenPoutou => "ChuurenPoutou Yakuman".to_string(),
            Yaku::Suukantsu => "Suukantsu Yakuman".to_string(),
            Yaku::Tenhou => "Tenhou Yakuman".to_string(),
            Yaku::Chiihou => "Chiihou Yakuman".to_string(),
            Yaku::SuuankouTankiWait => "Suuankou Yakuman Tanki Wait ".to_string(),
            Yaku::Daichiishin => "Daichiishin Yakuman".to_string(),
            Yaku::ChuurenPoutou9SidedWait => "ChuurenPoutou Yakuman 9 sided wait ".to_string(),
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
            Yaku::Haitei => 1,
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

            Yaku::Chinitsu => {
                if is_open {
                    return 5;
                }
                return 6;
            }

            //TODO gota be a better way of doing this
            Yaku::KazoeYakuman => 1,
            Yaku::KokushiMusou => 1,
            Yaku::KokushiMusou13SidedWait => 1,
            Yaku::Suuankou => 1,
            Yaku::Daisangen => 1,
            Yaku::Shousuushii => 1,
            Yaku::Daisuushii => 1,
            Yaku::Tsuuiisou => 1,
            Yaku::Chinroutou => 1,
            Yaku::Ryuuiisou => 1,
            Yaku::ChuurenPoutou => 1,
            Yaku::ChuurenPoutou9SidedWait => 1,
            Yaku::Suukantsu => 1,
            Yaku::Tenhou => 1,
            Yaku::Chiihou => 1,
            Yaku::SuuankouTankiWait => 1,
            Yaku::Daichiishin => 1,
        }
    }
    pub fn is_yakuman(&self) -> bool {
        match self {
            Yaku::KazoeYakuman => true,
            Yaku::KokushiMusou => true,
            Yaku::KokushiMusou13SidedWait => true,
            Yaku::Suuankou => true,
            Yaku::SuuankouTankiWait => true,
            Yaku::Daisangen => true,
            Yaku::Shousuushii => true,
            Yaku::Daisuushii => true,
            Yaku::Tsuuiisou => true,
            Yaku::Daichiishin => true,
            Yaku::Chinroutou => true,
            Yaku::Ryuuiisou => true,
            Yaku::ChuurenPoutou => true,
            Yaku::ChuurenPoutou9SidedWait => true,
            Yaku::Suukantsu => true,
            Yaku::Tenhou => true,
            Yaku::Chiihou => true,
            _ => false,
        }
    }
}
