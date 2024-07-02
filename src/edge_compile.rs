#![deny(warnings)]

use std::path::Path;
use std::collections::HashMap; // Add this import
use rspack_ids::NaturalChunkIdsPlugin;
use rspack_ids::NamedModuleIdsPlugin;
use rspack_core::JavascriptParserOptions;
use rspack_core::ModuleType;
use rspack_core::ParserOptions;
use rspack_core::ParserOptionsByModuleType;
use rspack_core::{
    Builtins, CacheOptions, ChunkLoading, ChunkLoadingType, Compiler, CompilerOptions, Context,
    CrossOriginLoading, DevServerOptions, EntryOptions, Environment, Experiments, Filename,
    HashDigest, HashFunction, HashSalt, MangleExportsOption, Mode, ModuleOptions, Optimization,
    OutputOptions, PathInfo, Plugin, PublicPath, Resolve, SideEffectOption, SnapshotOptions,
    StatsOptions, Target, UsedExportsOption, WasmLoading
};
use oxc_resolver::AliasValue; // Added this import
// use tokio::sync::RwLock;
// use rspack_fs::cfg_async;

// use rspack_plugin_externals::http_externals_rspack_plugin;
use rspack_plugin_entry::EntryPlugin;
use rspack_plugin_javascript::JsPlugin;
use rspack_plugin_schemes::DataUriPlugin;
use rspack_plugin_schemes::HttpUriPlugin;
use serde_json::Map;
use serde_json::Value;
use std::fs;
// use rspack_fs::AsyncReadableFileSystem;
use crate::memory_fs::MockFileSystem;

pub async fn compile(network_entry: Option<String>) -> HashMap<String, Vec<u8>> {
    let instance = MockFileSystem::new(); // Directly assign the instance
    let output_filesystem = instance.clone();
    let root = env!("CARGO_MANIFEST_DIR");
    let context = Context::new(root.to_string());
    let dist: std::path::PathBuf = Path::new(root).join("./dist");
    if !dist.exists() {
        fs::create_dir_all(&dist).expect("Failed to create dist directory");
    }
    let dist = dist.canonicalize().unwrap();
    let entry_request: String = network_entry
        .as_deref()
        .filter(|entry| !entry.is_empty())
        .map_or_else(
            || {
                Path::new(root)
                    .join("./fixtures/index.js")
                    .canonicalize()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            },
            |entry| entry.to_string(),
        );
    dbg!(network_entry.clone());

    let options = CompilerOptions {
        context: root.into(),
        dev_server: DevServerOptions::default(),
        output: OutputOptions {
            path: dist,
            pathinfo: PathInfo::Bool(false),
            clean: false,
            public_path: PublicPath::Auto,
            asset_module_filename: Filename::from(String::from("asset-[name].js")),
            wasm_loading: WasmLoading::Disable,
            webassembly_module_filename: Filename::from(String::from("webassembly.js")),
            unique_name: "main".into(),
            chunk_loading: ChunkLoading::Enable(ChunkLoadingType::Jsonp),
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
            script_type: String::from("javascript/module"),
            environment: Environment {
                r#const: Some(true),
                arrow_function: Some(true),
            },
        },
        target: Target::new(&vec!["es2022".to_string()]).unwrap(),
        mode: Mode::Development,
        resolve: Resolve {
            extensions: Some(vec![".js".to_string()]),
            alias: Some(vec![(
                "^/".to_string(),
                vec![AliasValue::Path("https://esm.sh/".to_string())], // Wrap in Vec
            )]),
            ..Default::default()
        },
        resolve_loader: Resolve {
            extensions: Some(vec![".js".to_string()]),

            ..Default::default()
        },
        module: ModuleOptions {
            parser: Some(ParserOptionsByModuleType::from_iter([(
                ModuleType::JsAuto,
                ParserOptions::Javascript(JavascriptParserOptions {
                    dynamic_import_mode: rspack_core::DynamicImportMode::Eager,
                    dynamic_import_prefetch: rspack_core::JavascriptParserOrder::Order(1),
                    dynamic_import_preload: rspack_core::JavascriptParserOrder::Order(1),
                    url: rspack_core::JavascriptParserUrl::Disable,
                    expr_context_critical: false,
                    wrapped_context_critical: false,
                    exports_presence: None,
                    import_exports_presence: None,
                    reexport_exports_presence: None,
                    strict_export_presence: false,
                    worker: vec![],
                }),
            )])),
            //    generator: Some(GeneratorOptionsByModuleType::from_iter(generator.iter())),
            ..Default::default()
        },
        stats: StatsOptions::default(),
        snapshot: SnapshotOptions,
        cache: CacheOptions::default(),
        experiments: Experiments::default(),
        optimization: Optimization {
            concatenate_modules: false,
            remove_available_modules: false, // Fixed the typo here
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

    let plugin_options = EntryOptions {
        name: Some("main".to_string()),
        runtime: None,
        chunk_loading: None,
        async_chunks: None,
        public_path: None,
        base_uri: None,
        filename: None,
        library: None,
        depend_on: None,
    };
    let entry_plugin = Box::new(EntryPlugin::new(context, entry_request.clone(), plugin_options));
    plugins.push(Box::<JsPlugin>::default());
    plugins.push(entry_plugin);
    // plugins.push(http_externals_rspack_plugin(false, true));
    plugins.push(Box::<NaturalChunkIdsPlugin>::default());
    plugins.push(Box::<NamedModuleIdsPlugin>::default());
    plugins.push(Box::<DataUriPlugin>::default());
    plugins.push(Box::<HttpUriPlugin>::default());

    let mut compiler = Compiler::new(options, plugins, instance);

    println!("Compiling with entry: {}", entry_request);
    compiler.build().await.expect("build failed");

    // Dump all files in the output_filesystem
    let files = output_filesystem.files.read().await;
    // for (path, content) in files.iter() {
    //     println!("File path: {:?}", path);
    //     println!("File content: {:?}", String::from_utf8_lossy(content));
    // }
    //
    // // Dump all directories in the output_filesystem
    // let directories = output_filesystem.directories.read().await;
    // for path in directories.keys() {
    //     println!("Directory path: {:?}", path);
    // }

    // Return the files HashMap
    files.iter()
        .map(|(path, content)| (path.to_string_lossy().to_string(), content.clone()))
        .collect()
}
