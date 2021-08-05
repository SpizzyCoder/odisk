use std::env;
use std::fs::File;
use std::io::{Write,stdin,stdout};
use std::path::Path;

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

  if unit == "b" || unit == "B" {
    // Leave the bytes as they are
  } else if unit == "k" || unit == "K" {
    chunksize *= 1024;
  } else if unit == "m" || unit == "M" {
    chunksize *= 1024 * 1024;
  } else if unit == "g" || unit == "G" {
    chunksize *= 1024 * 1024 * 1024;
  } else if unit == "t" || unit == "T" {
    chunksize *= 1024 * 1024 * 1024 * 1024;
  } else {
    eprintln!["Invalid unit {}",unit];
    return
  }

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

  print!["\n\n\n\n\n"]; // Print 5 newlines to get to the end

	print!["\n"];
}

fn print_status(bytes: usize) {
	let kib: f64 = bytes as f64 / 1024f64;
	let mib: f64 = bytes as f64 / 1048576f64; // 1024 * 1024 (1024^2)
	let gib: f64 = bytes as f64 / 1073741824f64; // 1024 * 1024 * 1024 (1024^3)
	let tib: f64 = bytes as f64 / 1099511627776f64; // 1024 * 1024 * 1024 * 1024 (1024^4)
  println!["  B: {}",bytes];
  println!["KiB: {}",kib];
  println!["MiB: {}",mib];
  println!["GiB: {}",gib];
  println!["TiB: {}",tib];
  print!["\r\x1b[5A"]; // Go five lines up
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
