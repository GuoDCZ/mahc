use crate::yaku::Yaku;

#[derive(Debug, PartialEq)]
pub enum CalculatorErrors {
    NoHan,
    NoFu,
    NoYaku,
}

impl CalculatorErrors {
    pub fn to_string(&self) -> String {
        match self {
            CalculatorErrors::NoHan => "No han provided!".to_string(),
            CalculatorErrors::NoFu => "No fu provided!".to_string(),
            CalculatorErrors::NoYaku => "No Yaku!".to_string(),
        }
    }
}

pub fn get_hand_score(
    tiles: Vec<String>,
    win: String,
    dora: u16,
    seat: String,
    prev: String,
    tsumo: bool,
    riichi: bool,
    doubleriichi: bool,
    ippatsu: bool,
    haitei: bool,
    rinshan: bool,
    chankan: bool,
    honba: u16,
) -> (Vec<u16>, Vec<Yaku>, Vec<mahc::Fu>, Vec<u16>, bool) {
    let hand = mahc::Hand::new(tiles, win, seat, prev).unwrap();
    let yaku = get_yaku_han(
        &hand,
        riichi,
        doubleriichi,
        ippatsu,
        haitei,
        rinshan,
        chankan,
        tsumo,
    );
    let mut han_and_fu: Vec<u16> = vec![];
    let fu: (u16, Vec<mahc::Fu>);
    //fuck you chiitoiistu, why u gota be different, AND YOU TOO PINFU
    //i can move this to calculatefu method maybe?
    if yaku.1.contains(&Yaku::Chiitoitsu) {
        fu = (25, vec![mahc::Fu::BasePointsChitoi]);
    } else {
        if yaku.1.contains(&Yaku::Pinfu) {
            if tsumo {
                fu = (20, vec![mahc::Fu::BasePoints]);
            } else {
                fu = (30, vec![mahc::Fu::BasePoints, mahc::Fu::ClosedRon]);
            }
        } else {
            fu = hand.calculate_fu(tsumo);
        }
    }
    han_and_fu = vec![yaku.0 + dora, fu.0];
    let scores = calculate(&han_and_fu, honba).unwrap();
    return (scores, yaku.1, fu.1, han_and_fu, hand.is_open());
}
pub fn get_yaku_han(
    hand: &mahc::Hand,
    riichi: bool,
    doubleriichi: bool,
    ippatsu: bool,
    haitei: bool,
    rinshan: bool,
    chankan: bool,
    tsumo: bool,
) -> (u16, Vec<Yaku>) {
    let mut yaku: Vec<Yaku> = vec![];
    let conditions = [
        (riichi, Yaku::Riichi),
        (doubleriichi, Yaku::DoubleRiichi),
        (ippatsu, Yaku::Ippatsu),
        (haitei, Yaku::Haitei),
        (rinshan, Yaku::RinshanKaihou),
        (chankan, Yaku::Chankan),
        (hand.is_tanyao(), Yaku::Tanyao),
        (hand.is_iipeikou(), Yaku::Iipeikou),
        (hand.is_ryanpeikou(), Yaku::Ryanpeikou),
        (hand.is_toitoi(), Yaku::Toitoi),
        (hand.is_sanshokudoujun(), Yaku::SanshokuDoujun),
        (hand.is_sanankou(tsumo), Yaku::Sanankou),
        (hand.is_honitsu(), Yaku::Honitsu),
        (hand.is_shousangen(), Yaku::Shousangen),
        (hand.is_junchantaiyao(), Yaku::JunchanTaiyao),
        (hand.is_honroutou(), Yaku::Honroutou),
        (hand.is_sankantsu(), Yaku::Sankantsu),
        (hand.is_ittsuu(), Yaku::Ittsuu),
        (hand.is_chantaiyao(), Yaku::Chantaiyao),
        (hand.is_chiitoitsu(), Yaku::Chiitoitsu),
        (hand.is_menzentsumo(tsumo), Yaku::MenzenTsumo),
        (hand.is_pinfu(), Yaku::Pinfu),
        (hand.is_sanshokudoukou(), Yaku::SanshokuDoukou),
    ];

    for (condition, yaku_type) in conditions {
        if condition {
            yaku.push(yaku_type);
        }
    }
    for _i in 0..hand.is_yakuhai() {
        yaku.push(Yaku::Yakuhai);
    }
    let mut yaku_han = 0;
    for y in &yaku {
        yaku_han += y.get_han(hand.is_open());
    }
    return (yaku_han, yaku);
}

pub fn calculate(args: &Vec<u16>, honba: u16) -> Result<Vec<u16>, CalculatorErrors> {
    let han = args[0];
    let fu = args[1];
    if han == 0 {
        return Err(CalculatorErrors::NoHan);
    }
    if fu == 0 {
        return Err(CalculatorErrors::NoFu);
    }
    let k = mahc::LimitHands::get_limit_hand(han, fu);
    match k {
        Some(limithand) => {
            let mut scores = limithand.get_score();
            scores[0] = scores[0] + honba as u16 * 300;
            scores[1] = scores[1] + honba as u16 * 100;
            scores[2] = scores[2] + honba as u16 * 300;
            scores[3] = scores[3] + honba as u16 * 100;
            scores[4] = scores[4] + honba as u16 * 100;
            return Ok(scores);
        }
        None => (),
    }

    let basic_points = fu * 2u16.pow((han + 2) as u32);

    let dealer_ron =
        (((basic_points * 6 + honba as u16 * 300) as f64 / 100.0).ceil() * 100.0) as u16;
    let dealer_tsumo =
        (((basic_points * 2 + honba as u16 * 100) as f64 / 100.0).ceil() * 100.0) as u16;
    let non_dealer_ron =
        (((basic_points * 4 + honba as u16 * 300) as f64 / 100.0).ceil() * 100.0) as u16;
    let non_dealer_tsumo_to_dealer =
        (((basic_points * 2 + honba as u16 * 100) as f64 / 100.0).ceil() * 100.0) as u16;
    let non_dealer_tsumo_to_non_dealer =
        (((basic_points + honba as u16 * 100) as f64 / 100.0).ceil() * 100.0) as u16;

    let scores: Vec<u16> = vec![
        dealer_ron,
        dealer_tsumo,
        non_dealer_ron,
        non_dealer_tsumo_to_non_dealer,
        non_dealer_tsumo_to_dealer,
    ];

    return Ok(scores);
}
