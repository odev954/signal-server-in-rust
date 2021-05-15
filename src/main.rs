mod server;

fn main() {
    match server::start() {
        Ok(_) => { }
        Err(e) => {
            panic!("The server execution was aborted. cause: {:?}", e);
        }
    }
}
