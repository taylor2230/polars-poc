use std::any::Any;

pub mod dataframe_functions {
    use std::collections::HashMap;
    use polars::error::PolarsResult;
    use polars::prelude::DataFrame;

    pub type TaskExecution<'a, 'b> = dyn Fn(&'a str, &'b mut DataFrame) -> PolarsResult<&'b mut DataFrame> ;

    pub fn flag_nulls<'a, 'b>(column: &'a str, data_frame: &'b mut DataFrame) -> PolarsResult<&'b mut DataFrame> {
        let rule_check = column.to_owned() + "_rule";
        let null_check = data_frame.column(column).unwrap().is_not_null().with_name(&*rule_check);
        data_frame.with_column(null_check)
    }

    pub fn get_all_tasks<'a, 'b>() -> HashMap<String, Box<TaskExecution<'a, 'b>>> {
        let mut functions: HashMap<String, Box<TaskExecution>> = HashMap::new();
        functions.insert("NonNull".parse().unwrap(), Box::new(flag_nulls));
        functions
    }
}