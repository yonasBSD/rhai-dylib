use rhai_dylib::loader::{libloading::Libloading, Loader};
use rhai_dylib::rhai::{config::hashing::set_ahash_seed, Engine, ImmutableString};

pub fn build_engine() -> Engine {
    if let Err(value) = set_ahash_seed(Some([1, 2, 3, 4])) {
        panic!("ahash seed has been overridden by a plugin: {value:?}");
    }

    // Inspect the type id generated by the binary.
    println!("engine: {:?}", std::any::TypeId::of::<ImmutableString>());

    let mut loader = Libloading::new();
    let mut engine = rhai_dylib::rhai::Engine::new();

    // Load the plugin.
    #[cfg(target_os = "linux")]
    engine.register_global_module(
        loader
            .load("./target/debug/libplugin.so")
            .expect("failed to load plugin"),
    );
    #[cfg(target_os = "windows")]
    engine.register_global_module(
        loader
            .load("./target/debug/libplugin.dll")
            .expect("failed to load plugin"),
    );

    engine
}
