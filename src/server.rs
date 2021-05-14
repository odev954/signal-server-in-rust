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
    let status : (String, bool) = login(stream.try_clone().expect("failed to reffrence TCP stream"));
    let mut partner : String = String::new();
    let mut stop : bool = false;

    if status.1
    {
        while !stop
        {
            match send_server_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone(), partner.clone())
            {
                Ok(_) => { println!("sent update!"); }
                Err(e) => { stop = true; eprintln!("{}", e); }
            }
            match recv_client_update(stream.try_clone().expect("failed to reffrence TCP stream"), status.0.clone())
            {
                Ok(res) => { partner = res; println!("recved update!"); }
                Err(e) => { stop = true; eprintln!("{}", e); }
            }
            //thread::sleep(std::time::Duration::from_millis(200));
        }
        (*USERS.lock().unwrap()).remove(&status.0.clone());
    }
}

fn login(stream : TcpStream) -> (String, bool)
{
    let args : Vec<String> = utils::get_request_args(stream, true);
    let is_logged : bool = args[0].parse::<i32>().unwrap() == LOGIN;
    let mut username : String = String::new();

    if is_logged
    {
        username = args[POS_USERNAME].clone();

        if !((*USERS.lock().unwrap()).contains(&username))
        {
            println!("New user logged in :: '{}'", username);
            (*USERS.lock().unwrap()).insert(username.clone());
        }
        else
        {
            panic!("User '{}' is already logged in!", username);
        }
    }

    (username, is_logged)
}

fn recv_client_update(stream : TcpStream, sender : String) -> Result<String, std::io::Error>
{
    let args = utils::get_request_args(stream, false);
    
    println!("args: {}", args.join(" | "));
    if args[0].parse::<i32>().unwrap() == CLI_UPDATE_M && args.len() == CLI_UPDATE_M_SIZE
    {
        if args[CLI_MSG_POS].len() > 0
        {
            let mut w = MESSAGES.write().unwrap();
            (*w).push_back(format!("{}&{}&{}", sender, args[POS_USERNAME], args[CLI_MSG_POS]))
        }
        Ok(args[POS_USERNAME].to_string())
    }
    else
    {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid client update message!"))
    }
}

fn send_server_update(mut stream : TcpStream, user : String, partner : String) -> Result<(), std::io::Error>
{    
    let online_users = (*USERS.lock().unwrap()).clone().into_iter().collect::<Vec<String>>().join("&");
    println!("sending update to {}", user);
    match get_chat_filename(user, partner) {
        Ok((fname, partner)) => {
            println!("success 1");
            if fname.len() > 0
            {
                match read_chat_file(fname) {
                    Ok(data) => {
                        println!("success 2");
                        match stream.write(
                            utils::format_server_update(
                                SEV_UPDATE_M, 
                                data,
                                partner,
                                online_users
                            ).as_bytes())
                        {
                            Ok(_) => {
                                println!("success 3");
                                Ok(())
                            }
                            Err(e) => {
                                println!("failed 3");
                                Err(e)
                            }
                        }
                    }
                    Err(e) => { 
                        println!("failed 2");                                
                        Err(e)
                    }
                }
            }
            else 
            {

                Err(std::io::Error::new(std::io::ErrorKind::Other, "Chat filename was empty!"))
            }
        }
        Err(_) => { 
            match stream.write(
                utils::format_server_update(
                    SEV_UPDATE_M, 
                    String::new(),
                    String::new(),
                    online_users
                ).as_bytes())
            {
                Ok(_) => {
                    println!("success 4");
                    Ok(())
                }
                Err(e) => {
                    println!("failed 4");
                    Err(e)
                }
            }
        }
    }
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

            match get_chat_filename(fields[0].to_string(), fields[1].to_string())
            {
                Ok((fname, _)) => { update_chat_file(fname, fields[0].to_string(), fields.join("&")); }
                Err(_) => { /* do nothing */ }
            }
            
            (*w).pop_front();
        }

    }
}

fn get_chat_filename(user : String, sender : String) -> std::io::Result<(String, String)>
{
    let result : std::io::Result<(String, String)>;
    if user != "" && sender != ""
    {
        if user >= sender
        {
            result = Ok((format!("{}&{}.txt", user, sender), sender));
        }
        else
        {
            result = Ok((format!("{}&{}.txt", sender, user), sender));
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