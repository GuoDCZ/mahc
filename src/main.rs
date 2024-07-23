mod calc;
mod lib;
pub mod yaku;

use clap::Parser;

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

    /// honba count
    #[arg(short, long, default_value_t = 0)]
    ba: u16,

    /// calculator mode
    #[arg(short, long, default_value = None, value_delimiter = ' ', num_args = 2)]
    manual: Option<Vec<u16>>,
}

pub fn parse_calculator(args: &Args) -> Result<String, calc::CalculatorErrors> {
    let honba = args.ba;
    let hanandfu = args.manual.clone().unwrap();
    let scores = calc::calculate(&hanandfu, honba);
    match scores {
        Ok(o) => {
            if honba != 0 {
                return Ok(format!(
                    "{} Han/ {} Fu/ {} Honba\nDealer: {} ({})\nnon-dealer: {} ({}/{})",
                    hanandfu[0], hanandfu[1], honba, o[0], o[1], o[2], o[3], o[4]
                ));
            }
            Ok(format!(
                "{} Han/ {} Fu\nDealer: {} ({})\nnon-dealer: {} ({}/{})",
                hanandfu[0], hanandfu[1], o[0], o[1], o[2], o[3], o[4]
            ))
        }
        Err(e) => Err(e),
    }
}
pub fn parse_hand(args: &Args) -> Result<String, calc::CalculatorErrors> {
    let result = calc::get_hand_score(
        args.tiles.clone().unwrap(),
        args.win.clone().unwrap(),
        args.dora.clone(),
        args.seat.clone(),
        args.prev.clone(),
        args.tsumo,
        args.riichi,
        args.doubleriichi,
        args.ippatsu,
        args.haitei,
        args.rinshan,
        args.chankan,
        args.ba,
    );

    //TODO VALIDATION (i dont care enough yet)
    let mut printout: String = String::new();
    if args.ba != 0 {
        if !result.1[0].is_yakuman() {
            printout.push_str(
                format!(
                    "\n{} Han/ {} Fu/ {} Honba",
                    result.3[0], result.3[1], args.ba
                )
                .as_str(),
            )
        } else {
            printout.push_str(format!("\n{} Han/ {} Fu", result.3[0], result.3[1]).as_str())
        }
    }

    printout.push_str(
        format!(
            "\nDealer: {} ({})\nNon-dealer: {} ({}/{})",
            result.0[0], result.0[1], result.0[2], result.0[3], result.0[4]
        )
        .as_str(),
    );

    if !result.1[0].is_yakuman() {
        if args.dora != 0 {
            printout.push_str(format!("\n\nDora: {}\n", args.dora).as_str());
        }
    }
    printout.push_str("\nYaku: ");
    for i in &result.1 {
        printout.push_str(format!("\n  {}", i.to_string(result.4)).as_str());
    }
    if !result.1[0].is_yakuman() {
        printout.push_str("\n\nFu: ");
        for i in result.2 {
            printout.push_str(format!("\n  {}", i.to_string()).as_str());
        }
    }
    Ok(printout)
}

