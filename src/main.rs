use std::ffi::OsString;
use std::fs;

use clap::Parser;
use mahc::calc;
use mahc::fu::Fu;
use mahc::hand::error::HandErr;
use mahc::yaku::Yaku;
use serde_json::json;

/// riichi mahjong calculator tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Hand tiles
    #[clap(long, value_delimiter = ' ', num_args = 1..)]
    tiles: Option<Vec<String>>,

    /// Winning tile
    #[arg(short, long)]
    win: Option<String>,

    /// Han from dora
    #[arg(short, long, default_value_t = 0)]
    dora: u16,

    /// seat wind
    #[arg(short, long, default_value = "Ew")]
    seat: String,

    /// prevelant wind
    #[arg(short, long, default_value = "Ew")]
    prev: String,

    /// is tsumo
    #[arg(short, long, default_value_t = false)]
    tsumo: bool,

    /// is riichi
    #[arg(short, long, default_value_t = false)]
    riichi: bool,

    /// is double riichi
    #[arg(long, default_value_t = false)]
    doubleriichi: bool,

    /// is ippatsu
    #[arg(short, long, default_value_t = false)]
    ippatsu: bool,

    /// is haitei
    #[arg(long, default_value_t = false)]
    haitei: bool,

    /// is rinshan
    #[arg(long, default_value_t = false)]
    rinshan: bool,

    /// is chankan
    #[arg(long, default_value_t = false)]
    chankan: bool,

    /// is tenhou/chihou
    #[arg(long, default_value_t = false)]
    tenhou: bool,

    /// honba count
    #[arg(short, long, default_value_t = 0)]
    ba: u16,

    /// calculator mode
    #[arg(short, long, default_value = None, value_delimiter = ' ', num_args = 2)]
    manual: Option<Vec<u16>>,

    /// file input
    #[arg(short, long, default_value = None)]
    file: Option<String>,

    /// stdout as json
    #[arg(long, default_value_t = false)]
    json: bool,
}

