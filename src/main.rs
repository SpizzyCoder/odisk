use std::env;
use std::fs::File;
use std::io::{Write,stdin,stdout};
use std::path::Path;

const PRINT_LINES: u32 = 11;

enum Mode {
  Zero,
  Random,
}

// arg1: Path to disk (/dev/sdc)
// arg2: Mode [z,r]
// arg3: The unit [b,k,m,g,t]
// arg4: MiB chunk size
fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 5 {
    println!["========================="];
    println!["{} v{}",env!["CARGO_PKG_NAME"],env!["CARGO_PKG_VERSION"]];
    println!["Syntax: {} <disk> <mode> <unit> <chunksize>",env!["CARGO_PKG_NAME"]];
    println!["Example: {} /dev/sdc r M 4",env!["CARGO_PKG_NAME"]];
    println![];
    println!["Note:"];
    println!["The <chunksize> * <unit> will be the allocated memory"];
    println!["The result must not be larger than the physical RAM you have available"];
    println!["=========="];
    println!["<disk>: The disk to overwrite (/dev/sdb)"];
    println!["<mode>: 'z' -> Overwrite with zeroes"];
    println!["        'r' -> Overwrite with random data"];
    println!["<unit>: The unit in which the disk should be overwritten"];
    println!["        'b' -> Bytes"];
    println!["        'B' -> Bytes"];
    println!["        'k' -> Kilobytes"];
    println!["        'K' -> Kibibytes"];
    println!["        'm' -> Megabytes"];
    println!["        'M' -> Mibibytes"];
    println!["        'g' -> Gigabytes"];
    println!["        'G' -> Gibibytes"];
    println!["        't' -> Terabytes"];
    println!["        'T' -> Tibibytes"];
    println!["<chunksize: The chunksize in which the disk should be overwritten"];
    println!["========================="];
    return
  }

  let diskpath: &str = &args[1];
  let mode: &str = &args[2];
  let unit: &str = &args[3];
  let mut chunksize: usize = match &args[4].parse() {
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

  let mode_enum: Mode = match mode {
    "z" => Mode::Zero,
    "r" => Mode::Random,
    _ => {
      eprintln!["Invalid mode {}",mode];
      return
    }
  };

  if !Path::new(diskpath).exists() {
    eprintln!["{} doesn't exist",diskpath];
    return
  }

  if !user_confirmation(diskpath) {
    return
  }

  let mut disk: File = match File::create(diskpath) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",diskpath,error];
      return
    }
  };

  overwrite(chunksize,&mut disk,mode_enum);
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

fn overwrite(chunksize: usize,disk: &mut File,mode: Mode) {
  let mut written_bytes_total: usize = 0;
  let mut memory: Vec<u8> = vec![0;chunksize];

  println![];

  loop {
    // Generate random bytes if the user wants to write with random bytes
    if matches![mode,Mode::Random] {
      if let Err(error) = getrandom::getrandom(&mut memory) {
        panic!["Failed to generate random bytes [Error: {}]",error];
      }
    }

    let written_bytes = match disk.write(&memory) {
      Ok(written_bytes) => written_bytes,
      Err(error) => {
        if error.kind() != std::io::ErrorKind::Interrupted {
          panic!["Failed to write [Error: {}]",error];
        }
        continue
      }
    };

    written_bytes_total += written_bytes;

    disk.sync_data().unwrap();
    print_status(written_bytes_total);

    if written_bytes != chunksize {
      break;
    }
  }

  for _ in 0..(PRINT_LINES + 1) {
    println![];
  }
}