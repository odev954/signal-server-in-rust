mod utils
{
    fn zfill(argument : i32, size : i32) -> String
    {
        let mut padded = argument.to_string();

        for _ in 0..(size - argument.to_string().len() as i32) 
        {
            padded = "0".to_owned() + &padded;
        }
        padded
    }

    fn is_buffer_overflow(arguments_length : Vec<i32>, max_buffer_size : i32) -> bool
    {
        arguments_length.iter().sum::<i32>() > max_buffer_size
    }
}