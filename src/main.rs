use magic::Cookie;
use magic::CookieFlags;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::{OsString, OsStr};
use anyhow::Context;

fn checkfile(file: &str, magic :&Cookie, db: &HashMap<String, HashSet<String>>) -> anyhow::Result<()> {
    let mimetype = magic.file(file.clone()).context("failed to find mime type of file.")?;
    let ext = std::path::Path::new(file).extension();
            
    let ext = match ext {
        Some(ext) => ext.to_str().context("failed to get file extention")?,
        None => ""
    };
    if (&mimetype != &"inode/directory" && &mimetype != &"inode/simlink") {
        match db.get(&mimetype) {
            None => println!("{}: is {}",&file, &mimetype),
            Some(exts) => if !exts.contains(&*ext) {
                println!("{}: is {}",&file, &mimetype);
            }
        }
    }
    Ok(())
}

fn run() -> anyhow::Result<()> {
    eprint!("Loading file extention data... ");
    let db = std::fs::read_to_string("db.json").context("failed getting db.json")?;
    let db :HashMap<String, HashSet<String>> = serde_json::from_str(&db).context("failed parsing file ext db")?;
    eprint!("[OK]\n");
    eprint!("Loading magic number data ... ");
    let cookie = Cookie::open(CookieFlags::from_bits(0b000000010000).context("failed initalizing libmagic flags")?).context("failed initalizing libmagic")?;
    cookie.load(&["magic.mgc"]);
    eprint!("[OK]\n");
    for line in std::io::stdin().lock().lines() {
            let line = line.unwrap();
            checkfile(&line,&cookie,&db).with_context(|| format!("failed to prossess file {}",line))?;
    };
    Ok(())
}

fn main() -> anyhow::Result<()> {
    run()
}
