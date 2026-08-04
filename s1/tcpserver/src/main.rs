use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];

        // ✅ 处理实际读取字节数
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            // 连接关闭
            continue;
        }

        // 只写回实际读取到的部分
        stream.write_all(&buffer[..bytes_read]).unwrap();
    }
}

