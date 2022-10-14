use color_eyre::Result;
use rand::Rng;
use std::{
    fs::File,
    io::{BufReader, Read, Seek},
};

/// The number of bytes to include in the header feature.
const HEADER_SIZE: usize = 8;

/// The number of bytes to include in the random_bytes feature.
const RANDOM_BYTES_SIZE: usize = 64;

pub struct Features {
    file_name: String,
    entropy: f32,
    header: [u8; HEADER_SIZE],
    random_bytes: [u8; RANDOM_BYTES_SIZE],
}

impl Features {
    pub fn new(
        file_name: String,
        entropy: f32,
        header: [u8; HEADER_SIZE],
        random_bytes: [u8; RANDOM_BYTES_SIZE],
    ) -> Self {
        Self {
            file_name,
            entropy,
            header,
            random_bytes,
        }
    }
}

pub fn get_features(file_name: &str, file: &File) -> Result<Features> {
    let entropy = get_entropy(file)?;
    let header = get_header(file)?;
    let random_bytes = get_random_bytes(file)?;

    Ok(Features::new(
        file_name.to_owned(),
        entropy,
        header,
        random_bytes,
    ))
}

fn get_random_bytes(file: &File) -> Result<[u8; RANDOM_BYTES_SIZE]> {
    // Read RANDOM_BYTES_SIZE bytes randomly from the file
    let mut random_bytes = [0; RANDOM_BYTES_SIZE];
    let mut reader = BufReader::new(file);

    let len = file.metadata()?.len();

    // Seek to a random position in the file
    let mut rng = rand::thread_rng();
    let mut buf = [0; 1];

    for i in 0..RANDOM_BYTES_SIZE {
        let pos = rng.gen_range(0..len);
        reader.seek(std::io::SeekFrom::Start(pos))?;
        reader.read_exact(&mut buf)?;
        random_bytes[i] = buf[0];
    }

    Ok(random_bytes)
}

/// Read the first 8 bytes of the file.
fn get_header(file: &File) -> Result<[u8; HEADER_SIZE]> {
    let mut header = [0; HEADER_SIZE];
    let mut reader = BufReader::with_capacity(HEADER_SIZE, file);
    reader.read_exact(&mut header)?;

    Ok(header)
}

/// Computes the entropy on the file, [0, 1]
fn get_entropy(file: &File) -> Result<f32> {
    let mut counts = [0; 256];
    let mut total = 0;

    for byte in file.bytes() {
        let byte = byte?;
        counts[byte as usize] += 1;
        total += 1;
    }

    let mut entropy = 0.0;

    for count in counts.iter() {
        if *count > 0 {
            let p = *count as f64 / total as f64;
            entropy -= p * p.log2();
        }
    }

    Ok(entropy as f32)
}
