use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
fn main() {
    let counter = 5;
    let handle = tokio::spawn(async {
        counter+=1;
        "return value"
    });

    // Do some other work

    // let out = handle.await.unwrap();
    println!("GOT {}", counter);
}