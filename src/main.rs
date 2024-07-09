mod calc;
pub mod fu;
mod lib;
pub mod yaku;
use clap::Parser;

/// riichi mahjong calculator tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Called tiles
    #[arg(short, long)]
    called: Option<String>,

    /// Winning tile
    #[arg(short, long)]
    win: Option<String>,

    /// dora tiles
    #[arg(short, long)]
    dora: Option<String>,

    /// seat wind
    #[arg(short, long, default_value = "e")]
    seat: String,

    /// prevelant wind
    #[arg(short, long, default_value = "e")]
    prev: String,

    /// is tsumo
    #[arg(short, long, default_value_t = false)]
    tsumo: bool,

    /// is riichi
    #[arg(short, long, default_value_t = false)]
    riichi: bool,

    /// han total
    #[arg(short, long, default_value_t = 0)]
    n: u8,

    /// fu total
    #[arg(short, long, default_value_t = 0)]
    fu: u8,

    /// honba count
    #[arg(short, long, default_value_t = 0)]
    ba: u8,

    /// calculator mode
    #[arg(short, long, default_value_t = false)]
    manual: bool,
}

pub fn parse_calculator(args: &Args) -> Result<String, calc::CalculatorErrors> {
    let han = args.n as u16;
    let fu = args.fu as u16;
    if han == 0 {
        return Err(calc::CalculatorErrors::NoHan);
    }
    if fu == 0 {
        return Err(calc::CalculatorErrors::NoFu);
    }
    let honba = args.ba;
    let scores = calc::calculate(han, fu, honba).unwrap();

    return Ok(format!(
        "Dealer: {} ({})\nnon-dealer: {} ({}/{})",
        scores[0], scores[1], scores[2], scores[3], scores[4],
    ));
}

fn main() {
    let args = Args::parse();
    if args.manual == true {
        let result = parse_calculator(&args);
        match result {
            Ok(o) => {
                println!("{}", o);
            }
            Err(e) => {
                println!("Error: {:?}", e.to_string());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_han_for_calc() {
        let args = Args::parse_from(&["", "--manual", "--fu", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), calc::CalculatorErrors::NoHan);
    }
    #[test]
    fn no_fu_for_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "4", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), calc::CalculatorErrors::NoFu);
    }

    #[test]
    fn valid_calc_input() {
        let args = Args::parse_from(&["", "--manual", "-n", "4", "--fu", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12500 (4200)\nnon-dealer: 8600 (2300/4200)".to_string())
        );
    }
    #[test]
    fn han_4_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "4", "--fu", "60", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)".to_string())
        );
    }

    #[test]
    fn han_3_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "3", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)".to_string())
        );
    }
    #[test]
    fn han_5_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "5", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)".to_string())
        );
    }
    #[test]
    fn haneman_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "6", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "-n", "7", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)".to_string())
        );
    }
    #[test]
    fn baiman_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "8", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "-n", "9", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "-n", "10", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)".to_string())
        );
    }
    #[test]
    fn sanbaiman_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "11", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "-n", "12", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)".to_string())
        );
    }
    #[test]
    fn kazoeyakuman_calc() {
        let args = Args::parse_from(&["", "--manual", "-n", "13", "--fu", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 48900 (16300)\nnon-dealer: 32900 (8300/16300)".to_string())
        );
    }
}
