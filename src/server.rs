#[path = "utils.rs"] mod utils;
use crossbeam::thread;
use std::sync::{Arc, Mutex};
//use std::time::Duration;
use std::sync::RwLock;

use std::net::TcpListener;
use std::net::TcpStream;

use std::collections::VecDeque;
use std::collections::HashSet;

pub struct Server
{
    _messages : RwLock<VecDeque<String>>,
    _users : Arc<Mutex<HashSet<String>>>
}

impl Server
{
    pub fn new() -> Server
    {
        Server
        {
            _messages : RwLock::new(VecDeque::<String>::new()),
            _users : Arc::new(Mutex::new(HashSet::<String>::new()))

        }
    }

    pub fn start(&mut self) ->  std::io::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:8826")?;
        
        //create thread for message handler
        thread::scope(|s| {
            s.spawn(|_| {
                self.message_handler(); //TODO: this thread spawning is blocking the execution...
            });
        }).unwrap();

        println!("The server is currently running on <127.0.0.1:8826>");

        //listen to incoming connections
        for stream in listener.incoming()
        {
            match stream {
                Ok(stream) => {
                    //create thread for client
                    thread::scope(|s| {
                        s.spawn(|_| {
                            self.client_handler(stream);
                        });
                    }).unwrap();
                }
                Err(_) => { /* connection failed */ }
            }        
        }

        Ok(())
    }

    fn client_handler(&mut self, stream : TcpStream)
    {
        let status : (String, bool) = self.login(stream.try_clone().expect("failed to reffrence TCP stream"));
        let mut partner : String = String::new();
        let mut stop : bool = false;

        if status.1
        {
            while !stop
            {
                match self.send_server_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone(), partner.clone())
                {
                    Ok(_) => { }
                    Err(_) => { stop = true }
                }
                match self.recv_client_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone())
                {
                    Ok(res) => { partner = res }
                    Err(_) => { stop = true }
                }
            }
            (*self._users.lock().unwrap()).remove(&status.0.clone());
        }
    }

    fn login(&mut self, stream : TcpStream) -> (String, bool)
    {
        (String::new(), false)
    }

    fn recv_client_update(&mut self, stream : TcpStream, sender : String) -> Result<String, std::io::Error>
    {
        Ok(String::new())
    }

    fn send_server_update(&mut self, stream : TcpStream, user : String, partner : String) -> Result<(), std::io::Error>
    {
        Ok(())
    }

    fn message_handler(&mut self)
    {
        loop
        {
            while !self._messages.read().unwrap().is_empty()
            {
                let r = self._messages.read().unwrap();
                let mut w = self._messages.write().unwrap();
                let fields : Vec<&str> = (*r).front().expect("msg: &str").split('&').collect();
                
                self.update_chat_file(self.create_chat_file(fields[0].to_string(), fields[1].to_string()), fields[0].to_string(), fields.join("&"));
                (*w).pop_front();
            }

        }
    }

    fn create_chat_file(&self, user1 : String, user2 : String) -> String
    {
        String::new()
    }

    fn update_chat_file(&self, fname : String, sender : String, data : String)
    {
        
    }

    fn read_chat_file(fname : String) -> String
    {
        String::new()
    }
}