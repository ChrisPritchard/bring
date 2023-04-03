use std::{net::{TcpListener, TcpStream}, process::exit, env, io::copy, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("args: remote-host remote-port [local-port]");
        exit(1);
    }

    let mut local_port = &args[2];
    if args.len() > 3 {
        local_port = &args[3];
    }
    let listener = TcpListener::bind(format!("0.0.0.0:{}", local_port)).unwrap();

    for stream in listener.incoming() {
        let mut in_stream = stream.unwrap();
        let mut in_stream_rev = in_stream.try_clone().unwrap();

        let mut out_stream = TcpStream::connect(format!("{}:{}", args[1], args[2])).unwrap();
        let mut out_stream_rev = out_stream.try_clone().unwrap();

        let handle1 = thread::spawn(move || {
            copy(&mut in_stream, &mut out_stream).unwrap();
        });

        let handle2 = thread::spawn(move || {
            copy(&mut out_stream_rev, &mut in_stream_rev).unwrap();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}
