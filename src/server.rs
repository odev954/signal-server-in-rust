#[path = "utils.rs"] mod utils;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;

use std::net::TcpListener;
use std::net::TcpStream;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::fs;
use std::io::prelude::*;
use lazy_static::lazy_static;

/* message codes */
const LOGIN : i32 = 200;
const SEV_UPDATE_M : i32 = 101;
const CLI_UPDATE_M : i32 = 204;

/* other constants */
const POS_USERNAME : usize = 2;
const CLI_UPDATE_M_SIZE : usize = 5;
const CLI_MSG_POS : usize = 4;

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
                    let ip = stream.local_addr().unwrap().ip().to_string();
                    match std::panic::catch_unwind(|| { 
                        client_handler(stream); 
                    }) 
                    {
                        Ok(_) => { }
                        Err(_) => {
                            println!("Connection with '{}' was closed.", ip);
                        }
                    }
                });
            }
            Err(_) => { /* connection failed */ }
        }        
    }

    Ok(())
}

fn client_handler(stream : TcpStream)
{
    let result = login(stream.try_clone().expect("failed to reffrence TCP stream"));
    let mut partner : String = String::new();
    let mut stop : bool = false;

    match result {
        Ok(status) => { 
            if status.1
            {
                while !stop
                {
                    match send_server_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone(), partner.clone())
                    {
                        Ok(_) => { }
                        Err(e) => { 
                            stop = true; 
                            println!("{}", e); 
                        }
                    }
                    match recv_client_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone())
                    {
                        Ok(res) => { 
                            partner = res; 
                        }
                        Err(e) => { 
                            stop = true; 
                            println!("{}", e); 
                        }
                    }
                    thread::sleep(std::time::Duration::from_millis(200));
                }
                (*USERS.lock().unwrap()).remove(&status.0.clone());
            }
        }
        Err(_) => {  }
    }
}

fn login(stream : TcpStream) -> std::io::Result<(String, bool)>
{
    let result = utils::get_request_args(stream, true);
    let is_login_msg : bool;
    let username : String;

    match result {
        Ok(args) => {
            username = args[POS_USERNAME].clone();
            is_login_msg = args[0].parse::<i32>().unwrap() == LOGIN;
            
            if is_login_msg
            {
                if !((*USERS.lock().unwrap()).contains(&username))
                {
                    println!("New user logged in :: '{}'", username);
                    (*USERS.lock().unwrap()).insert(username.clone());
                    Ok((username, is_login_msg))
                }
                else
                {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, format!("User '{}' is already logged in!", username)))
                }
            }
            else
            {
                Err(std::io::Error::new(std::io::ErrorKind::Other, format!("User '{}' is already logged in!", username)))
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}

fn recv_client_update(stream : TcpStream, sender : String) -> Result<String, std::io::Error>
{
    let result = utils::get_request_args(stream, false);
    
    match result {
        Ok(args) => {
            println!("args: {}", args.join(" | "));
            if args[0].parse::<i32>().unwrap() == CLI_UPDATE_M && args.len() == CLI_UPDATE_M_SIZE
            {
                if args[CLI_MSG_POS].len() > 0
                {
                    (*MESSAGES.write().unwrap()).push_back(format!("{}&{}&{}", sender, args[POS_USERNAME], args[CLI_MSG_POS]))
                }
                Ok(args[POS_USERNAME].to_string())
            }
            else
            {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid client update message!"))
            }
        }
        Err(e) => { Err(e) }
    }
    
}

fn send_server_update(mut stream : TcpStream, user : String, partner : String) -> Result<(), std::io::Error>
{    
    let online_users = (*USERS.lock().unwrap()).clone().into_iter().collect::<Vec<String>>().join("&");
    match get_chat_filename(user, partner.clone()) {
        Ok(fname) => {
            if fname.len() > 0
            {
                match read_chat_file(fname.clone()) {
                    Ok(data) => {
                        match stream.write(
                            utils::format_server_update(
                                SEV_UPDATE_M, 
                                data,
                                partner,
                                online_users
                            ).as_bytes())
                        {
                            Ok(_) => {
                                Ok(())
                            }
                            Err(e) => {
                                Err(e)
                            }
                        }
                    }
                    Err(_) => {
                        match fs::File::create(fname)
                        {
                            Ok(_) => {
                                send_default_server_update(stream, online_users.clone())
                            }
                            Err(e) => {
                                Err(e)
                            }
                        }
                    }
                }
            }
            else 
            {

                Err(std::io::Error::new(std::io::ErrorKind::Other, "Chat filename was empty!"))
            }
        }
        Err(_) => { 
            send_default_server_update(stream, online_users.clone())
        }
    }
}

fn send_default_server_update(mut stream : TcpStream, online_users : String) -> Result<(), std::io::Error>
{
    match stream.write(
        utils::format_server_update(
            SEV_UPDATE_M, 
            String::new(),
            String::new(),
            online_users
        ).as_bytes())
    {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

fn message_handler()
{
    let mut is_empty : bool;
    loop
    {
        {
            let r = MESSAGES.read().unwrap();
            is_empty = (*r).is_empty();
        }
        while !is_empty
        {
            {
                let r = MESSAGES.read().unwrap();
                let fields : Vec<&str> = (*r).front().expect("").split('&').collect();
            
                match get_chat_filename(fields[0].to_string(), fields[1].to_string())
                {
                    Ok(fname) => { 
                        match update_chat_file(fname, fields[0].to_string(), fields[2].to_string())
                        {
                            Ok(_) => { /* do nothing */ }
                            Err(_) => { /* do nothing */ }
                        }
                    }
                    Err(_) => { /* do nothing */ }
                }
            }

            {
                (*MESSAGES.write().unwrap()).pop_front();
            }
        }
    }
}

fn get_chat_filename(user : String, sender : String) -> std::io::Result<String>
{
    let result : std::io::Result<String>;
    if user != "" && sender != ""
    {
        if user <= sender
        {
            result = Ok(format!("{}&{}.txt", user, sender));
        }
        else
        {
            result = Ok(format!("{}&{}.txt", sender, user));
        }
    }
    else 
    {
        result = Err(std::io::Error::new(std::io::ErrorKind::Other, "Empty usernames!"));
    }
    result
}

fn update_chat_file(fname : String, sender : String, data : String) -> std::io::Result<()>
{
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(fname)
        .unwrap();

    let data : String = format!("&MAGSH_MESSAGE&&Author&{}&DATA&{}", sender, data);
    match file.write_all(data.as_bytes())
    {
        Ok(_) => { Ok(()) }
        Err(e) => { Err(e) }
    }
}

fn read_chat_file(fname : String) -> std::io::Result<String>
{
    fs::read_to_string(fname)
}