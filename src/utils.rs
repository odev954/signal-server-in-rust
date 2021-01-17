use std::net::TcpStream;
use std::io::Read;

const BUFFER_SIZE: usize = 100;

/*
zero padding fill to a integer argument.
input: integer argument, size of padded string.
output: padded string.
*/
pub fn zfill(argument : i32, size : i32) -> String
{
    let mut padded = argument.to_string();

    for _ in 0..(size - argument.to_string().len() as i32) 
    {
        padded = "0".to_owned() + &padded;
    }
    padded
}

pub fn get_request_args(mut stream : TcpStream, is_login_msg : bool) -> Vec<String>
{
    let mut args : Vec<String> = Vec::<String>::new();
    let mut lens : Vec<usize> = Vec::<usize>::new();
    let mut buff = [0; BUFFER_SIZE];
    let mut namelen : usize = 0;
    let mut msglen : usize = 0;
    
    stream.read(&mut buff);
    
    args.push(String::from_utf8(buff[0..3].to_vec()).unwrap());
    args.push(String::from_utf8(buff[3..5].to_vec()).unwrap());
    
    namelen = args[1].parse::<usize>().unwrap();
    args.push(String::from_utf8(buff[5..namelen + 5].to_vec()).unwrap());
    
    if !is_login_msg //check if its not a login message
    {
        args.push(String::from_utf8(buff[namelen + 5..namelen + 7].to_vec()).unwrap());
        msglen = args[3].parse::<usize>().unwrap();
        if namelen > 0
        {
            args.push(String::from_utf8(buff[namelen..].to_vec()).unwrap());
            if msglen + namelen + 7 > BUFFER_SIZE
            {
                while msglen > args[4].len()
                {
                    stream.read(&mut buff);
                    args[4] = format!("{}{}", args[4], String::from_utf8(buff[0..msglen - args[4].len()].to_vec()).unwrap());
                }
            }
        }
    }
    

    args
}


