# bee

My wife and I both love the New York Times' [Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) daily puzzle. She's a little smarter than me, but I know how to code and wanted to learn Rust so I made a little utility to help generate words for me.

Usage:

```sh
bee abcdefg
```

(The first letter is the required one.)

Output:

```sh
[8]: cabbaged, debagged, fabaceae, fagaceae
[7]: acceded, babbage, baggage, cabbage, debadge, defaced, effaced, facaded, feedbag
[6]: abedge, abegge, accede, afaced, bacaba, baccae, badaga, badged, baffed, bagdad, bagged, beaded, bedaff, bedead, bedeaf, cabbed, cadded, cadged, dabbed, daffed, dagaba, dagged, decade, deface, defade, degage, efface, facade, fadged, fagged, gabbed, gabgab, gadaba, gadaea, gadbee, gadded, gaffed, gagged
[5]: abaca, abada, abaff, abede, adage, addda, added, adead, aface, agada, agade, baaed, bacca, badge, bagge, bedad, begad, caaba, cabaa, cabda, cadee, cadge, caeca, caffa, caged, cecca, dabba, dagga, debag, decad, faade, faced, faded, fadge, gadge, gaffe, gaged, gagee
[4]: abac, abba, abbe, abed, acad, acca, acce, aced, adad, adda, affa, agad, agag, agba, aged, agee, baba, babe, bade, baff, baga, bead, caba, caca, cace, cade, cafe, caff, cage, ceca, dabb, dace, dada, dade, daff, dead, deaf, deda, dgag, ecad, ecca, edda, edea, egad, egba, faba, face, fade, faff, fage, gabe, gade, gaea, gaed, gaff, gaga, gage
```

The word list is a modified version of `words_alpha.txt` from the [`dwyl/english-words`](https://github.com/dwyl/english-words) repository, which is public domain. It is definitely _not_ the one used by the New York Times and therefore contains many words that will be rejected by the puzzle.
