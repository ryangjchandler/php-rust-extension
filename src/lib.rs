use ext_php_rs::{prelude::*, zend::ModuleEntry, info_table_start, info_table_row, info_table_end, convert::IntoZval, flags::DataType, types::{Zval, ZendHashTable}};
use serde_json::{Result, Value, Number};

#[php_function]
pub fn calculate_pi(iterations: usize) -> f64 {
    let mut k = 1.0;
    let mut s = 0.0;

    for i in 0..iterations {
        if i % 2 == 0 {
            s += 4.0 / k;
        } else {
            s -= 4.0 / k;
        }

        k += 2.0;
    }

    return s
}

#[php_function]
pub fn fast_json_decode(input: String) -> Zval {
    convert(serde_json::from_str(&input).unwrap())
}

fn convert(value: Value) -> Zval {
    let mut zv = Zval::new();
    match value {
        Value::String(s) => zv.set_string(&s, false).unwrap(),
        Value::Number(n) => zv.set_double(n.as_f64().unwrap()),
        Value::Bool(b) => zv.set_bool(b),
        Value::Null => zv.set_null(),
        Value::Array(items) => {
            let mut ht = ZendHashTable::with_capacity(items.len().try_into().unwrap());
            for val in items.into_iter() {
                ht.push(convert(val)).unwrap();
            }
            zv.set_hashtable(ht);
        },
        Value::Object(items) => {
            let mut ht = ZendHashTable::with_capacity(items.len().try_into().unwrap());
            for (key, value) in items {
                ht.insert(&key, convert(value)).unwrap();
            }
            zv.set_hashtable(ht);
        }
    };
    zv
}

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("Ryan's Rust-powered Extension", "enabled");
    info_table_end!();
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module.info_function(php_module_info)
}