# bee

My wife and I both love the New York Times' [Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) daily puzzle. She's a little smarter
than me, but I know how to code and wanted to learn Rust so I made a little utility to
help generate words for me.

Usage:

```sh
bee abcdefg
```

The first letter is the required one.

The word list is a modified version of `words_alpha.txt` from the [`dwyl/english-words`](https://github.com/dwyl/english-words) repository, which is public domain. It is definitely _not_ the one used by the New York Times and therefore contains many words that will be rejected by the puzzle.
