//! Aspectrolog DB — stub HashMap
use std::collections::HashMap;
#[derive(Default)]
pub struct AspectDb { pub kv: HashMap<String, String> }