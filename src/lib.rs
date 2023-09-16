//! smart crate for terminal coloring.
//!
//! uses macros instead of methods.
//!
//! ## usage
//!
//! heres how it works:
//! ```rust
//! # use comat::cprintln;
//! # use std::time::{Duration, Instant};
//! cprintln!("the traffic light is {bold_red}red.{reset}");
//! cprintln!("the traffic light will be {green}green{reset} at {:?}.", Instant::now() + Duration::from_secs(40));
//! ```
//!
//! ## why you should use comat instead of {yansi, owo_colors, colored, ..}
//!
//! - no method pollution, your intellisense remains fine
//! - compact: shorter than even raw ansi. see:
//!   ```
//!   # use comat::cprint;
//!   # let thing = 0;
//!   cprint!("{thing:red}.");
//!   ```
//!   vs
//!   ```
//!   print!("\x1b[0;34;31mred\x1b[0m.");
//!   ```
//!   vs
//!   ```ignore
//!   print!("{}.", "red".red());
//!   ```
//! - intuitive: you dont have to
//!   ```ignore
//!   println!("{} {} {}", thing1.red().on_blue(), thing2.red().on_blue(), thing3.italic());.
//!   ```
//!   instead, simply
//!   ```
//!   # use comat::cprintln;
//!   # let thing1 = 0; let thing2 = 5; let thing3 = 4;
//!   cprintln!("{red}{on_blue}{thing1} {thing2} {thing3:italic}");
//!   ```
//!
//! ## syntax
//!
//! `{{` gives you a `{`, to get a `{{` use `{{{{`.
//!
//! `{color}` adds that effect/color to the string. it does not reset afterwards.
//!
//! if the color inside a `{}` is not found, it doesnt touch the block, for convenience.
//!
//! `{thing:color}` will reset everything before the block, color it, and reset that color. similar to `thing.color()` with other libs.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::dbg_macro, missing_docs)]
use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, Result, Token};

mod cfstr;
use cfstr::CFStr;

#[proc_macro]
/// Macro that simply modifies the format string to have colors.
/// Mostly for testing. Use [`cformat_args!`] instead where possible.
pub fn comat(input: TokenStream) -> TokenStream {
    let str = parse_macro_input!(input as CFStr);
    str.to_token_stream().into()
}

struct One {
    cfstr: CFStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for One {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let cfstr = input.parse::<CFStr>()?;
        let _ = input.parse::<Token![,]>();
        Ok(Self {
            cfstr,
            args: Punctuated::<Expr, Token![,]>::parse_terminated(input)?,
        })
    }
}

impl ToTokens for One {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.cfstr.to_tokens(tokens);
        tokens.append(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone));
        self.args.to_tokens(tokens);
    }
}

// NOTE: many of these can be made as decl macros, but decl macros can't be exported from proc macro crates yet.

#[proc_macro]
/// Print text, colorfully, to stdout, with a newline.
///
/// See also [`println`].
/// ```
/// # use comat::*;
/// let magic = 4;
/// cprintln!("{red}look its red{reset}! {bold_blue}{magic}{reset} is the magic number!");
/// ```
pub fn cprintln(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as One);
    quote! { println!(#f) }.into()
}

#[proc_macro]
/// Print text, colorfully, to stdout, without a newline.
///
/// See also [`print`].
/// ```
/// # use comat::*;
/// cprint!("{yellow}i am a warning. {reset}why do you dislike me?");
/// ```
pub fn cprint(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as One);
    quote! { print!(#f) }.into()
}

#[proc_macro]
/// Format text, colorfully.
///
/// See also [`format`].
/// ```
/// # use comat::*;
/// let favorite_thing = "teddy bears";
/// let message = cformat!("the {red}bogeymen{reset} will get your {favorite_thing:underline}");
/// # assert_eq!(message, "the \x1b[0;34;31mbogeymen\x1b[0m will get your \x1b[0m\x1b[24mteddy bears\x1b[0m");
/// ```
pub fn cformat(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as One);
    quote! { format!(#f) }.into()
}

#[proc_macro]
/// Produce [`fmt::Arguments`](std::fmt::Arguments). Sometimes functions take these.
///
/// See also [`format_args`].
/// ```
/// # use comat::*;
/// let args = cformat_args!("{bold_red}fatal error. {reset}killing {blue}everything{reset}");
/// // NOTE: do not do this. instead use cprintln.
/// println!("{}", args);
pub fn cformat_args(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as One);
    quote! { format_args!(#f) }.into()
}
/// Colorfully panic.
///
/// See also [`panic`].
/// ```should_panic
/// # use comat::cpanic;
/// cpanic!("why is the bound {red}bad");
/// ```
#[proc_macro]
pub fn cpanic(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as One);
    quote! { panic!(#f) }.into()
}

struct Two {
    a: Expr,
    cfstr: CFStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Two {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let a = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let cfstr = input.parse::<CFStr>()?;
        let _ = input.parse::<Token![,]>();
        Ok(Self {
            a,
            cfstr,
            args: Punctuated::<Expr, Token![,]>::parse_terminated(input)?,
        })
    }
}

impl ToTokens for Two {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.a.to_tokens(tokens);
        tokens.append(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone));
        self.cfstr.to_tokens(tokens);
        tokens.append(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone));
        self.args.to_tokens(tokens);
    }
}

#[proc_macro]
/// Write to a buffer colorfully, with no newline.
///
/// See also [`write`]
/// ```
/// # use comat::cwrite;
/// use std::io::Write;
/// let mut buf = vec![];
/// cwrite!(buf, "{green}omg there's going to be ansi sequences in a {black}Vec<u8>{reset}!");
/// # assert_eq!(buf, [27, 91, 48, 59, 51, 52, 59, 51, 50, 109, 111, 109, 103, 32, 116, 104, 101, 114, 101, 39, 115, 32, 103, 111, 105, 110, 103, 32, 116, 111, 32, 98, 101, 32, 97, 110, 115, 105, 32, 115, 101, 113, 117, 101, 110, 99, 101, 115, 32, 105, 110, 32, 97, 32, 27, 91, 48, 59, 51, 52, 59, 51, 48, 109, 86, 101, 99, 60, 117, 56, 62, 27, 91, 48, 109, 33]);
/// ```
pub fn cwrite(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as Two);
    quote! { write!(#f) }.into()
}

#[proc_macro]
/// Write to a buffer colorfully, with newline.
///
/// See also [`writeln`]
/// ```
/// # use comat::cwriteln;
/// use std::io::Write;
/// let mut buf = vec![];
/// cwriteln!(buf, "hey look: {strike}strike'd text{reset}!");
/// # assert_eq!(buf, [104, 101, 121, 32, 108, 111, 111, 107, 58, 32, 27, 91, 57, 109, 115, 116, 114, 105, 107, 101, 39, 100, 32, 116, 101, 120, 116, 27, 91, 48, 109, 33, 10]);
/// ```
pub fn cwriteln(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as Two);
    quote! { writeln!(#f) }.into()
}
