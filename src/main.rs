use std::fs::File;
use std::io:BufReader;
use rodio::{Decoder, OutputStream source::Source};	// Rust audio
use rtmidi::{RtMidiIn, RtMidiOut, RtMidiError};		// getting MIDI input

fn main() -> Result<(), RtMidiError> {
    println!("Hello, world!");
}
