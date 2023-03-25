use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use clap::{Arg, App, ArgMatches};

fn main() {
    /////////////////////////////////////////////////////////////
    // Parse arguments
    /////////////////////////////////////////////////////////////
    let matches = App::new("0xdump")
        .version("v1.1")
        .about("Hexdump similar to pwntools phd tool.")
        .author("Travis Phillips")
    .arg(Arg::with_name("skip")
        .long("skip")
        .short("s")
        .takes_value(true)
        .value_name("SKIP")
        .default_value("0")
        .help("bytes to skip (default: 0)"))
    .arg(Arg::with_name("count")
        .long("count")
        .short("c")
        .takes_value(true)
        .value_name("COUNT")
        .default_value("0")
        .help("bytes to dump (default: 0)"))
    .arg(Arg::with_name("width")
        .long("width")
        .short("w")
        .takes_value(true)
        .value_name("WIDTH")
        .default_value("16")
        .help("Column Width per line (default: 16)"))
    .arg(Arg::with_name("no-color")
        .long("no-color")
        .short("n")
        .takes_value(false)
        .help("Print without colors"))
    .arg(Arg::with_name("file")
        .help("File to hexdump."))
    .get_matches();

    /////////////////////////////////////////////////////////////
    // Check if a file was provided, if not, assume we are
    // reading from stdin, otherwise dump the file.
    /////////////////////////////////////////////////////////////
    if matches.is_present("file") {
        let buffer = read_file(&matches);
        hexdump(buffer, matches);
    } else {
        let buffer =  read_stdin();
        hexdump(buffer, matches);
    }
}

/////////////////////////////////////////////////////////////
// Read data from stdin and return it as a u8 vector.
/////////////////////////////////////////////////////////////
fn read_stdin() -> Vec<u8> {
    println!("\nReading from stdin...\n");
    let mut buffer = Vec::new();
    let _len = io::stdin().read_to_end(&mut buffer);
    return buffer;
}

/////////////////////////////////////////////////////////////
// Read data from file and return it as a u8 vector.
/////////////////////////////////////////////////////////////
fn read_file(matches: &ArgMatches) -> Vec<u8> {
    let path = Path::new(matches.value_of("file").unwrap());
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    let _len = file.read_to_end(&mut buffer);
    println!("\nHexdump of \x1b[32;1m{}\x1b[0m (\x1b[36;1m{}\x1b[0m bytes):\n",
             display, buffer.len());
    return buffer;
}

/////////////////////////////////////////////////////////////
// Print out a hex dump of the u8 vector of data.
/////////////////////////////////////////////////////////////
fn hexdump(buffer: Vec<u8>, matches: ArgMatches) {
    let mut i = 0;
    let mut line_len = 0;
    let skip = matches.value_of("skip").unwrap().parse::<u32>().unwrap();
    let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let use_count = matches.is_present("count");
    let count = matches.value_of("count").unwrap().parse::<u32>().unwrap();
    let mut matched_prev = false;
    let mut hex_bytes = String::from(" ");
    let mut hex_text = String::from(" ");
    let mut prev_line = String::from("");
    let mut offset_text = String::from("");

    /////////////////////////////////////////////////////////////////
    // generate color codes.
    /////////////////////////////////////////////////////////////////
    let mut gy = "\x1b[30;1m";
    let mut r = "\x1b[31m";
    let mut gn = "\x1b[32m";
    #[cfg(target_os = "linux")]
    let mut b = "\x1b[34;1m";
    #[cfg(target_os = "windows")]
    let mut b = "\x1b[34m";
    let mut n = "\x1b[0m";
    let mut bold = "\x1b[0;1m";

    /////////////////////////////////////////////////////////////////
    // Clear color codes if user specified --no-color switch.
    /////////////////////////////////////////////////////////////////
    if matches.is_present("no-color") {
        gy = "";
        r = "";
        gn = "";
        b = "";
        n = "";
        bold = "\x1b[0;1m";
    }

    /////////////////////////////////////////////////////////////////
    // loop through each byte in the buffer.
    /////////////////////////////////////////////////////////////////
    for x in &buffer {
        /////////////////////////////////////////////////////////////
        // If the user wants to skip bytes, skip till then.
        /////////////////////////////////////////////////////////////
        if i < skip {
            i += 1;
            continue;
        }

        /////////////////////////////////////////////////////////////
        // If the user is using the count parameter, keep an eye on
        // it and break when hit.
        /////////////////////////////////////////////////////////////
        if use_count && count > 0 && (i-skip) >= count {
            break;
        }

        /////////////////////////////////////////////////////////////
        // If the line is clear, initalize the starting text.
        /////////////////////////////////////////////////////////////
        if line_len == 0 {
            hex_bytes = String::from(" ");
            hex_text = format!("{} │{}", bold, n);
            offset_text = format!("{}{:08x} ", n, i);
        } else if line_len % 4 == 0 {
            /////////////////////////////////////////////////////////
            //  If the length is at 32 bit offset, inject a bar.
            /////////////////////////////////////////////////////////
            hex_bytes.push_str(" ");
            hex_text = format!("{}{}│{}", hex_text, b, n);
        }

        /////////////////////////////////////////////////////////
        //  Populate the buffers accordingly by btye.
        /////////////////////////////////////////////////////////
        if x == &0x00 {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, gy, x, n);
            hex_text = format!("{}{}.{}", hex_text, gy, n);
        } else if x == &0x0a || x == &0x0d { // i'm kinda leaning to make these cyan.
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, r, x, n);
            hex_text = format!("{}{}.{}", hex_text, r, n);
        } else if x == &0xff {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, gn, x, n);
            hex_text = format!("{}{}.{}", hex_text, gn, n);
        } else if x < &0x20 || x > &0x7e {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, b, x, n);
            hex_text = format!("{}{}.{}", hex_text, b, n);
        } else {
            hex_bytes = format!("{}{}{:02x} ", hex_bytes, n, x);
            hex_text = format!("{}{}{}", hex_text, n, *x as char);
        }

        /////////////////////////////////////////////////////////
        //  increment our counters.
        /////////////////////////////////////////////////////////
        i += 1;
        line_len += 1;

        if line_len == width {
            hex_text = format!("{}{}│", hex_text, bold);
            let curr_line = format!("{}{}", hex_bytes, hex_text);
            if curr_line != prev_line {
                println!("{}{}", offset_text, curr_line);
                matched_prev = false;
            } else if curr_line == prev_line && matched_prev == false {
                println!("*");
                matched_prev = true;
            }
            prev_line = curr_line;
            line_len = 0;
        }
    }

    /////////////////////////////////////////////////////////////////
    // Handle any remaining bytes in the buffer if it didn't trigger
    // the width size to dump in the loop.
    /////////////////////////////////////////////////////////////////
    if line_len != width && line_len != 0 {
        if hex_bytes.len() > 1 {
            print!("{}{}", offset_text, hex_bytes);
            while line_len != width {
                if line_len % 4 == 0 {
                    print!(" ");
                }
                print!("   ");
                line_len += 1;
            }
            hex_text = format!("{}{}│{}", hex_text, bold, n);
            println!("{}", hex_text);
        }
    }
    println!("");
}
