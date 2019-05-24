use ktrace;

fn child() {
    println!("hello, world");
}

fn main() {
    let (sender, receiver) = crossbeam::channel::unbounded();
    let payload = ktrace::Payload::Fn(Box::new(child));
    println!("starting tracer");
    unsafe {
        std::thread::spawn(move || {
            ktrace::run(payload, sender).unwrap();
        });
    }
    println!("listening for events");
    loop {
        let event = match receiver.recv() {
            Ok(x) => x,
            Err(_) => break,
        };
        println!("got {:?}", event);
    }
}
