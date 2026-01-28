## 技術スタック

## 1. コア言語・ランタイム
言語: Rust (最新のStableチャネル推奨)

ゲームエンジン: Bevy Engine

特徴: ECSアーキテクチャ、データ指向、ホットリロード対応。

グラフィックスAPI: wgpu (Bevy標準)

バックエンド: Vulkan, Metal, DX12, WebGPU, WebGL2。

## 2. UIシステム
ゲーム内メインUI: Bevy UI (公式標準)

Flexbox (Taffy) によるレイアウト管理。

デバッグ・ツール用UI: bevy_egui

即時実行型GUI。パラメータ調整やインスペクタに最適。

高度なスタイリング: Belly または Sickle UI

CSSライクな記述や、より複雑なウィジェットが必要な場合に採用。

## 3. 物理演算・衝突判定
物理エンジン: Rapier (bevy_rapier)

Rust製の決定論的物理エンジン。2D/3D両対応。

軽量な衝突判定: bevy_mod_raycast

マウスピックや単純なレイキャストが必要な場合。

## 4. アセット管理
3Dモデル: glTF 2.0 (.glb / .gltf)

Bevyが最も得意とするフォーマット。PBRマテリアルを含む。

画像/テクスチャ: PNG, KTX2 (GPU圧縮テクスチャ)

フォント: TTF, OTF

音声: OGG, WAV, MP3 (bevy_audio)

## 5. Web/WASM ツールチェーン
コンパイルターゲット: wasm32-unknown-unknown

ビルド・バンドルツール: Trunk

WASMのビルド、HTML生成、アセット管理、ローカルサーバを統合。

実行支援: wasm-bindgen

最適化: wasm-opt (Binaryen)

WASMバイナリサイズを削減するために必須。

## 6. 開発ユーティリティ
IDE: VS Code (拡張機能: rust-analyzer) または RustRover

バージョン管理: Git

高速ビルド設定: LLDリンカ (ビルド時間の短縮に極めて重要)

インスペクタ: bevy-inspector-egui

実行中にエンティティのコンポーネント値をリアルタイム編集できる必須ツール。
