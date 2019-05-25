extern crate getopts;
extern crate kifuwarabe_wcsc29_lib;

mod arguments;

use arguments::*;
use kifuwarabe_wcsc29_lib::audio_compo::audio_rack::*;
use kifuwarabe_wcsc29_lib::audio_compo::cassette_deck::*;
use kifuwarabe_wcsc29_lib::instrument::position::*;
use kifuwarabe_wcsc29_lib::media::cassette_tape::*;
use kifuwarabe_wcsc29_lib::studio::application::*;
use kifuwarabe_wcsc29_lib::*;
use kifuwarabe_wcsc29_lib::conv::converter::*;

fn main() {
    // Command line arguments.
    let args = Arguments::parse();
    let in_file = match args.input_file {
        Some(x) => x,
        None => "".to_string(),
    };
    let tape_file_name_without_extension = match args.output_file {
        Some(x) => x,
        None => "".to_string(),
    };

    if !in_file.is_empty() && !tape_file_name_without_extension.is_empty() {
        // The application contains all immutable content.
        let mut app = Application::new();

        if args.debug {
            app.kifuwarabe_flag = true;
            app.comm.println("Debug on!");
        } else {
            app.kifuwarabe_flag = false;
        }

        // Position.
        let mut position = Position::new_honshogi_origin(&app);

        // Rack.
        let mut rack = AudioRack::new(&app);
        {
            rack.set_file_name_without_extension_of_tape_box(
                Slot::Learning,
                &tape_file_name_without_extension,
            );

            // ラーニング・テープ作成。
            let mut tape = CassetteTape::new_facing_right(&app);
            tape.set_file_full_name_without_extension(&tape_file_name_without_extension);
            rack.add_tape_to_tape_box(Slot::Learning, tape, &app);
            rack.seek_of_next_tape(Slot::Learning, &app);
        }

        // 棋譜変換。
        Converter::convert(in_file, &mut rack, &mut position, &app);
    } else {
        // 対局モード。
        main_loop();
    }

    // No panic is successful.
    std::process::exit(0);
}
