use std::fs::File;
use std::io::{Write,stdin,stdout};
use std::path::Path;

use clap::Parser;
use clap::ArgEnum;

const PRINT_LINES: u32 = 11;

#[derive(ArgEnum,Clone,Copy,Debug)]
enum Mode {
  Zero,
  Random,
}

#[derive(ArgEnum,Clone,Copy,Debug)]
enum Unit {
  B,
  Kb,
  Kib,
  Mb,
  Mib,
  Gb,
  Gib,
  Tb,
  Tib,
}

/// Simple program to overwrite disks
#[derive(Parser,Debug)]
#[clap(version,about)]
pub struct Args {
  /// Chunksize
  #[clap(short,long,default_value_t = 1)]
  chunksize: usize,

  /// Unit
  #[clap(short,long,arg_enum,default_value_t = Unit::B)]
  unit: Unit,

  /// Mode
  #[clap(short,long,arg_enum,default_value_t = Mode::Zero)]
  mode: Mode,

  /// Path
  #[clap()]
  path: String,
}

fn main() {
  let args: Args = Args::parse();
  let diskpath: &Path = Path::new(&args.path);

  if !diskpath.exists() {
    eprintln!["{} doesn't exist",diskpath.display()];
    return
  }

  if args.chunksize < 1 {
    eprintln!["Chunksize can't be 0 or negative"];
    return
  }

  if !user_confirmation(&args.path) {
    return
  }

  let write_block: usize = match args.unit {
    Unit::B   => args.chunksize * 1,
    Unit::Kb  => args.chunksize * 1000_usize,
    Unit::Kib => args.chunksize * 1024_usize,
    Unit::Mb  => args.chunksize * 1000_usize.pow(2),
    Unit::Mib => args.chunksize * 1024_usize.pow(2),
    Unit::Gb  => args.chunksize * 1000_usize.pow(3),
    Unit::Gib => args.chunksize * 1024_usize.pow(3),
    Unit::Tb  => args.chunksize * 1000_usize.pow(4),
    Unit::Tib => args.chunksize * 1024_usize.pow(4)
  };

  let mut disk: File = match File::create(diskpath) {
    Ok(file) => file,
    Err(error) => {
      eprintln!["Couldn't open {} [Error: {}]",diskpath.display(),error];
      return
    }
  };

  overwrite(write_block,&mut disk,args.mode);
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