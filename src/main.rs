extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;

mod arguments;

use arguments::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
use kifuwarabe_wcsc29_lib::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_kif::kif_tape::*;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_converter::CsaConverter;
use kifuwarabe_wcsc29_lib::sheet_music_format::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
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
    let in_file = match args.input_file {
        Some(x) => x,
        None => "".to_string()
    };
    let tape_file_name_without_extension = match args.output_file {
        Some(x) => x,
        None => "".to_string()
    };

    if !in_file.is_empty() && ! tape_file_name_without_extension.is_empty() {
        // The application contains all immutable content.
        let mut app = Application::new();

        if args.debug {
            app.kifuwarabe_flag = true;
            app.comm.println("Debug on!");
        }
        
        // 棋譜解析。
        let extension = get_extension_from_filename(&in_file).unwrap_or_else(|| panic!(app.comm.panic("Fail. get_extension_from_filename."))).to_uppercase();

        // Position.
        let mut position = Position::new_honshogi_origin(&app);

        // Deck.
        let mut deck = CassetteDeck::new_for_tape_conversion(
            &tape_file_name_without_extension,
            &app
        );

        match extension.as_str() {
            "KIF" => {
                // Training data.
                let ktape = KifTape::from_file(&in_file, &app);

                // Play out.
                KifConverter::play_out_kifu_tape(&ktape, &mut position, &mut deck, &app);

                // Write.
                deck.write_leaning_tape_fragment(position.get_board_size(), &app);
            },
            "CSA" => {
                // Training data.
                let ctape = CsaTape::from_file(&in_file, &app);

                // Play out.
                CsaConverter::play_out_csa_tape(&ctape, &mut position, &mut deck, &app);

                // Write.
                deck.write_leaning_tape_fragment(position.get_board_size(), &app);
            }
            _ => {print!("Pass extension: {}", extension)}
        }
    } else {
        // 対局モード。
        main_loop();
    }

    // No panic is successful.
    std::process::exit(0);
}
