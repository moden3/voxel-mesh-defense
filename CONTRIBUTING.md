# Voxel Mesh Defense 開発ガイドライン (CONTRIBUTING)

Voxel Mesh Defense プロジェクトへようこそ！
本プロジェクトは、AIエージェント（Google Antigravity等）と人間のエンジニアが協調して自律的に開発を進めるためのフレームワークを採用しています。

開発に参加する際は、以下のガイドラインとワークフローに従ってください。

## 1. 開発ワークフロー（必須）

本プロジェクトでは、コードを書き始める前に必ず設計フェーズを経る必要があります。
詳細なルールは `skills/skills/00_workflow_rules.md` に記載されています。
AIエージェントにタスクを依頼する際は、以下のフェーズ順序を遵守させてください。

1. **要件定義 (Requirements Engineer)**: `docs/requirements.md` の更新
2. **システム設計 (System Architect)**: `docs/system_design.md`, `docs/detailed_design.md` の更新
3. **実装 (Software Engineer)**: `src/` 配下のソースコード実装
4. **テスト (QA Engineer)**: ユニットテスト、統合テストの実装

## 2. コーディング規約

本プロジェクトはRustとBevy ECSのベストプラクティスに基づいています。

- **RustfmtとClippyの利用**:
  コミット前に必ず以下のコマンドを実行し、警告がないことを確認してください。
  ```bash
  cargo fmt
  cargo clippy -- -D warnings
  ```
- **コメントは「Why」を書く**:
  「何をしているか（What）」はコードを読めば分かるように命名規則（変数名、関数名）で表現し、コメントには「なぜその設計・アルゴリズムを採用したか（Why）」を記述してください。
- **ECS (Entity Component System) の遵守**:
  オブジェクト指向のような継承は避け、データ（Component）と振る舞い（System）を分離するデータ指向設計を徹底してください。

## 3. Pull Requestの作成

1. `main` ブランチから機能用の新しいブランチを作成します（例: `feature/add-flow-field`）。
2. コードを変更し、テストを追加します（`cargo test` が通ることを確認）。
3. コミットメッセージは分かりやすく、何の変更か明確に記載してください。
4. PRを作成し、レビュアー（人間またはエージェント）の承認を得てからマージします。
