mod tasks;

use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use polars::prelude::*;
use crate::tasks::task_functions::dataframe_functions::TaskExecution;

fn main() {
    let csv_path = "/Users/shanetaylor/Downloads/cities.csv";
    let df = &mut CsvReader::from_path(csv_path)
        .unwrap()
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap_or_default();

    let function_map: HashMap<String, Box<TaskExecution>> = tasks::task_functions::dataframe_functions::get_all_tasks();
    let boxed_func: &Box<TaskExecution> = function_map.get("NonNull").unwrap();
    let target_column: &str = "LatD";
    let x: &mut DataFrame = boxed_func(target_column, df).unwrap();
    println!("{}", x);
}
