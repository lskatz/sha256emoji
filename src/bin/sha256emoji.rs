extern crate clap;
extern crate emoji;
extern crate sha2;

// for opening files
use std::fs::File;
use std::io::prelude::*;

//use clap::{Arg, App, SubCommand};
use clap::{Arg,App,crate_version};
use sha2::{Sha256,Digest};

use emoji::lookup_by_glyph::iter_emoji;

fn main() {
    let matches = App::new("sha256emoji")
                          .version(crate_version!())
                          .arg(Arg::with_name("path")
                              .value_name("path-to-file")
                              .help("Path to the filename(s)")
                              .takes_value(false)
                              .multiple(true)
                           )
                          .get_matches();

    // Read the input file as binary
    for filename in matches.values_of("path").unwrap() {
      let mut file       = File::open(filename).unwrap();
      let mut buf:Vec<u8>= Vec::new();
      file.read_to_end(&mut buf).expect("Could not read the input file to the end");

      // Make a vector of size 32 for a hashsum
      let mut hasher = Sha256::new();
      hasher.update(buf);
      let hashsum_vec = hasher.finalize()
                        .to_vec();

      let emoji = hashsum_to_emoji(&hashsum_vec);

      println!("{}\t{}", filename, emoji);
    }
}

fn hashsum_to_emoji(hashsum_vec:&Vec<u8>) -> &str{

    // Convert the vector of zero-to-255 numbers to vector of hex
    let mut hashsum_hex = vec![];
    for i in hashsum_vec {
      let hex:String = format!("{:x}", i);
      hashsum_hex.push(hex);
    }
    //println!("hashum_hex: {:?}", hashsum_hex.join(""));
    // make a string
    let digest_hex:String = hashsum_hex.join("");

    /* The full string for the sha256 digest is too large
     * for converting to an int and so we will take a
     * substring of 31 digits. 32 digits seems to parse
     * without overflow and so 31 seems safe.
     */
    let max_hashsum_digits:usize = 32;
    let digest_int:u128 = u128::from_str_radix(&digest_hex[0..max_hashsum_digits], 16).unwrap();
    //println!("digest int: {}", digest_int);

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
        return emoji.glyph;
        //println!("{}\t{}", filename, emoji.glyph);
        //break;
      }
      emoji_largeness += fractional_step;
    }

    return "";
}

