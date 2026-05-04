# 1. アーキテクチャ概要
本作は、Rust製ゲームエンジン「Bevy」を使用した3Dゲームです。WASM (WebAssembly) をターゲットとしてビルドし、ブラウザ上で動作させることで、UIテストやブラウザ経由でのプレイを容易にします。
データ指向設計であるECS (Entity Component System) アーキテクチャを全面に採用し、メインスレッドとワーカースレッドを活用して、数千単位のワーカーやボクセル演算（セル・オートマトン等）を高速に処理します。

# 2. 技術スタック
- **言語**: Rust (高速かつメモリ安全性が高く、ECSとの相性が良い)
- **ターゲット**: WebAssembly (`wasm32-unknown-unknown`)
- **ゲームエンジン**: Bevy (Webブラウザ上でのWebGL2描画にネイティブ対応)
- **ビルドツール**: `wasm-server-runner` または `cargo-make` を利用し、ローカルでのブラウザ起動をサポート
- **地形生成**: `noise` クレート (Perlinノイズ等を利用した地下空洞・鉱脈の自動生成)
- **物理演算**: カスタムAABB衝突判定とセル・オートマトン (汎用物理エンジンを避け、ボクセル特化の軽量アルゴリズムを採用)

# 3. システムコンポーネント図
```mermaid
graph TD
    Input[入力システム] --> GameLoop[Bevy App]
    GameLoop --> VoxelCore[ボクセル管理コンポーネント]
    GameLoop --> EnergySim[エネルギー網シミュレーション]
    GameLoop --> SwarmAI[スウォームAIコンポーネント]
    GameLoop --> Renderer[レンダリングシステム]
    GameLoop --> PickingSim[マウスピッキング(DDA)システム]
    
    VoxelCore --> ChunkData[(チャンクデータ)]
    EnergySim --> CellAutomaton[セル・オートマトンエンジン]
    SwarmAI --> FlowField[フローフィールド生成器]
    PickingSim --> VoxelCore
    
    ChunkData --> Renderer
    CellAutomaton --> Renderer
    FlowField --> Renderer
```

# 4. インフラストラクチャ設計
Webブラウザ向けアプリケーション（WASM）としてビルド・デプロイします。`wasm-server-runner` などを利用してローカル開発環境での動作確認を行い、将来的にはCI (GitHub Actions) を用いて、静的解析(clippy)、自動テスト（cargo test）、およびGitHub Pages等の静的ホスティング環境への自動デプロイをパイプラインに組み込み、品質を担保します。
