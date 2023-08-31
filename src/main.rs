use sled;
use std::io::Write;
use std::{fs, str};

mod pictrs_types;
use crate::pictrs_types::{Alias, DeleteToken};

fn main() {
    let db = sled::open("./v0.4.0-alpha.1").unwrap();
    let names = db.tree_names();

    _ = fs::create_dir("./text_dumps");

    for n in names.iter() {
        let tree = db.open_tree(n).unwrap();
        let name_vec = tree.name();
        let name = str::from_utf8(&name_vec).unwrap();
        println!("{:?}", name);

        if name != "pict-rs-alias-delete-tokens-tree".to_string(){
            println!("Skipping tree {}", name);
            continue;
        }
        let mut iter = tree.iter();
        if !tree.is_empty() {
            let filename = format!("./text_dumps/{}.txt", name);
            let mut file = fs::File::create(filename).unwrap();

            for _i in 0..tree.len() {
                let item = iter.next();
                match Some(item) {
                    None => continue,
                    item => match item.unwrap().unwrap() {
                        Ok(item) => {
                            let key_vec = item.0;
                            let val_vec = item.1;
                            let key = Alias::from_slice(&key_vec).unwrap();
                            let val = DeleteToken::from_slice(&val_vec).unwrap();
                            _ = writeln!(&mut file, "{} | {}", key, val);
                        }
                        Err(_e) => {
                            println!("Error encountered during item unrwapping, continuing...");
                            continue;
                        }
                    }
                }
            }
        }
    }
}
