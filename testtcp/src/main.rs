use std::net::TcpStream;

fn main(){
// Server to connect to
    let mut stream = TcpStream::connect("127.0.0.1:100100")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here