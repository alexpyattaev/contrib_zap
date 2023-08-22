use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
//use support::TokioIo;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
//#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    while let Ok((connection, _)) = listener.accept().await {
        tokio::spawn(handle_connection(connection));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    //stream.set_nodelay(true).unwrap();

    let mut buffer = [0; 1024];
    let nbytes = stream.read(&mut buffer).await.unwrap();
    if nbytes == 0 {
        return;
    }

    let status_line = "HTTP/1.1 200 OK";

    let contents = "HELLO from RUST!";

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).await.unwrap();
}
