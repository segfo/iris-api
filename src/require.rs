use std::sync::Arc;
use std::path::PathBuf;
use crate::core::CoreServices;

#[derive(Debug)]
pub struct CoreData{
    pub basedir:PathBuf,
    pub plugindir:PathBuf,
    pub core_services:Box<CoreServices>,
}

// このプラグインが特定機能を実装していることを示す返却値
pub enum ImplBehavior{
    FunctionA(Arc<A>)
}

// すべてのプラグインが実装しなければならないTrait
pub trait PluginEntry{
    // プラグイン初期化
    fn iris_entry(data:CoreData)->Result<Arc<Plugin>,Box<std::error::Error>>;
}

// すべてのプラグインが実装しなければならないTrait
pub trait Plugin:std::fmt::Debug + Sync + Send {
    // Coreがシャットダウンする時に呼ばれる（この中でハンドルを開放したり終了処理をしなければならない。）
    fn core_ending_session(&self)->Result<(),Box<std::error::Error>>;
    fn get_behavior(&self)->ImplBehavior;
}

// プラグインが特定機能を実装していることを表すTrait
// これを実装していると、Coreはいい感じのタイミングで呼び出せる。
pub trait A{
    fn init_a(&self)->u32;
}
