use crate::object::{GitObject, ObjectType};

pub fn hash_object_exec(write: &bool, file: &str) {
    let go = GitObject::create(ObjectType::Blob, file);
    let hash = if *write { go.store() } else { go.hash() };
    print!("{}", hash);
}
