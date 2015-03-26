#![feature(plugin)]
#![plugin(docopt_macros)]


extern crate self_encryption;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rustc-serialize;
extern crate docopt;
use self_encryption::*;

docopt!(Args derive Debug, "
Usage: SelfEncryptor --encrypt <data_map> <storage_name>
       SelfEncryptor --decrypt <storage_name> <content>

Options:
    --encrypt 
    --decrypt 
", arg_data_map: DataMap,
   arg_storage: Vec<u8>,
   arg_content: String);




fn main() {



 let args: Args = docopt.decode().unwrap_or_else(|e| e.exit());


struct MyStorage {
  temp_dir : TempDir
}

impl MyStorage {
  fn new() -> MyStorage {
    MyStorage { temp_dir: match TempDir::new(arg.arg_storage_name) {
        Ok(dir) => dir,
        Err(e) => panic!("couldn't create temporary directory: {}", e)
    } }
  }
}


impl Storage for MyStorage {
  fn get(&self, name: Vec<u8>) -> Vec<u8> {
    let file_name = String::from_utf8(name).unwrap();
    let file_path = self.temp_dir.path().join(Path::new(&file_name)); 
    let mut f = match std::fs::File::open(&file_path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    //f.read_to_string(&mut s); put f into a string
    match f.read_to_string(&mut s){
        Err(why) => panic!("couldn't read: {}", why.description()),
        Ok(_) => print!("contains:\n{}", s),
    }
    s.into_bytes()
  }

  fn put(&mut self, name: Vec<u8>, data: Vec<u8>) {
    let file_name = String::from_utf8(name).unwrap();
    let file_path = self.temp_dir.path().join(Path::new(&file_name)); 
    let mut f = match std::fs::File::create(&file_path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };
    f.write_all(&data);
  }
}


  // impl Storage for MyStorage {
  //   fn get(&self, file_path: Vec<u8>) -> Vec<u8> {
  //   let file_path = args.arg_file_path.unwrap();
  //   let path = Path::new(file_path); 
  //   let mut f = match std::fs::File::open(&path) {
  //       // The `desc` field of `IoError` is a string that describes the error
  //       Err(why) => panic!("couldn't open: {}", why.description()),
  //       Ok(file) => file,
  //   };
  //   let mut s = String::new();
  //   match f.read_to_string(&mut s){
  //       Err(why) => panic!("couldn't read: {}", why.description()),
  //       Ok(_) => print!("contains:\n{}", s),
  //   }
  //   s.into_bytes()
  //  }
  
  //  fn put(&mut self, file_path: Vec<u8>, data: Vec<u8>) {
  //   let file_path = args.arg_file_path.unwrap();
  //   let path = Path::new(file_path);
  //   let mut f = match std::fs::File::create(&path) {
  //       // The `desc` field of `IoError` is a string that describes the error
  //       Err(why) => panic!("couldn't open: {}", why.description()),
  //       Ok(file) => file,
  //   };
  //   f.write_all(&data);
  //    }
  //  }
   

 }     

    
     

  