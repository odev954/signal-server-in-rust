use std::net::TcpStream;
use std::io::Read;

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

/*
check if there is an buffer overflow in TCP window (with a fixed size).
input: argument lengths vector, maximum buffer size.
output: buffer overflow statment.
*/
pub fn is_buffer_overflow(arguments_length : Vec<i32>, max_buffer_size : i32) -> bool
{
    arguments_length.iter().sum::<i32>() > max_buffer_size
}

pub fn get_request_args(mut stream : TcpStream) -> Vec<String>
{
    let mut args : Vec<String> = Vec::<String>::new();
    let mut buff : Vec<u8> = Vec::<u8>::new();
    
    stream.read(&mut buff[0..3]);
    args.push(String::from_utf8(buff[0..3].to_vec()).unwrap());
    buff.clear();

    args
}


