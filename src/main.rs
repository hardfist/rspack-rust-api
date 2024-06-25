use rspack_core::{
    Builtins, CacheOptions, Compiler, CompilerOptions, Context, DevServerOptions, EntryOptions,
    Experiments, Filename, MangleExportsOption, Mode, ModuleOptions, Optimization, OutputOptions,
    PathInfo, Plugin, PublicPath, Resolve, SideEffectOption, SnapshotOptions, StatsOptions, Target,
    UsedExportsOption, WasmLoading, ChunkLoading, ChunkLoadingType, CrossOriginLoading, HashFunction, 
    HashDigest, HashSalt, Environment
};
use rspack_plugin_entry::EntryPlugin;
use std::sync::Arc;
use rspack_fs::AsyncNativeFileSystem;
use serde_json::Map;
use serde_json::Value;

fn main() {
    let output_filesystem = Arc::new(AsyncNativeFileSystem::default());

    let options = CompilerOptions {
        context: "some_context".into(),
        dev_server: DevServerOptions::default(),
        output: OutputOptions {
            path: "dist".into(),
            pathinfo: PathInfo::Bool(false),
            clean: true,
            public_path: PublicPath::Auto,
            asset_module_filename: Filename::from(String::from("asset-[name].js")),
            wasm_loading: WasmLoading::Disable,
            webassembly_module_filename: Filename::from(String::from("webassembly.js")),
            unique_name: "main".into(),
            chunk_loading: ChunkLoading::Enable(ChunkLoadingType::Import),
            chunk_loading_global: String::new(),
            filename: Filename::from(String::from("[name].js")),
            chunk_filename: Filename::from(String::from("[id].js")),
            cross_origin_loading: CrossOriginLoading::Disable,
            css_filename: Filename::from(String::from("[name].css")),
            css_chunk_filename: Filename::from(String::from("[id].css")),
            hot_update_main_filename: Filename::from(String::from("[name].[hash].hot-update.js")),
            hot_update_chunk_filename: Filename::from(String::from("[id].[hash].hot-update.js")),
            hot_update_global: String::new(),
            library: None,
            enabled_library_types: None,
            strict_module_error_handling: false,
            global_object: String::from("window"),
            import_function_name: String::from("import"),
            iife: false,
            module: false,
            trusted_types: None,
            source_map_filename: Filename::from(String::from("[file].map")),
            hash_function: HashFunction::MD4,
            hash_digest: HashDigest::Hex,
            hash_digest_length: 20,
            hash_salt: HashSalt::Salt(String::from("salt")),
            async_chunks: false,
            worker_chunk_loading: ChunkLoading::Disable,
            worker_wasm_loading: WasmLoading::Disable,
            worker_public_path: String::new(),
            script_type: String::from("text/javascript"),
            environment: Environment {
                r#const: Some(true),
                arrow_function: Some(true),
            },
        },
        target: Target::new(&vec!["es2022".to_string()]).unwrap(),
        mode: Mode::Development,
        resolve: Resolve::default(),
        resolve_loader: Resolve::default(),
        module: ModuleOptions::default(),
        stats: StatsOptions::default(),
        snapshot: SnapshotOptions::default(),
        cache: CacheOptions::default(),
        experiments: Experiments::default(),
        optimization: Optimization {
            concatenate_modules: false,
            remove_available_modules: false,
            provided_exports: false,
            mangle_exports: MangleExportsOption::False,
            inner_graph: true,
            used_exports: UsedExportsOption::default(),
            side_effects: SideEffectOption::default(),
        },
        profile: false,
        bail: false,
        builtins: Builtins::default(),
        __references: Map::<String, Value>::new(),
        node: None,
    };
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    let context = Context::from("some_context");
    let entry_request = "some_entry_request".to_string();
    let plugin_options = EntryOptions {
        name: None,
        runtime: None,
        chunk_loading: None,
        async_chunks: None,
        public_path: None,
        base_uri: None,
        filename: None,
        library: None,
        depend_on: None,
    };
    let plugin = Box::new(EntryPlugin::new(context, entry_request, plugin_options));
    plugins.push(plugin);

    let compiler = Compiler::new(options, plugins, output_filesystem);
}
