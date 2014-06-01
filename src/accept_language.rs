// Goal: Parse "fr-FR,fr;q=0.8,en-US;q=0.6,en;q=0.4".

use std::string::String;
use std::ascii::OwnedStrAsciiExt;
use std::from_str::FromStr;
use std::option::collect;

/// A single component of a LanguageTag.  Must 1 to 8 ASCII alphabetic and
/// digit characters.  We preserve case, but must otherwise treat tags as
/// case-insensitive, according to RFC 3066.
struct Subtag(String);

/// Return a Subtag, or None if the input string is invalid.
fn subtag(s: &str) -> Option<Subtag> { from_str(s) }

impl FromStr for Subtag {
    fn from_str(s: &str) -> Option<Subtag> {
        if !(regex!(r"^[A-Za-z0-9]{1,8}$").is_match(s)) {
            return None
        }
        from_str(s).map(Subtag)
    }
}

impl Eq for Subtag {
    /// Subtags are compared in a case-insenstive fashion, as specified by RFC
    /// 3066.
    fn eq(&self, other: &Subtag) -> bool {
        let &Subtag(ref str1) = self;
        let &Subtag(ref str2) = other;
        // We can do better than this!
        str1.to_string().into_ascii_lower() ==
            str2.to_string().into_ascii_lower()
    }
}

#[test]
fn test_tag_from_str() {
    assert!(subtag("en") == Some(Subtag(String::from_str("en"))));
    assert!(subtag("x") == Some(Subtag(String::from_str("x"))));
    assert!(subtag("abcd1234") == Some(Subtag(String::from_str("abcd1234"))));
    assert!(subtag("") == None);
    assert!(subtag("abcd12345") == None);
}

#[test]
fn test_tag_eq() {
    assert!(subtag("en").unwrap() == subtag("en").unwrap());
    assert!(subtag("gb").unwrap() == subtag("GB").unwrap());
    assert!(subtag("en").unwrap() != subtag("fr").unwrap());
}

/// A tag describing a language, as defined in RFC 1766.
#[deriving(Eq)]
struct LanguageTag {
    components: Vec<Subtag>
}

/// Return a LanguageTag, or None if the input string is invalid.
fn language_tag(s: &str) -> Option<LanguageTag> { from_str(s) }

impl FromStr for LanguageTag {
    fn from_str(s: &str) -> Option<LanguageTag> {
        if !(regex!(r"^[A-Za-z]{1,8}(-[A-Za-z0-9]{1,8})*$").is_match(s)) {
            return None
        }
        let parsed: Option<Vec<Subtag>> = collect(s.split('-').map(subtag));
        parsed.map(|components| LanguageTag { components: components })
    }
}

#[test]
fn test_language_tag() {
    fn st(s: &str) -> Subtag { subtag(s).unwrap() }
    let lt = language_tag;

    assert!(lt("en").unwrap().components == vec!(st("en")));
    assert!(lt("fr-FR").unwrap().components == vec!(st("fr"), st("FR")));

    assert!(lt("").is_none());
    assert!(lt("en-").is_none());
    assert!(lt("123-US").is_none());
    assert!(lt("en-123").is_some());
}

/// An HTTP language range, as defined in RFC 3066.  This can be matched
/// against a LanguageTag.
#[deriving(Eq)]
enum LanguageRange {
    Prefix(LanguageTag),
    Wildcard
}

/// Return a LanguageTag, or None if the input string is invalid.
fn language_range(s: &str) -> Option<LanguageRange> { from_str(s) }

impl FromStr for LanguageRange {
    fn from_str(s: &str) -> Option<LanguageRange> {
        if s == "*" {
            return Some(Wildcard)
        }
        language_tag(s).map(Prefix)
    }
}

#[test]
fn test_language_range() {
    assert!(language_range("en").unwrap() ==
            Prefix(language_tag("en").unwrap()));
    assert!(language_range("*").unwrap() == Wildcard);
    assert!(language_range("en-").is_none());
}

impl LanguageRange {
    /// A LanguageRange matches if it's a wildcard, if it's a prefix of a
    /// tag (honoring subtag boundaries), or if it exactly matches a tag.
    fn matches(&self, tag: &LanguageTag) -> bool {
        match self {
            &Wildcard => true,
            &Prefix(ref pattern) => {
                let patc = pattern.components.as_slice();
                let tagc = tag.components.as_slice();
                if patc.len() > tagc.len() { return false; }
                patc == tagc.slice_to(patc.len())
            }
        }
    }
}

#[test]
fn test_language_range_matches() {
    let en = language_tag("en").unwrap();
    let fr = language_tag("fr").unwrap();
    let fr_FR = language_tag("fr-FR").unwrap();
    let fr_range = language_range("fr").unwrap();
    assert!(Wildcard.matches(&en));
    assert!(!fr_range.matches(&en));
    assert!(fr_range.matches(&fr));
    assert!(fr_range.matches(&fr_FR));
}


//=========================================================================
// Everything below this line is very much a work in progress.

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
