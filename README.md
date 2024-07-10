
# Riichi Mahjong Calculator Via the Terminal

# ![this.jpg](https://c.tenor.com/xfbt6ap9IYIAAAAC/tenor.gif)
command options
===============

```
--help
-t --tiles   tiles                (default: none)
-w --win     winning tile         (default: last tile in hand)
-d --dora    dora (and ura) tiles (default: none)
-s --seat    seat wind            (default: East)
-p --prev    prevelant wind       (default: East)
-t --tsumo   tsumo                (default: false)
-r --riichi  riichi               (default: false)
-m --manual  calc mode            (default: false)
-n           han                  (default: 0)
-fu          fu                   (default: 0)
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
<br>
- pin:  1p, 2p, 3p, 4p, 5p, 6p, 7p, 8p, 9p
<br>
- sou:  1s, 2s, 3s, 4s, 5s, 6s, 7s, 8s, 9s
<br>
- wind: ESWS (east, south, west, north)
<br>
- drgn: rgw (red, green, white) 
<br>


#### Examples
Args { inhand: Some(["234po", "123s", "999R", "444mo", "33s"])

##### Calculator Mode
```
~/$ mahc -m -n 4 --fu 30 --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```



##### Normal Mode NOT YET IMPLEMENTED
```
~/$ mahc --tiles 234p 33s 222po EEEw rrrrdo -w 3s -d 3s4p -s E -p E
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

