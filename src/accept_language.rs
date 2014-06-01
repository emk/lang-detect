// Goal: Parse "fr-FR,fr;q=0.8,en-US;q=0.6,en;q=0.4".

use std::string::String;
use std::ascii::OwnedStrAsciiExt;

/// A single component of a LanguageTag.  Must contain '1*8ALPHA', that is,
/// 1 to 8 ASCII alphabetic characters.  We preserve case, but must
/// otherwise treat tags as case-insensitive, according to RFC 1766.
struct Tag(String);

impl Tag {
    fn from_str(s: &str) -> Tag { Tag(String::from_str(s)) }
}

impl Eq for Tag {
    /// Tags are compared in a case-insenstive fashion, as specified by RFC
    /// 1766.
    fn eq(&self, other: &Tag) -> bool {
        let &Tag(ref str1) = self;
        let &Tag(ref str2) = other;
        // We can do better than this!
        str1.to_string().into_ascii_lower() ==
            str2.to_string().into_ascii_lower()
    }
}

#[test]
fn test_tag_eq() {
    assert!(Tag::from_str("en") == Tag::from_str("en"));
    assert!(Tag::from_str("gb") == Tag::from_str("GB"));
    assert!(Tag::from_str("en") != Tag::from_str("fr"));
}

/// A tag describing a language, as defined in RFC 1766.
struct LanguageTag {
    components: Vec<Tag>
}

impl LanguageTag {
    fn from_str(s: &str) -> LanguageTag {
        let v: Vec<Tag> = s.split('-').map(|t| Tag::from_str(t)).collect();
        LanguageTag { components: v }
    }
}

#[test]
fn test_language_tag() {
    let en = LanguageTag::from_str("en");
    assert!(en.components == vec!(Tag::from_str("en")));
    let fr_fr = LanguageTag::from_str("fr-FR");
    assert!(fr_fr.components == vec!(Tag::from_str("fr"), Tag::from_str("FR")));
}

/// An HTTP language range, as defined in RFC 2616.  This can be matched
/// against a LanguageTag.
enum LanguageRange {
    Tags(Tag, Vec<Tag>),
    Wildcard
}

/// An HTTP quality value, as defined in RFC 2616.  A floating point number
/// from 0 to 1, inclusive, with up to three digits of precision after the
/// decimal point.  The decimal point and trailing digits are optional.
struct QValue(f32);

impl QValue {
    fn from_str(s: &str) -> QValue {
        let f: f32 = from_str(s).unwrap();
        QValue(f)
    }
    fn to_f32(&self) -> f32 { let &QValue(f) = self; f }
}

#[test]
fn test_qvalue() {
    assert!(QValue::from_str("0").to_f32() == 0.0);
    assert!(QValue::from_str("0.").to_f32() == 0.0);
    assert!(QValue::from_str("0.000").to_f32() == 0.0);
    assert!(QValue::from_str("0.5").to_f32() == 0.5);
    assert!(QValue::from_str("1").to_f32() == 1.0);
}
