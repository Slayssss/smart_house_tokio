use data_generator::Data;
use serde_json;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Data::new();

    let listener = TcpListener::bind("127.0.0.1:5555")
        .await
        .expect("Problem with TCP listener");

    loop {
        let (mut stream, _) = listener.accept().await.expect("Problem accepting");

        tokio::spawn(async move {
            let mut interval = interval(tokio::time::Duration::from_secs(5));

            loop {
                interval.tick().await;

                let serialized_data = serde_json::to_string(&data.get_new_value()).unwrap();

                stream
                    .write_all(serialized_data.as_bytes())
                    .await
                    .expect("Problem with transport data to server");
            }
        });
    }
}
