#![allow(unused_imports)]
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
  pub major : u32,
  pub minor : u32,
  pub patch : u32
}

impl Display for Version {
  fn fmt( &self, fmtr : &mut fmt::Formatter ) -> fmt::Result {
    write!( fmtr, "{}.{}.{}", self.major, self.minor, self.patch )
  }
}

impl FromStr for Version {
  type Err = String;

  fn from_str( s : &str ) -> Result<Version, Self::Err> {
    let strings = "0.0.0";
    let mut parts: Vec<Result<u32, &str>> = 
    strings.split( '.' )
    .map( | elm | elm.parse::<u32>()
                           .map_err( |_| elm ) )
    .collect();

    //println!("Results: {:?}", parts);
    let partst : Vec<Result<u32, &str>> =
      s.split( '.' )
      .map( | elm | elm.parse::<u32>()
                             .map_err( |_| elm ) )
      .collect();

    if partst.len() == 1 {
      let parta: Result<u32, &str> = partst[0];
      let partb: Result<u32, &str> = parts[1];
      let partc: Result<u32, &str> = parts[2];
      let parthole: Vec<Result<u32, &str>> = vec![parta, partb, partc];
      parts = parthole;
    }else if partst.len() == 2 {
      let parta: Result<u32, &str> = partst[0];
      let partb: Result<u32, &str> = partst[1];
      let partc: Result<u32, &str> = parts[2];
      let parthole: Vec<Result<u32, &str>> = vec![parta, partb, partc];
      parts = parthole;
    }else{
      parts = partst;
    }

    if parts.len() != 3 {
      return
        Err( format!( "Invalid version format: expected 3 components, got {}."
           , parts.len() ) );
    }

    for part in &parts {
      match part {
        &Err( err ) =>
          return
            Err( format!( "Invalid version format: expected integer, got '{}'."
                         , err ) ),
        _ => {}
      }
    }

    Ok( Version {
      major: parts[0].unwrap(),
      minor: parts[1].unwrap(),
      patch: parts[2].unwrap()
    } )
  }
}

/// Gets the current version as a string.
#[macro_export]
macro_rules! version(
  () => ( env!( "CARGO_PKG_VERSION" ) )
);



/// Comparison operators enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cmp {
    /// Equal (`==`, `=`).
    /// When version `A` is equal to `B`.
    Eq,

    /// Not equal (`!=`, `!`, `<>`).
    /// When version `A` is not equal to `B`.
    Ne,

    /// Less than (`<`).
    /// When version `A` is less than `B` but not equal.
    Lt,

    /// Less or equal (`<=`).
    /// When version `A` is less than or equal to `B`.
    Le,

    /// Greater or equal (`>=`).
    /// When version `A` is greater than or equal to `B`.
    Ge,

    /// Greater than (`>`).
    /// When version `A` is greater than `B` but not equal.
    Gt,
}
use Cmp::*;

pub fn compare_version(_a: &str, test: Cmp, _b: &str) -> bool{
    match test {
        Cmp::Eq => {if _a == _b {return true;}},
        Cmp::Ne => {if _a != _b {return true;}},
        Cmp::Lt => {if _a < _b {return true;}},
        Cmp::Le => {if _a <= _b {return true;}},
        Cmp::Ge => {if _a >= _b {return true;}},
        Cmp::Gt => {if _a > _b {return true;}},
    }
    return false;
}

#[test]
fn string_test() {
  // Bad test is bad.
  assert_eq!( version!(), "3.0.0" );
}
#[test]
fn version_struct_test() {
    let ver = FromStr::from_str( &version!() );
  assert_eq!( ver, Ok( Version { major: 3, minor: 0, patch: 0 } ) );
}
#[test]
fn invalid_test() {
    let invalids = [ "nope", "1.0", "1.0.x", "1.x.0", "x.0.1", "x.x.x" ];

  for invalid in &invalids {
    let invv : Result<Version, String> = FromStr::from_str( invalid );
    assert!( invv.is_err() );
  }
}
#[test]
fn mayor_test() {
    let mut test_ret = false;
    if version!() > "0.0.1" {test_ret = true;}

    assert_eq!( true, test_ret );
}
#[test]
fn minor_test() {
    let mut test_ret = false;
    if version!() < "3.0.1" {test_ret = true;}

    assert_eq!( true, test_ret );
}
#[test]
fn equal_test() {
    let mut test_ret = false;
    if version!() == "3.0.0" {test_ret = true;}

    assert_eq!( true, test_ret );
}
#[test]
fn test_eq() {
    assert!(compare_version("0.1.0",Eq, "0.1.0"));
}
#[test]
fn test_ge() {
    assert!(compare_version("0.1.1",Ge, "0.1.0"));
}
#[test]
fn test_le() {
    assert!(compare_version("0.0.9",Le, "0.1.0"));
}
#[test]
fn test_ne() {
    assert!(compare_version("1.0.0",Ne, "0.1.0"));
}
#[test]
fn test_lt() {
    assert!(compare_version("0.0.9",Lt, "0.1.0"));
}
#[test]
fn test_gt() {
    assert!(compare_version("0.1.9",Gt, "0.1.0"));
}
#[test]
fn test_gt2() {
    assert!(compare_version("0.6",Gt, "0.1"));
}
/*
#[test]
fn does_it_work() {
  let ver = FromStr::from_str( &version!() );
  assert_eq!( ver, Ok( Version { major: 2, minor: 0, patch: 1 } ) );

  let invalids = [ "nope", "1.0", "1.0.x", "1.x.0", "x.0.1", "x.x.x" ];

  for invalid in &invalids {
    let invv : Result<Version, String> = FromStr::from_str( invalid );
    assert!( invv.is_err() );
  }

  // Bad test is bad.
  assert_eq!( version!(), "2.0.1" );
}
*/
#[test]
fn two_digit_test() {
  let invv : Result<Version, String> = FromStr::from_str( "7.2" );
  assert!( invv.is_ok() );

  let ver = FromStr::from_str( "7.2" );
  assert_eq!( ver, Ok( Version { major: 7, minor: 2, patch: 0 } ) );
  println!("Results: {:?}", ver);
  //println!("{}",invv.unwrap_err());
  
}
#[test]
fn one_digit_test() {
  let invv : Result<Version, String> = FromStr::from_str( "7" );
  assert!( invv.is_ok() );

  let ver = FromStr::from_str( "7" );
  assert_eq!( ver, Ok( Version { major: 7, minor: 0, patch: 0 } ) );
  println!("Results: {:?}", ver);
  //println!("{}",invv.unwrap_err());
  
}