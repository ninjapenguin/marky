# Basic Markov Chain implementation in Rust

## Instructions

```
cargo run <path_to_text_corpus> <seed_word_1> <seed_word_2>
```

Alice in wonderland is included as an example corpus from which to read

## Examples

```
File Read..Generating chain from {...}/alice29.txt
Markov Chain Generated
Breaking on punctuation
===========

But she did not like to show you!
```

```
File Read..Generating chain from {...}/alice29.txt
Markov Chain Generated
Breaking on punctuation
===========

Alice was rather glad there WAS no one to listen to her, still it was a dispute going on between the executioner, the King, `unless it was all very well without--Maybe it's always pepper that makes you forget to talk.
```

## ToDo

 - Make seed word count configurable
 - Include probabalistic weightings within chain