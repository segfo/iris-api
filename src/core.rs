pub trait CoreServices : std::fmt::Debug + Sync + Send{
    fn write_console(&self,s:String);
    fn read_console(&self)->Result<String,std::io::Error>;
}
