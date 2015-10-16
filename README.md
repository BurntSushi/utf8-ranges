utf8-ranges
===========
This crate converts contiguous ranges of Unicode scalar values to UTF-8 byte
ranges. This is useful when constructing byte based automata from Unicode.
Stated differently, this lets one embed UTF-8 decoding as part of one's
automaton.
