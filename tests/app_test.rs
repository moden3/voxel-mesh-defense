use bevy::prelude::*;
use voxel_mesh_defense::voxel::{Chunk, CHUNK_SIZE};
use voxel_mesh_defense::flow_field::FlowField;

#[test]
fn test_app_initialization() {
    let mut app = App::new();
    
    // 最小限のプラグイン構成で初期化をテスト
    // レンダリングなしのHeadlessモードに近い状態でテスト
    app.add_plugins(MinimalPlugins);
    
    // コンポーネントが正しく登録されているか等の確認
    app.world_mut().spawn((Chunk::new(), FlowField::default()));
    
    app.update();
    
    // システムがパニックせずに1フレーム実行できることを確認
    assert!(app.world().entities().len() > 0);
}
