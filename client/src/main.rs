use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    //let listener = TcpListener::bind("127.0.0.1:5557").await.unwrap();

    let server_addr: SocketAddr = "127.0.0.1:5556".parse().unwrap();

    loop {
        let mut stream = TcpStream::connect(server_addr).await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let _n = match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let recieved_data = &buf[..n];
                        let string_data = String::from_utf8_lossy(recieved_data);
                        println!("{}", string_data);
                    }
                    Err(err) => panic!("Error reading from source data stream: {}", err),
                };
            }
        });
    }
}
