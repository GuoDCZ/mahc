
# Riichi Mahjong Calculator Via the Terminal

# ![this.jpg](https://c.tenor.com/xfbt6ap9IYIAAAAC/tenor.gif)
# ![demo gif](demo.gif)

Options
===============

```
--help
--tiles   tiles                (default: none)
-w --win     winning tile         (default: last tile in hand)
-d --dora    dora (and ura) tiles (default: none)
-s --seat    seat wind            (default: East)
-p --prev    prevelant wind       (default: East)
-t --tsumo   tsumo                (default: false)
-r --riichi  riichi               (default: false)
-m --manual  calc mode            (default: false)
-b --ba      honba count          (default: 0)
.
.
.
.
and so on and so on 
```

Notation
-------
- man:  1m, 2m, 3m, 4m, 5m, 6m, 7m, 8m, 9m
- pin:  1p, 2p, 3p, 4p, 5p, 6p, 7p, 8p, 9p
- sou:  1s, 2s, 3s, 4s, 5s, 6s, 7s, 8s, 9s
- wind: ESWS (east, south, west, north)
- drgn: rgw (red, green, white) 
- o: signifies an open set


#### Examples

##### Calculator Mode
```
~/$ mahc -m 4 30 --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```



##### Normal Mode NOT YET IMPLEMENTED
note: the winning group has to go last (this is to calculate fu correctly)
```
~/$ mahc --tiles 234p 222po EEEw rrrrdo 33s -w 3s -d 0 -s E -p E
> Dealer ron
  1 Han/ 40 Fu 2000 pts 

  Yaku: 
    Yakuhai             1 Han 
  Fu:
    Base points        20 Fu
    Single wait        2  Fu
    Simple triplet     20 Fu
    Non-simples kan    16 Fu
```

- this hand has:
- 234pin, East Trip and 3sou in hand <br>
- 2pin pon, red open kan <br>
- 3sou winning tile <br>
- 3sou and 4pin dora <br>
- east seat wind <br>
- east prevalent wind <br>

##### implemented hand validations as of yet

###### One Han Yaku
- [x] Tanyao
- [x] Iipeikou 
- [x] Yakuhai 
- [ ] MenzenTsumo
- [ ] Pinfu
- [ ] Riichi
- [ ] Ippatsu
- [ ] HaiteiRaoyue
- [ ] HouteiRaoyui
- [ ] RinshanKaihou
- [ ] Chankan

###### Two Han Yaku
- [ ] DoubleRiichi
- [ ] Toitoi
- [ ] Ittsuu
- [ ] SanshokuDoujun
- [ ] Chantaiyao
- [ ] Sanankou
- [ ] SanshokuDoukou
- [ ] Sankantsu
- [ ] Honroutou
- [ ] Shousangen
- [ ] Chiitoitsu

###### Three Han Yaku
- [ ] Honitsu
- [ ] JunchanTaiyao
- [x] Ryanpeikou //done

###### Six Han Yaku
- [ ] Chinitsu

###### Yakuman 
- [ ] KazoeYakuman
- [ ] KokushiMusou
- [ ] Suuankou
- [ ] Daisangen
- [ ] Shousuushii
- [ ] Daisuushii
- [ ] Tsuuiisou
- [ ] Chinroutou
- [ ] Ryuuiisou
- [ ] ChuurenPoutou
- [ ] Suukantsu
- [ ] Tenhou
- [ ] Chiihou


