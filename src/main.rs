use rodio::{Decoder, OutputStream, source::Source};
use std::{fs::File, io::BufReader};

fn main() {
    // Créer un flux de sortie audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Ouvrir le fichier audio
    let file = File::open("sounds/Beat_It_1.wav").expect("Failed to open sound file");
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Jouer le fichier audio
    stream_handle.play_raw(source.convert_samples()).expect("Failed to play sound");

    // Maintenir le programme en vie pour permettre la lecture audio
    std::thread::sleep(std::time::Duration::from_secs(100));
}