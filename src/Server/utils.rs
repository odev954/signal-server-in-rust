pub mod utils
{
    use std::net::TcpStream;
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

    pub fn get_request_args(stream : TcpStream) -> Vec<String>
    {
        Vec::<String>::new()
    }
}

