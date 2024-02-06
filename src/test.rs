const TEST_DOC: &str = r##";FFMETADATA1
title=bike\\shed
;this is a comment
artist=FFmpeg troll team

[CHAPTER]
TIMEBASE=1/1000
START=0
#chapter ends at 0:01:00
END=60000
title=chapter \#1
[STREAM]
title=multi\
line
"##;

const RECREATED_DOC: &str = r##";FFMETADATA1
title=bike\\shed
artist=FFmpeg troll team

[CHAPTER]
TIMEBASE=1/1000
START=0
END=60000
title=chapter \#1

[STREAM]
title=multi\
line

"##;

use crate::FFMetadata;

#[test]
fn parse_section_header() {
  crate::section_header("[TEST]\n").unwrap();
}

#[test]
fn parse_kv() {
  crate::kv("test=ok\n").unwrap();
}

#[test]
fn parse_kv_escaped() {
  let (input, (k, v)) = crate::kv("test= ok\\\n\n").unwrap();
  assert_eq!(input, "");
  assert_eq!(k, String::from("test"));
  assert_eq!(v, String::from(" ok\n"));
}

#[test]
fn parse_simple() {
  let meta = FFMetadata::parse(";FFMETADATA1\ntest=doc\n").unwrap();
  assert_eq!(meta.global[0], (String::from("test"), String::from("doc")));
}

#[test]
fn parse_it() {
  let meta = FFMetadata::parse(TEST_DOC).unwrap();
  assert_eq!(format!("{}", meta), RECREATED_DOC);
}

#[test]
fn bad_magic() {
  assert!(FFMetadata::parse("\n;FFMETADATA1").is_err());
  assert!(FFMetadata::parse(";FFMETADATA\n").is_err());
}
