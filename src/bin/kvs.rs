use structopt::StructOpt;
use kvs::KvStore;
use std::process;


#[derive(StructOpt, Debug)]
#[structopt(name="kvs")]
enum Opt {
  Get { key: String },
  Set { key: String, value: String },
  Rm { key: String },
}

fn main(){
  let opt = Opt::from_args();
  let store = KvStore::new();
  match opt {
    Opt::Get { key } => { 
      store.get(key).unwrap();
    },
    Opt::Set { key, value } => {
      store.set(key, value);
    },
    Opt::Rm { key } => {
      store.remove(key);
    }
  }
  process::exit(1);
}