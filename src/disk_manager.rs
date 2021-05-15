use std::io;
use std::path::Path;
use std::fs::OpenOptions;

pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file: File,
    // 採番するページIDを決めるカウンタ
    nest_page: u64,
}

impl DiskManager {
    // コンストラクタ
    pub fn new(data_file: File) -> io::Result<Self> {}
    // ファイルパスを指定して開く
    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {}
    // 新しいページIDを採番する
    pub fn allovate_page(&mut self) -> PageId {}
    //  ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {}
    // データをページに書き出す
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {}
}
