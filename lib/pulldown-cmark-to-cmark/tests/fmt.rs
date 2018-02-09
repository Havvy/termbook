extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark_to_cmark::fmt::{cmark, State};
use pulldown_cmark::{Event, Options, Parser, Tag};

fn fmts(s: &str) -> (String, State) {
    let mut buf = String::new();
    let s = cmark(Parser::new_ext(s, Options::all()), &mut buf, None).unwrap();
    (buf, s)
}

fn fmtss(s: &str, state: State) -> (String, State) {
    let mut buf = String::new();
    let s = cmark(Parser::new_ext(s, Options::all()), &mut buf, Some(state)).unwrap();
    (buf, s)
}

fn fmtes(e: &[Event], s: State) -> (String, State) {
    let mut buf = String::new();
    let s = cmark(e.iter(), &mut buf, Some(s)).unwrap();
    (buf, s)
}

mod lazy_newlines {
    use super::{fmtes, fmts};
    use super::{Event, State, Tag};

    #[test]
    fn after_emphasis_there_is_no_newline() {
        for t in &[
            Tag::Emphasis,
            Tag::Strong,
            Tag::Code,
            Tag::List(None),
            Tag::List(Some(1)),
            Tag::BlockQuote,
            Tag::Link("".into(), "".into()),
            Tag::Image("".into(), "".into()),
            Tag::FootnoteDefinition("".into()),
        ] {
            assert_eq!(
                fmtes(&[Event::End(t.clone())], State::default()).1,
                State {
                    newlines_before_start: 0,
                    ..Default::default()
                }
            )
        }
    }

    #[test]
    fn after_anything_else_it_has_one_newline() {
        for e in &[
            Event::End(Tag::CodeBlock("".into())),
            Event::End(Tag::Item),
            Event::End(Tag::Rule),
            Event::End(Tag::TableRow),
            Event::End(Tag::TableHead),
            Event::End(Tag::Table(vec![])),
        ] {
            assert_eq!(
                fmtes(&[e.clone()], State::default()).1,
                State {
                    newlines_before_start: 1,
                    ..Default::default()
                }
            )
        }
    }

    #[test]
    fn after_some_types_it_has_multiple_newlines() {
        for md in &["paragraph", "## headline"] {
            assert_eq!(
                fmts(md),
                (
                    String::from(*md),
                    State {
                        newlines_before_start: 2,
                        ..Default::default()
                    }
                )
            )
        }
    }
}

#[test]
fn it_applies_newlines_before_start_before_html() {
    assert_eq!(
        fmtes(
            &[Event::Html("<e>".into())],
            State {
                newlines_before_start: 2,
                ..Default::default()
            }
        ),
        (
            "\n\n<e>".into(),
            State {
                newlines_before_start: 0,
                ..Default::default()
            }
        )
    )
}

#[test]
fn it_applies_newlines_before_start_before_any_start_tag() {
    assert_eq!(
        fmtes(
            &[Event::Start(Tag::Paragraph), Event::Text("h".into())],
            State {
                newlines_before_start: 2,
                ..Default::default()
            }
        ),
        (
            "\n\nh".into(),
            State {
                newlines_before_start: 0,
                ..Default::default()
            }
        )
    )
}
