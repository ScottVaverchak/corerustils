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

```yes``` is more or less complete. This was the first program implemented. Accept this as my apology for the implementation.

### cat

```cat``` will print out multiple files and error out when it the given file is a directory or does not exist. The following switches will be implemented:

| Switch       | Description                                                                   | Implemented? |
|--------------|-------------------------------------------------------------------------------|--------------|
| ```-b```     | Number the non-blank output lines, starting at 1.                             | Yes          |
| ```-n```     | Number the output lines, starting at 1.                                       | Yes          |
| ```-s```     | Squeeze multiple adjacent empty lines, causing the output to be single spaced | Yes          |

All other switches will not be implemented initially (or ever).

Switches I ignored for now:

| Switch   | Description |  
|----------|-------------|
| blank    | This will echo back to you |
| ```-e``` | Display non-printing characters (see the -v option), and display a dollar sign (`$') at the end of each line. |
| ```-t``` | Display non-printing characters (see the -v option), and display tab characters as `^I'. |
| ```-u``` | Disable output buffering. |
| ```-v``` | Display non-printing characters so they are visible.  Control characters print as `^X' for control-X; the delete character (octal 0177) prints as `^?'.  Non-ASCII characters (with the high bit set) are printed as `M-' (for meta) followed by the character for the low 7 bits. |

```-e, -t, -v``` would all seem to use the same logic, so these maybe completed first. 

Performance of the current implementation is questionable and will be revisted when I'm a hardened rustician with claw scars and the opinion that everything should be implemtned in rust. 

cat works as far as I'm concerned and if complete. An updated tabe of ignored switches will be added
