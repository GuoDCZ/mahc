command options
===============

```
--help
-c --chi     called tiles         (default: last tile in hand)
-w --win     winning tile         (default: last tile in hand)
-d --dora    dora (and ura) tiles (default: none)
-s --seat    seat wind            (default: East)
-p --prev    prevelant wind       (default: East)
-t --tsumo   tsumo                (default: false)
-r --riichi  riichi               (default: false)
-n           han                  (default: 0)
-fu          fu                   (default: 0)
-b --ba      honba count          (default: 0)
-m --manual  calc mode            (default: false)
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
- wind: 1z, 2z, 3z, 4z or ESWS (east, south, west, north)
<br>
- drgn: 5z, 6z, 7z or RGW (red, green, white) 
<br>


#### Examples

##### Normal Mode
```
~/$ mahc 234p3s -c 2pp4mcRk -w 3s -d 3s4p -s E -p E
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
- 234pin and 3sou in hand <br>
- 2pin pon, 4man chi, red kan <br>
- 3sou winning tile <br>
- 3sou and 4pin dora <br>
- east seat wind <br>
- east prevalent wind <br>

##### Calculator Mode
```
~/$ mahc --m 4han-30fu --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```









