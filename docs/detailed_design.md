# 1. データモデル設計
ゲーム内のコアデータはBevyのComponentとして定義されます。

```mermaid
erDiagram
    Chunk ||--o{ Voxel : contains
    Chunk {
        int x
        int y
        int z
        VoxelArray voxels
        bool is_dirty
    }
    Voxel {
        VoxelType type
        int pressure
    }
    Worker ||--o| Transform : has
    Worker {
        WorkerState state
        float energy
    }
    Pipe ||--o| Transform : has
    Pipe {
        int capacity
        int current_flow
    }
```

# 2. APIエンドポイント仕様
本作はスタンドアロンゲームであるため、Web APIは存在しません。代わりに、システム間のメッセージング（BevyのEvents）をAPI仕様とみなします。
- `Event<VoxelChangedEvent>`: ボクセルの破壊・設置時に発火し、メッシュ再生成とフローフィールド更新をトリガーする。
- `Event<SpawnWorkerEvent>`: プレイヤーが空間上にマーカーを置いた際に発火し、ワーカーエンティティを生成する。
- `Event<MarkerPlacedEvent>`: プレイヤーがボクセルをクリックしてマーカーを配置した際に発火し、フローフィールドの再計算をトリガーする。

# 3. 主要コンポーネント設計
- **Voxel/Chunk System**: `Chunk` コンポーネントは 32x32x32 の固定サイズのボクセル配列を保持します。メッシュ生成システムは `is_dirty` フラグを監視し、変更があった場合のみ再生成処理（メッシング）を行います。
- **Mouse Picking (DDA) System**: カメラからのレイ（光線）を飛ばし、3Dグリッド上をステップ実行する高速なDDA（Digital Differential Analyzer）アルゴリズムを用いて交差するボクセルを特定します。外部の物理エンジンに依存しません。
- **Cellular Automaton System**: 毎フレーム（または固定Tick単位で）、`Pipe` コンポーネントを持つボクセルの `pressure`（背圧）を近接ボクセルへ伝播します。重力をシミュレートするため、下方向への伝播係数を高く設定します。
- **Flow Field System**: 目的地（マーカー等）から幅優先探索（BFS）やDijkstraアルゴリズムを用いて各ボクセル空間の距離（コスト）を計算し、ワーカーが向かうべき方向（ベクター場）をキャッシュします。これにより数千のワーカーの経路探索コストを大幅に削減します。
- **Swarm (Worker) System**: ワーカーエンティティは自身の現在座標からフローフィールドの方向ベクトルを参照し、キネマティック（直接座標更新）に移動します。

# 4. 状態遷移と主要シーケンス

以下は自律型ワーカー（スウォーム）の行動状態遷移です。

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> MovingToTarget : マーカー検知
    MovingToTarget --> Working : 目的地到達
    Working --> MovingToNode : エネルギー低下
    MovingToNode --> Recharging : インフラノード到達
    Recharging --> Idle : 充電完了
```

# 5. エラーハンドリング方針
- チャンク境界外へのボクセルアクセス（範囲外参照）試行時は、Panicを避け安全に `VoxelType::Empty` を返却する方針とします。
- WASM（ブラウザ）環境では、`console_error_panic_hook` を使用してパニック発生時にブラウザのコンソールへ詳細なスタックトレースを出力します。
- メモリ不足やエンティティ生成失敗などの予期せぬエラー時には、例外を握りつぶさず、Bevyの `error!` マクロを用いてコンソールにエラーログを出力し、安全にステートをフォールバックします。
