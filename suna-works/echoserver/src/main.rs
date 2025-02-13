use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    println!("{}", addr);
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?; // [1] ソケットの作成
    loop {
        let (mut stream, _) = listener.accept()?; // [2] スレッドをブロックし、クライアントからの確立要求を待機
        // スレッドを立ち上げて接続に対処する
        thread::spawn(move || { // [3] 新しいスレッドを生む, moveとあるのでstreamが新しいスレッドへmove
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap(); // [4] スレッドをブロック、データの受信を待機
                println!("{:?}", buffer);
                if nbytes == 0 { // [6] Ctrl+cとか
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap(); // [5] クライアントに対して書き込む
            }
        });
    }
}
