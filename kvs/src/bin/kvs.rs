use kvs::KvStore;
use std::io::{Error, ErrorKind};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs")]
enum Opt {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut store = KvStore::new();
    match opt {
        Opt::Get { key } => match store.get(key) {
            Some(val) => {
              println!("{}", val);
              Ok(())
            },
            None => { 
              eprintln!("Key not found.");
              Err(Error::from(ErrorKind::Other))
            }
        },
        Opt::Set { key, value } => match store.set(key.clone(), value.clone()) {
            Some(_) => {
              println!("Pair updated ({}, {})", key, value);
              Ok(())
            },
            None => {
              println!("Pair inserted ({}, {})", key, value);
              Ok(())
            },
        },
        Opt::Rm { key } => match store.remove(&key) {
            Some(value) => {
              println!("Pair updated ({}, {})", key, value);
              Ok(())
            },
            None => { 
              eprintln!("Key not found.");
              Err(Error::from(ErrorKind::Other))
            }
        },
    }
}
