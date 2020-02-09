extern crate nom;

use nom::{Err, IResult};
use nom::branch::permutation;
use nom::character::complete::{alpha1, char, multispace0, multispace1, digit1, anychar};
use nom::bytes::complete::{tag};
use nom::error::ErrorKind;
use nom::multi::separated_list;
use nom::sequence::delimited;

#[derive(Debug,PartialEq)]
pub struct Term {
  pub require: Vec<String>,
  pub import: Vec<String>,
}

#[derive(Debug,PartialEq)]
pub enum SATySFiType {
  BlockText,
  InlineText,
  SATySFiString,
  SATySFiBool,
  SATySFiInt,
  SATySFiFloat,
  SATySFiFunction,
}


fn get_satysfi_type (s:&str) -> IResult<&str, SATySFiType> {
  let t =
    delimited(
      char('('),
      delimited(multispace0, alpha1, multispace0),
      char(')'),
    )(s)?;
  match t {
    (_,"inline-text") => {
      Ok((s,SATySFiType::InlineText))
    }
    (_,"block-text") => {
      Ok((s,SATySFiType::BlockText))
    }
    (_,"string") => {
      Ok((s,SATySFiType::SATySFiString))
    }
    (_,"int") => {
      Ok((s,SATySFiType::SATySFiInt))
    }
    (_,"float") => {
      Ok((s,SATySFiType::SATySFiFloat))
    }
    (_,"bool") => {
      Ok((s,SATySFiType::SATySFiBool))
    }
    (_,"function") => {
      Ok((s,SATySFiType::SATySFiFunction))
    }
    _ => {
      panic!("Error: {}", "error");
    }
  }
}


fn get_int (s:&str) -> IResult<&str, &str> {
  delimited(
    char('('),
    delimited(multispace0, digit1, multispace0),
    char(')'),
  )(s)
}


fn list(s: &str) -> IResult<&str, Vec<&str>> {
  delimited(
    char('['),
    delimited(
      multispace0,
      separated_list(permutation((multispace0, char(';'), multispace0)), alpha1),
      multispace0,
    ),
    char(']'),
  )(s)
}


fn read_require (s: &str) -> IResult<&str, Vec<&str>> {
  let (s,_) = tag("require")(s)?;
  let (s,_) = multispace0(s)?;
  list(s)
}


fn read_import (s: &str) -> IResult<&str, Vec<&str>> {
  let (s,_) = tag("import")(s)?;
  let (s,_) = multispace0(s)?;
  list(s)
}


fn read_attrib (s: &str) -> IResult<&str, Vec<&str>> {
  let (s,_) = tag("attrib")(s)?;
  let (s,_) = multispace0(s)?;
  list(s)
}


fn delete_space (s:&str) -> IResult<&str, &str> {
 multispace0(s)
}

pub fn config_parser (text:&str) {
  let (s1,import_lst) = read_import(text).unwrap();
  let (s2,require_lst) = read_require(text).unwrap();
  println!("{}",s1);
}

pub fn test_parser (text:&str) {
  let (s0,_) = delete_space(text).unwrap();
  let (s1,require_lst) = read_require(s0).unwrap();
  let (s1_1,_) = delete_space(s1).unwrap();
  let (s2,import_lst) = read_import(s1_1).unwrap();
  let (s2_1,_) = delete_space(s2).unwrap();
  let (s3,attrib_lst) = read_attrib(s2_1).unwrap();
  println!("s1:{}",s1);
  println!("require_lst:{:?}",require_lst);
  println!("s2:{}",s2);
  println!("import_lst:{:?}",import_lst);
  println!("s3:{}",s3);
  println!("attrib_lst:{:?}",attrib_lst);
}