use std::env;
use std::fs::File;
use std::io::{Write,stdin,stdout};
use std::path::Path;

const PRINT_LINES: u32 = 11;

// arg1: Path to disk (/dev/sdc)
// arg2: The unit [b,k,m,g,t]
// arg3: MiB chunk size
fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 4 {
    println!["========================="];
    println!["deletedisk v1.0"];
    println!["Syntax: deletedisk <disk> <Unit> <Chunksize>"];
    println!["Example: deletedisk /dev/sdc M 4"];
    println![];
    println!["Note:"];
    println!["The <Chunksize> * <Unit> will be the allocated memory"];
    println!["The result must not be larger than the physical RAM you have available"];
    println!["========================="];
    return
  }

  let diskpath: &str = &args[1];
  let unit: &str = &args[2];
  let mut chunksize: usize = match &args[3].parse() {
    Ok(chunksize) => *chunksize,
    Err(error) => {
      eprintln!["Couldn't parse string to unsigned int {} [Error: {}]",args[3],error];
      return
    }
  };

  match unit {
    "b" => {},
    "B" => {},
    "k" => chunksize *= 1000_usize,
    "K" => chunksize *= 1024_usize,
    "m" => chunksize *= 1000_usize.pow(2),
    "M" => chunksize *= 1024_usize.pow(2),
    "g" => chunksize *= 1000_usize.pow(3),
    "G" => chunksize *= 1024_usize.pow(3),
    "t" => chunksize *= 1000_usize.pow(4),
    "T" => chunksize *= 1024_usize.pow(4),
    _ => {
      eprintln!["Invalid unit {}",unit];
      return
    }
  };

  if !Path::new(diskpath).exists() {
    eprintln!["{} doesn't exist",diskpath];
    return
  }

  let mut disk: File = match File::create(diskpath) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",diskpath,error];
      return
    }
  };

  if !user_confirmation(diskpath) {
    return
  }

  let mut written_bytes_total: usize = 0;
  let zeroed_memory: Vec<u8> = vec![0;chunksize];

  print!["\n"];

  loop {
    written_bytes_total += match disk.write(&zeroed_memory) {
      Ok(written_bytes) => written_bytes,
      Err(error) => {
        if error.kind() != std::io::ErrorKind::Interrupted {
          break
        }
        0
      }
    };

    disk.sync_data().unwrap();
    print_status(written_bytes_total);
  }

  for _ in 0..PRINT_LINES {
    print!["\n"];
  }

  print!["\n"];
}

fn print_status(bytes: usize) {
  let kib: f64 = bytes as f64 / 1024_f64;
  let mib: f64 = bytes as f64 / 1024_f64.powf(2_f64);
  let gib: f64 = bytes as f64 / 1024_f64.powf(3_f64);
  let tib: f64 = bytes as f64 / 1024_f64.powf(4_f64);
  let kb: f64 = bytes as f64 / 1000_f64;
  let mb: f64 = bytes as f64 / 1000_f64.powf(2_f64);
  let gb: f64 = bytes as f64 / 1000_f64.powf(3_f64);
  let tb: f64 = bytes as f64 / 1000_f64.powf(4_f64);

  // Clear the lines before the print
  for _ in 0..PRINT_LINES {
    println!["\x1B[2K"];
  }
  print!["\r\x1B[{}A",PRINT_LINES]; // Go lines up

  println!["B: {}",bytes];
  println![];
  println!["KiB: {}",kib];
  println!["MiB: {}",mib];
  println!["GiB: {}",gib];
  println!["TiB: {}",tib];
  println![];
  println!["KB: {}",kb];
  println!["MB: {}",mb];
  println!["GB: {}",gb];
  println!["TB: {}",tb];
  print!["\r\x1B[{}A",PRINT_LINES]; // Go lines up
}

fn user_confirmation(disk: &str) -> bool {
  print!["Overwrite {}? [y,n] ",disk];
  stdout().flush().expect("Couldn't flush stdout");

  let mut userinput: String = String::new();
  stdin().read_line(&mut userinput).expect("Coulnd't read from stdin");

  userinput = userinput.replace("\n","");
  userinput = userinput.replace("\r","");

  if userinput == "y" || userinput == "Y" {
    return true
  }

  return false
}
