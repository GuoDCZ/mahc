#[derive(Debug, PartialEq)]
pub enum CalculatorErrors {
    NoHan,
    NoFu,
}

impl CalculatorErrors {
    pub fn to_string(&self) -> String {
        match self {
            CalculatorErrors::NoHan => "No han provided!\n use -n <number>".to_string(),
            CalculatorErrors::NoFu => "No fu provided!\n use --fu <number>".to_string(),
        }
    }
}

pub fn calculate(han: u16, fu: u16, honba: u8) -> Result<Vec<u16>, CalculatorErrors> {
    let k = mahc::LimitHands::get_limit_hand(han, fu);
    match k {
        Some(limithand) => {
            println!("{:?}", limithand);
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
