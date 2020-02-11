extern crate xml;

//use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute;
use std::vec;
use json;
use json::JsonValue;


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

fn type_paren_l (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(_) => {"[".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => {"{".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => {"'<".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::SATySFiString) => {"```".to_string()}
    _ => {"".to_string()}
  }
}


fn type_paren_r (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(_) => {"]".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => {"}".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => {">".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::SATySFiString) => {"```".to_string()}
    _ => {"".to_string()}
  }
}


fn type_paren_l_lst (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(SATySFiTypeTerm::InlineText) => {"{".to_string()}
    SATySFiType::List(SATySFiTypeTerm::BlockText) => {"'<".to_string()}
    SATySFiType::List(SATySFiTypeTerm::SATySFiString) => {"```".to_string()}
    _ => {"".to_string()}
  }
}


fn type_paren_r_lst (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::List(SATySFiTypeTerm::InlineText) => {"}".to_string()}
    SATySFiType::List(SATySFiTypeTerm::BlockText) => {">".to_string()}
    SATySFiType::List(SATySFiTypeTerm::SATySFiString) => {"```".to_string()}
    _ => {"".to_string()}
  }
}

fn type_paren (t:&SATySFiType, text:&str) -> String{
  match t {
    SATySFiType::List(_) => {format!("[{}]",&text)}
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => {format!("{{{0}}}", &text)}
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => {format!("'<{}>",&text)}
    SATySFiType::Normal(SATySFiTypeTerm::SATySFiString) => {format!("```{}```",&text)}
    _ => {text.to_string()}
  }
}


fn type_semicolon (config:&&JsonValue, btag:&str) -> String{
  let value = &config[btag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => {";".to_string()}
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => {";".to_string()}
    SATySFiType::List(SATySFiTypeTerm::InlineText) => {";".to_string()}
    SATySFiType::List(SATySFiTypeTerm::BlockText) => {";".to_string()}
    _ => {"".to_string()}
  }
}


fn get_attributes (vec:&Vec<attribute::OwnedAttribute>) -> Vec<(String, String)> {
  let mut l:Vec<(String, String)> = vec![];
  let len = vec.len();
  for i in  0 .. len {
    let v = &vec[i];
    let local_name = &v.name.local_name;
    let value = &v.value;
    l.push((local_name.to_string(), value.to_string()))
  }
  l
}


fn read_type (t:&&str) -> SATySFiType {
  match t {
    &"inline-text" => {SATySFiType::Normal(SATySFiTypeTerm::InlineText)}
    &"block-text" => {SATySFiType::Normal(SATySFiTypeTerm::BlockText)}
    &"string" => {SATySFiType::Normal(SATySFiTypeTerm::SATySFiString)}
    &"bool" => {SATySFiType::Normal(SATySFiTypeTerm::SATySFiBool)}
    &"int" => {SATySFiType::Normal(SATySFiTypeTerm::SATySFiInt)}
    &"float" => {SATySFiType::Normal(SATySFiTypeTerm::SATySFiFloat)}
    &"function" => {SATySFiType::Normal(SATySFiTypeTerm::SATySFiFunction)}
    &"inline-text list" => {SATySFiType::List(SATySFiTypeTerm::InlineText)}
    &"block-text list" => {SATySFiType::List(SATySFiTypeTerm::BlockText)}
    &"string list" => {SATySFiType::List(SATySFiTypeTerm::SATySFiString)}
    &"bool list" => {SATySFiType::List(SATySFiTypeTerm::SATySFiBool)}
    &"int list" => {SATySFiType::List(SATySFiTypeTerm::SATySFiInt)}
    &"float list" => {SATySFiType::List(SATySFiTypeTerm::SATySFiFloat)}
    &"function list" => {SATySFiType::List(SATySFiTypeTerm::SATySFiFunction)}
    _ => {SATySFiType::Normal(SATySFiTypeTerm::SATySFiFunction)}
  }
}

