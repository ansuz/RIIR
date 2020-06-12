#![cfg_attr(feature = "span", feature(proc_macro_span))]

use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
pub fn print_literally(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let string = print_ts(ts.into());
    let ts = quote! {
        fn main() {
            print!(#string);
        }
    };
    ts.into()
}

fn print_ts(ts: TokenStream) -> String {
    let mut output = String::new();

    #[cfg_attr(feature = "span", allow(unused_mut, unused_variables))]
    let mut was_hash = false;
    #[cfg_attr(not(feature = "span"), allow(unused_mut, unused_variables))]
    let mut last_span: Option<Span> = None;

    for tt in ts {
        #[cfg(feature = "span")]
        {
            if let Some(last_span) = last_span {
                let end = last_span.end();
                let start = tt.span().start();
                let end_col = if start.line > end.line {
                    let lines = start.line - end.line;
                    output.extend((0..lines).map(|_| '\n'));
                    0
                } else {
                    end.column
                };
                let space_size = start.column as isize - end_col as isize;
                output.extend((0..space_size).map(|_| ' '));
            }
        }
        #[cfg_attr(not(feature = "span"), allow(unused_assignments))]
        {
            last_span = Some(tt.span());
        }
        match tt {
            TokenTree::Ident(ident) => {
                #[cfg_attr(not(feature = "span"), allow(unused_mut))]
                let mut string = ident.to_string();
                #[cfg(feature = "span")]
                {
                    fill_space(ident.span(), &mut string);
                }
                #[cfg(not(feature = "span"))]
                {
                    string.push(' ');
                }
                output.push_str(&string);
            }
            TokenTree::Punct(punct) => {
                let mut string = punct.as_char().to_string();
                #[cfg(feature = "span")]
                {
                    fill_space(punct.span(), &mut string);
                }
                #[cfg(not(feature = "span"))]
                {
                    use proc_macro2::Spacing;

                    if punct.as_char() == '#' {
                        if !was_hash {
                            string = format!("\n{}", string);
                        }
                        was_hash = true;
                    }
                    if punct.spacing() == Spacing::Joint {
                        string.push(' ');
                    }
                }
                output.push_str(&string);
            }
            TokenTree::Literal(lit) => {
                #[cfg_attr(not(feature = "span"), allow(unused_mut))]
                let mut string = lit.to_string();
                #[cfg(feature = "span")]
                {
                    fill_space(lit.span(), &mut string);
                }
                output.push_str(&string);
            }
            TokenTree::Group(group) => {
                let mut string = String::new();
                let (l, r) = match group.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::None => ("", ""),
                };
                string.push_str(l);
                string.push_str(&print_ts(group.stream()));
                string.push_str(r);
                #[cfg(feature = "span")]
                {
                    fill_space(group.span(), &mut string);
                }
                output.push_str(&string);
            }
        }
    }
    output
}

#[cfg(feature = "span")]
fn fill_space(span: Span, string: &mut String) {
    let start_col = if span.end().line > span.start().line {
        let lines = span.end().line - span.start().line;
        string.extend((0..lines).map(|_| '\n'));
        -1
    } else {
        (span.start().column + string.len()) as isize
    };
    let space_size = span.end().column as isize - start_col;
    string.extend((0..space_size).map(|_| ' '));
}

#[cfg(test)]
#[test]
fn test() {
    use std::{fs, path::PathBuf};

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.txt");
    let content = fs::read_to_string(path).unwrap();
    let actual = print_ts(content.parse().unwrap());
    #[cfg(feature = "span")]
    {
        assert_eq!(actual, content.trim());
    }
    #[cfg(not(feature = "span"))]
    {
        // TODO write unit tests for non-span-sensitive code
    }
}
