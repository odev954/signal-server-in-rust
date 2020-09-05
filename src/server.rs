pub mod utils;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::VecDeque;
use std::collections::HashSet;

pub struct Server
{
    _messages : VecDeque<String>,
    _users : HashSet<String>
}

impl Server
{
    fn new() -> Server
    {
        Server
        {
            _messages : VecDeque::<String>::new(),
            _users : HashSet::<String>::new()
        }
    }
    
    pub fn start(mut self) ->  std::io::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:8826")?;

        for stream in listener.incoming()
        {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        self.client_handler(stream);
                    });
                }
                Err(e) => { /* connection failed */ }
            }        
        }

        Ok(())
    }

    fn client_handler(&mut self, stream : TcpStream)
    {
        let mut status : (String, bool) = (String::new(), false);
        let mut partner : String = String::new();

        status = self.login(stream);
        
        if status.1
        {
            loop
            {
                self.send_server_update(stream, status.0, partner);
                partner = self.recv_client_update(stream, status.0);
            }
        }
    }

    fn login(&mut self, stream : TcpStream) -> (String, bool)
    {
        let mut args : Vec<String> = get_request_args(stream);

        (String::new(), false)
    }

    fn recv_client_update(&mut self, stream : TcpStream, sender : String) -> String
    {
        String::new()
    }

    fn send_server_update(&mut self, stream : TcpStream, user : String, partner : String)
    {

    }

    fn message_handler(&mut self)
    {

    }

    fn create_chat_file(user1 : String, user2 : String) -> String
    {
        String::new()
    }

    fn update_chat_file(&mut self, fname : String, sender : String, data : String)
    {

    }

    fn read_chat_file(fname : String) -> String
    {
        String::new()
    }
}