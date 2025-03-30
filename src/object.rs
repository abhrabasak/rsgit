use crate::errors::ErrorCode;
use crate::utils;

#[derive(Debug)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
            ObjectType::Tag => write!(f, "tag"),
        }
    }
}

pub struct GitObject {
    pub kind: ObjectType,
    _size: usize,
    value: Vec<u8>,
}

impl GitObject {
    pub fn load(hash: &str) -> Result<Self, ErrorCode> {
        let object_path = Self::to_path(hash);
        let file_content = utils::zlib_decode_file_to_string(&object_path);

        let (header, body) = file_content.split_once('\0').unwrap_or_default();
        let (kind, object_size) = header.split_once(' ').unwrap_or_default();

        let size = object_size
            .parse::<usize>()
            .map_err(|_| ErrorCode::IntegerParseError)
            .unwrap();
        assert_eq!(body.len(), size);

        let object_type = match kind {
            "blob" => ObjectType::Blob,
            _ => panic!("{}", ErrorCode::UnsupportedObjectType),
        };

        Ok(Self {
            kind: object_type,
            _size: size,
            value: body.as_bytes().to_vec(),
        })
    }

    pub fn create(object_type: ObjectType, file_path: &str) -> Self {
        let file_content = utils::file_read(file_path);
        let size = file_content.len();

        Self {
            kind: object_type,
            _size: size,
            value: file_content,
        }
    }

    pub fn hash(&self) -> String {
        let header = format!("{} {}\0", self.kind, self.value.len());

        let mut file_content = Vec::new();
        file_content.extend_from_slice(&header.as_bytes());
        file_content.extend_from_slice(&self.value);

        let hash = utils::sha1_hash(&file_content);
        hash
    }

    pub fn store(&self) -> String {
        let header = format!("{} {}\0", self.kind, self.value.len());

        let mut file_content = Vec::new();
        file_content.extend_from_slice(&header.as_bytes());
        file_content.extend_from_slice(&self.value);

        let hash = utils::sha1_hash(&file_content);
        let encoded = utils::zlib_encode(&file_content);
        let path = Self::to_path(&hash);
        utils::file_write(&path, &encoded);
        hash
    }

    pub fn cat(&self) {
        match self.kind {
            ObjectType::Blob => print!("{}", String::from_utf8(self.value.clone()).unwrap()),
            _ => println!("Other object type: {:?}", self.kind),
        }
    }

    fn to_path(hash: &str) -> String {
        let (folder, file) = hash.split_at(2);
        format!(".git/objects/{}/{}", folder, file)
    }
}
