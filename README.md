[![Build Status](https://travis-ci.org/matt-clement/cram.svg?branch=master)](https://travis-ci.org/matt-clement/cram)

# Cram
This repository was created for the sole purpose for me to learn more about both Rust and compression algorithms. If you have some time and have any advice regarding the two aforementioned topics, my implementations, or my code in general, I would love to hear it!

# `cargo run`
Currently, running this program through cargo will ask for a string, show you the internal representation of it using my simple run length encoding algorithm and then convert it back into a string and print that out as well. Not very exciting.

# Ideas for improvements:
## Not yet complete
* Learn how to write decent rust docs
* [RLE] General storage method (i.e. a simple byte stream, not a vec of enum members)
* [RLE] Concatenate adjacent literals.
* Other (better) compression algorithms.
## Done
* Split out algorithmic code into modules.
* [RLE] Handle both runs and literals (i.e. only encode a run if it has a certain number of repeats).
