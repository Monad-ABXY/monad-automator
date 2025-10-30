## mona_auto

## global

## core

## ----- action.rs -----
action-display-leftclick = 左クリック：({$x}, {$y})
action-display-leftclick-image = 左クリック：画像位置
action-display-rightclick = 右クリック：({$x}, {$y})
action-display-rightclick-image = 右クリック：画像位置
action-display-drag = ドラッグ：({$start_x}, {$start_y}) → ({$end_x}, {$end_y})
action-display-scroll-up = スクロールアップ：({$x}, {$y})
action-display-scroll-down = スクロールダウン：({$x}, {$y})
action-image-position = 画像位置
action-display-scroll-up-image = スクロールアップ：{action-image-position}
action-display-scroll-down-image = スクロールダウン：{action-image-position}
action-display-keyinput = キー入力
action-display-textinput = テキスト入力
action-display-delay = 待機 {$millis}ms
action-display-send-discord = Discordに送信
action-display-enable = {$name} を有効化
action-display-disable = {$name} を無効化
action-display-condition-compare = 条件: {$left} {$op} {$right}
action-display-condition-image = 条件: 画像一致「{$target}」

## gui

## ----- action_panel.rs -----
action-panel-header = マッチアクション
action-panel-button-add-action = アクションを追加
# Left Click
action-panel-left-click = 左クリック
# Right Click
action-panel-right-click = 右クリック
# Drag
action-panel-drag = ドラッグ
# Scroll
action-panel-scroll = スクロール
action-panel-scroll-direction-option-up = 上へ
action-panel-scroll-direction-option-down = 下へ
# Key Input
action-panel-key-input = キー入力
action-panel-key-add = キーを追加
action-panel-key-type-down = キーダウン
action-panel-key-type-up = キーアップ
action-panel-key-type-down-and-up = キーダウンとアップ
action-panel-key-type-delay = 遅延
action-panel-key-custom-vk = カスタム VK:
# Text Input
action-panel-text-input = テキスト入力
# Delay
action-panel-delay = 待機
# Send Discord
action-panel-send-discord = Discord メッセージを送信
action-panel-label-webhook-url = Webhook URL
action-panel-label-message = メッセージ
action-panel-send-screenshot = スクリーンショットを送信
# Toggle Image Enable
action-panel-toggle-image-enable = 画像の有効化／無効化
action-panel-label-target = 対象
action-panel-enable-enabled = 有効化
action-panel-enable-disabled = 無効化
# Check Condition
action-panel-check-condition = 条件確認
action-panel-cond-kind-compare = 比較
action-panel-cond-kind-image-match = 画像一致
action-panel-label-left = 左側
action-panel-label-op = 演算子
action-panel-label-right = 右側
action-panel-hint-contains = contains は文字列でのみ動作します。
# Variable
action-panel-variable = 変数
action-panel-label-operator = 演算子
action-panel-label-value = 値
action-panel-hint-toggle-no-value = 「toggle」は値を必要としません。
action-panel-hint-var-expected-type = 期待される型: {$ty}
# 
action-panel-checkbox-use-matched-position = マッチした位置を使用
action-panel-duration-ms = 継続時間（ms）:
action-panel-button-open-coordinate-picker = 座標ピッカー
action-panel-label-millis-with-seconds = ミリ秒（{ $seconds }秒）
# Context
action-panel-context-edit = 編集
action-panel-context-delete = 削除
action-panel-context-move-up = 上へ移動
action-panel-context-move-down = 下へ移動

## ----- add_action_modal.rs -----
# Heading
add-action-modal-heading-edit = アクションを追加
add-action-modal-heading-add = 確認
# Button
add-action-modal-button-confirm = 確認
add-action-modal-button-cancel = キャンセル

## ----- control_panel.rs -----
# Label
control-panel-label-target-window = 対象ウィンドウ
# Button
control-panel-button-find = 対象ウィンドウを検索
control-panel-button-start = 開始
control-panel-button-stop = 停止(F8)
control-panel-button-add = 追加
control-panel-button-delete = 削除

