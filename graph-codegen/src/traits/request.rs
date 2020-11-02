use crate::parser::filter::ModifierMap;
use crate::parser::Request;
use inflector::Inflector;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

lazy_static! {
    /// Matches any number. Some of the graph request data has
    /// numbers in the name of the operation id such as
    /// groups.users.get.23a. This becomes an issue when parsing
    /// the resource id. For instance, the method name for an
    /// individual request is taken from the last part of the resource id
    /// and method names really should not be named 23a.
    static ref NUM_REG: Regex = Regex::new(r"[0-9]").unwrap();

    /// matches ids attached to the resource name such as groups({id}).
    static ref PATH_ID_REG: Regex = Regex::new(r"(\(\{)(\w+)(}\))").unwrap();

    /// Matches named ids such as {group-id}.
    static ref PATH_ID_NAMED_REG: Regex = Regex::new(r"(\{)(\w+-\w+)(})").unwrap();
}

pub trait RequestParserBuilder<RHS: ?Sized = Self> {
    fn build(&self, modifier: &ModifierMap) -> Request;
}

pub trait RequestParser<RHS = Self> {
    fn method_name(&self) -> String;
    fn operation_mapping(&self) -> String;
    fn transform_path(&self) -> String;
    fn links(&self) -> HashSet<String> {
        Default::default()
    }
}

pub trait Modify<T> {
    fn modify(&self, value: &mut T);
}

impl RequestParser for &str {
    fn method_name(&self) -> String {
        let mut method_name = String::new();
        if let Some(index) = self.rfind('.') {
            let last: &str = self[index + 1..].as_ref();
            if NUM_REG.is_match(last) {
                if let Some(idx) = self[..index].rfind('.') {
                    method_name.push_str(self[idx + 1..index].as_ref());
                }
            } else {
                method_name.push_str(self[index + 1..].as_ref());
            }
        } else {
            method_name.push_str(&self);
        }

        if method_name.is_empty() {
            self.to_snake_case()
        } else {
            method_name.to_snake_case()
        }
    }

    fn operation_mapping(&self) -> String {
        let mut op_mapping = String::new();

        if self.contains('.') {
            let mut ops: Vec<&str> = self.split('.').collect();
            ops.retain(|s| !s.is_empty());

            if let Some(last) = ops.pop() {
                if !NUM_REG.is_match(last) {
                    if ops.len() > 1 {
                        op_mapping = ops.join(".");
                    } else {
                        op_mapping = ops.join("");
                    }
                }
            }
        } else {
            op_mapping = self.to_string();
        }

        if op_mapping.ends_with('.') {
            op_mapping.truncate(op_mapping.len() - 1);
        }

        if op_mapping.matches('.').count() == 1 {
            let mut queue: VecDeque<&str> = op_mapping.split('.').collect();
            let first = queue.pop_front().unwrap();
            let last = queue.pop_front().unwrap();
            if first[..first.len() - 1].eq(last) {
                op_mapping = first.to_string();
            } else if last[..last.len() - 1].eq(first) {
                op_mapping = last.to_string();
            }
        }

        op_mapping
    }

    fn transform_path(&self) -> String {
        let mut path = self.to_string();
        let path_clone = path.clone();

        let mut count = 1;
        // Replaces ids in paths attached to the resource name such as groups({id})
        for cap in PATH_ID_REG.captures_iter(path_clone.as_str()) {
            let s = cap[0].to_string();
            if count == 1 {
                path = path.replace(s.as_str(), "/{{id}}");
            } else {
                path = path.replace(s.as_str(), &format!("/{{{{id{}}}}}", count));
            }
            count += 1;
        }

        let mut count = 1;
        // Replaces named ids such as {group-id}.
        for cap in PATH_ID_NAMED_REG.captures_iter(path_clone.as_str()) {
            let s = cap[0].to_string();
            if count == 1 {
                path = path.replace(s.as_str(), "{{id}}");
            } else {
                path = path.replace(s.as_str(), &format!("{{{{id{}}}}}", count));
            }
            count += 1;
        }
        if path.contains("microsoft.graph.") {
            path.replace("microsoft.graph.", "")
        } else {
            path
        }
    }

    fn links(&self) -> HashSet<String> {
        let mut links: HashSet<String> = HashSet::new();

        if self.contains('.') {
            let mut vec: Vec<&str> = self.split('.').collect();
            vec.retain(|s| !s.is_empty());

            let mut iter = vec.iter().peekable();

            while let Some(current) = iter.next() {
                if let Some(next) = iter.peek() {
                    links.insert(format!("{}.{}", current, next));
                }
            }
        } else {
            links.insert(self.to_string());
        }

        links
    }
}

impl RequestParser for String {
    fn method_name(&self) -> String {
        self.as_str().method_name()
    }

    fn operation_mapping(&self) -> String {
        self.as_str().operation_mapping()
    }

    fn transform_path(&self) -> String {
        self.as_str().transform_path()
    }

    fn links(&self) -> HashSet<String> {
        self.as_str().links()
    }
}
