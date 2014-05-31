use std::string::String;
use std::ascii::OwnedStrAsciiExt;

/// A single component of a LanguageTag.  Must contain '1*8ALPHA', that is,
/// 1 to 8 ASCII alphabetic characters.  We preserve case, but must
/// otherwise treat tags as case-insensitive, according to RFC 1766.
struct Tag(String);

impl Eq for Tag {
    /// Tags are compared in a case-insenstive fashion, as specified by RFC
    /// 1766.
    fn eq(&self, other: &Tag) -> bool {
        // We can do better than this!
        let &Tag(ref str1) = self;
        let &Tag(ref str2) = other;
        str1.to_string().into_ascii_lower() ==
            str2.to_string().into_ascii_lower()
    }
}

#[test]
fn test_tag_eq() {
    assert!(Tag(String::from_str("en")) == Tag(String::from_str("en")));
    assert!(Tag(String::from_str("gb")) == Tag(String::from_str("GB")));
    assert!(Tag(String::from_str("en")) != Tag(String::from_str("fr")));
}

/// An HTTP quality value, as defined in RFC 2616.  A floating point number
/// from 0 to 1, inclusive, with up to three digits of precision after the
/// decimal point.  The decimal point and trailing digits are optional.
struct QValue(f32);

/// A tag describing a language, as defined in RFC 1766.
struct LanguageTag(Tag,Vec<Tag>);

/// An HTTP language range, as defined in RFC 2616.  This can be matched
/// against a LanguageTag.
enum LanguageRange {
    Tags(Tag, Vec<Tag>),
    Wildcard
}