## ----- coordinate_picker_viewport.rs -----
# Heading
coordinate-picker-heading = 座標を選択
# Button
coordinate-picker-button-close = 閉じる

## ----- error_modal.rs -----
# Heading
error-modal-heading = エラー
# Button
error-modal-button-ok = 確認

## ----- image_edit_viewport.rs -----
# Heading
image-edit-viewport-heading = 画像編集
# Button
image-edit-viewport-button-retake = 再キャプチャ
image-edit-viewport-button-crop = 切り抜き
image-edit-viewport-button-roi = ROI範囲を指定
image-edit-viewport-button-ok = OK
image-edit-viewport-button-cancel = キャンセル
# Image Range
image-edit-viewport-label-image-range = 画像範囲
# ROI Range
image-edit-viewport-label-roi-range = ROI範囲
image-edit-viewport-checkbox-use-crop = 切り抜き選択を使用
image-edit-viewport-warning-roi-size = ⚠ ROIサイズが画像より小さい

## ----- image_list_panel.rs -----
# Label
image-list-panel-label = 画像リスト
# Button
image-list-panel-button-add-folder = 🗁 追加
image-list-panel-button-add-image = 🗋 追加
image-list-panel-button-manage-variables = 変数管理
# Context
image-list-panel-context-rename = 名前を変更
image-list-panel-context-delete = 削除
image-list-panel-context-move-up = 上へ移動
image-list-panel-context-move-down = 下へ移動
image-list-panel-context-duplicate = 複製

## ----- image_preview_panel.rs -----
# Label
image-preview-panel-no-selection = ⚠ 選択された画像がありません
image-preview-panel-no-path = ⚠ プロジェクトパスが設定されていません
image-preview-panel-load-failed = ⚠ 画像が見つからないか、読み込みに失敗しました
image-preview-panel-always-run = 常に実行
# Button
image-preview-panel-button-retake = 再キャプチャ
# Modal
image-preview-panel-modal-heading = 画像編集
image-preview-panel-note = 新しく開いたウィンドウで完了してください

## ----- menu_bar.rs -----
# File
menu-file = ファイル
menu-file-new = 新規作成
menu-file-open = 開く
menu-file-save = 保存
# Menu
menu-menu = メニュー
menu-lang = 🌏 言語
menu-theme = テーマ
menu-theme-dark = ダーク
menu-theme-light = ライト
menu-check-updates = 更新を確認
menu-help = ヘルプ
menu-quit = 終了

## ----- project_panel.rs -----
project-panel-description = プロジェクトの説明:

## ----- setting_panel.rs -----
# Input Type
setting-panel-label-input-type = 入力タイプ
# Capture Type
setting-panel-label-capture-type = キャプチャタイプ
# Loop Frequency
setting-panel-label-loop-per-second = ループ頻度
setting-panel-hover-loop-per-second = 毎秒、画像リストをチェックする回数を設定します。
setting-panel-loop-per-second-very-low = 非常に低
setting-panel-loop-per-second-low = 低
setting-panel-loop-per-second-medium = 中
setting-panel-loop-per-second-high = 高
setting-panel-loop-per-second-very-high = 非常に高
# Sensitivity
setting-panel-label-threshold = 閾値
setting-panel-hover-threshold = 閾値が高いほど、画面とより類似している必要があり、一致として認識されます。
setting-panel-threshold-low = 低
setting-panel-threshold-medium = 中
setting-panel-threshold-high = 高
setting-panel-threshold-very-high = 非常に高

## ----- variables_viewport.rs -----
variables-viewport-heading = 変数管理
variables-viewport-button-add-var = ➕ 変数を追加
variables-viewport-button-save = 保存
variables-viewport-var-name = 変数名
variables-viewport-var-kind = 種類
variables-viewport-var-value = 値
variables-viewport-actions = 操作
variables-viewport-button-delete = 削除
variables-viewport-warning-type-value-mismatch = ⚠ 型と値が一致しません

