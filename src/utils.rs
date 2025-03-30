use std::{
    fs::File,
    io::{Read, Write},
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::object::ObjectType;

pub fn file_read(file: &str) -> Vec<u8> {
    let mut file_handle = File::open(file).unwrap();
    let mut file_content = Vec::new();
    file_handle.read_to_end(&mut file_content).unwrap();
    file_content
}

pub fn file_write(file: &str, content: &[u8]) {
    let mut file_handle = File::create(file).unwrap();
    file_handle.write_all(content).unwrap();
}

pub(crate) fn sha1_hash(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn object_hash_to_path(hash: &str) -> String {
    let (folder, file) = hash.split_at(2);
    format!(".git/objects/{}/{}", folder, file)
}

pub fn object_parse(content: &str) -> (ObjectType, &str) {
    let (header, body) = content.split_once('\0').unwrap();
    let (kind, object_size) = header.split_once(' ').unwrap();

    let size = object_size.parse::<usize>().unwrap();
    assert_eq!(body.len(), size);

    let object_type = match kind {
        "blob" => ObjectType::Blob,
        _ => panic!("Unsupported object type"),
    };

    (object_type, body)
}

pub fn object_write(object_type: ObjectType, content: &[u8]) -> String {
    let content_size = content.len();
    let header = format!("{} {}\0", object_type, content_size);

    let mut object_file_content = Vec::new();
    object_file_content.extend_from_slice(&header.as_bytes());
    object_file_content.extend_from_slice(content);
    let compressed_data = zlib_encode(&object_file_content);

    let hash = sha1_hash(&object_file_content);
    let path = object_hash_to_path(&hash);
    file_write(&path, &compressed_data);
    hash
}

pub fn zlib_encode(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

pub fn zlib_decode_file_to_string(file_path: &str) -> String {
    let file_handle = File::open(file_path).unwrap();
    let mut decoder = ZlibDecoder::new(file_handle);
    let mut decoded_content = String::new();
    decoder.read_to_string(&mut decoded_content).unwrap();
    decoded_content
}
