use rspack_core::{Compilation, CompilerAfterEmit, Plugin};
use rspack_hook::{plugin, plugin_hook};
#[derive(Debug)]
#[plugin]
pub struct EmitPlugin {
    options: EmitPluginOptions,
}
#[derive(Debug)]
pub struct EmitPluginOptions {}
impl EmitPlugin {
    pub fn new(options: EmitPluginOptions) -> Self {
        Self::new_inner(options)
    }
}
#[plugin_hook(CompilerAfterEmit for EmitPlugin)]
async fn emit_file(&self, compilation: &mut Compilation) -> rspack_error::Result<()> {
    let assets = compilation.assets();
    let stats = compilation.get_stats();
    //dbg!(assets,stats);
    dbg!(assets);
    Ok(())
}
impl Plugin for EmitPlugin {
    fn name(&self) -> &'static str {
        "EmitPlugin"
    }
    fn apply(
        &self,
        ctx: rspack_core::PluginContext<&mut rspack_core::ApplyContext>,
        _options: &mut rspack_core::CompilerOptions,
    ) -> rspack_error::Result<()> {
        ctx.context
            .compiler_hooks
            .after_emit
            .tap(emit_file::new(self));
        Ok(())
    }
}
