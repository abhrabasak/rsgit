use std::{fs::File, io::Read};

use flate2::read::ZlibDecoder;

#[derive(Debug)]
pub enum ObjectType {
    Blob,
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

pub fn zlib_decode_file_to_string(file_path: &str) -> String {
    let file_handle = File::open(file_path).unwrap();
    let mut decoder = ZlibDecoder::new(file_handle);
    let mut decoded_content = String::new();
    decoder.read_to_string(&mut decoded_content).unwrap();
    decoded_content
}
