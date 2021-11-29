use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::str;

fn bufferize(s:&str,l:usize) -> Vec<u8>{ //changes any less than 1024-byte string into a uniformly-sized vector.
    let mut st:Vec<u8> = s.as_bytes().to_vec();
    let mut barr:Vec<u8> = vec![0;l-s.len()];
    st.append(&mut barr);
    return st;
}

/* #[tokio::main]
pub async fn mocap_bind(ip:&str) -> Result<u8,std::io::Error>{
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
     println!("connecting to {}:49986",ip.to_string());
    let listener = TcpListener::bind("0:49986").await?; //bind local ip defined in config to port 49986
    println!("Listening on: {}", format!("{}:49986",ip.to_string())); //gives feedback... ideally. as i said, nothing happens.

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?; //create socket

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let buf:Vec<u8> = bufferize("iFacialMocap_sahuasouryya9218sauhuiayeta91555dy3719|sendDataVersion=v2",1024); //this is the initial data request blob

            // In a loop, read data from the socket and write the data back.
            loop {
                

                socket
                    .write_all(&buf[0..1024]) //write request blob to stream, but it never gets this far
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
 */


pub fn mocap_bind(sd:std::sync::mpsc::Sender<String>,ip:String) -> std::io::Result<()> {
    let mode = "udp";
    if mode == "udp"{
        let sock = UdpSocket::bind("0.0.0.0:49983")?;
        let buf = b"iFacialMocap_sahuasouryya9218sauhuiayeta91555dy3719|sendDataVersion=v2";
        sock.send_to(buf, format!("{}:49983",ip))?;
        loop{
            //let mut d:&[u8] = &mut [];
                let mut d = [0;1024];
                let _ = sock.recv_from(&mut d)?;
                let _ = sd.send(String::from(str::from_utf8(&d).unwrap()));
            }
        //Ok(())
    }else{
        println!("{:#?}",sd);
        println!("{}",format!("connecting to {}:49986",ip));
        let mut stream = TcpStream::connect(format!("{}:49983",ip))?;
        println!("{}",format!("connected to {}:49986",ip));
        stream.write(b"iFacialMocap_UDPTCP_sahuasouryya9218sauhuiayeta91555dy3719|sendDataVersion=v2")?;
        println!("sent");
        loop{
        //let mut d:&[u8] = &mut [];
            let mut d = String::from("");
            let _ = stream.read_to_string(&mut d)?;
            let _ = sd.send(d);
        
        }
    }
    
    
    
} // the stream is closed here