use crate::book::Book;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Explorer {
    // 当前阅读的章节,户退出的时候进行保存
    pub selected: usize,
    // 当前阅读的内容位置, 用户退出的时候进行保存
    pub position: usize,
    // 章节列表
    pub book: Book,
}
