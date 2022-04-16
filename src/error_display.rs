use std::io;
use std::io::ErrorKind;
use std::process::exit;

pub fn no_file_name_given_error() -> ! {
    println!("no filename given");
    exit(1);
}

pub fn file_open_error(err: io::Error, file_name: String) -> ! {
    println!(
        "error opening file '{}': {}",
        file_name,
        match err.kind() {
            ErrorKind::NotFound => "not found",
            ErrorKind::PermissionDenied => "permission denied",
            _ => "unhandled errorkind",
        }
    );
    exit(1);
}

pub fn buffer_fill_error(err: io::Error) -> ! {
    println!("error filling buffer: {}", err);
    exit(1);
}
