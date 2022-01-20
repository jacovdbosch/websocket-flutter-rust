#![allow(dead_code)]
use futures::StreamExt;
use std::{io::Result, sync::Mutex};
use tokio::net::{TcpListener, TcpStream};

use lazy_static::lazy_static;

lazy_static! {
    static ref CONNECTION_COUNT: Mutex<u32> = Mutex::new(0);
}

#[tokio::main]
async fn main() -> Result<()> {
    let socket = TcpListener::bind("localhost:8080").await?;

    while let Ok((stream, _)) = socket.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

fn increase_connection_count() {
    let mut guard = CONNECTION_COUNT.lock().unwrap();
    *guard += 1;
}

fn decrease_connection_count() {
    let mut guard = CONNECTION_COUNT.lock().unwrap();
    *guard -= 1;
}

fn print_connection_count() {
    let guard = CONNECTION_COUNT.lock().unwrap();
    println!("{}", *guard);
}

async fn handle_connection(stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("addr: {}", addr);

    increase_connection_count();

    let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        print_connection_count();

        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                decrease_connection_count();
                break;
            }
        };

        if msg.is_close() {
            decrease_connection_count();
            break;
        }

        if msg.is_text() {
            let message_data = msg.into_text().unwrap();
            println!("{}", message_data);
        }
    }
}
