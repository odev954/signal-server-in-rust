mod server;

fn main() {
    match server::Server::new().start() {
        Ok(_) => { }
        Err(e) => {
            panic!("The server execution was aborted. cause: {:?}", e);
        }
    }
}
