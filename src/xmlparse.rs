extern crate xml;

//use std::fs::File;
use serde_json::json;
use serde_json::Value;
use std::io::BufReader;
use std::vec;
use xml::attribute;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
enum SATySFiTypeTerm {
  BlockText,
  InlineText,
  SATySFiString,
  SATySFiBool,
  SATySFiInt,
  SATySFiFloat,
  SATySFiFunction,
}

#[derive(Debug)]
enum SATySFiType {
  Normal(SATySFiTypeTerm),
  List(SATySFiTypeTerm),
}

fn type_paren_l(config: &&Value, tag: &str) -> String {
  let value = get_attrib_value(&config, tag);
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(_) => "[".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => "{".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => "'<".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::SATySFiString) => "```".to_string(),
    _ => "".to_string(),
  }
}

fn type_paren_r(config: &&Value, tag: &str) -> String {
  let value = get_attrib_value(&config, tag);
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(_) => "]".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => "}".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => ">".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::SATySFiString) => "```".to_string(),
    _ => "".to_string(),
  }
}

fn type_paren_l_lst(config: &&Value, tag: &str) -> String {
  let value = get_attrib_value(&config, tag);
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(SATySFiTypeTerm::InlineText) => "{".to_string(),
    SATySFiType::List(SATySFiTypeTerm::BlockText) => "'<".to_string(),
    SATySFiType::List(SATySFiTypeTerm::SATySFiString) => "```".to_string(),
    _ => "".to_string(),
  }
}

fn type_paren_r_lst(config: &&Value, tag: &str) -> String {
  let value = get_attrib_value(&config, tag);
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(SATySFiTypeTerm::InlineText) => "}".to_string(),
    SATySFiType::List(SATySFiTypeTerm::BlockText) => ">".to_string(),
    SATySFiType::List(SATySFiTypeTerm::SATySFiString) => "```".to_string(),
    _ => "".to_string(),
  }
}

fn type_paren(t: &SATySFiType, text: &str) -> String {
  match t {
    SATySFiType::List(_) => format!("[{}]", &text),
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => format!("{{{0}}}", &text),
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => format!("'<{}>", &text),
    SATySFiType::Normal(SATySFiTypeTerm::SATySFiString) => format!("```{}```", &text),
    _ => text.to_string(),
  }
}

fn type_semicolon(config: &&Value, btag: &str) -> String {
  let value = get_attrib_value(&config, btag);
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => ";".to_string(),
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => ";".to_string(),
    SATySFiType::List(SATySFiTypeTerm::InlineText) => ";".to_string(),
    SATySFiType::List(SATySFiTypeTerm::BlockText) => ";".to_string(),
    _ => "".to_string(),
  }
}

fn get_attributes(vec: &Vec<attribute::OwnedAttribute>) -> Vec<(String, String)> {
  let mut l: Vec<(String, String)> = vec![];
  let len = vec.len();
  for i in 0..len {
    let v = &vec[i];
    let local_name = &v.name.local_name;
    let value = &v.value;
    l.push((local_name.to_string(), value.to_string()))
  }
  l
}

fn read_type(t: &&str) -> SATySFiType {
  match t {
    &"inline-text" => SATySFiType::Normal(SATySFiTypeTerm::InlineText),
    &"block-text" => SATySFiType::Normal(SATySFiTypeTerm::BlockText),
    &"string" => SATySFiType::Normal(SATySFiTypeTerm::SATySFiString),
    &"bool" => SATySFiType::Normal(SATySFiTypeTerm::SATySFiBool),
    &"int" => SATySFiType::Normal(SATySFiTypeTerm::SATySFiInt),
    &"float" => SATySFiType::Normal(SATySFiTypeTerm::SATySFiFloat),
    &"function" => SATySFiType::Normal(SATySFiTypeTerm::SATySFiFunction),
    &"inline-text list" => SATySFiType::List(SATySFiTypeTerm::InlineText),
    &"block-text list" => SATySFiType::List(SATySFiTypeTerm::BlockText),
    &"string list" => SATySFiType::List(SATySFiTypeTerm::SATySFiString),
    &"bool list" => SATySFiType::List(SATySFiTypeTerm::SATySFiBool),
    &"int list" => SATySFiType::List(SATySFiTypeTerm::SATySFiInt),
    &"float list" => SATySFiType::List(SATySFiTypeTerm::SATySFiFloat),
    &"function list" => SATySFiType::List(SATySFiTypeTerm::SATySFiFunction),
    _ => SATySFiType::Normal(SATySFiTypeTerm::SATySFiFunction),
  }
}

fn is_list(config: &&Value, btag: &str) -> bool {
  let value = get_attrib_value(&config, btag);
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let t = read_type(satysfi_type_str);
  match t {
    SATySFiType::List(_) => true,
    _ => false,
  }
}

fn get_attrib_value(config: &&Value, tag_name: &str) -> Value {
  let mut stack = json!({});
  for v in config.as_array().unwrap().iter() {
    let tag = &v["tag"].as_str().unwrap_or("");
    if &tag == &&tag_name {
      stack = v.clone()
    }
  }
  stack
}

