use color_eyre::{eyre::ContextCompat, Result};
use flate2::{bufread::GzEncoder, Compression};
use rand::Rng;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

/// The number of bytes to include in the header feature.
pub const HEADER_SIZE: usize = 8;

/// The number of bytes to include in the random_bytes feature.
pub const RANDOM_BYTES_SIZE: usize = 32;

#[derive(Debug, Serialize)]
pub struct Features {
    file_name: String,
    entropy: f32,
    header: [u8; HEADER_SIZE],
    random_bytes: [u8; RANDOM_BYTES_SIZE],
    compression_ratio: f32,
}

impl Features {
    pub fn new(
        file_name: String,
        entropy: f32,
        header: [u8; HEADER_SIZE],
        random_bytes: [u8; RANDOM_BYTES_SIZE],
        compression_ratio: f32,
    ) -> Self {
        Self {
            file_name,
            entropy,
            header,
            random_bytes,
            compression_ratio,
        }
    }
}

pub fn get_features(file_name: &Path, file: &File) -> Result<Features> {
    let entropy = get_entropy(file)?;
    let header = get_header(file)?;
    let random_bytes = get_random_bytes(file)?;
    let compression_ratio = get_compression_ratio(file)?;

    Ok(Features::new(
        file_name
            .to_str()
            .with_context(|| "Could not get file name")?
            .to_string(),
        entropy,
        header,
        random_bytes,
        compression_ratio,
    ))
}

fn get_compression_ratio(file: &File) -> Result<f32> {
    // man gzip says the default level is 6
    let mut gz_encoder = GzEncoder::new(BufReader::new(file), Compression::new(6));

    let mut buffer = Vec::new();
    gz_encoder.read_to_end(&mut buffer)?;

    let orig_len = file.metadata()?.len() as f32;
    let compress_len = buffer.len() as f32;

    // return the compression ratio
    let ratio = 1f32 - (compress_len / orig_len);

    Ok(ratio)
}

fn get_random_bytes(file: &File) -> Result<[u8; RANDOM_BYTES_SIZE]> {
    // Read RANDOM_BYTES_SIZE bytes randomly from the file
    let mut random_bytes = [0; RANDOM_BYTES_SIZE];
    let mut reader = BufReader::new(file);

    let len = file.metadata()?.len();

    // Seek to a random position in the file
    let mut rng = rand::thread_rng();
    let mut buf = [0; 1];

    for i in random_bytes.iter_mut() {
        let pos = rng.gen_range(0..len);
        reader.seek(SeekFrom::Start(pos))?;
        reader.read_exact(&mut buf)?;
        *i = buf[0];
    }

    Ok(random_bytes)
}

/// Read the first 8 bytes of the file.
fn get_header(file: &File) -> Result<[u8; HEADER_SIZE]> {
    let mut header = [0; HEADER_SIZE];
    let mut reader = BufReader::with_capacity(HEADER_SIZE, file);
    reader.seek(SeekFrom::Start(0))?;
    let _ = reader.read(&mut header)?;

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
