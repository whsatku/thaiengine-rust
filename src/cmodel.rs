#![allow(dead_code)]
extern crate libc;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use libc::FILE;

#[repr(C)]
pub struct FileId {
	pub magic: u16,
	pub file_type: u16,
	pub entry_size: u16,
	_filler: u16,
	pub num_entry: u32,
	pub timestamp: i64, // time_t = long int
}

impl FileId {
	pub fn new() -> FileId {
		FileId {
			magic: 0,
			file_type: 0,
			entry_size: 0,
			_filler: 0,
			num_entry: 0,
			timestamp: 0,
		}
	}
}

#[repr(C)]
pub struct DataRecord{
	pub id: u32,
	pub lang: u16,
	pub length: u16,
	pub tail_space: bool,
	pub is_unused: bool,
	pub numeric: bool,
	_filler: bool,
	pub map_file_pos: u32,
	pub timestamp: i64,
	raw_text: [c_char; 1023],
}

impl DataRecord {
	pub fn new() -> DataRecord {
		DataRecord {
			id: 0,
			lang: 0,
			length: 0,
			tail_space: false,
			is_unused: false,
			numeric: false,
			_filler: false,
			map_file_pos: 0,
			timestamp: 0,
			raw_text: [0; 1023],
		}
	}

	// pub fn text(&self) -> String {
	// 	CStr::from_ptr(&self.raw_text);
	// }
}

extern {
	fn read(s: *const c_char);
	fn syllable_read_metadata(file: *mut FILE, id: *const FileId);
	fn syllable_skip_to_data(file: *mut FILE);
	fn syllable_read_record(file: *mut FILE, id: *const DataRecord);
}

pub struct DataFile{
	fp: *mut FILE
}
impl DataFile {
	pub fn open(file : String) -> DataFile{
		return unsafe{
			let file_c = CString::new(file).unwrap().as_ptr();
			let fp = libc::fopen(file_c, CString::new("rb").unwrap().as_ptr());

			DataFile {
				fp: fp
			}
		}
	}

	pub fn metadata(&self) -> FileId {
		return unsafe {
			let mut metadata = Box::new(FileId::new());
			syllable_read_metadata(self.fp, &mut *metadata);
			return *metadata;
		};
	}

	pub fn to_data(&self){
		unsafe{ syllable_skip_to_data(self.fp); }
	}

	pub fn record(&self) -> DataRecord {
		return unsafe {
			let mut record = Box::new(DataRecord::new());
			syllable_read_record(self.fp, &mut *record);
			return *record;
		};
	}

	pub fn has_more(&self) -> bool{
		return unsafe{
			return libc::feof(self.fp) == 0;
		}
	}
}
