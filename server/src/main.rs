use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use data_generator::Data;
use server::Thermometer;

#[tokio::main]
async fn main() {
    let source_data_addr = "127.0.0.1:5555";

    let server_addr: SocketAddr = "127.0.0.1:5556".parse().unwrap();
    let listener = TcpListener::bind(&server_addr).await.unwrap();

    //let (sender, _) = mpsc::channel(100);

    let mut thermometer = Thermometer::new();

    loop {
        let mut source_data_stream = TcpStream::connect(source_data_addr).await.unwrap();

        println!("Connected to source data stream");

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            match source_data_stream.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => {
                    println!("Received data from source data stream");
                    let recieved_data = &buf[..n];
                    let new_data: Data = serde_json::from_slice(recieved_data).unwrap();
                    thermometer.update(&new_data);
                    println!("Data received from source data: {:?}", new_data);
                }
                Err(err) => panic!("Error reading from source data stream: {}", err),
            };
        });

        match listener.accept().await {
            Ok((mut stream, _)) => {
                println!("Sending data to client stream");
                tokio::spawn(async move {
                    let _ = stream.write_all(thermometer.clone().get_data().as_bytes());
                });
            }
            Err(e) => {
                println!("Error listening on socket {}", e);
            }
        }
    }
}
