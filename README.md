# Iris api
Lantana core向けのインタフェースモジュール  
RustでプラグインDLLを作る時に本モジュールのTraitを実装しなければならない。  

# プラグインモジュールの例
``` Rust
struct Plugin;

impl iris_api::core::PluginRequireApi for Plugin{
    #[no_mangle]
    fn iris_entry(data:iris_api::core::ParentData)->Result<iris_api::core::Initialize,Box<std::error::Error>> {
        data.core_api.write_console("plugin boot!".to_owned());
        data.core_api.write_console(format!("module basedir: {:?}",data.basedir));
        data.core_api.write_console(format!("module plugin basedir: {:?}",data.plugindir));

        Ok(iris_api::core::Initialize::Success)
    }
}
```
