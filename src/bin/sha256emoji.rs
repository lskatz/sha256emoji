extern crate clap;
extern crate emoji;
extern crate sha256;

//use clap::{Arg, App, SubCommand};
use clap::{Arg,App,crate_version};
use sha256::digest_file;
use std::path::Path;

use emoji::lookup_by_glyph::iter_emoji;

fn main() {
    let matches = App::new("sha256emoji")
                          .version(crate_version!())
                          .arg(Arg::with_name("path")
                              .value_name("path-to-file")
                              .help("Path to the filename")
                              .takes_value(false)
                           )
                          .get_matches();

    let filename       = matches.value_of("path").unwrap();
    let input          = Path::new(filename);
    let digest:String  = digest_file(input).expect("Was not able to hashsum the file");
    /* The full string for the sha256 digest is too large
     * for converting to an int and so we will take a
     * substring of 31 digits. 32 digits seems to parse
     * without overflow and so 31 seems safe.
     */
    let max_hashsum_digits:usize = 30;
    let digest_int:u128 = u128::from_str_radix(&digest[0..max_hashsum_digits], 16).unwrap();

    // What percentage of the way are we from zero to max?
    let truncated_max_int:u128 = u128::from_str_radix(&"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"[0..max_hashsum_digits], 16).unwrap();
    let hashsum_largeness = digest_int as f64 / truncated_max_int as f64;
    
    let mut i:u16 = 0; // not more than number of emojis, currently in thousands
    let mut emoji_range:Vec<&emoji::Emoji> = Vec::new();
    for emoji in iter_emoji() {
      i += 1;
      emoji_range.push(emoji);
    }
    let num_emojis = i; // immutable instead of using i
    let fractional_step = 1 as f64 / num_emojis as f64;

    // Figure out which emoji corresponds to the largeness of the hashsum
    let mut emoji_largeness:f64 = 0 as f64;
    for emoji in iter_emoji() {
      if emoji_largeness > hashsum_largeness {
        println!("{}\t{}", filename, emoji.glyph);
        break;
      }
      emoji_largeness += fractional_step;
    }
}

