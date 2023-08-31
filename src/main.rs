use sled;
use std::io::Write;
use std::{fs, str};

fn main() {
    let db = sled::open("./v0.4.0-alpha.1").unwrap();
    let names = db.tree_names();

    _ = fs::create_dir("./text_dumps");

    for n in names.iter() {
        let tree = db.open_tree(n).unwrap();
        let name_vec = tree.name();
        let name = str::from_utf8(&name_vec).unwrap();
        println!("{:?}", name);

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
                            let key = str::from_utf8(&key_vec);
                            let val = str::from_utf8(&val_vec);

                            let str1: String;
                            let str2: String;
                            match key {
                                Ok(key) => {
                                    str1 = key.to_string();
                                }
                                Err(_e) => {
                                    str1 = "".to_string();
                                }
                            }

                            match val {
                                Ok(val) => {
                                    str2 = val.to_string();
                                }
                                Err(_e) => {
                                    str2 = "".to_string();
                                }
                            }
                            _ = writeln!(&mut file, "{} | {}", str1, str2);
                        }
                        Err(_e) => {
                            continue;
                        }
                    }
                }
            }
        }
    }
}
