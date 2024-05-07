use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut num_threads = 5; // Replace with the desired number of threads
    let mut binary_path = "/usr/local/bin/app";
    let mut quote_dir = ".";

    if args.len() > 3 {
        num_threads = args[1].parse().expect("Invalid number of threads");
        binary_path = &args[2];
        quote_dir = &args[3];
    }

    let binary_path = binary_path.to_string();
    let quote_dir = quote_dir.to_string();
    let mut handles = vec![];
    let start_time = Instant::now();
    for i in 0..num_threads {
        let binary_clone = binary_path.clone();
        let quote_clone = quote_dir.clone();
        let handle = thread::spawn(move || {
            println!("Thread {:?} start", thread::current().id());
            let mut index = i;
            let start_time = Instant::now();
            loop {
                let filename = format!("{}/quote{}.dat", quote_clone, index);
                if Path::new(&filename).exists() {
                    // println!("Thread {:?} - file {}", thread::current().id(), filename);
                    let output = Command::new(&binary_clone)
                        .arg("--quote")
                        .arg(&filename)
                        .stdout(Stdio::null())
                        .output()
                        .expect("Failed to execute command");

                    if output.status.success() {
                        _ = String::from_utf8_lossy(&output.stdout);
                        // println!(
                        //     "Thread {:?} - Output: {}",
                        //     thread::current().id(),
                        //     output_str
                        // );
                    } else {
                        let error_str = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Thread {:?} - Error: {}", thread::current().id(), error_str);
                    }
                    index += num_threads;
                } else {
                    println!(
                        "Thread {:?} - No more files to read after {}",
                        thread::current().id(),
                        filename
                    );
                    break;
                }
            }
            let elapsed_time = start_time.elapsed(); // End timing here
            println!(
                "Thread {:?} - Elapsed time: {:?}",
                thread::current().id(),
                elapsed_time
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join().unwrap_or_else(|_| {
            eprintln!("Thread failed to join.");
        });
    }
    let elapsed_time = start_time.elapsed(); // End timing here
    println!("All Elapsed time: {:?}", elapsed_time);
}
