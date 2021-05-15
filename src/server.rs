#[path = "utils.rs"] mod utils; //import utility module

/* used modules */
// threads
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;
//net
use std::net::TcpListener;
use std::net::TcpStream;
//data structs
use std::collections::VecDeque;
use std::collections::HashSet;
//file system
use std::fs;
use std::io::prelude::*;
//other
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
    static ref MESSAGES : Arc<RwLock<VecDeque<String>>> =  Arc::new(RwLock::new(VecDeque::new())); //message queue
    static ref USERS : Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new())); //online users list
}

/*
starts the server.
input: none.
output: execution result.
*/
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

/*
handles the client.
input: conversation socket stream.
output: none.
*/
fn client_handler(stream : TcpStream)
{
    let result = login(stream.try_clone().expect("failed to reffrence TCP stream")); //login result
    let mut partner : String = String::new(); //partner name
    let mut stop : bool = false; //stop session flag

    match result {
        Ok(status) => { 
            if status.1 //if logged in
            {
                while !stop
                {
                    //send update and check for errors
                    match send_server_update(stream.try_clone().expect("failed to reference TCP stream"), status.0.clone(), partner.clone())
                    {
                        Ok(_) => { }
                        Err(e) => { 
                            stop = true; 
                            println!("{}", e); 
                        }
                    }
                    //get update and check for errors
                    match recv_client_update(stream.try_clone().expect("failed to reference TCP stream"), status.0.clone())
                    {
                        Ok(res) => { 
                            partner = res; 
                        }
                        Err(e) => { 
                            stop = true; 
                            println!("{}", e); 
                        }
                    }
                    thread::sleep(std::time::Duration::from_millis(200)); //wait 200ms
                }
                
                {
                    (*USERS.lock().unwrap()).remove(&status.0.clone());
                }
            }
        }
        Err(_) => {  }
    }
}

/*
handles login request.
input: conversation socket stream.
output: execution result, tuple of username and logged in flag.
*/
fn login(stream : TcpStream) -> std::io::Result<(String, bool)>
{
    let result = utils::get_request_args(stream, true); //arguments
    let is_login_msg : bool; //is login message flag
    let already_logged : bool; //is already logged flag
    let username : String; //current username

    match result {
        Ok(args) => {
            username = args[POS_USERNAME].clone(); //get username
            is_login_msg = args[0].parse::<i32>().unwrap() == LOGIN; //check if it is a login message
            
            {
                already_logged = (*USERS.lock().unwrap()).contains(&username); //check if already logged
            }

            if is_login_msg
            {
                if !already_logged
                {
                    println!("New user logged in :: '{}'", username);
                    (*USERS.lock().unwrap()).insert(username.clone()); //append username to online users list
                    Ok((username, is_login_msg)) //return
                }
                else
                {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, format!("User '{}' is already logged in!", username)))  //raise error
                }
            }
            else
            {
                Err(std::io::Error::new(std::io::ErrorKind::Other, format!("User '{}' sent invalid login message!", username)))  //raise error
            }
        }
        Err(e) => {
            Err(e) //raise error
        }
    }
}

/*
recieves client update message.
input: conversation socket stream, sender username.
output: execution result, partner username.
*/
fn recv_client_update(stream : TcpStream, sender : String) -> Result<String, std::io::Error>
{
    let result = utils::get_request_args(stream, false); //arguments
    
    match result {
        Ok(args) => {
            //check if it's a valid client update message
            if args[0].parse::<i32>().unwrap() == CLI_UPDATE_M && args.len() == CLI_UPDATE_M_SIZE
            {
                //check if message is not empty
                if args[CLI_MSG_POS].len() > 0
                {
                    //apend to messages
                    (*MESSAGES.write().unwrap()).push_back(format!("{}&{}&{}", sender, args[POS_USERNAME], args[CLI_MSG_POS]));
                }
                Ok(args[POS_USERNAME].to_string()) //return
            }
            else
            {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid client update message!")) //raise error
            }
        }
        Err(e) => { Err(e) }
    }
    
}

/*
sends server update message.
input: conversation socket stream, username, partner username.
output: execution result.
*/
fn send_server_update(mut stream : TcpStream, user : String, partner : String) -> Result<(), std::io::Error>
{    
    let online_users : String;
    {
        //get online users string
        online_users = (*USERS.lock().unwrap()).clone().into_iter().collect::<Vec<String>>().join("&");
    }

    match get_chat_filename(user, partner.clone()) {
        Ok(fname) => {
            if fname.len() > 0
            {
                //read char file contant
                match fs::read_to_string(fname.clone()) {
                    Ok(data) => {
                        //send update
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
                        //create a new chat file for the session
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
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Chat filename was empty!")) //raise error
            }
        }
        Err(_) => { 
            send_default_server_update(stream, online_users.clone())
        }
    }
}

/*
sends server default update message.
input: conversation socket stream, online users string.
output: execution result.
*/
fn send_default_server_update(mut stream : TcpStream, online_users : String) -> Result<(), std::io::Error>
{
    //send default update message
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

/*
handles incoming messages.
input: conversation socket stream, username, partner username.
output: execution result.
*/
fn message_handler()
{
    let mut fields : Vec<String>; //message fields
    let mut is_empty : bool; //is messages queue empty flag
    loop
    {
        {
            //chack if empty
            let r = MESSAGES.read().unwrap();
            is_empty = (*r).is_empty();
        }
        if !is_empty
        {
            {
                //collect last message fields
                let r = MESSAGES.read().unwrap();
                fields = (*r).front().expect("cannot reference message").split('&').map(|s| s.to_string()).collect();
            }

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

            {
                (*MESSAGES.write().unwrap()).pop_front(); //remove last message
            }
            
            fields.clear(); //reset fields
        }
    }
}

/*
assembles the chat filename for 2 given users.
input: username, partner username.
output: execution result, filename.
*/
fn get_chat_filename(user : String, sender : String) -> std::io::Result<String>
{
    let result : std::io::Result<String>;
    if user != "" && sender != "" //check if not empty
    {
        //sort usernames and format filename
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
        result = Err(std::io::Error::new(std::io::ErrorKind::Other, "Empty usernames!")); //raise error
    }
    result
}

/*
updates the chat file.
input: filename, sender username, message data.
output: execution result.
*/
fn update_chat_file(fname : String, sender : String, data : String) -> std::io::Result<()>
{
    let mut file = fs::OpenOptions::new() //open file for appending data
        .write(true)
        .append(true)
        .open(fname)
        .unwrap();

    let data : String = format!("&MAGSH_MESSAGE&&Author&{}&DATA&{}", sender, data); //format record
    match file.write_all(data.as_bytes()) //write to chat file
    {
        Ok(_) => { Ok(()) }
        Err(e) => { Err(e) }
    }
}