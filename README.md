[![Rust](https://github.com/prfiredragon/version/actions/workflows/rust.yml/badge.svg)](https://github.com/prfiredragon/version/actions/workflows/rust.yml)
[![DevSkim](https://github.com/prfiredragon/version/actions/workflows/devskim.yml/badge.svg)](https://github.com/prfiredragon/version/actions/workflows/devskim.yml)
# version
`version` is a very simple library who's job is to return the version of your crate if you're building with Cargo.

## Usage:
```rust
#[macro_use]
extern crate version;

use version::Version;

// ...

version!() // Returns something like "1.0.0"

let ver : Version = FromStr::from_str( version!() ).unwrap();

```


## Usage:
```rust
use version::{compare_version::*, Version, Cmp::*};

// ...

#[test]
fn one_digit_test() {
  let invv : Result<Version, String> = FromStr::from_str( "7" );
  assert!( invv.is_ok() );

  let ver = FromStr::from_str( "7" );
  assert_eq!( ver, Ok( Version { major: 7, minor: 0, patch: 0 } ) );
  println!("Results: {:?}", ver);
  
}
#[test]
fn test_gt_two_poss_version() {

  let a: Version  = FromStr::from_str("0.6").unwrap();
  let b: Version  = FromStr::from_str("0.1").unwrap();
  println!("{:?}", a);
  println!("{:?}", b);
  
  assert!(compare_version::from_version(&a,Gt, &b));
}
#[test]
fn test_gt_two_poss() {
    assert!(compare_version::from_str("0.6",Gt, "0.1"));
}

```


## Notes:
This only works if you're building with Cargo since the macro fetches the version digits from enviroment variables set by Cargo ( `CARGO_PKG_VERSION_{MAJOR, MINOR, PATCH}` ).
