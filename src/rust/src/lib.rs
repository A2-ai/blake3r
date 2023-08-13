use extendr_api::prelude::*;
use std::fs::File;
use std::io::{self, Read};
use blake3::Hasher;
use std::io::Result;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hash_file_with_blake3_impl(file_path: &str) -> String {
    match hash_file_with_blake3(&file_path) {
        Ok(hash) => hash,
        Err(e) => e.to_string(),
    }
}

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hash_file_with_blake3_direct_impl(file_path: &str) -> String {
    match hash_file_with_blake3_direct(&file_path) {
        Ok(hash) => hash,
        Err(e) => e.to_string(),
    }
}

fn hash_file_with_blake3_direct(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut hasher = Hasher::new();
    let mut buffer = [0u8; 16384]; // 16 KB buffer size

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    Ok(hash_result.to_string())
}

fn hash_file_with_blake3(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;

    let mmap = match maybe_memmap_file(&file) {
        Ok(Some(mmap)) => mmap,
        Ok(None) => {
            // Fallback to reading the file traditionally if memory mapping isn't possible
            return hash_file_with_blake3_direct(file_path);
        }
        Err(e) => return Err(e),
    };
    let mut hasher = Hasher::new();
    hasher.update_rayon(&mmap);
    Ok(hasher.finalize().to_string())
}

// Mmap a file, if it looks like a good idea. Return None in cases where we
// know mmap will fail, or if the file is short enough that mmapping isn't
// worth it. However, if we do try to mmap and it fails, return the error.
fn maybe_memmap_file(file: &File) -> Result<Option<memmap2::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    Ok(if !metadata.is_file() {
        // Not a real file.
        None
    } else if file_size > isize::max_value() as u64 {
        // Too long to safely map.
        // https://github.com/danburkert/memmap-rs/issues/69
        None
    } else if file_size == 0 {
        // Mapping an empty file currently fails.
        // https://github.com/danburkert/memmap-rs/issues/72
        None
    } else if file_size < 16 * 1024 {
        // Mapping small files is not worth it.
        None
    } else {
        // Explicitly set the length of the memory map, so that filesystem
        // changes can't race to violate the invariants we just checked.
        let map = unsafe {
            memmap2::MmapOptions::new()
                .len(file_size as usize)
                .map(file)?
        };
        Some(map)
    })
}





// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod blake3r;
    fn hash_file_with_blake3_impl;
    fn hash_file_with_blake3_direct_impl;
}
