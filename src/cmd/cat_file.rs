use crate::object::GitObject;

pub fn cat_file_exec(pretty_print: &bool, hash: &str) {
    let go = GitObject::load(hash).unwrap();
    if *pretty_print {
        go.cat();
    }
}