fn is_list (config:&&JsonValue, btag:&str) -> bool {
  let value = &config[btag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let t = read_type(satysfi_type_str);
  match t {
    SATySFiType::List(_) => {true}
    _ => {false}
  }
}

fn make_attrib_string (config:&&JsonValue, name:&str, local_attribs_lst:&Vec<(String, String)>) -> String {
  let value = &config[name];
  let attribs = &value["attribs"];
  let len = attribs.len();
  let mut c:Vec<(&str,SATySFiType,usize)> = vec![];
  for i in 0 .. len {
    let hoge:Vec<&str> = attribs[i].as_str().unwrap_or("").split(',').collect();
    let tag = hoge.iter().nth(0).unwrap_or(&"").trim();
    let st = hoge.iter().nth(1).unwrap_or(&"function").trim();
    let t = read_type(&st);
    let nm = hoge.iter().nth(2).unwrap_or(&"0").trim().parse().unwrap_or(0);
    c.push((tag,t,nm));
  };
  let len = value["len"].as_usize().unwrap_or(attribs.len());
  let mut lst:Vec<Option<(&SATySFiType,String)>> = vec![None; len];
  let len = local_attribs_lst.len();
  for i in 0 .. len {
    let (tag,v) = &local_attribs_lst[i];
    let (_,t,n) = &mut c.iter().filter( |(x,_,_)| x == tag).next().unwrap_or(&("",SATySFiType::Normal(SATySFiTypeTerm::SATySFiFunction),0));
    let _ = if n <= &0 {(None)} else {lst.remove(n - 1)};
    let _ = if n <= &0 {()} else {lst.insert(n - 1, Some((t,v.to_string())))};
  };
  let len = lst.len();
  let mut c = String::new();
  for i in 0 .. len {
    let op = &lst[i];
    let s =
      match op {
        Some((t,v)) => {format!("(Some({}))", type_paren(t, v))}
        None => {"(None)".to_string()}
      };
    c.push_str(&s)
  };
  c
}


fn to_cmd (config:&&JsonValue, btag:&str, name:&str) -> String {
  let bvalue = &config[btag];
  let satysfi_type_str = &bvalue["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  let value = &config[name];
  let new_name = &value["rename"].as_str().unwrap_or(name);
  match satysfi_type {
    SATySFiType::Normal(SATySFiTypeTerm::InlineText) => {format!("\\{}",new_name)}
    SATySFiType::Normal(SATySFiTypeTerm::BlockText) => {format!("+{}",new_name)}
    SATySFiType::List(SATySFiTypeTerm::InlineText) => {format!("\\{}",new_name)}
    SATySFiType::List(SATySFiTypeTerm::BlockText) => {format!("+{}",new_name)}
    _ => {new_name.to_string()}
  }
}

fn escape (t:String) -> String {
  let s =
    &t.replace("\\", "\\\\")
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
    .replace("@", "\\@")
    ;
  s.to_string()
}


pub fn xml2string (xml:BufReader<std::fs::File>, data:&JsonValue) -> String {

  let config_attrib_lst = &data["attrib"];

  let mut stack:Vec<String> = vec![String::new()];

  let parser = EventReader::new(xml);
  let mut xml_text = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_attribs_lst = get_attributes(&attributes);
                let name = format!("{}",name);
                let a_s = make_attrib_string(&config_attrib_lst,&name ,&local_attribs_lst);
                let btag = stack.iter().last().unwrap();
                let cmd = &to_cmd(&config_attrib_lst,btag,&name);
                let s = if is_list(&config_attrib_lst,&btag) {
                  format!("({}{}{}({}",type_paren_l_lst(&config_attrib_lst,&btag), cmd, a_s,type_paren_l(&config_attrib_lst,&name))
                } else {
                  format!("{}{}({}", cmd, a_s,type_paren_l(&config_attrib_lst,&name))
                };
                xml_text.push_str(&s);
                stack.push(name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                let name = format!("{}",name);
                let _ = stack.pop();
                let btag = stack.iter().last().unwrap();
                let s = if is_list(&config_attrib_lst,&btag) {
                  format!("{}){}{});",type_paren_r(&config_attrib_lst,&name), type_semicolon(&config_attrib_lst,&btag),type_paren_r_lst(&config_attrib_lst,&btag))
                } else {
                  format!("{}){}", type_paren_r(&config_attrib_lst,&name), type_semicolon(&config_attrib_lst,&btag))
                };
                xml_text.push_str(&s);
            }
            Ok(XmlEvent::CData(..)) => {
              ()
            }
            Ok(XmlEvent::ProcessingInstruction {..}) => {
              ()
            }
            Ok(XmlEvent::Characters(text)) => {
              let t = format!("{}\n",&escape(text));
              xml_text.push_str(&t);
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {}
        }
  };

  return xml_text;
}