use libc::c_char;
use libc::size_t;
use std::slice;
use std::ffi::*; //{CStr, CString,}
use std::iter;
use std::fs::File;
use std::path::{Path};

use polars::prelude::*;//{CsvReader, DataType, Field, Schema, DataFrame,};
use polars::prelude::{Result as PolarResult};
use polars::frame::DataFrame;
use polars::datatypes::DataType;

pub struct SeriesC {
    se: Series,
}

impl SeriesC {
    fn new() -> SeriesC {
        SeriesC {
            se: Series::new_empty("anon", &DataType::UInt32),
        }
    }

    fn say(&self) {
        println!{"{}", self.se}
    }

    fn set(&mut self, new_se: Series) {
        self.se = new_se;
    }
}

// extern functions for Series Container
#[no_mangle]
pub extern "C" fn se_new() -> *mut SeriesC {
    Box::into_raw(Box::new(SeriesC::new()))
}

#[no_mangle]
pub extern "C" fn se_free(ptr: *mut SeriesC) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn se_say(ptr: *mut SeriesC) {
    let se_c = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    se_c.say();
}

// helper functions for DataFrame Container
pub fn df_load_csv(spath: &str) -> PolarResult<DataFrame> {
    let fpath = Path::new(spath);
    let file = File::open(fpath).expect("Cannot open file.");

    CsvReader::new(file)
    .has_header(true)
    .finish()
}

pub struct DataFrameC {
    df: DataFrame,
}

impl DataFrameC {
    fn new() -> DataFrameC {
        DataFrameC {
            df: DataFrame::default(),
        }
    }

    fn read_csv(&mut self, string: String) {
        self.df = df_load_csv(&string).unwrap(); 
    }

    fn head(&self) {
        println!{"{}", self.df.head(Some(5))};
    }

    fn column(&self, string: String) -> Series {
        self.df.column(&string).unwrap().clone()
    }
}

// extern functions for DataFrame Container
#[no_mangle]
pub extern "C" fn df_new() -> *mut DataFrameC {
    Box::into_raw(Box::new(DataFrameC::new()))
}

#[no_mangle]
pub extern "C" fn df_free(ptr: *mut DataFrameC) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn df_read_csv(
    ptr: *mut DataFrameC,
    string: *const c_char,
) {
    let df_c = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let spath = unsafe {
        CStr::from_ptr(string).to_string_lossy().into_owned()
    };
    df_c.read_csv(spath);
}

#[no_mangle]
pub extern "C" fn df_head(ptr: *mut DataFrameC) {
    let df_c = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    df_c.head();
}

#[no_mangle]
pub extern "C" fn df_column(
    ptr: *mut DataFrameC,
    string: *const c_char,
) -> *mut SeriesC {
    let df_c = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let colname = unsafe {
        CStr::from_ptr(string).to_string_lossy().into_owned()
    };
    let col = df_c.column(colname);
    let mut se_c = SeriesC::new();
    se_c.set( col );
    Box::into_raw(Box::new(se_c))
}

#[no_mangle]
pub extern "C" fn df_select(
    ptr: *mut DataFrameC,
    colspec: *const *const c_char,
    len: size_t, 
) -> *mut DataFrameC {
    let df_c = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let mut output = Vec::<String>::new();
    unsafe {
        assert!(!colspec.is_null());

        for item in slice::from_raw_parts(colspec, len as usize) {
            output.push(CStr::from_ptr(*item).to_string_lossy().into_owned());
        };
    println!("{:?}", output);
    println!("{:?}", output[0]);
    };

//iamerejh - get new df from selection
    //let col = df_c.columns(colspec);
    let mut se_c = SeriesC::new();
    //se_c.set( col );
    Box::into_raw(Box::new(se_c))
}

// ------------------------------------------------------------------

//#[no_mangle]
//pub extern "C" fn df_read_csv(string: *const c_char) {
//    let df = df_load_csv(&str_in(string)).unwrap();
//    println!{"{}", df.head(Some(5))};
//
//    let c = df.column("petal.length").unwrap();
//    println!{"{}", c};
//
//    let x = df
//            .groupby(["variety"])
//            .unwrap()
//            .select(["petal.length"])
//            .sum();
//
//    println!{"{:?}", x};
//}


// Rust FFI Omnibus: Slice Arguments
#[no_mangle]
pub extern "C" fn sum_of_even(n: *const u32, len: size_t) -> u32 {
    let numbers = unsafe {
        assert!(!n.is_null());
        slice::from_raw_parts(n, len as usize)
    };

    numbers
        .iter()
        .filter(|&v| v % 2 == 0)
        .sum()
}

