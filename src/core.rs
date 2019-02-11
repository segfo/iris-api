use std::path::PathBuf;

pub trait CoreServices{
    fn write_console(&self,s:String);
    fn read_console(&self)->Result<String,std::io::Error>;
}

pub struct ParentData{
    pub basedir:PathBuf,
    pub plugindir:PathBuf,
    pub core_api:Box<CoreServices>
}

pub enum Initialize{
    Success
}

pub trait PluginRequireApi{
    fn iris_entry(data:ParentData)->Result<Initialize,Box<std::error::Error>>;
}
