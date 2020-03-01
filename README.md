# Coreutils... but in rust 🤔🦀

## What?

This is a recreation of [coreutil](https://en.wikipedia.org/wiki/List_of_GNU_Core_Utilities_commands) in rust. This is for learning and less for use. Anyone mad enough to use this on their computers can buy me a coffee.

## Why rust?

Rust is an interesting language I have been looking into for the past few years. Plus, this will pass the time until Jai is released to the general public.

## So this will be exactly like coreutils?
As close as possible. Some options might not be implmented.

## Status
Anything not listed below hasn't been worked on yet. 

* [yes](#yes)
* [cat](#cat)

### yes 

More or less complete? This was the first program implemented. I will revist it evnetually

### cat

```cat``` will print out multiple files and error out when it the given file is a directory or does not exist. The following switches will be implemented:

| switch | description                                                                   |
|--------|-------------------------------------------------------------------------------|
| -b     | Number the non-blank output lines, starting at 1.                             |
| -n     | Number the output lines, starting at 1.                                       |
| -s     | Squeeze multiple adjacent empty lines, causing the output to be single spaced |

All other switches will not be implemented initially (or ever).

Performance of the current implementation is questionable and will be revisted when I'm a hardened rustician with claw scars and the opinion that everything should be implemtned in rust. 