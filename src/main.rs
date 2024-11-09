use std::thread;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::stdin;
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

fn choose_file() -> Option<String> {
	println!("Please select an audio file.");
	let mut filename = String::new();
	loop {
		print!("{}", &filename);
		stdin().read_line(&mut filename).expect("Failed to read line");
		if !filename.is_empty() && filename.ends_with(".wav") || filename.ends_with(".mp3") || filename.ends_with(".ogg") {
			break;
		}
		println!("Please enter a valid file name.");
		filename.clear();
	}

	let path = Path::new(&filename);
	Some(path.to_str().unwrap().to_string())
}


fn main() {
	// Get an output stream handle to the default physical sound device
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();
	// Load a sound from a file, using a path relative to Cargo.toml

	let file = BufReader::new(File::open("feather.mp3").unwrap());
	// let file = BufReader::new(File::open(choose_file().expect("No file was chosen!")).unwrap());
	// Decode that sound file into a source
	let source = Decoder::new(file).unwrap();
	// Play the sound directly on the device
	sink.append(source);
	
	// The sound plays in a separate audio thread,
	// so we need to keep the main thread alive while it's playing.
	//sink.sleep_until_end();

	let mut user_input = String::new();
	loop {
		println!("Enter one of the following options into the prompt:");
		println!("p: pause");
		println!("n: next");
		println!(">: play");
		println!("s: seek");
		println!("l: time left");
		println!("e: exit");
		stdin().read_line(&mut user_input).expect("[ERROR] Failed to read line!");

		if user_input.eq("e\n"){
			break;
		}
		else if user_input.eq("p\n") {
			sink.pause();
		}
		else if user_input.eq(">\n") {
			sink.play();
		}
	}
		
}
