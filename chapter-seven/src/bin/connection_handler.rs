use std::{
    collections::HashMap,
    net::Ipv6Addr,
    sync::{Arc, RwLock, atomic::AtomicUsize},
    thread,
    time::Duration,
};

struct Client {
    ip: Ipv6Addr,
}

struct ConnectionHandler {
    clients: RwLock<HashMap<usize, Client>>,
    next_id: AtomicUsize,
}

impl Client {
    fn new(ip: Ipv6Addr) -> Self {
        Self { ip }
    }
}

impl ConnectionHandler {
    fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            next_id: AtomicUsize::new(0),
        }
    }

    fn client_count(&self) -> usize {
        self.clients
            .read()
            .expect("Failed to lock clients for reading")
            .len()
    }

    fn add_connection(&self, ip: Ipv6Addr) -> usize {
        let last = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.clients
            .write()
            .expect("Failed to lock clients for writing")
            .insert(last, Client::new(ip));
        last
    }

    fn remove_connection(&self, id: usize) -> Option<()> {
        self.clients
            .write()
            .expect("Failed to lock clients for writing")
            .remove(&id)
            .and(Some(()))
    }
}

fn main() {
    let connections = Arc::new(ConnectionHandler::new());

    let connector = {
        let connections = Arc::clone(&connections);
        let dummy_ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
        let ten_millis = Duration::from_millis(10);
        thread::spawn(move || {
            for _ in 0..20 {
                connections.add_connection(dummy_ip);
                thread::sleep(ten_millis);
            }
        })
    };

    let disconnector = {
        let connections = Arc::clone(&connections);
        let fifty_millis = Duration::from_millis(50);
        thread::spawn(move || {
            thread::sleep(fifty_millis);
            connections.remove_connection(2);
        })
    };

    let five_millis = Duration::from_millis(5);
    for _ in 0..40 {
        let count = connections.client_count();
        println!("Active connections: {}", count,);
        thread::sleep(five_millis);
    }

    connector.join().expect("The connector thread panicked");
    disconnector
        .join()
        .expect("The disconnector thread panicked");
}