pub fn parse_calculator(args: &Args) -> Result<String, HandErr> {
    let honba = args.ba;
    let hanandfu = args.manual.clone().unwrap();
    let scores = calc::calculate(&hanandfu, honba)?;
    let printout: Result<String, HandErr> = if args.json {
        Ok(json_calc_out(scores, honba, hanandfu))
    } else {
        Ok(default_calc_out(scores, honba, hanandfu))
    };
    printout
}
pub fn parse_hand(args: &Args) -> Result<String, HandErr> {
    if args.tiles.is_none() {
        return Err(HandErr::NoHandTiles);
    }
    if args.win.is_none() {
        return Err(HandErr::NoWinTile);
    }
    if args.tsumo && args.chankan {
        return Err(HandErr::ChankanTsumo);
    }
    if args.rinshan && (!args.tsumo) {
        return Err(HandErr::RinshanWithoutTsumo);
    }
    if args.rinshan && args.ippatsu {
        return Err(HandErr::RinshanIppatsu);
    }
    if args.riichi && args.doubleriichi {
        return Err(HandErr::DuplicateRiichi);
    }
    if args.ippatsu && !(args.riichi || args.doubleriichi) {
        return Err(HandErr::IppatsuWithoutRiichi);
    }
    if args.doubleriichi && args.ippatsu && args.haitei {
        return Err(HandErr::DoubleRiichiHaiteiIppatsu);
    }
    if args.doubleriichi && args.haitei && args.chankan {
        return Err(HandErr::DoubleRiichiHaiteiChankan);
    }
    let result = calc::get_hand_score(
        args.tiles.clone().unwrap(),
        args.win.clone().unwrap(),
        args.dora,
        args.seat.clone(),
        args.prev.clone(),
        args.tsumo,
        args.riichi,
        args.doubleriichi,
        args.ippatsu,
        args.haitei,
        args.rinshan,
        args.chankan,
        args.tenhou,
        args.ba,
    )?;

    //TODO VALIDATION (i dont care enough yet)

    let printout: String = if args.json {
        json_hand_out(result, args)
    } else {
        default_hand_out(result, args)
    };
    Ok(printout)
}
pub fn json_calc_out(result: Vec<u32>, honba: u16, hanandfu: Vec<u16>) -> String {
    let out = json!({
    "han" : hanandfu[0],
    "fu" : hanandfu[1],
    "honba" : honba,
        "scores" : {
            "dealer" : {
                "ron" : result[0],
                "tsumo" : result[1]
            },
            "non-dealer" : {
                "ron" : result[2],
                "tsumo" : {
                    "dealer" : result[4],
                    "non-dealer" : result[3]
                }
            }
        }
    });
    out.to_string()
}
pub fn default_calc_out(score: Vec<u32>, honba: u16, hanandfu: Vec<u16>) -> String {
    if honba != 0 {
        return format!(
            "\n{} Han/ {} Fu/ {} Honba\nDealer: {} ({})\nnon-dealer: {} ({}/{})",
            hanandfu[0], hanandfu[1], honba, score[0], score[1], score[2], score[3], score[4]
        );
    }
    format!(
        "\n{} Han/ {} Fu\nDealer: {} ({})\nnon-dealer: {} ({}/{})",
        hanandfu[0], hanandfu[1], score[0], score[1], score[2], score[3], score[4]
    )
}
pub fn json_hand_out(
    result: (Vec<u32>, Vec<Yaku>, Vec<Fu>, Vec<u16>, bool),
    args: &Args,
) -> String {
    let out = json!({
        "han" : result.3[0],
        "fu" : result.3[1],
        "honba" : args.ba,
        "dora" : args.dora,
        "fuString" : result.2.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        "yakuString" : result.1.iter().map(|x| x.to_string(result.4)).collect::<Vec<String>>(),
        "scores" : {
            "dealer" : {
                "ron" : result.0[0],
                "tsumo" : result.0[1]
            },
            "non-dealer" : {
                "ron" : result.0[2],
                "tsumo" : {
                "dealer" : result.0[4],
                "non-dealer" : result.0[3]
                }
            }
        }
    });
    out.to_string()
}
pub fn default_hand_out(
    result: (Vec<u32>, Vec<Yaku>, Vec<Fu>, Vec<u16>, bool),
    args: &Args,
) -> String {
    let mut out: String = String::new();
    if !result.1[0].is_yakuman() {
        if args.ba != 0 {
            out.push_str(
                format!(
                    "\n{} Han/ {} Fu/ {} Honba",
                    result.3[0], result.3[1], args.ba
                )
                .as_str(),
            )
        } else {
            out.push_str(format!("\n{} Han/ {} Fu", result.3[0], result.3[1]).as_str())
        }
    }

    out.push_str(
        format!(
            "\nDealer: {} ({})\nNon-dealer: {} ({}/{})",
            result.0[0], result.0[1], result.0[2], result.0[3], result.0[4]
        )
        .as_str(),
    );

    if !result.1[0].is_yakuman() && args.dora != 0 {
        out.push_str(format!("\nDora: {}", args.dora).as_str());
    }
    out.push_str("\nYaku: ");
    for i in &result.1 {
        out.push_str(format!("\n  {}", i.to_string(result.4)).as_str());
    }
    if !result.1[0].is_yakuman() {
        out.push_str("\nFu: ");
        for i in result.2 {
            out.push_str(format!("\n  {}", i).as_str());
        }
    }
    out
}
pub fn parse_file(args: &Args) {
    let string = fs::read_to_string(args.file.as_ref().unwrap());
    if string.is_err() {
        eprintln!("Error: Unable to read file {}", args.file.as_ref().unwrap());
        return;
    }
    let string = string.unwrap();
    let lines = string.lines();
    for string in lines {
        if string.is_empty() {
            continue;
        }
        let mut current_line_args = vec![OsString::from("mahc")];
        for arg in string.split_whitespace() {
            current_line_args.push(arg.into());
        }
        let args = Args::parse_from(&current_line_args);
        if args.file.is_some() {
            parse_file(&args);
        } else if args.manual.is_some() {
            let result = parse_calculator(&args);
            printout(result);
        } else {
            let result = parse_hand(&args);
            printout(result);
        }
    }
}
pub fn printout(result: Result<String, HandErr>) {
    match result {
        Ok(o) => {
            println!("{}", o);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn main() {
    let args = Args::parse();
    if args.file.is_some() {
        parse_file(&args);
    } else if args.manual.is_some() {
        let result = parse_calculator(&args);
        printout(result);
    } else {
        let result = parse_hand(&args);
        printout(result);
    }
}

#[cfg(test)]
mod test {
    use mahc::hand::error::HandErr;

    use super::*;

    #[test]
    fn no_han_for_calc() {
        let args = Args::parse_from(["", "--manual", "0", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), HandErr::NoHan);
    }

    #[test]
    fn no_fu_for_calc() {
        let args = Args::parse_from(["", "--manual", "4", "0", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), HandErr::NoFu);
    }

    #[test]
    fn valid_calc_input() {
        let args = Args::parse_from(["", "--manual", "4", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n4 Han/ 30 Fu/ 3 Honba\nDealer: 12500 (4200)\nnon-dealer: 8600 (2300/4200)"
                .to_string())
        );
    }
    #[test]
    fn han_1_fu_30_calc() {
        let args = Args::parse_from(["", "--manual", "1", "30"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n1 Han/ 30 Fu\nDealer: 1500 (500)\nnon-dealer: 1000 (300/500)".to_string())
        );
    }
    #[test]
    fn han_2_fu_80_calc() {
        let args = Args::parse_from(["", "--manual", "2", "80"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n2 Han/ 80 Fu\nDealer: 7700 (2600)\nnon-dealer: 5200 (1300/2600)".to_string())
        );
    }
    #[test]
    fn han_3_mangan_calc() {
        let args = Args::parse_from(["", "--manual", "3", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n3 Han/ 70 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn han_4_mangan_calc() {
        let args = Args::parse_from(["", "--manual", "4", "60", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n4 Han/ 60 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn han_5_mangan_calc() {
        let args = Args::parse_from(["", "--manual", "5", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n5 Han/ 70 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn haneman_calc() {
        let args = Args::parse_from(["", "--manual", "6", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n6 Han/ 70 Fu/ 3 Honba\nDealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "7", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n7 Han/ 70 Fu/ 3 Honba\nDealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)"
                .to_string())
        );
    }
    #[test]
    fn baiman_calc() {
        let args = Args::parse_from(["", "--manual", "8", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n8 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "9", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n9 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "10", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n10 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
    }
    #[test]
    fn sanbaiman_calc() {
        let args = Args::parse_from(["", "--manual", "11", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n11 Han/ 70 Fu/ 3 Honba\nDealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)"
                .to_string())
        );
        let args = Args::parse_from(["", "--manual", "12", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n12 Han/ 70 Fu/ 3 Honba\nDealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)"
                .to_string())
        );
    }
    #[test]
    fn kazoeyakuman_calc() {
        let args = Args::parse_from(["", "--manual", "13", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("\n13 Han/ 70 Fu/ 3 Honba\nDealer: 48900 (16300)\nnon-dealer: 32900 (8300/16300)"
                .to_string())
        );
    }
}
