use std::fs::File;
use std::io::Error;

trait FileReader {
    fn read_file(&self, filename: String) -> Result<u8, Error>;
    fn file_exists(&self, filename: String) -> Result<bool, Error>;
}

trait FileWriter {
    fn write_file(&self, filename: String, data: u8) -> Result<(), Error>;
    fn update_file_name(&self, filename: String) -> Result<String, Error>;
}

impl FileWriter for WriteAccess {
    fn write_file(&self, filename: String, data: u8) -> Result<(), Error> {
        todo!()
    }

    fn update_file_name(&self, filename: String) -> Result<String, Error> {
        todo!()
    }
}

impl FileReader for ReadAccess {
    fn read_file(&self, filename: String) -> Result<u8, Error> {
        todo!()
    }

    fn file_exists(&self, filename: String) -> Result<bool, Error> {
        todo!()
    }
}

type ReadAccess = ();
type WriteAccess = ();

type FullAccess = (ReadAccess, WriteAccess);

struct FileAudit{}

struct FileManagement{}

struct Member {
    file: FileAudit,
}

struct Admin {
}

fn main() {
    println!("Hello, world!");
}
