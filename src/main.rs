extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;

mod arguments;

use arguments::*;
use kifuwarabe_wcsc29_lib::*;
use kifuwarabe_wcsc29_lib::kif_conv::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::csa_conv::csa_converter::CsaConverter;
use std::ffi::OsStr;
use std::path::Path;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn main() {
    // Command line arguments.
    let args = Arguments::parse();
    let in_file = args.input_file.unwrap();
    let out_file = args.output_file.unwrap();

    if !in_file.is_empty() && ! out_file.is_empty() {
        // 棋譜解析。
        let ext = get_extension_from_filename(&in_file).unwrap().to_uppercase();

        match ext.as_str() {
            "KIF" => {
                KifConverter::convert_kif(&in_file, &out_file);
            },
            "CSA" => {
                CsaConverter::convert_csa(&in_file, &out_file);
            }
            _ => {print!("Pass extension: {}", ext)}
        }

    } else {
        main_loop();
    }
}
