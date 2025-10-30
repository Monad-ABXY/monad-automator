## mona_auto

## global

## core

## ----- action.rs -----
action-display-leftclick = 左键点击：({$x}, {$y})
action-display-leftclick-image = 左键点击：图像位置
action-display-rightclick = 右键点击：({$x}, {$y})
action-display-rightclick-image = 右键点击：图像位置
action-display-drag = 拖动：从 ({$start_x}, {$start_y}) 到 ({$end_x}, {$end_y})
action-display-scroll-up = 向上滚动：({$x}, {$y})
action-display-scroll-down = 向下滚动：({$x}, {$y})
action-image-position = 图像位置
action-display-scroll-up-image = 向上滚动：{action-image-position}
action-display-scroll-down-image = 向下滚动：{action-image-position}
action-display-keyinput = 按键输入
action-display-textinput = 文本输入
action-display-delay = 等待 {$millis} 毫秒
action-display-send-discord = 发送到 Discord
action-display-enable = 启用 {$name}
action-display-disable = 禁用 {$name}
action-display-condition-compare = 条件: {$left} {$op} {$right}
action-display-condition-image = 条件: 图像匹配「{$target}」

## gui

## ----- action_panel.rs -----
action-panel-header = 匹配操作
action-panel-button-add-action = 添加操作
# Left Click
action-panel-left-click = 左击
# Right Click
action-panel-right-click = 右击
# Drag
action-panel-drag = 拖拽
# Scroll
action-panel-scroll = 滚动
action-panel-scroll-direction-option-up = 向上
action-panel-scroll-direction-option-down = 向下
# Key Input
action-panel-key-input = 键盘输入
action-panel-key-add = 添加按键
action-panel-key-type-down = 按下
action-panel-key-type-up = 抬起
action-panel-key-type-down-and-up = 按下并抬起
action-panel-key-type-delay = 延迟
action-panel-key-custom-vk = 自定义 VK：
# Text Input
action-panel-text-input = 文本输入
# Delay
action-panel-delay = 延迟
# Send Discord
action-panel-send-discord = 发送 Discord 消息
action-panel-label-webhook-url = Webhook URL
action-panel-label-message = 消息
action-panel-send-screenshot = 发送截图
# Toggle Image Enable
action-panel-toggle-image-enable = 启用/禁用图像
action-panel-label-target = 目标
action-panel-enable-enabled = 启用
action-panel-enable-disabled = 禁用
# Check Condition
action-panel-check-condition = 条件检查
action-panel-cond-kind-compare = 比较
action-panel-cond-kind-image-match = 图像匹配
action-panel-label-left = 左边
action-panel-label-op = 运算符
action-panel-label-right = 右边
action-panel-hint-contains = contains 仅适用于字符串。
# Variable
action-panel-variable = 变量
action-panel-label-operator = 运算符
action-panel-label-value = 值
action-panel-hint-toggle-no-value = toggle 不需要值。
action-panel-hint-var-expected-type = 预期类型: {$ty}
# 
action-panel-checkbox-use-matched-position = 使用匹配位置
action-panel-duration-ms = 持续时间（毫秒）：
action-panel-button-open-coordinate-picker = 打开坐标选
action-panel-label-millis-with-seconds = 毫秒（{ $seconds } 秒）
# Context
action-panel-context-edit = 编辑
action-panel-context-delete = 删除
action-panel-context-move-up = 上移
action-panel-context-move-down = 下移

## ----- add_action_modal.rs -----
# Heading
add-action-modal-heading-edit = 添加操作
add-action-modal-heading-add = 确定
# Button
add-action-modal-button-confirm = 确定
add-action-modal-button-cancel = 取消

## ----- control_panel.rs -----
# Label
control-panel-label-target-window = 目标窗口
# Button
control-panel-button-find = 查找目标窗口
control-panel-button-start = 开始
control-panel-button-stop = 停止(F8)
control-panel-button-add = 添加
control-panel-button-delete = 删除

## ----- coordinate_picker_viewport.rs -----
# Heading
coordinate-picker-heading = 选择坐标
# Button
coordinate-picker-button-close = 关闭

## ----- error_modal.rs -----
# Heading
error-modal-heading = 错误
# Button
error-modal-button-ok = 确定

## ----- image_edit_viewport.rs -----
# Heading
image-edit-viewport-heading = 图像编辑
# Button
image-edit-viewport-button-retake = 重新截图
image-edit-viewport-button-crop = 指定裁剪区域
image-edit-viewport-button-roi = 指定ROI区域
image-edit-viewport-button-ok = 确定
image-edit-viewport-button-cancel = 取消
# Image Range
image-edit-viewport-label-image-range = 图像范围
# ROI Range
image-edit-viewport-label-roi-range = ROI 范围
image-edit-viewport-checkbox-use-crop = 使用图像范围
image-edit-viewport-warning-roi-size = ⚠ ROI 范围小于图像范围

## ----- image_list_panel.rs -----
# Label
image-list-panel-label = 图像列表
# Button
image-list-panel-button-add-folder = 🗁 添加文件夹
image-list-panel-button-add-image = 🗋 添加文件
image-list-panel-button-manage-variables = 管理变量
# Context
image-list-panel-context-rename = 重命名
image-list-panel-context-delete = 删除
image-list-panel-context-move-up = 上移
image-list-panel-context-move-down = 下移
image-list-panel-context-duplicate = 复制

## ----- image_preview_panel.rs -----
# Label
image-preview-panel-no-selection = ⚠ 未选择图像
image-preview-panel-no-path = ⚠ 无项目路径
image-preview-panel-load-failed = ⚠ 图像不存在或加载失败
image-preview-panel-always-run = 始终执行
# Button
image-preview-panel-button-retake = 重新截图
# Modal
image-preview-panel-modal-heading = 图像编辑
image-preview-panel-note = 请在新窗口中完成操作

