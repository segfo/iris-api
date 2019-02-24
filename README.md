# Iris api
Lantana core向けのインタフェースモジュール  
RustでプラグインDLLを作る時に本モジュールのTraitを実装しなければならない。  

# プラグインモジュールの例
``` Rust
use std::sync::Arc;
struct Require;
#[derive(Debug)]
struct Plugin{
    core:iris_api::require::CoreData
}

impl iris_api::require::PluginEntry for Require{
    // プラグインのエントリポイント（必ず実装し、iris_api::require::PluginのTrait Objectを返却しなければならない）
    #[no_mangle]
    fn iris_entry(data:iris_api::require::CoreData)->Result<Arc<iris_api::require::Plugin>,Box<std::error::Error>> {
        data.core_services.write_console("plugin boot!\n".to_owned());
        data.core_services.write_console(format!("module basedir: {:?}\n",data.basedir));
        data.core_services.write_console(format!("module plugin basedir: {:?}\n",data.plugindir));

        #[cfg(unix)]
        data.core_services.write_console(format!("linux!\n"));
        #[cfg(windows)]
        data.core_services.write_console(format!("Windows!\n"));
        Ok(Arc::new(Plugin{core:data}))
    }
}

impl iris_api::require::Plugin for Plugin  {
    // Core終了時の処理
    fn core_ending_session(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.core.core_services.write_console("[+] while process closing...(please input enter)\n input >".to_owned());
        let s = self.core.core_services.read_console().unwrap().replace("\n","").replace("\r","");
        self.core.core_services.write_console(format!("[+] process closing...(input: {})\n",s));
        Ok(())
    }
    // プラグインの実装を呼び出すインタフェース
    fn get_behavior(&self)->iris_api::require::ImplBehavior{
        iris_api::require::ImplBehavior::FunctionA(Arc::new(Test{}))
    }
}

// プラグインの実装
struct Test{}
impl iris_api::require::A for Test{
    fn init_a(&self)->u32{0}
}

```