fn make_attrib_string(
  config: &&Value,
  name: &str,
  local_attribs_lst: &Vec<(String, String)>,
) -> String {
  let value = get_attrib_value(&config, name);
  let attribs = &value["attribs"];
  let array_opt = attribs.as_array();
  let attribs_a_len = match array_opt {
    Some(a) => a.len(),
    None => 0,
  };
  let mut c: Vec<(&str, SATySFiType, u64)> = vec![];
  for i in 0..attribs_a_len {
    let tag = attribs[i]["tag"].as_str().unwrap_or(&"").trim();
    let st = attribs[i]["type"].as_str().unwrap_or(&"function").trim();
    let t = read_type(&st);
    let nm = attribs[i]["num"].as_u64().unwrap_or(0);
    c.push((tag, t, nm));
  }
  let len_opt = value["len"].as_u64();
  let len = match len_opt {
    Some(u) => u as usize,
    None => attribs_a_len,
  };
  let mut lst: Vec<Option<(&SATySFiType, String)>> = vec![None; len];
  let len = local_attribs_lst.len();
  for i in 0..len {
    let (tag, v) = &local_attribs_lst[i];
    let (_, t, n) = &mut c.iter().filter(|(x, _, _)| x == tag).next().unwrap_or(&(
      "",
      SATySFiType::Normal(SATySFiTypeTerm::SATySFiFunction),
      0,
    ));
    let n_usize = n.clone() as usize;
    let _ = if n <= &0 {
      None
    } else {
      lst.remove(n_usize - 1)
    };
    let _ = if n <= &0 {
      ()
    } else {
      lst.insert(n_usize - 1, Some((t, v.to_string())))
    };
  }
  let len = lst.len();
  let mut c = String::new();
  for i in 0..len {
    let op = &lst[i];
    let s = match op {
      Some((t, v)) => format!("(Some({}))", type_paren(t, v)),
      None => "(None)".to_string(),
    };
    c.push_str(&s)
  }
  c
}

fn top_lowercase(s: String) -> String {
  let mut main_str = String::new();
  let v: Vec<char> = s.chars().collect();
  let len = v.len();
  for i in 0..len {
    if i == 0 {
      let st = format!("{}", v[i]).to_lowercase();
      main_str.push_str(&st)
    } else {
      let st = format!("{}", v[i]);
      main_str.push_str(&st)
    }
  }
  main_str
}

fn to_cmd(config: &&Value, btag: &str, name: &str) -> String {
  let bvalue = get_attrib_value(&config, btag);
  let satysfi_type_str = &bvalue["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  let value = get_attrib_value(&config, name);
  let new_name = &value["rename"].as_str().unwrap_or(name);
  match satysfi_type {
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => format!("\\{}", new_name),
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => format!("+{}", new_name),
    SATySFiType::List(SATySFiTypeTerm::InlineText) => format!("\\{}", new_name),
    SATySFiType::List(SATySFiTypeTerm::BlockText) => format!("+{}", new_name),
    _ => top_lowercase(new_name.to_string()),
  }
}

fn escape(t: String) -> String {
  let s = &t
    .replace("\\", "\\\\")
    .replace("{", "\\{")
    .replace("}", "\\}")
    .replace("<", "\\<")
    .replace(">", "\\>")
    .replace("%", "\\%")
    .replace("$", "\\$")
    .replace("#", "\\#")
    .replace(";", "\\;")
    .replace("|", "\\|")
    .replace("*", "\\*")
    .replace("@", "\\@");
  s.to_string()
}

pub fn xml2string(xml: BufReader<std::fs::File>, data: &Value) -> String {
  let config_attrib_lst = &data["attrib"];

  let mut stack: Vec<String> = vec![String::new()];

  let parser = EventReader::new(xml);
  let mut xml_text = String::new();
  for e in parser {
    match e {
      Ok(XmlEvent::StartElement {
        name, attributes, ..
      }) => {
        let local_attribs_lst = get_attributes(&attributes);
        let name = format!("{}", name);
        println!("start {}", name);
        let a_s = make_attrib_string(&config_attrib_lst, &name, &local_attribs_lst);
        let btag = stack.iter().last().unwrap();
        let cmd = &to_cmd(&config_attrib_lst, btag, &name);
        let s = if is_list(&config_attrib_lst, &btag) {
          format!(
            "({}{}{}({}",
            type_paren_l_lst(&config_attrib_lst, &btag),
            cmd,
            a_s,
            type_paren_l(&config_attrib_lst, &name)
          )
        } else {
          format!("{}{}({}", cmd, a_s, type_paren_l(&config_attrib_lst, &name))
        };
        xml_text.push_str(&s);
        stack.push(name);
      }
      Ok(XmlEvent::EndElement { name }) => {
        let name = format!("{}", name);
        println!("end {}", name);
        let _ = stack.pop();
        let btag = stack.iter().last().unwrap();
        let s = if is_list(&config_attrib_lst, &btag) {
          format!(
            "{}){}{});",
            type_paren_r(&config_attrib_lst, &name),
            type_semicolon(&config_attrib_lst, &btag),
            type_paren_r_lst(&config_attrib_lst, &btag)
          )
        } else {
          format!(
            "{}){}",
            type_paren_r(&config_attrib_lst, &name),
            type_semicolon(&config_attrib_lst, &btag)
          )
        };
        xml_text.push_str(&s);
      }
      Ok(XmlEvent::CData(..)) => (),
      Ok(XmlEvent::ProcessingInstruction { .. }) => (),
      Ok(XmlEvent::Characters(text)) => {
        let t = format!("{}\n", &escape(text));
        xml_text.push_str(&t);
      }
      Err(e) => {
        panic!("Error: {}", e);
      }
      _ => {}
    }
  }

  return xml_text;
}
