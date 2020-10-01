
#[macro_export]
macro_rules! hello {
    ($val: expr) => {
        format!("Hello, {}", $val);
    };
}

#[macro_export]
macro_rules! comp {
    ($x: ident $for: ident $y:ident $in:ident [$start: expr; $end: expr], $($cond: expr), *) => {{
        let mut list = Vec::new();
        for val in $start..$end {
            if $($cond(val))*{
                list.push(val);
            }
        }
        list
    }};
}

#[macro_export]
macro_rules! dict {
    ($($key: expr => $value: expr), *) => {
        {use std::collections::HashMap;
            let mut map = HashMap::new();
            $(map.insert($key, $value); )*

            map
        }
    };
}
