extern crate xml;

//use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute;
use std::vec;
use json;
use json::JsonValue;


#[derive(Debug)]
enum SATySFiType {
  BlockText,
  InlineText,
  SATySFiString,
  SATySFiBool,
  SATySFiInt,
  SATySFiFloat,
  SATySFiFunction,
}

fn type_paren_l (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::InlineText => {"{".to_string()}
    SATySFiType::BlockText => {"'<".to_string()}
    SATySFiType::SATySFiString => {"```".to_string()}
    _ => {"".to_string()}
  }
}


fn type_paren_r (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::InlineText => {"}".to_string()}
    SATySFiType::BlockText => {">".to_string()}
    SATySFiType::SATySFiString => {"```".to_string()}
    _ => {"".to_string()}
  }
}


fn type_paren (t:&SATySFiType, text:&str) -> String{
  match t {
    SATySFiType::InlineText => {format!("{{{0}}}", &text)}
    SATySFiType::BlockText => {format!("'<{}>",&text)}
    SATySFiType::SATySFiString => {format!("```{}```",&text)}
    _ => {text.to_string()}
  }
}

fn type_semicolon (config:&&JsonValue, tag:&str) -> String{
  let value = &config[tag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::InlineText => {";".to_string()}
    SATySFiType::BlockText => {";".to_string()}
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
    &"inline-text" => {SATySFiType::InlineText}
    &"block-text" => {SATySFiType::BlockText}
    &"string" => {SATySFiType::SATySFiString}
    &"bool" => {SATySFiType::SATySFiBool}
    &"int" => {SATySFiType::SATySFiInt}
    &"float" => {SATySFiType::SATySFiFloat}
    &"function" => {SATySFiType::SATySFiFunction}
    _ => {SATySFiType::SATySFiFunction}
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
    let (_,t,n) = &mut c.iter().filter( |(x,_,_)| x == tag).next().unwrap_or(&("",SATySFiType::SATySFiFunction,0));
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
  let value = &config[btag];
  let satysfi_type_str = &value["type"].as_str().unwrap_or("function");
  let satysfi_type = read_type(satysfi_type_str);
  match satysfi_type {
    SATySFiType::InlineText => {format!("\\{}",name)}
    SATySFiType::BlockText => {format!("+{}",name)}
    _ => {name.to_string()}
  }
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
                xml_text.push_str(&format!("{}{}({}", cmd, a_s,type_paren_l(&config_attrib_lst,&name)));
                stack.push(name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                let name = format!("{}",name);
                xml_text.push_str(&format!("{}){}", type_paren_r(&config_attrib_lst,&name), type_semicolon(&config_attrib_lst,&name)));
                stack.pop();
            }
            Ok(XmlEvent::CData(text)) => {
              xml_text.push_str(&text);
            }
            Ok(XmlEvent::ProcessingInstruction {name, ..}) => {
              let t = format!("{}\n",name);
              xml_text.push_str(&t);
            }
            Ok(XmlEvent::Characters(text)) => {
              let t = format!("{}\n",&text);
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