use std::{fmt::Error, io::{BufRead, BufReader, Read, Write}, net, result, usize};
use net::{TcpListener, TcpStream};
// use log::{error, warn, info, debug, trace};

const REQUEST_CHUNK: usize = 2048;

fn main() {
    let listener:TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler(stream);
    }
    println!("Hello, world!");
}

fn read_header(mut buffer: BufReader<&mut TcpStream>) -> String{
    let mut header:String = String::new().to_owned();
    loop {
        let mut cargo:String = String::new();
        let read_status:Result<usize, std::io::Error> = buffer.read_line(&mut cargo);

        match read_status {
            Ok(0) => break,
            Ok(bytes) => println!("HTTP header bytes read: {bytes}"),
            Err(error) => panic!("Issue happen when trying to read the tcpstream: {}", error)
        };

        // End of the header reached
        println!("Parts: ^{cargo}$");
        if cargo == "\r\n"{
            println!("End of header is reached");
            break;
        }
        header.push_str(&cargo);
    }
    return header;
}

fn handler(mut stream: TcpStream){
    let buf_reader:BufReader<&mut _> = BufReader::new(&mut stream);
    let header:String = read_header(buf_reader);
    let format:&str = &header;
    println!("{:?}", format);
}
