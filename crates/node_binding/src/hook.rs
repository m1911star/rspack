use std::sync::RwLock;

/// rust support hooks
#[derive(PartialEq)]
pub enum Hook {
  FinishMake,
  BuildModule,
  ThisCompilation,
  ProcessAssetsStageAdditional,
  ProcessAssetsStagePreProcess,
  ProcessAssetsStageDerived,
  ProcessAssetsStageAdditions,
  ProcessAssetsStageNone,
  ProcessAssetsStageOptimize,
  ProcessAssetsStageOptimizeCount,
  ProcessAssetsStageOptimizeCompatibility,
  ProcessAssetsStageOptimizeSize,
  ProcessAssetsStageDevTooling,
  ProcessAssetsStageOptimizeInline,
  ProcessAssetsStageSummarize,
  ProcessAssetsStageOptimizeHash,
  ProcessAssetsStageOptimizeTransfer,
  ProcessAssetsStageAnalyse,
  ProcessAssetsStageReport,
  AfterProcessAssets,
  Emit,
  AssetEmitted,
  ShouldEmit,
  AfterEmit,
  OptimizeChunkModules,
  BeforeCompile,
  AfterCompile,
  FinishModules,
  OptimizeModules,
  AfterOptimizeModules,
  OptimizeTree,
  /// webpack `compilation.hooks.chunkAsset`
  ChunkAsset,
  ContextModuleFactoryBeforeResolve,
  NormalModuleFactoryResolveForScheme,
  NormalModuleFactoryCreateModule,
  AfterResolve,
  BeforeResolve,
  SucceedModule,
  StillValidModule,
  ExecuteModule,
  RuntimeModule,
}

impl From<String> for Hook {
  fn from(s: String) -> Self {
    match s.as_str() {
      "finishMake" => Hook::FinishMake,
      "buildModule" => Hook::BuildModule,
      "thisCompilation" => Hook::ThisCompilation,
      "processAssetsStageAdditional" => Hook::ProcessAssetsStageAdditional,
      "processAssetsStagePreProcess" => Hook::ProcessAssetsStagePreProcess,
      "processAssetsStageDerived" => Hook::ProcessAssetsStageDerived,
      "processAssetsStageAdditions" => Hook::ProcessAssetsStageAdditions,
      "processAssetsStageNone" => Hook::ProcessAssetsStageNone,
      "processAssetsStageOptimize" => Hook::ProcessAssetsStageOptimize,
      "processAssetsStageOptimizeCount" => Hook::ProcessAssetsStageOptimizeCount,
      "processAssetsStageOptimizeCompatibility" => Hook::ProcessAssetsStageOptimizeCompatibility,
      "processAssetsStageOptimizeSize" => Hook::ProcessAssetsStageOptimizeSize,
      "processAssetsStageDevTooling" => Hook::ProcessAssetsStageDevTooling,
      "processAssetsStageOptimizeInline" => Hook::ProcessAssetsStageOptimizeInline,
      "processAssetsStageSummarize" => Hook::ProcessAssetsStageSummarize,
      "processAssetsStageOptimizeHash" => Hook::ProcessAssetsStageOptimizeHash,
      "processAssetsStageOptimizeTransfer" => Hook::ProcessAssetsStageOptimizeTransfer,
      "processAssetsStageAnalyse" => Hook::ProcessAssetsStageAnalyse,
      "processAssetsStageReport" => Hook::ProcessAssetsStageReport,
      "afterProcessAssets" => Hook::AfterProcessAssets,
      "emit" => Hook::Emit,
      "assetEmitted" => Hook::AssetEmitted,
      "shouldEmit" => Hook::ShouldEmit,
      "afterEmit" => Hook::AfterEmit,
      "optimizeChunkModules" => Hook::OptimizeChunkModules,
      "beforeCompile" => Hook::BeforeCompile,
      "afterCompile" => Hook::AfterCompile,
      "finishModules" => Hook::FinishModules,
      "optimizeModules" => Hook::OptimizeModules,
      "afterOptimizeModules" => Hook::AfterOptimizeModules,
      "optimizeTree" => Hook::OptimizeTree,
      "chunkAsset" => Hook::ChunkAsset,
      "contextModuleFactoryBeforeResolve" => Hook::ContextModuleFactoryBeforeResolve,
      "normalModuleFactoryCreateModule" => Hook::NormalModuleFactoryCreateModule,
      "normalModuleFactoryResolveForScheme" => Hook::NormalModuleFactoryResolveForScheme,
      "afterResolve" => Hook::AfterResolve,
      "beforeResolve" => Hook::BeforeResolve,
      "succeedModule" => Hook::SucceedModule,
      "stillValidModule" => Hook::StillValidModule,
      "executeModule" => Hook::ExecuteModule,
      "runtimeModule" => Hook::RuntimeModule,
      hook_name => panic!("{hook_name} is an invalid hook name"),
    }
  }
}

#[derive(Default)]
pub struct DisabledHooks(RwLock<Vec<Hook>>);

impl DisabledHooks {
  pub fn set_disabled_hooks(&self, hooks: Vec<String>) -> napi::Result<()> {
    let mut disabled_hooks = self.0.write().expect("failed to write lock");
    *disabled_hooks = hooks.into_iter().map(Into::into).collect::<Vec<Hook>>();
    Ok(())
  }

  pub fn is_hook_disabled(&self, hook: &Hook) -> bool {
    self.0.read().expect("").contains(hook)
  }
}
