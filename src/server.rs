#[path = "utils.rs"] mod utils;
use std::thread;
use std::sync::{Arc, Mutex};
//use std::time::Duration;
use std::sync::RwLock;

use std::net::TcpListener;
use std::net::TcpStream;

use std::collections::VecDeque;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static!{
    static ref MESSAGES : Arc<RwLock<VecDeque<String>>> =  Arc::new(RwLock::new(VecDeque::new()));
    static ref USERS : Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
}

pub fn start() ->  std::io::Result<()>
{
    let listener = TcpListener::bind("127.0.0.1:8826")?;

    //create thread for message handler
    thread::spawn(message_handler);

    println!("The server is currently running on <127.0.0.1:8826>");

    //listen to incoming connections
    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) => {
                //create thread for client
                thread::spawn(move || {
                    client_handler(stream);
                });
            }
            Err(_) => { /* connection failed */ }
        }        
    }

    Ok(())
}

fn client_handler(stream : TcpStream)
{
    let status : (String, bool) = login(stream.try_clone().expect("failed to reffrence TCP stream"));
    let mut partner : String = String::new();
    let mut stop : bool = false;

    if status.1
    {
        while !stop
        {
            match send_server_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone(), partner.clone())
            {
                Ok(_) => { }
                Err(_) => { stop = true }
            }
            match recv_client_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone())
            {
                Ok(res) => { partner = res }
                Err(_) => { stop = true }
            }
        }
        (*USERS.lock().unwrap()).remove(&status.0.clone());
    }
}

fn login(stream : TcpStream) -> (String, bool)
{
    (String::new(), false)
}

fn recv_client_update(stream : TcpStream, sender : String) -> Result<String, std::io::Error>
{
    Ok(String::new())
}

fn send_server_update(stream : TcpStream, user : String, partner : String) -> Result<(), std::io::Error>
{
    Ok(())
}

fn message_handler()
{
    loop
    {
        while !MESSAGES.read().unwrap().is_empty()
        {
            let mut w = MESSAGES.write().unwrap();
            let r = MESSAGES.read().unwrap();
            let fields : Vec<&str> = (*r).front().expect("msg: &str").split('&').collect();
            
            update_chat_file(create_chat_file(fields[0].to_string(), fields[1].to_string()), fields[0].to_string(), fields.join("&"));
            (*w).pop_front();
        }

    }
}

fn create_chat_file(user1 : String, user2 : String) -> String
{
    String::new()
}

fn update_chat_file(fname : String, sender : String, data : String)
{
    
}

fn read_chat_file(fname : String) -> String
{
    String::new()
}