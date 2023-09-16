use proc_macro2::Literal;
use quote::{ToTokens, TokenStreamExt};
use syn::{parse::Parse, LitStr, Result};

fn name2ansi(name: &str) -> Option<&'static str> {
    Some(match name {
        "black" => "\x1b[0;34;30m",
        "red" => "\x1b[0;34;31m",
        "green" => "\x1b[0;34;32m",
        "yellow" => "\x1b[0;34;33m",
        "blue" => "\x1b[0;34;34m",
        "magenta" => "\x1b[0;34;35m",
        "cyan" => "\x1b[0;34;36m",
        "white" => "\x1b[0;34;37m",
        "default" => "\x1b[0;34;39m",

        "bold_black" => "\x1b[1;34;30m",
        "bold_red" => "\x1b[1;34;31m",
        "bold_green" => "\x1b[1;34;32m",
        "bold_yellow" => "\x1b[1;34;33m",
        "bold_blue" => "\x1b[1;34;34m",
        "bold_magenta" => "\x1b[1;34;35m",
        "bold_cyan" => "\x1b[1;34;36m",
        "bold_white" => "\x1b[1;34;37m",
        "bold_default" => "\x1b[1;34;39m",

        "on_black_bold" => "\x1b[1;34;40m",
        "on_red_bold" => "\x1b[1;34;41m",
        "on_green_bold" => "\x1b[1;34;42m",
        "on_yellow_bold" => "\x1b[1;34;43m",
        "on_blue_bold" => "\x1b[1;34;44m",
        "on_magenta_bold" => "\x1b[1;44;35m",
        "on_cyan_bold" => "\x1b[1;34;46m",
        "on_white_bold" => "\x1b[1;34;47m",
        "on_default_bold" => "\x1b[1;34;49m",

        "on_black" => "\x1b[0;34;40m",
        "on_red" => "\x1b[0;34;41m",
        "on_green" => "\x1b[0;34;42m",
        "on_yellow" => "\x1b[0;34;43m",
        "on_blue" => "\x1b[0;34;44m",
        "on_magenta" => "\x1b[0;44;35m",
        "on_cyan" => "\x1b[0;34;46m",
        "on_white" => "\x1b[0;34;47m",
        "on_default" => "\x1b[0;34;49m",

        "reset" => "\x1b[0m",
        "dim" => "\x1b[2m",
        "italic" => "\x1b[3m",
        "underline" => "\x1b[24m",
        "blinking" => "\x1b[5m",
        "hide" => "\x1b[8m",
        "strike" => "\x1b[9m",
        "bold" => "\x1b[1m",
        _ => return None,
    })
}

pub struct CFStr(String);

impl Parse for CFStr {
    fn parse(stream: syn::parse::ParseStream) -> Result<Self> {
        let input = stream.parse::<LitStr>()?.value();
        let mut chars = input.chars().peekable();
        let mut temp = String::new();
        let mut out = String::new();
        while let Some(ch) = chars.next() {
            match ch {
                '{' => {
                    match chars.next() {
                        Some('{') => {
                            out.push('{');
                            continue;
                        }
                        Some('}') => {
                            out.push('{');
                            out.push('}');
                            continue;
                        }
                        Some(ch) => temp.push(ch),
                        None => return Err(stream.error("unexpected eof")),
                    }
                    for ch in chars.by_ref() {
                        match ch {
                            '}' => {
                                if let Some(a) = name2ansi(&temp) {
                                    out.push_str(a);
                                    temp.clear();
                                    break;
                                } else if let Some((b, a)) = temp.split_once(':') {
                                    if let Some(a) = name2ansi(a) {
                                        out.push_str(name2ansi("reset").unwrap());
                                        out.push_str(a);
                                        out.push('{');
                                        out.push_str(b);
                                        out.push('}');
                                        out.push_str(name2ansi("reset").unwrap());
                                        temp.clear();
                                        break;
                                    }
                                }
                                out.push('{');
                                out.push_str(&temp);
                                out.push('}');
                                temp.clear();
                                break;
                            }
                            t => temp.push(t),
                        }
                    }
                }
                '}' => match chars.next() {
                    Some('}') => {
                        out.push('}');
                        continue;
                    }
                    _ => return Err(stream.error("unexpected text")),
                },
                c => out.push(c),
            }
        }
        Ok(Self(out))
    }
}

impl ToTokens for CFStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Literal::string(&self.0));
    }
}
