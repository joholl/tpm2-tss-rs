use std::{
    any,
    collections::HashMap,
    fmt::{self, Display},
};

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
        return Logger {
            prefix: prefix,
            level: 0,
            field_names: HashMap::new(),
        };
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
        static TOP_LEVEL_ENUM: [&'static str; 2] = ["enum_discriminant", "enum_variant"];
        self.field_names
            .get(&self.level)
            .unwrap_or(&TOP_LEVEL_ENUM.as_ref())
    }

    pub fn set_field_names(&mut self, fields: &'static [&'static str]) {
        self.field_names.insert(self.level, fields);
    }

    pub fn log(&self, args: fmt::Arguments) {
        log::info!("{} {:i$}{}", self.prefix, "", args, i = self.indent());
    }

    pub fn log_primitive<T: Display>(&self, v: T) {
        self.log(format_args!("= {} ({})", v, any::type_name::<T>()));
    }
}
