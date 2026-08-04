use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    // ✅ 使用 write_all，确保数据完全写出
    stream.write_all(b"Hello").unwrap();

    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).unwrap(); // 读固定长度更安全

    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
