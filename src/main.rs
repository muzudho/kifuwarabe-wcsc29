extern crate kifuwarabe_wcsc29_lib;
extern crate getopts;

mod arguments;

use arguments::*;
use kifuwarabe_wcsc29_lib::application::*;
use kifuwarabe_wcsc29_lib::*;
use kifuwarabe_wcsc29_lib::kifu_kif::kif_tape::*;
use kifuwarabe_wcsc29_lib::kifu_kif::kif_converter::KifConverter;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_converter::CsaConverter;
use kifuwarabe_wcsc29_lib::kifu_csa::csa_tape::*;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_deck::*;
use kifuwarabe_wcsc29_lib::object_rpm::cassette_tape_box::*;
use kifuwarabe_wcsc29_lib::shogi_ban::position::*;
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
    let tape_fragment_file_for_write = match args.output_file {
        Some(x) => x,
        None => "".to_string()
    };

    if !in_file.is_empty() && ! tape_fragment_file_for_write.is_empty() {
        // 棋譜解析。
        let extension = get_extension_from_filename(&in_file).unwrap().to_uppercase();

        // The application contains all immutable content.
        let app = Application::new();

        // Position.
        let mut position = Position::new_honshogi_origin();

        // Deck.
        let mut deck = CassetteDeck::new_change(
            Some(CassetteTapeBox::from_file(tape_fragment_file_for_write, position.get_board_size(), &app)),
            position.get_board_size(),
            &app
        );

        match extension.as_str() {
            "KIF" => {
                // Training data.
                let ktape = KifTape::from_file(&in_file);

                // Play out.
                KifConverter::play_out_kifu_tape(&ktape, &mut position, &mut deck, &app);

                // Write.
                deck.write_tape_fragment(position.get_board_size(), &app);
            },
            "CSA" => {
                // Training data.
                let ctape = CsaTape::from_file(&in_file, &app.comm);

                // Play out.
                CsaConverter::play_out_csa_tape(&ctape, &mut position, &mut deck, &app);

                // Write.
                deck.write_tape_fragment(position.get_board_size(), &app);
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
