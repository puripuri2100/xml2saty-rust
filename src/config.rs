use json;
use json::JsonValue;
use std::fs;


fn make_require_package (lst:&JsonValue) -> String {
  let mut st = String::new();
  let len = lst.len();
  for i in 0 .. len {
    let s = format!("@require: {}\n", lst[i]);
    st.push_str(&s)
  };
  st
}


fn make_import_package (lst:&JsonValue) -> String {
  let mut st = String::new();
  let len = lst.len();
  for i in 0 .. len {
    let s = format!("@import: {}\n", lst[i]);
    st.push_str(&s)
  };
  st
}


pub fn parse (path:&str) -> JsonValue {
  let data = fs::read_to_string(path).unwrap();
  json::parse(&data).unwrap()
}

pub fn header (v:JsonValue) -> String {
  let require_list = &v["require"];
  let import_list = &v["import"];
  let require_str = make_require_package(&require_list);
  let import_str = make_import_package(&import_list);
  format!("{}\n{}", require_str, import_str)
}


pub fn package (p:&Option<&str>, t:String) -> String {
  match p {
    None => {t}
    Some(s) => {
      let pvec:Vec<&str> = s.split(',').collect();
      let m_name = pvec.iter().nth(0).unwrap_or(&"");
      let f_name = pvec.iter().nth(1).unwrap_or(&"");
      format!("module {} = struct\nlet {} = \n{}\nend", m_name, f_name, t,)
    }
  }
}