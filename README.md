# Terminal EPUB Reader

## 简介

Terminal EPUB Reader 是一个基于 Rust 语言开发的终端 EPUB 阅读器。它旨在为用户提供在终端环境下阅读电子书的能力，同时保持轻量级和易于操作的特性。本阅读器支持在多种操作系统上运行，提供了便捷的键盘导航以及章节快速跳转功能。

## 特性

章节快速跳转： 快速跳转到指定章节，提升阅读效率。
锚点定位： 对于单文件EPUB书籍，提供精准的章节和小节锚点定位。
终端优化显示： 针对终端环境优化的文本显示，保持清晰易读。
自动存储阅读进度： 阅读进度自动保存，方便下次继续阅读。

## 构建指南

### 本地环境
1. 确保您的系统已安装 Rust 以及 Cargo。
2. 克隆仓库：

```sh
Copy code
git clone https://github.com/your-github/terminal-epub-reader.git
```

3. 进入项目目录：
```sh
Copy code
cd terminal-epub-reader
```

4. 构建项目：
```sh
Copy code
cargo build --release
```
### 使用方法
- 运行程序：
```sh
Copy code
cargo run -- path/to/your/book.epub
```

### 操作指南：
- j 或 向下箭头：向下滚动
- k 或 向上箭头：向上滚动
- q：退出阅读器
- 数字键：快速跳转到对应的章节

<img width="800" alt="image" src="https://github.com/daiw-ing/ter-epub/assets/62934849/2e6c7677-fde0-47b9-9fc8-65fc6888bbe5">


### 通知
**正在开发中,在发布可用版本前,请不随意构建本程序**
