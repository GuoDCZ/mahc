pub enum YakuhaiType {
    Ton,
    Nan,
    Sha,
    Pei,
    Haku,
    Hatsu,
    Chun,
}

pub enum Yaku {
    // One Han Yaku
    MenzenTsumo,
    Riichi,
    Ippatsu,
    Pinfu,
    Iipeikou,
    HaiteiRaoyue,
    HouteiRaoyui,
    RinshanKaihou,
    Chankan,
    Tanyao,
    Yakuhai(YakuhaiType),

    // Two Han Yaku
    DoubleRiichi,
    Chantaiyao,
    SanshokuDoujun,
    Ittsuu,
    Toitoi,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Chiitoitsu,
    Honroutou,
    Shousangen,

    // Three Han Yaku
    Honitsu,
    JunchanTaiyao,
    Ryanpeikou,

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

