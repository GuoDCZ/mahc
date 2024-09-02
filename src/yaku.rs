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
            Yaku::Tanyao => "Tanyao: 1",
            Yaku::Iipeikou => "Iipeikou: 1",
            Yaku::Yakuhai => "Yakuhai: 1",
            Yaku::MenzenTsumo => "MenzenTsumo: 1",
            Yaku::Pinfu => "Pinfu: 1",
            Yaku::Riichi => "Riichi: 1",
            Yaku::Ippatsu => "Ippatsu: 1",
            Yaku::Haitei => "Haitei: 1",
            Yaku::RinshanKaihou => "RinshanKaihou: 1",
            Yaku::Chankan => "Chankan: 1",

            Yaku::DoubleRiichi => "DoubleRiichi: 2",
            Yaku::Toitoi => "Toitoi: 2",
            Yaku::Ittsuu => {
                if is_open {
                    "Ittsuu: 1"
                } else {
                    "Ittsuu: 2"
                }
            }
            Yaku::SanshokuDoujun => {
                if is_open {
                    "Sanshoku Doujun: 1"
                } else {
                    "Sanshoku Doujun: 2"
                }
            }
            Yaku::Chantaiyao => {
                if is_open {
                    "Chantaiyao: 1"
                } else {
                    "Chantaiyao: 2"
                }
            }
            Yaku::Sanankou => "Sanankou: 2",
            Yaku::SanshokuDoukou => "SanshokuDoukou: 2",
            Yaku::Sankantsu => "Sankantsu: 2",
            Yaku::Honroutou => "Honroutou: 2",
            Yaku::Shousangen => "Shousangen: 2",
            Yaku::Chiitoitsu => "Chiitoitsu: 2",

            Yaku::Honitsu => {
                if is_open {
                    "Honitsu: 2"
                } else {
                    "Honitsu: 3"
                }
            }
            Yaku::JunchanTaiyao => {
                if is_open {
                    "JunchanTaiyao: 2"
                } else {
                    "JunchanTaiyao: 3"
                }
            }
            Yaku::Ryanpeikou => "Ryanpeikou: 3",

            Yaku::Chinitsu => {
                if is_open {
                    "Chinitsu: 5"
                } else {
                    "Chinitsu: 6"
                }
            }

            //TODO gota be a better way of doing this
            Yaku::KazoeYakuman => "Kazoe Yakuman ",
            Yaku::KokushiMusou => "KokushiMusou Yakuman",
            Yaku::KokushiMusou13SidedWait => "KokushiMusou Yakuman 13 sided wait",
            Yaku::Suuankou => "Suuankou Yakuman",
            Yaku::Daisangen => "Daisangen Yakuman",
            Yaku::Shousuushii => "Shousuushii Yakuman",
            Yaku::Daisuushii => "Daisuushii Yakuman",
            Yaku::Tsuuiisou => "Tsuuiisou Yakuman",
            Yaku::Chinroutou => "Chinroutou Yakuman",
            Yaku::Ryuuiisou => "Ryuuiisou Yakuman",
            Yaku::ChuurenPoutou => "ChuurenPoutou Yakuman",
            Yaku::Suukantsu => "Suukantsu Yakuman",
            Yaku::Tenhou => "Tenhou Yakuman",
            Yaku::Chiihou => "Chiihou Yakuman",
            Yaku::SuuankouTankiWait => "Suuankou Yakuman Tanki Wait ",
            Yaku::Daichiishin => "Daichiishin Yakuman",
            Yaku::ChuurenPoutou9SidedWait => "ChuurenPoutou Yakuman 9 sided wait ",
        }
        .to_string()
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
                2
            }
            Yaku::SanshokuDoujun => {
                if is_open {
                    return 1;
                }
                2
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
                3
            }
            Yaku::JunchanTaiyao => {
                if is_open {
                    return 2;
                }
                3
            }
            Yaku::Ryanpeikou => 3,

            Yaku::Chinitsu => {
                if is_open {
                    return 5;
                }
                6
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
        matches!(
            self,
            Yaku::KazoeYakuman
                | Yaku::KokushiMusou
                | Yaku::KokushiMusou13SidedWait
                | Yaku::Suuankou
                | Yaku::SuuankouTankiWait
                | Yaku::Daisangen
                | Yaku::Shousuushii
                | Yaku::Daisuushii
                | Yaku::Tsuuiisou
                | Yaku::Daichiishin
                | Yaku::Chinroutou
                | Yaku::Ryuuiisou
                | Yaku::ChuurenPoutou
                | Yaku::ChuurenPoutou9SidedWait
                | Yaku::Suukantsu
                | Yaku::Tenhou
                | Yaku::Chiihou
        )
    }
}
