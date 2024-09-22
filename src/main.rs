use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, source::Source};	// Rust audio
// use rtmidi::{RtMidiIn, RtMidiOut, RtMidiError};		// getting MIDI input
use rfd::FileDialog;	// picking files
use freya::prelude::*;
use freya::hotreload::FreyaCtx;

const VERSION:&str = "v0.1.0";

// fn main() {
// 	dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

// 	launch(app);
// }

// fn app() -> Element {
// 	let mut count = use_signal(|| 0);

// 	render!(
// 		rect {
// 			height: "10%",
// 			width: "100%",
// 			main_align: "center",
// 			cross_align: "center",
// 			background: "rgb(119, 119, 119)",
// 			color: "white",
// 			label {
// 				font_size: "15",
// 				font_weight: "bold",
// 				"Jack's Music Box {VERSION}"
// 			}
// 		}
// 		rect {
// 			height: "55%",
// 			width: "100%",
// 			main_align: "center",
// 			cross_align: "center",
// 			background: "rgb(0, 119, 182)",
// 			color: "white",
// 			label {
// 				font_size: "50",
// 				font_weight: "bold",
// 				"Select a file to play!"
// 			}
// 			label {
// 				font_size: "25",
// 				font_weight: "bold",
// 				"MP3, WAV, Vorbis, FLAC, MP4, or AAC"
// 			}
// 		}
// 		rect {
// 			height: "35%",
// 			width: "100%",
// 			main_align: "center",
// 			cross_align: "center",
// 			direction: "horizontal",
// 			Button {
// 				onclick: move |_| count += 1,
// 				label { "Select File" }
// 			}
// 		}
// 	)

// 	// Freya example to find files: 
// 	// https://github.com/marc2332/freya/blob/main/examples/file_explorer.rs
// }

fn main() {
	let files = FileDialog::new()
		.add_filter("music", &["mp3", "wav"])
		.set_directory("/")
		.pick_file();
	// Get an output stream handle to the default physical sound device
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();
	// Load a sound from a file, using a path relative to Cargo.toml
	let file = BufReader::new(File::open("pacer_test.mp3").unwrap());
	// Decode that sound file into a source
	let source = Decoder::new(file).unwrap();
	// Play the sound directly on the device
	sink.append(source);

	// The sound plays in a separate audio thread,
	// so we need to keep the main thread alive while it's playing.
	sink.sleep_until_end();
}
