# 0xdump
A colorized command-line hex dump utility similar to Pwntool's phd command written in Rust. This was mostly an exercise in learning rust, but I was also interested in something a little faster than the phd tool which can be a bit slow on larger files that wasn't the normal unix hexdump command, which lacks colorization.

For example, if you were to hexdump `/bin/bash` (1.1M); on my system `phd` takes 31.349s. `0xdump` on the other hand does it in 2.625s. I'm not sure what slows down phd so much. My guess is it has something to do with python's buffering of printing output, but i'm not 100% sure on that. However, 0xdump will do it a little bit faster which makes it nicer for day to day use and compiles for windows as well as a standalone exe so you won't need the entire framework just for that functionalilty.

However it is worth noting that this application will be in the ballpark of 791 kb once built, so there is that trade-off.

# Getting started

## Installation

### Build From Source (Linux)
A make file is included in this repo. This will build the binary for release and run strip against it to reduce the binary size.

1. Compile:
`make`
2. Install 0xdump:
`make install`

### Build From Source (Windows)
The application can be built using `cargo build --release`. The resulting binary will be found in target/release/.

# Color Coding Bytes
This application uses the following formatting for coloring bytes in the hex dump

| Color                        | Meaning                                            |
| ---------------------------- | -------------------------------------------------- |
| Default Shell text color     | Printable ASCII Text                               |
| Grey                         | Null Byte                                          |
| Red                          | Carriage Return or Line Feed                       |
| Green                        | 0xff                                               |
| Blue                         | Non-Printable Byte not handled by the above colors |

# Screenshots
![0xdump Help Screen](https://gist.githubusercontent.com/jaxhax-travis/f889a1df3abd1c18e91d7f6d20a48394/raw/8939eb6f1459b2c6fa882e1418cc735e68cbd060/0xdump_help.png "0xdump Help Screen")

0xdump Help Screen

![0xdump vs phd output](https://gist.githubusercontent.com/jaxhax-travis/f889a1df3abd1c18e91d7f6d20a48394/raw/8939eb6f1459b2c6fa882e1418cc735e68cbd060/0xdump_phd_side_by_side.png "0xdump vs phd output")

0xdump vs phd Output

![0xdump Reading from stdin](https://gist.githubusercontent.com/jaxhax-travis/f889a1df3abd1c18e91d7f6d20a48394/raw/8939eb6f1459b2c6fa882e1418cc735e68cbd060/0xdump_help.png "0xdump Reading from stdin")

0xdump Reading From Stdin

![phd and 0xdump Benchmarks Against /bin/bash](https://gist.githubusercontent.com/jaxhax-travis/f889a1df3abd1c18e91d7f6d20a48394/raw/bf66943692fcf712a2937d4ae700888757149bee/phd_vs_0xdump.png "phd and 0xdump Benchmarks Against /bin/bash")

phd and 0xdump Benchmarks Against /bin/bash