## ----- window_resize_modal.rs -----
# Heading
window-resize-modal-heading = ウィンドウサイズが以前と一致しません
# Option
window-resize-modal-option-restore = 前のサイズに復元
window-resize-modal-option-update = ⚠ 現在のサイズに更新
# Button
window-resize-modal-button-confirm = 確認
window-resize-modal-button-cancel = キャンセル

## message

## ----- message_action.rs -----
message-action-error-cant-find-matched-position = 一致する位置が見つかりません。元の位置にフォールバックします: { $x } { $y }
message-action-left-click = 左クリック: {$x}, {$y}
message-action-right-click = 右クリック: {$x}, {$y}
message-action-drag = ドラッグ: ({$x1}, {$y1}) → ({$x2}, {$y2})
message-action-scroll = スクロール: {$x}, {$y}
message-action-key-input = キー入力
message-action-text-input = テキスト入力: "{$text}"
message-action-delay = 待機: {$millis}ミリ秒
message-action-send-discord = Discordに送信: "{$message}"
message-action-send-discord-with-screenshot = Discordに送信: "{$message}" - スクリーンショット付き
message-action-toogle-enable = 画像状態設定: "{$name}" を有効化
message-action-toogle-disable = 画像状態設定: "{$name}" を無効化
message-action-condition-check-success = 条件チェック成功: {$condition}
message-action-condition-check-fail = 条件チェック失敗: {$condition}

## ----- message_automation_loop.rs -----
message-automation-loop-error-cant-find-window = 自動化対象のウィンドウが見つかりません。自動化ループを開始できません。
message-automation-loop-error-fail-capture = キャプチャに失敗しました: {$error}
message-automation-loop-error-fail-load-template = テンプレート画像の読み込みに失敗しました: {$error}
message-automation-loop-template-found = {$name} を発見: ({$x}, {$y}), 類似度: {$similarity}
message-automation-loop-found = {$name} を検出しました
message-automation-loop-error-match-failed = 類似度 {$similarity} - マッチ失敗

## ----- message_project.rs -----
message-project-invalid-index = 無効なインデックス: {$index}
message-project-failed-create-dir = ディレクトリの作成に失敗しました: {$error}
message-project-failed-create-json = JSONファイルの作成に失敗しました: {$error}
message-project-failed-saved-json = JSONファイルの保存に失敗しました: {$error}
message-project-successfully-saved-json = JSONファイルを正常に保存しました: {$path}
message-project-failed-create-file = ファイルの作成に失敗しました: {$path}
error-project-failed-open-file = ファイルの読み込みに失敗しました: {$error}
error-project-failed-parse-json = JSONの解析に失敗しました: {$error}
error-project-file-selection-canceled = ファイル選択がキャンセルされました
error-project-file-create-project = 新しいプロジェクトの作成に失敗しました
error-project-no-first-window = 最初のウィンドウがありません
error-project-last-first-window = 最後のウィンドウがありません

## ----- message_setting.rs -----
message-setting-failed-create-ron = RONファイルの作成に失敗しました: {$path}
message-setting-successfully-saved-ron = RONファイルを正常に保存しました: {$path}
message-setting-failed-saved-ron = RONファイルの保存に失敗しました: {$path}
message-setting-failed-create-ron = ファイルの作成に失敗しました: {$path}
error-setting-failed-open-file = ファイルの読み込みに失敗しました: {$error}
error-setting-failed-parse-ron = RONの解析に失敗しました: {$error}

## ----- message_window.rs -----
error-window-saved-windows-empty = 保存されたウィンドウのリストが空です。
error-window-failed-find-top-hwnd = トップレベルウィンドウ '{$title}' (クラス: '{class}') のHWNDが見つかりませんでした。
error-window-cant-find-parent-hwnd = {$title}' ウィンドウの親HWNDが見つかりません。探索を中止します。
error-window-cant-find-low-hwnd = 親ウィンドウ '{$title2}' の下に '{$title1}' (クラス: '{$class}') ウィンドウが見つかりません。
message-window-save-information = 情報を保存するには、対象ウィンドウを左クリックしてください...