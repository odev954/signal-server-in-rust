use std::net::TcpStream;
use std::io::BufReader;
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
        padded = "0".to_owned() + &padded; //add padding
    }
    padded
}

/*
function will receive client's request and return the argument vector of the request.
input: client tcp stream, login message flag.
output: argument vector.
*/
pub fn get_request_args(stream : TcpStream, is_login_msg : bool) -> std::io::Result<Vec<String>>
{
    let mut args : Vec<String> = Vec::<String>::new(); //arguments
    let mut reader : BufReader<TcpStream> = BufReader::new(stream); //buffer reader
    let mut buffer : Vec<u8> = Vec::<u8>::new(); //buffer

    buffer.resize(3, 0); //set size to match length of code field
    match reader.read_exact(&mut buffer) //read from buffer and check for errors
    {
        Ok(_) => { 
            args.push(String::from_utf8(buffer.clone()).unwrap()); //append argument

            buffer.resize(2, 0); //set size to match length of username field
            match reader.read_exact(&mut buffer)
            {
                Ok(_) => { 
                    args.push(String::from_utf8(buffer.clone()).unwrap()); //append argument

                    buffer.resize(args[1].parse::<usize>().unwrap(), 0); //set size to match username field
                    match reader.read_exact(&mut buffer)
                    {
                        Ok(_) => { 
                            args.push(String::from_utf8(buffer.clone()).unwrap()); //append argument

                            if !is_login_msg //check if its not a login message
                            {
                                buffer.resize(5, 0); //set size to match length of message field
                                match reader.read_exact(&mut buffer)
                                {
                                    Ok(_) => { 
                                        args.push(String::from_utf8(buffer.clone()).unwrap()); //append argument
        
                                        buffer.resize(args[3].parse::<usize>().unwrap(), 0); //set size to match message field
                                        match reader.read_exact(&mut buffer)
                                        {
                                            Ok(_) => { 
                                                args.push(String::from_utf8(buffer.clone()).unwrap()); //append argument
                                                Ok(args) //return
                                            }
                                            Err(e) => {
                                                Err(e)
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        Err(e) //raise error
                                    }
                                }
                            } 
                            else
                            {
                                Ok(args) //return
                            }
                        }
                        Err(e) => {
                            Err(e) //raise error
                        }
                    }
                }
                Err(e) => {
                    Err(e) //raise error
                }
            }
        }
        Err(e) => {
            Err(e) //raise error
        }
    }
}

/*
formats a server update message.
input: message code, chat data, partner username, username.
output: formated message.
*/
pub fn format_server_update(code :i32, data : String, partner : String, users : String) -> String
{
    format!("{}{}{}{}{}{}{}", 
            zfill(code, 3), 
            zfill(data.len() as i32, 5), 
            data, 
            zfill(partner.len() as i32, 2), 
            partner, 
            zfill(users.len() as i32, 5), users)
}