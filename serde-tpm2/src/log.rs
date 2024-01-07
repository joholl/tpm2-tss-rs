use std::{
    any,
    collections::HashMap,
    fmt::{self, Display},
    panic::Location,
    path::Path,
};

use project_root::get_project_root;

pub struct Logger {
    /// logging prefix
    prefix: String,
    /// struct/enum level
    level: usize,
    /// field names of struct per level
    field_names: HashMap<usize, &'static [&'static str]>,
}

impl Logger {
    pub fn new(prefix: String) -> Logger {
        Logger {
            prefix,
            level: 0,
            field_names: HashMap::new(),
        }
    }

    pub fn level_push(&mut self) {
        self.level += 1;
    }

    pub fn level_pop(&mut self) {
        if self.level == 0 {
            panic!("Cannot pop non-existant indent. This is a bug.");
        }
        self.level -= 1;
    }

    fn indent(&self) -> usize {
        const INDENT_SPACES: usize = 4;
        self.level * INDENT_SPACES
    }

    pub fn get_field_names(&self) -> &'static [&'static str] {
        static TOP_LEVEL_ENUM: [&str; 2] = ["enum_discriminant", "enum_variant"];
        self.field_names
            .get(&self.level)
            .unwrap_or(&TOP_LEVEL_ENUM.as_ref())
    }

    pub fn set_field_names(&mut self, fields: &'static [&'static str]) {
        self.field_names.insert(self.level, fields);
    }

    #[track_caller]
    pub fn log(&self, args: fmt::Arguments) {
        // resolve caller path (can be relative or absolute)
        let caller_file = Path::new(Location::caller().file());
        let caller_file = match caller_file.is_relative() {
            true => caller_file,
            false => caller_file
                .strip_prefix(get_project_root().unwrap())
                .unwrap(),
        };

        log::info!(
            "{}:{}:{} - {} {:i$}{}",
            caller_file.display(),
            Location::caller().line(),
            Location::caller().column(),
            self.prefix,
            "",
            args,
            i = self.indent()
        );
    }

    #[track_caller]
    pub fn log_primitive<T: Display>(&self, v: T) {
        self.log(format_args!("= {} ({})", v, any::type_name::<T>()));
    }
}
