use futures::Future;
use libp2p::Transport;
use libp2p_websocket::WsConfig;
use tokio_core::reactor::Core;
use tokio_tcp::TcpListener;

struct TcpConfig {
    handle: Core,
    pub listener: TcpListener,
}

impl TcpConfig {
    fn new(handle: Core) -> TcpConfig {
        TcpConfig {
            handle: handle,
            listener: TcpListener::bind(&"127.0.0.1:0".parse().unwrap()).unwrap(),
        }
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpConfig::new(core(Box<Future<Item = (), Error = ()> Send> {
        let listener = TcpListener::bind(&"127.0.0.1:0".parse().unwrap()).unwrap();
        Box::new(listener.incoming().for_each(|_| Ok(())))
    }));

    let ws = WsConfig::new().listen_on(tcp.listener.local_addr().unwrap(), handle.clone());
    let transport = TcpConfig::new(handle.clone()).handle.handle(ws);
    core.run(transport).unwrap();

    println!("Listening on {}", tcp.listener.local_addr().unwrap());

    // TODO: signal to exit
    core.run(handle).unwrap();

    println!("Bye!");

}
