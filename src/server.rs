mod utils;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::VecDeque;
use std::collections::HashSet;

struct Server
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
    
    fn handle_client(stream : TcpStream)
    {
        
    }
}