fn main() {
    let args = Args::parse();
    if args.manual != None {
        let result = parse_calculator(&args);
        match result {
            Ok(o) => {
                println!("{}", o);
            }
            Err(e) => {
                println!("Error: {:?}", e.to_string());
            }
        }
    } else {
        let result = parse_hand(&args);
        println!("{}", result.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn yaku_cuurenpoutou() {
        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "55s".to_string(),
                "678s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chuurenpoutou(), true);
        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "55m".to_string(),
                "678s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chuurenpoutou(), false);
        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "234s".to_string(),
                "555s".to_string(),
                "678s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chuurenpoutou(), false);
        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "678s".to_string(),
                "999s".to_string(),
                "55s".to_string(),
            ],
            "5s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chuurenpoutou9sided(), true);
        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "678s".to_string(),
                "55s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chuurenpoutou9sided(), false);
    }
    #[test]
    fn yaku_ryuuiisou() {
        let out = lib::Hand::new(
            vec![
                "234s".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ryuuiisou(), true);
        let out = lib::Hand::new(
            vec![
                "345s".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ryuuiisou(), false);

        let out = lib::Hand::new(
            vec![
                "234s".to_string(),
                "234s".to_string(),
                "666s".to_string(),
                "gggd".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ryuuiisou(), false);
    }

    #[test]
    fn yaku_chinroutou() {
        let out = lib::Hand::new(
            vec![
                "111so".to_string(),
                "1111m".to_string(),
                "999s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chinroutou(), true);
        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "1111m".to_string(),
                "999s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chinroutou(), false);
        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "1111m".to_string(),
                "789s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chinroutou(), false);
    }

    #[test]
    fn yaku_suuankoutankiwait() {
        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "111s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_suuankoutankiwait(), true);
        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_suuankoutankiwait(), false);
    }

    #[test]
    fn yaku_suuankou() {
        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "111s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_suuankou(false), true);

        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_suuankou(true), true);
        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_suuankou(false), false);
    }

    #[test]
    fn yaku_daisangen() {
        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "gggd".to_string(),
                "wwwwd".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_daisangen(), true);
        let out = lib::Hand::new(
            vec![
                "rrrrd".to_string(),
                "ggggd".to_string(),
                "wwwwd".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_daisangen(), true);

        let out = lib::Hand::new(
            vec![
                "rrrrd".to_string(),
                "gggd".to_string(),
                "wwd".to_string(),
                "888p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_daisangen(), false);
    }
    #[test]
    fn yaku_chinitsu() {
        let out = lib::Hand::new(
            vec![
                "222p".to_string(),
                "123p".to_string(),
                "345p".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chinitsu(), true);
        let out = lib::Hand::new(
            vec![
                "222p".to_string(),
                "123p".to_string(),
                "345p".to_string(),
                "88s".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chinitsu(), false);
    }

    #[test]
    fn yaku_sanshokudoukou() {
        let out = lib::Hand::new(
            vec![
                "222p".to_string(),
                "222m".to_string(),
                "222s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "6m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanshokudoukou(), true);
        let out = lib::Hand::new(
            vec![
                "222p".to_string(),
                "2222m".to_string(),
                "222s".to_string(),
                "3333s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanshokudoukou(), true);
        let out = lib::Hand::new(
            vec![
                "222p".to_string(),
                "333m".to_string(),
                "222s".to_string(),
                "11s".to_string(),
                "333s".to_string(),
            ],
            "3s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanshokudoukou(), false);
    }

    #[test]
    fn yaku_pinfu() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "6m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_pinfu(), true);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "678po".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "5m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_pinfu(), false);
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "5m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_pinfu(), false);
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_pinfu(), false);
    }

    #[test]
    fn yaku_menzentsumo() {
        let out = lib::Hand::new(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_menzentsumo(true), true);

        let out = lib::Hand::new(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_menzentsumo(false), false);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_menzentsumo(false), false);
    }

    #[test]
    fn yaku_chiitoitsu() {
        let out = lib::Hand::new(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chiitoitsu(), true);
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chiitoitsu(), false);
    }

    #[test]
    fn yaku_chantaiyao() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chantaiyao(), true);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "999m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chantaiyao(), false);
        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "999m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_chantaiyao(), false);
    }

    #[test]
    fn yaku_ittsuu() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "456p".to_string(),
                "789p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ittsuu(), true);

        let out = lib::Hand::new(
            vec![
                "789m".to_string(),
                "456m".to_string(),
                "123m".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ittsuu(), true);
        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "789s".to_string(),
                "456s".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ittsuu(), true);
        let out = lib::Hand::new(
            vec![
                "123m".to_string(),
                "456m".to_string(),
                "678m".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_ittsuu(), false);
    }

    #[test]
    fn yaku_sankantsu() {
        let out = lib::Hand::new(
            vec![
                "9999so".to_string(),
                "123p".to_string(),
                "SSSSw".to_string(),
                "EEEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sankantsu(), true);
        let out = lib::Hand::new(
            vec![
                "9999so".to_string(),
                "123p".to_string(),
                "SSSw".to_string(),
                "EEEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sankantsu(), false);
    }

    #[test]
    fn yaku_honroutou() {
        let out = lib::Hand::new(
            vec![
                "999s".to_string(),
                "111p".to_string(),
                "SSSw".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_honroutou(), true);

        let out = lib::Hand::new(
            vec![
                "999s".to_string(),
                "123p".to_string(),
                "SSSw".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_honroutou(), false);

        let out = lib::Hand::new(
            vec![
                "999s".to_string(),
                "111p".to_string(),
                "111s".to_string(),
                "999m".to_string(),
                "99p".to_string(),
            ],
            "9p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_honroutou(), false);
    }

    #[test]
    fn yaku_junchantaiyao() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "123p".to_string(),
                "999m".to_string(),
                "789s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_junchantaiyao(), true);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "123p".to_string(),
                "999m".to_string(),
                "789s".to_string(),
                "ggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_junchantaiyao(), false);

        let out = lib::Hand::new(
            vec![
                "111s".to_string(),
                "111m".to_string(),
                "999m".to_string(),
                "999s".to_string(),
                "11p".to_string(),
            ],
            "1p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_junchantaiyao(), false);
    }

    #[test]
    fn yaku_shousangen() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "wwwwd".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_shousangen(), true);
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "wwwwd".to_string(),
                "rrd".to_string(),
                "234p".to_string(),
            ],
            "4p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_shousangen(), false);
    }

    #[test]
    fn yaku_honitsu() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567p".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_honitsu(), true);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567m".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_honitsu(), false);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567p".to_string(),
                "111p".to_string(),
                "33p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //should be chinitsu not honitsu
        assert_eq!(out.is_honitsu(), false);
    }

    #[test]
    fn yaku_sanankou() {
        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanankou(false), false);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanankou(true), true);

        let out = lib::Hand::new(
            vec![
                "123p".to_string(),
                "123s".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanankou(true), false);
    }

    #[test]
    fn yaku_sanshokudoujun() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "234s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanshokudoujun(), true);

        let out = lib::Hand::new(
            vec![
                "678s".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "234s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanshokudoujun(), true);

        let out = lib::Hand::new(
            vec![
                "111p".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_sanshokudoujun(), false);
    }

    #[test]
    fn yaku_toitoi() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "EEEw".to_string(),
                "gggd".to_string(),
                "222p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_toitoi(), true);

        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "2222m".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_toitoi(), true);

        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "2222m".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
                "456so".to_string(),
            ],
            "5s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_toitoi(), false);
    }

    #[test]
    fn yaku_yakuhai() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = lib::Hand::new(
            vec![
                "EEEEwo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 2);
        let out = lib::Hand::new(
            vec![
                "3333mo".to_string(),
                "WWWm".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = lib::Hand::new(
            vec![
                "3333mo".to_string(),
                "222m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 0);
    }
    #[test]
    fn yaku_ryanpeikou() {
        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "es".to_string(),
            "ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_ryanpeikou(), true);

        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "678m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "es".to_string(),
            "ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_ryanpeikou(), false);
    }

    #[test]
    fn yaku_iipeko() {
        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_iipeikou(), false);

        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_iipeikou(), false);

        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_iipeikou(), true);
    }

    #[test]
    fn yaku_tanyao() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "22s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_tanyao(), false);
        let out = lib::Hand::new(
            vec![
                "333mo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_tanyao(), false);
        let out = lib::Hand::new(
            vec![
                "555mo".to_string(),
                "678p".to_string(),
                "22s".to_string(),
                "333s".to_string(),
                "345m".to_string(),
            ],
            "4m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_tanyao(), true);
    }

    #[test]
    fn fu_calc_simpleopenkan_simpleclosedkan() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "789m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 60);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenTriplet,
                lib::Fu::SimpleOpenKan,
                lib::Fu::SimpleClosedKan,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_calc_edge_wait() {
        let out = lib::Hand::new(
            vec![
                "555po".to_string(),
                "234m".to_string(),
                "11s".to_string(),
                "rrrdo".to_string(),
                "789m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 30);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::SimpleOpenTriplet,
                lib::Fu::NonSimpleOpenTriplet,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn random_fu() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "567m".to_string(),
                "567p".to_string(),
                "55s".to_string(),
                "456s".to_string(),
            ],
            "6s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 30);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenTriplet,
            ]
        );
    }

    #[test]
    fn fu_cal_middle_wait() {
        let out = lib::Hand::new(
            vec![
                "123mo".to_string(),
                "rrrrdo".to_string(),
                "EEEEw".to_string(),
                "WWw".to_string(),
                "456p".to_string(),
            ],
            "5p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 80);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenKan,
                lib::Fu::NonSimpleClosedKan,
                lib::Fu::Toitsu,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_kans_seat_wind() {
        let out = lib::Hand::new(
            vec![
                "123mo".to_string(),
                "rrrrdo".to_string(),
                "456po".to_string(),
                "EEEEw".to_string(),
                "WWw".to_string(),
            ],
            "Ww".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 80);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenKan,
                lib::Fu::NonSimpleClosedKan,
                lib::Fu::Toitsu,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_nontimple_closed_trip() {
        let out = lib::Hand::new(
            vec![
                "111mo".to_string(),
                "rrrd".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(false).0, 40);
        assert_eq!(
            out.calculate_fu(false).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::NonSimpleOpenTriplet,
                lib::Fu::NonSimpleClosedTriplet,
                lib::Fu::NonSimpleOpenTriplet
            ]
        );
    }

    #[test]
    fn fu_cal_tsu_singlewait_simple_trip_closed_simple_trip_closed_nonsimple_kan() {
        let out = lib::Hand::new(
            vec![
                "444m".to_string(),
                "789p".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 70);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::SimpleClosedTriplet,
                lib::Fu::SimpleOpenTriplet,
                lib::Fu::NonSimpleClosedKan,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn invalid_group_sequence_not_in_order() {
        let out = lib::Hand::new(
            vec![
                "135m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_group_size_too_small() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_group_size_too_big() {
        let out = lib::Hand::new(
            vec![
                "SSSSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_suit() {
        let out = lib::Hand::new(
            vec![
                "hhho".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidSuit);
    }

    #[test]
    fn identify_pair() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.pairs()[0].value, "S");
        assert_eq!(out.pairs()[0].group_type, lib::GroupType::Pair);
        assert_eq!(out.pairs()[0].suit, lib::Suit::Wind);
        assert_eq!(out.pairs()[0].isopen, false);
    }

    #[test]
    fn hand_too_small() {
        let out = lib::Hand::new(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidShape);
        let out = lib::Hand::new(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        println!("{:?}", out);
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidShape);
    }
    #[test]
    fn hand_too_big() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidShape);
    }

    #[test]
    fn identify_tilegroup_closed_wind_trip_SSS() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.triplets()[0].value, "S");
        assert_eq!(out.triplets()[0].group_type, lib::GroupType::Triplet);
        assert_eq!(out.triplets()[0].suit, lib::Suit::Wind);
        assert_eq!(out.triplets()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_open_wind_kan_EEEE() {
        let out = lib::Hand::new(
            vec![
                "EEEEwo".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.kans()[0].value, "E");
        assert_eq!(out.kans()[0].group_type, lib::GroupType::Kan);
        assert_eq!(out.kans()[0].suit, lib::Suit::Wind);
        assert_eq!(out.kans()[0].isopen, true);
    }

    #[test]
    fn identify_tilegroup_closed_dragon_kan_RRRR() {
        let out = lib::Hand::new(
            vec![
                "rrrrd".to_string(),
                "rrrrd".to_string(),
                "SSw".to_string(),
                "rrrrd".to_string(),
                "rrrrd".to_string(),
            ],
            "rd".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.kans()[0].value, "r");
        assert_eq!(out.kans()[0].group_type, lib::GroupType::Kan);
        assert_eq!(out.kans()[0].suit, lib::Suit::Dragon);
        assert_eq!(out.kans()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_closed_manzu_trip_111() {
        let out = lib::Hand::new(
            vec![
                "111m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.triplets()[0].value, "1");
        assert_eq!(out.triplets()[0].group_type, lib::GroupType::Triplet);
        assert_eq!(out.triplets()[0].suit, lib::Suit::Manzu);
        assert_eq!(out.triplets()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_closed_souzu_seq_789() {
        let out = lib::Hand::new(
            vec![
                "789s".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.sequences()[0].value, "7");
        assert_eq!(out.sequences()[0].group_type, lib::GroupType::Sequence);
        assert_eq!(out.sequences()[0].suit, lib::Suit::Souzu);
        assert_eq!(out.sequences()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_open_pinzu_234() {
        let out = lib::Hand::new(
            vec![
                "234po".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.sequences()[0].value, "2");
        assert_eq!(out.sequences()[0].group_type, lib::GroupType::Sequence);
        assert_eq!(out.sequences()[0].suit, lib::Suit::Pinzu);
        assert_eq!(out.sequences()[0].isopen, true);
    }

    #[test]
    fn no_han_for_calc() {
        let args = Args::parse_from(&["", "--manual", "0", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), calc::CalculatorErrors::NoHan);
    }

    #[test]
    fn no_fu_for_calc() {
        let args = Args::parse_from(&["", "--manual", "4", "0", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), calc::CalculatorErrors::NoFu);
    }

    #[test]
    fn valid_calc_input() {
        let args = Args::parse_from(&["", "--manual", "4", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("4 Han/ 30 Fu/ 3 Honba\nDealer: 12500 (4200)\nnon-dealer: 8600 (2300/4200)"
                .to_string())
        );
    }
    #[test]
    fn han_1_fu_30_calc() {
        let args = Args::parse_from(&["", "--manual", "1", "30"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("1 Han/ 30 Fu\nDealer: 1500 (500)\nnon-dealer: 1000 (300/500)".to_string())
        );
    }
    #[test]
    fn han_2_fu_80_calc() {
        let args = Args::parse_from(&["", "--manual", "2", "80"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("2 Han/ 80 Fu\nDealer: 7700 (2600)\nnon-dealer: 5200 (1300/2600)".to_string())
        );
    }
    #[test]
    fn han_3_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "3", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("3 Han/ 70 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn han_4_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "4", "60", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("4 Han/ 60 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn han_5_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "5", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("5 Han/ 70 Fu/ 3 Honba\nDealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)"
                .to_string())
        );
    }
    #[test]
    fn haneman_calc() {
        let args = Args::parse_from(&["", "--manual", "6", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("6 Han/ 70 Fu/ 3 Honba\nDealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)"
                .to_string())
        );
        let args = Args::parse_from(&["", "--manual", "7", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("7 Han/ 70 Fu/ 3 Honba\nDealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)"
                .to_string())
        );
    }
    #[test]
    fn baiman_calc() {
        let args = Args::parse_from(&["", "--manual", "8", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("8 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
        let args = Args::parse_from(&["", "--manual", "9", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("9 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
        let args = Args::parse_from(&["", "--manual", "10", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("10 Han/ 70 Fu/ 3 Honba\nDealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)"
                .to_string())
        );
    }
    #[test]
    fn sanbaiman_calc() {
        let args = Args::parse_from(&["", "--manual", "11", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("11 Han/ 70 Fu/ 3 Honba\nDealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)"
                .to_string())
        );
        let args = Args::parse_from(&["", "--manual", "12", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("12 Han/ 70 Fu/ 3 Honba\nDealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)"
                .to_string())
        );
    }
    #[test]
    fn kazoeyakuman_calc() {
        let args = Args::parse_from(&["", "--manual", "13", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("13 Han/ 70 Fu/ 3 Honba\nDealer: 48900 (16300)\nnon-dealer: 32900 (8300/16300)"
                .to_string())
        );
    }
}
