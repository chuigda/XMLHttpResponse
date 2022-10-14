use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PyOutputValue {
    PlainValue(String),
    Array(Vec<String>)
}

impl PyOutputValue {
    pub fn from_py_output(s: &str) -> Self {
        let s = s.trim();
        if s.contains(':') {
            let parts = s.split(':')
                .map(urlencoding::decode)
                .map(|r| r.unwrap_or(Cow::Borrowed("undefined")).to_string())
                .collect();
            PyOutputValue::Array(parts)
        } else {
            PyOutputValue::PlainValue(
                urlencoding::decode(s)
                    .unwrap_or(Cow::Borrowed("undefined"))
                    .to_string()
            )
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct EvalContext {
    values: BTreeMap<String, PyOutputValue>,
    x_for_chain: RefCell<Vec<String>>
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
            x_for_chain: RefCell::new(Vec::new())
        }
    }

    pub fn resolve_py_output(output: &str) -> Self {
        let mut ret = Self::new();
        let lines = output.trim().split('\n');
        for line in lines {
            ret.resolve_py_output_line(line);
        }
        ret
    }

    pub fn resolve_py_output_line(&mut self, line: &str) {
        let line = line.trim();
        if !line.starts_with('$') || !line.contains('=') {
            return
        }

        let mut split = line.splitn(2, '=');
        let var_name = split.next().unwrap().trim();
        let value = split.next().unwrap().trim();

        self.values.insert(
            var_name.to_string(),
            PyOutputValue::from_py_output(value)
        );
    }

    pub fn push_x_for(&self, var_name: impl ToString) {
        self.x_for_chain.borrow_mut().push(var_name.to_string());
    }

    pub fn pop_x_for(&self) {
        self.x_for_chain.borrow_mut().pop().unwrap();
    }

    pub fn eval_var_expr<'a>(&'a self, var_expr: &'a str) -> Option<&'a str> {
        if !var_expr.starts_with('$') {
            return Some(var_expr);
        }

        let mut parts = var_expr.split(':');
        let mut curr_var_value = self.lookup_var(parts.next()?)?;

        while let Some(part) = parts.next() {
            if let PyOutputValue::Array(arr) = curr_var_value {
                if let Ok(idx) = part.parse::<usize>() {
                    if idx < arr.len() {
                        curr_var_value = self.lookup_var(arr[idx].as_str())?;
                        continue;
                    }
                }
            }

            return None;
        }

        match curr_var_value {
            PyOutputValue::PlainValue(v) => Some(v.as_str()),
            PyOutputValue::Array(_) => Some("Array")
        }
    }

    pub fn eval_var_expr_raw(&self, var_expr: &str) -> Option<&PyOutputValue> {
        if !var_expr.starts_with('$') {
            return None;
        }

        let mut parts = var_expr.split(':');
        let mut curr_var_value = self.lookup_var(parts.next()?)?;

        while let Some(part) = parts.next() {
            if let PyOutputValue::Array(arr) = curr_var_value {
                if let Ok(idx) = part.parse::<usize>() {
                    if idx < arr.len() {
                        curr_var_value = self.lookup_var(arr[idx].as_str())?;
                        continue;
                    }
                }
            }

            return None;
        }

        Some(curr_var_value)
    }

    pub fn lookup_var(&self, var_name: &str) -> Option<&PyOutputValue> {
        let x_for_chain = self.x_for_chain.borrow();
        self.values.get(if var_name == "$XFORV" {
            x_for_chain.last()?.as_str()
        } else {
            var_name
        })
    }
}
