
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use regex::Regex;
use std::thread;
use std::time::Duration;

fn add(num1: i32, num2: i32, num3: i32) -> i32 {
    num1 + num2 + num3
}

fn product(num1: i32, num2: i32, num3: i32) -> i32 {
    num1 * num2 * num3
}

fn fac(num1: i32, num2: i32, num3: i32) -> i32 {
    let tmp: i32 = num1 + num2 + num3;
    let mut result = 1;

    for i in 1..=tmp {
        result *= i;
    }

    result
}

fn handle_client(mut stream: TcpStream) -> u8 {
    let mut buffer = [0; 1024];
    let re = Regex::new(r"<SYN>([A-Z]),(\d)(\d)(\d)<STX>").unwrap();

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                
                if let Some(captures) = re.captures(message.as_ref()) {
                    let letter = captures.get(1).map_or("", |m| m.as_str());

                    let n1 = captures.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                    let n2 = captures.get(3).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                    let n3 = captures.get(4).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                    
                    let result: String;

                    match letter {
                        "A" => {
                            result = add(n1, n2, n3).to_string();
                        },
                        "B" => {
                            result = product(n1, n2, n3).to_string();
                        },
                        "C" => {
                            result = fac(n1, n2, n3).to_string();
                        },
                        "D" => {
                            for _ in 1..=3 {
                                let duration = Duration::from_secs(1);
                                thread::sleep(duration);
                            }
                            result = String::from("Retardo de mensaje");
                        },
                        _ => {
                            result = String::from("Comando no encontrado");
                        }
                    }

                    let message = format!("{},<ACK>,<SYN>{},{}{}{}<STX>", result, letter, n1, n2, n3);
                    stream.write_all(message.as_bytes()).expect("Error with client message");

                } else {
                    let error_message = format!("Error,<NAK>,{}", message);
                    stream.write_all(error_message.as_bytes()).expect("Error with client message");
                }
            },
            Err(_) => {
                return 0;
            }
        }
    }

    1
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").expect("Error with the socket vinculation");
    println!("Server listening on: 12345");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(err) => {
                eprintln!("Error with the connection: {}", err);
            }
        }
    }
}
