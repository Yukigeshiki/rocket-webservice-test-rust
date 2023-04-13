use std::fmt::Debug;

pub fn info<T>(output: &T)
where
    T: Debug,
{
    println!("INFO {:?}", output)
}

pub fn error<T>(output: &T)
where
    T: Debug,
{
    eprintln!("ERROR {:?}", output)
}