## ----- menu_bar.rs -----
# File
menu-file = 文件
menu-file-new = 新建
menu-file-open = 打开
menu-file-save = 保存
# Menu
menu-menu = 菜单
menu-lang = 🌏 语言
menu-theme = 主题
menu-theme-dark = 深色
menu-theme-light = 浅色
menu-check-updates = 检查更新
menu-help = 帮助
menu-quit = 退出

## ----- project_panel.rs -----
project-panel-description = 项目描述：

## ----- setting_panel.rs -----
# Input Type
setting-panel-label-input-type = 输入方式
# Capture Type
setting-panel-label-capture-type = 截图方式
# Loop Frequency
setting-panel-label-loop-per-second = 每循环频率
setting-panel-hover-loop-per-second = 设置每秒检查图像列表的次数。
setting-panel-loop-per-second-very-low = 非常低
setting-panel-loop-per-second-low = 低
setting-panel-loop-per-second-medium = 中等
setting-panel-loop-per-second-high = 高
setting-panel-loop-per-second-very-high = 非常高
# Sensitivity
setting-panel-label-threshold = 阈值
setting-panel-hover-threshold = 阈值越高，图像必须与屏幕越相似才会被视为匹配。
setting-panel-threshold-low = 低
setting-panel-threshold-medium = 中
setting-panel-threshold-high = 高
setting-panel-threshold-very-high = 非常高

## ----- variables_viewport.rs -----
variables-viewport-heading = 管理变量
variables-viewport-button-add-var = ➕ 添加变量
variables-viewport-button-save = 保存
variables-viewport-var-name = 变量名
variables-viewport-var-kind = 类型
variables-viewport-var-value = 值
variables-viewport-actions = 操作
variables-viewport-button-delete = 删除
variables-viewport-warning-type-value-mismatch = ⚠ 类型与值不匹配

## ----- window_resize_modal.rs -----
# Heading
window-resize-modal-heading = 窗口大小与之前不一致
# Option
window-resize-modal-option-restore = 恢复为之前的大小
window-resize-modal-option-update = ⚠ 更新为当前大小
# Button
window-resize-modal-button-confirm = 确认
window-resize-modal-button-cancel = 取消

## message

## ----- message_action.rs -----
message-action-error-cant-find-matched-position = 无法找到匹配位置，使用原始位置代替：{$x} {$y}
message-action-left-click = 左击：{$x}, {$y}
message-action-right-click = 右击：{$x}, {$y}
message-action-drag = 拖动：({$x1}, {$y1}) → ({$x2}, {$y2})
message-action-scroll = 滚动：{$x}, {$y}
message-action-key-input = 键盘输入
message-action-text-input = 文本输入："{$text}"
message-action-delay = 等待：{$millis}毫秒
message-action-send-discord = 发送到 Discord："{$message}"
message-action-send-discord-with-screenshot = 发送到 Discord："{$message}" - 包含截图
message-action-toogle-enable = 设置图像状态：启用 "{$name}"
message-action-toogle-disable = 设置图像状态：禁用 "{$name}"
message-action-condition-check-success = 条件检查成功：{$condition}
message-action-condition-check-fail = 条件检查失败：{$condition}

## ----- message_automation_loop.rs -----
message-automation-loop-error-cant-find-window = 无法找到自动化目标窗口，无法启动自动化循环。
message-automation-loop-error-fail-capture = 捕获失败：{$error}
message-automation-loop-error-fail-load-template = 加载模板图像出错：{$error}
message-automation-loop-template-found = 找到 {$name}：({$x}, {$y})，相似度：{$similarity}
message-automation-loop-found = 已检测到 {$name}
message-automation-loop-error-match-failed = 相似度 {$similarity} - 匹配失败

## ----- message_project.rs -----
message-project-invalid-index = 无效的索引：{$index}
message-project-failed-create-dir = 创建目录失败：{$error}
message-project-failed-create-json = 创建 JSON 文件失败：{$error}
message-project-failed-saved-json = 保存 JSON 文件失败：{$error}
message-project-successfully-saved-json = 成功保存 JSON 文件：{$path}
message-project-failed-create-file = 创建文件失败：{$path}
error-project-failed-open-file = 打开文件失败：{$error}
error-project-failed-parse-json = 解析 JSON 失败：{$error}
error-project-file-selection-canceled = 已取消文件选择
error-project-file-create-project = 创建新项目失败
error-project-no-first-window = 没有可用的第一个窗口
error-project-last-first-window = 没有可用的最后一个窗口

## ----- message_setting.rs -----
message-setting-failed-create-ron = 创建 RON 文件失败：{$path}
message-setting-successfully-saved-ron = 成功保存 RON 文件：{$path}
message-setting-failed-saved-ron = 保存 RON 文件失败：{$path}
message-setting-failed-create-ron = 创建文件失败：{$path}
error-setting-failed-open-file = 打开文件失败：{$error}
error-setting-failed-parse-ron = 解析 RON 失败：{$error}

## ----- message_window.rs -----
error-window-saved-windows-empty = 保存的窗口列表为空。
error-window-failed-find-top-hwnd = 保存的窗口列表为空。
error-window-cant-find-parent-hwnd = 无法找到窗口 '{$title}' 的父 HWND，停止遍历。
error-window-cant-find-low-hwnd = 在父窗口 '{$title2}' 下找不到窗口 '{$title1}'（类名：'{$class}'）。
message-window-save-information = 请左键点击窗口以保存其信息...