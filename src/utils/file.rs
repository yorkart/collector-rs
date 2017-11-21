
use std::io;
use std::fmt;
use std::fmt::Formatter;
use std::error;
use std::fs::File;
use std::io::Read;
use std::io::Seek;

use bytes::BytesMut;
use bytes::BufMut;

#[cfg(not(target_os = "windows"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatOS {
    pub inode: u64,
    pub device: u64,
}

#[cfg(not(target_os = "windows"))]
pub fn get_file_stat(path :&str) -> Result<StatOS, FSError> {
    use std::os::linux::fs::MetadataExt;

    let file = File::open(path.to_owned())?;
    let metadata = file.metadata()?;

    let stat_os = StatOS{
        inode: metadata.st_ino() as u64,
        device: metadata.st_dev(),
    };

    Ok(stat_os)
}

#[cfg(target_os = "windows")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatOS {
    pub idx_hi: u64,
    pub idx_lo: u64,
    pub vol: u64,
}

#[cfg(target_os = "windows")]
pub fn get_file_stat(path :&str) -> Result<StatOS, FSError> {
    use std::os::windows::fs::MetadataExt;

    let file = File::open(path.to_owned())?;
    let metadata = file.metadata()?;

    let stat_os = StatOS{
        inode: metadata.st_ino() as u64,
        device: metadata.st_dev(),
    };

    Ok(stat_os)
}


pub struct Reader {
    file : File,
}

impl Reader {
    pub fn new(path :&str) -> Result<Reader, FSError> {
        let file = File::open(path.to_owned())?;
        Ok(Reader{file})
    }

    pub fn watch(&self) {
        loop {

        }
    }


    pub fn read_chunk(&mut self, size: usize, byte_mut: &mut BytesMut) -> Option<usize> {
        let len = self.file.metadata().unwrap().len();
        println!("file len : {}", len);
        let mut buffer = Vec::new();
        unsafe {
            buffer.reserve(size);
            buffer.set_len(size);
        }

//
//        let mut buffer = [0; 1024];
        match self.file.read(&mut buffer[..size]) {
            Ok(0) => {
                return None
            }
            Ok(len) => {
//                println!("read len {} / {}", len, buffer.len());

                byte_mut.reserve(size);
                byte_mut.put_slice(&buffer[..len]);
                //self.file.seek(io::SeekFrom::Current(0)).unwrap();

                Some(len)
            },
            Err(e) => {
                println!("error {:?}", e);
                return None
            },
        }
    }
}

#[derive(Debug)]
pub enum FSError {
    Io(io::Error),
}

impl fmt::Display for FSError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            FSError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for FSError {
    fn description(&self) -> &str {
        match *self {
            FSError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            FSError::Io(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for FSError {
    fn from(e: io::Error) -> Self {
        FSError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    pub fn get_file_stat_test() {
        let t = get_file_stat("/data/workspace/rust/collector-rs/log/collector-rs.log");
        match t {
            Ok(stat_os) => println!("{:?}", stat_os),
            Err(e) => println!("error {:?}", e)
        }

        let mut reader = Reader::new("/data/workspace/rust/collector-rs/log/collector-rs.log").unwrap();

        let mut bs = BytesMut::new();

        loop {
            match reader.read_chunk(104, &mut bs) {
                Some(_) => {
                    let len = bs.len();
                    let a = bs.split_to(len);

                    for x in a {
                        print!("{}", x as char);
                    }
//                    println!(" ");
                },
                None => {
                    thread::sleep_ms(1000);
                },
            }
        }

        println!("END");
    }
}