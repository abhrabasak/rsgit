use std::{
    fs::File,
    io::{Read, Write},
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::errors::ErrorCode;

pub fn file_read(file: &str) -> Vec<u8> {
    let mut file_handle = File::open(file)
        .map_err(|_| ErrorCode::FileNotFound)
        .unwrap();
    let mut file_content = Vec::new();
    file_handle
        .read_to_end(&mut file_content)
        .map_err(|_| ErrorCode::FileReadError)
        .unwrap();
    file_content
}

pub fn file_write(file: &str, content: &[u8]) {
    let mut file_handle = File::create(file)
        .map_err(|_| ErrorCode::FileWriteError)
        .unwrap();
    file_handle
        .write_all(content)
        .map_err(|_| ErrorCode::FileWriteError)
        .unwrap();
}

pub(crate) fn sha1_hash(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn zlib_encode(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|_| ErrorCode::FileWriteError)
        .unwrap();
    encoder
        .finish()
        .map_err(|_| ErrorCode::EncodingError)
        .unwrap()
}

pub fn zlib_decode_file_to_string(file_path: &str) -> String {
    let file_handle = File::open(file_path)
        .map_err(|_| ErrorCode::FileNotFound)
        .unwrap();
    let mut decoder = ZlibDecoder::new(file_handle);
    let mut decoded_content = String::new();
    decoder
        .read_to_string(&mut decoded_content)
        .map_err(|_| ErrorCode::DecodingError)
        .unwrap();
    decoded_content
}
