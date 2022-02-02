use magic::Cookie;
use magic::CookieFlags;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::{OsString, OsStr};

fn main() {
    eprint!("Loading file extention data... ");
    let db = std::fs::read_to_string("db.json").unwrap();
    let db :HashMap<String, HashSet<String>> = serde_json::from_str(&db).unwrap();
    eprint!("[OK]\n");
    eprint!("Loading magic number data ... ");
    let cookie = Cookie::open(CookieFlags::from_bits(0b000000010000).unwrap()).unwrap();
    cookie.load(&["magic.mgc"]);
    eprint!("[OK]\n");
    for line in std::io::stdin().lock().lines() {
            let line = line.unwrap();
            
            let mimetype = cookie.file(line.clone()).unwrap();
            let ext = std::path::Path::new(&line).extension();
            
            let ext =match ext {
                Some(ext) => ext.to_str().unwrap(),
                None => ""
            };
            if (&mimetype != &"inode/directory" && &mimetype != &"inode/simlink") {
                match db.get(&mimetype) {
                    None => println!("{}: is {}",&line, &mimetype),
                    Some(exts) => if !exts.contains(&*ext) {
                        println!("{}: is {} '{}'",&line, &mimetype,ext);
                    }
                }
            }            
    }
}
