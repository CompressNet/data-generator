use color_eyre::{eyre::ContextCompat, Result};
use flate2::{bufread::GzEncoder, Compression};
use rand::Rng;
use serde::Serialize;
<<<<<<< HEAD
use std::{io::Read, path::Path};
=======
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path, time::Duration,
};
>>>>>>> fa7b055 (add file size & compression time)

/// The number of bytes to include in the header feature.
pub const HEADER_SIZE: usize = 8;

/// The number of bytes to include in the random_bytes feature.
pub const RANDOM_BYTES_SIZE: usize = 32;

#[derive(Debug, Serialize)]
pub struct Features {
    file_name: String,
    file_size: u64,
    entropy: f32,
    header: [u8; HEADER_SIZE],
    random_bytes: [u8; RANDOM_BYTES_SIZE],
    compress_time_ms: u128 ,
    compression_ratio: f32,
}

impl Features {
    pub fn new(
        file_name: String,
        file_size: u64,
        entropy: f32,
        header: [u8; HEADER_SIZE],
        random_bytes: [u8; RANDOM_BYTES_SIZE],
        compress_time_ms: u128 ,
        compression_ratio: f32,
    ) -> Self {
        Self {
            file_name,
            file_size,
            entropy,
            header,
            random_bytes,
            compress_time_ms,
            compression_ratio,
        }
    }
}

pub fn get_features(file_name: &Path, file: &[u8]) -> Result<Features> {
    let entropy = get_entropy(file)?;
    let file_sz = file.metadata()?.len();
    let header = get_header(file)?;
    let random_bytes = get_random_bytes(file)?;
    let (compress_time, compression_ratio) = get_compression_ratio_and_time(file)?;

    Ok(Features::new(
        file_name
            .to_str()
            .with_context(|| "Could not get file name")?
            .to_string(),
        file_sz,
        entropy,
        header,
        random_bytes,
        compress_time.as_millis(),
        compression_ratio,
    ))
}

<<<<<<< HEAD
fn get_compression_ratio(file: &[u8]) -> Result<f32> {
=======
fn get_compression_ratio_and_time(file: &File) -> Result<(Duration, f32)> {
    let start_time = std::time::Instant::now();

>>>>>>> fa7b055 (add file size & compression time)
    // man gzip says the default level is 6
    let mut gz_encoder = GzEncoder::new(file, Compression::new(6));

    let mut buffer = Vec::new();
    gz_encoder.read_to_end(&mut buffer)?;

<<<<<<< HEAD
    let orig_len = file.len() as f32;
=======
    let end_time = std::time::Instant::now();

    let orig_len = file.metadata()?.len() as f32;
>>>>>>> fa7b055 (add file size & compression time)
    let compress_len = buffer.len() as f32;

    // return the compression ratio
    let ratio = 1f32 - (compress_len / orig_len);

    Ok((end_time - start_time, ratio))
}

fn get_random_bytes(file: &[u8]) -> Result<[u8; RANDOM_BYTES_SIZE]> {
    // Read RANDOM_BYTES_SIZE bytes randomly from the file
    let mut random_bytes = [0; RANDOM_BYTES_SIZE];

    let len = file.len();

    if len == 0 {
        return Ok(random_bytes);
    }

    // Seek to a random position in the file
    let mut rng = rand::thread_rng();

    for i in random_bytes.iter_mut() {
        let pos = rng.gen_range(0..len);
        *i = file[pos];
    }

    Ok(random_bytes)
}

/// Read the first 8 bytes of the file.
fn get_header(file: &[u8]) -> Result<[u8; HEADER_SIZE]> {
    let mut header = [0; HEADER_SIZE];

    let len = ..file.len().min(HEADER_SIZE);
    header[len].copy_from_slice(&file[len]);

    Ok(header)
}

/// Computes the entropy on the file
fn get_entropy(file: &[u8]) -> Result<f32> {
    let mut counts = [0; 256];
    let mut total = 0;

    for byte in file.iter() {
        counts[*byte as usize] += 1;
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
