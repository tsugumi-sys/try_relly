use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

use zerocopy::{AsBytes, FromBytes};

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64);

impl PageId {
    pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

    pub fn valid(self) -> Option<PageId> {
        if self == Self::INVALID_PAGE_ID {
            None
        } else {
            Some(self)
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }
}

impl Default for PageId {
    fn default() -> Self {
        self::INVALID_PAGE_ID // check: why this number is default?
    }
}

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

imple DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {}
    // pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {}
    // pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {}
    // pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {}
    // pub fn allocate_page(&mut self) -> PageId {}
    // pub sync(&mut self) -> io::Result<()> {}
}
