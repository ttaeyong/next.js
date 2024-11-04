use anyhow::Result;
use turbo_tasks::{ValueDefault, Vc};
use turbopack_core::{
    chunk::{AsyncModuleInfo, ChunkItem, ChunkType, ChunkingContext},
    ident::AssetIdent,
    module::Module,
    reference::ModuleReferences,
};

use super::{asset::EcmascriptModulePartAsset, part_of_module, split_module};
use crate::{
    chunk::{EcmascriptChunkItem, EcmascriptChunkItemContent, EcmascriptChunkType},
    tree_shake::asset::SideEffectsModule,
    EcmascriptModuleContent,
};

/// This is an implementation of [ChunkItem] for
/// [Vc<EcmascriptModulePartAsset>].
///
/// This is a pointer to a part of an ES module.
#[turbo_tasks::value(shared)]
pub struct EcmascriptModulePartChunkItem {
    pub(super) module: Vc<EcmascriptModulePartAsset>,
    pub(super) chunking_context: Vc<Box<dyn ChunkingContext>>,
}

#[turbo_tasks::value_impl]
impl EcmascriptChunkItem for EcmascriptModulePartChunkItem {
    #[turbo_tasks::function]
    fn content(self: Vc<Self>) -> Vc<EcmascriptChunkItemContent> {
        panic!("content() should never be called");
    }

    #[turbo_tasks::function]
    async fn content_with_async_module_info(
        &self,
        async_module_info: Option<Vc<AsyncModuleInfo>>,
    ) -> Result<Vc<EcmascriptChunkItemContent>> {
        let module = self.module.await?;

        let split_data = split_module(module.full_module);
        let parsed = part_of_module(split_data, module.part);

        let analyze = self.module.analyze().await?;
        let async_module_options = analyze.async_module.module_options(async_module_info);

        let module_type_result = *module.full_module.determine_module_type().await?;

        let content = EcmascriptModuleContent::new(
            parsed,
            module.full_module.ident(),
            module_type_result.module_type,
            self.chunking_context,
            analyze.references,
            analyze.code_generation,
            analyze.async_module,
            analyze.source_map,
            analyze.exports,
            async_module_info,
        );

        Ok(EcmascriptChunkItemContent::new(
            content,
            self.chunking_context,
            module.full_module.await?.options,
            async_module_options,
        ))
    }

    #[turbo_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        self.chunking_context
    }
}

#[turbo_tasks::value_impl]
impl ChunkItem for EcmascriptModulePartChunkItem {
    #[turbo_tasks::function]
    fn references(&self) -> Vc<ModuleReferences> {
        self.module.references()
    }

    #[turbo_tasks::function]
    fn asset_ident(&self) -> Vc<AssetIdent> {
        self.module.ident()
    }

    #[turbo_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        Vc::upcast(self.chunking_context)
    }

    #[turbo_tasks::function]
    async fn ty(&self) -> Result<Vc<Box<dyn ChunkType>>> {
        Ok(Vc::upcast(
            Vc::<EcmascriptChunkType>::default().resolve().await?,
        ))
    }

    #[turbo_tasks::function]
    fn module(&self) -> Vc<Box<dyn Module>> {
        Vc::upcast(self.module)
    }

    #[turbo_tasks::function]
    fn is_self_async(&self) -> Vc<bool> {
        self.module.is_async_module()
    }
}

#[turbo_tasks::value(shared)]
pub(super) struct SideEffectsModuleChunkItem {
    pub module: Vc<SideEffectsModule>,
    pub chunk_item: Vc<Box<dyn EcmascriptChunkItem>>,
}

#[turbo_tasks::value_impl]
impl ChunkItem for SideEffectsModuleChunkItem {
    #[turbo_tasks::function]
    fn references(&self) -> Vc<ModuleReferences> {
        self.chunk_item.references()
    }

    #[turbo_tasks::function]
    fn asset_ident(&self) -> Vc<AssetIdent> {
        self.module.ident()
    }

    #[turbo_tasks::function]
    fn ty(&self) -> Vc<Box<dyn ChunkType>> {
        Vc::upcast(EcmascriptChunkType::value_default())
    }

    #[turbo_tasks::function]
    fn module(&self) -> Vc<Box<dyn Module>> {
        Vc::upcast(self.module)
    }

    #[turbo_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        EcmascriptChunkItem::chunking_context(self.chunk_item)
    }
}
#[turbo_tasks::value_impl]
impl EcmascriptChunkItem for SideEffectsModuleChunkItem {
    #[turbo_tasks::function]
    fn content(&self) -> Vc<EcmascriptChunkItemContent> {
        EcmascriptChunkItem::content(self.chunk_item)
    }

    #[turbo_tasks::function]
    async fn content_with_async_module_info(
        &self,
        async_module_info: Option<Vc<AsyncModuleInfo>>,
    ) -> Vc<EcmascriptChunkItemContent> {
        self.chunk_item
            .content_with_async_module_info(async_module_info)
    }

    #[turbo_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        EcmascriptChunkItem::chunking_context(self.chunk_item)
    }
}
