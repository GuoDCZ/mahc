
# Riichi Mahjong Calculator Via the Terminal

![demo gif](demo.gif)

### Notation 

#### Suits

| Suit  | Tiles                                      |
|-------|--------------------------------------------|
| Man (Characters) | 1m, 2m, 3m, 4m, 5m, 6m, 7m, 8m, 9m |
| Pin (Circles)    | 1p, 2p, 3p, 4p, 5p, 6p, 7p, 8p, 9p |
| Sou (Bamboos)    | 1s, 2s, 3s, 4s, 5s, 6s, 7s, 8s, 9s |

#### Honors

| Type  | Notation          |
|-------|-------------------|
| Winds | Ew, Sw, Ww, Nw    |
| Dragons | rd, gd, wd      |

#### Special Notation

| Description     | Example           |
|-----------------|-------------------|
| Open Sets       | 234po (an open sequence of 2, 3, 4 in Pin suit) |

- eg: EEEw (triplet of east wind)
- eg: 234m (sequence of 2 3 4 Man)
- eg: rrrrdo (open quad of red dragon)
- eg: 11s (pair of 1 sou)
- eg: 8m (8 man tile)

## Arguments
```
      --tiles <TILES>...          Hand tiles
  -w, --win <WIN>                 Winning tile
  -d, --dora <DORA>               Han from dora [default: 0]
  -s, --seat <SEAT>               seat wind [default: Ew]
  -p, --prev <PREV>               prevelant wind [default: Ew]
  -t, --tsumo                     is tsumo
  -r, --riichi                    is riichi
      --doubleriichi              is double riichi
  -i, --ippatsu                   is ippatsu
      --haitei                    is haitei
      --rinshan                   is rinshan
      --chankan                   is chankan
  -b, --ba <BA>                   honba count [default: 0]
  -m, --manual <MANUAL> <MANUAL>  calculator mode
  -h, --help                      Print help
  -V, --version                   Print version
```

### Examples

#### Calculator Mode
```
~/$ mahc -m 4 30 --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```

#### Normal Mode *not fully yet implemented (some yaku missing)*
note: the winning group has to go last (this is to calculate fu correctly)
``` 
~/$ mahc --tiles rrrd EEEw 234p 234p 11p -w 1p -p Ew -s Ew

7 Han/ 50 Fu
Dealer: 18000 (6000)
Non-dealer: 12000 (3000/6000)
Yaku:
  Iipeikou: 1
  Honitsu: 3
  Yakuhai: 1
  Yakuhai: 1
  Yakuhai: 1

Fu:
  BasePoints: 20
  ClosedRon: 10
  NonSimpleClosedTriplet: 8
  NonSimpleClosedTriplet: 8
  SingleWait: 2

```
---
#### Implemented hand validations as of yet

##### One Han Yaku
- [x] Tanyao
- [x] Iipeikou 
- [x] Yakuhai 
- [x] MenzenTsumo
- [x] Pinfu
- [x] Riichi
- [x] Ippatsu
- [x] Haitei
- [x] RinshanKaihou
- [x] Chankan

##### Two Han Yaku
- [x] DoubleRiichi
- [x] Toitoi
- [x] Ittsuu
- [x] SanshokuDoujun
- [x] Chantaiyao
- [x] Sanankou
- [x] SanshokuDoukou
- [x] Sankantsu
- [x] Honroutou
- [x] Shousangen
- [x] Chiitoitsu

##### Three Han Yaku
- [x] Honitsu
- [x] JunchanTaiyao
- [x] Ryanpeikou 

##### Six Han Yaku
- [x] Chinitsu

##### Yakuman 
- [x] KazoeYakuman
- [x] KokushiMusou
- [x] KokushiMusou 13 sided wait
- [x] Suuankou
- [x] Suuankou tanki wait
- [x] Daisangen
- [x] Shousuushii
- [x] Daisuushii
- [x] Tsuuiisou
- [x] Daiichishin 
- [x] Chinroutou
- [x] Ryuuiisou
- [x] ChuurenPoutou
- [x] ChuurenPoutou 9 sided wait
- [x] Suukantsu
- [ ] Tenhou NOT SURE WHAT TO DO WITH THIS
- [ ] Chiihou NOT SURE WHAT TO DO WITH THIS

# TODO
- validation a hand is possible (eg not having 20 east tiles :) 
- add all da yaku DONE
- validation on if yaku is there DONE
- validate winning tile DONE DONE
- propogate the errors up for a nice printout DONE
- validate stuff like cant riichi and double riichi. all that haitei, chankan 
rinshan shizz DONE

# Contributing
- If you spot a bug (which there probabably are many), put in an issue with how to reproduce it. 
- keep in mind, we pretty far from finishing it currently. so FUCK validation (for the time being) 


![this.jpg](https://64.media.tumblr.com/07006d83e5810b3c651254e7b9a3e713/c4dc091a7806e504-ef/s400x600/cdfb08014450e71074a0a8763a67661485d59f8c.gif)
