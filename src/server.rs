// src/server.rs
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use simplelog::*;

fn handle_client(mut stream: TcpStream) {

    let mut buffer = [0, 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Log recebido: {}", String::from_utf8_lossy(&buffer));
            //TODO() logica de tratar e guardar os dados
        }
        Err(e) => eprintln!("Erro ao ler dados: {}", e)
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Erro ao aceitar conexão: {}", e),
        }
    }
}