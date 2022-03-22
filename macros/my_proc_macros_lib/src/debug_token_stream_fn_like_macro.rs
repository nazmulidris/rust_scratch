use proc_macro::TokenStream;
use quote::ToTokens;
use r3bl_rs_utils::utils::{eprint_header, style_primary, style_prompt};
use syn::{ItemFn, parse_str};

// TODO: visualize the TokenStream

/// https://docs.rs/syn/1.0.52/syn/macro.parse_macro_input.html
pub fn simple_function_macro_make_a_fn_impl(input: TokenStream) -> TokenStream {
  let output_token_stream_str = "fn foo() -> u32 { 42 }";
  let output = output_token_stream_str.parse().unwrap();

  let ast_item_fn: ItemFn = parse_str::<ItemFn>(output_token_stream_str).unwrap();

  viz_token_stream("input", &input);

  viz_token_stream(
    &format!("{} {}", "output of ", output_token_stream_str),
    &output,
  );

  viz_ast(ast_item_fn);

  output
}

/// https://docs.rs/syn/1.0.52/syn/fn.parse_str.html
/// https://docs.rs/syn/1.0.52/syn/struct.ItemFn.html
/// https://docs.rs/syn/1.0.52/syn/struct.Attribute.html
/// https://docs.rs/syn/1.0.52/syn/enum.Visibility.html
/// https://docs.rs/syn/1.0.52/syn/struct.Signature.html
/// https://docs.rs/syn/1.0.52/syn/struct.Block.html
/// https://docs.rs/syn/1.0.52/syn/enum.Stmt.html
/// https://github.com/dtolnay/proc-macro-workshop#debugging-tips
fn viz_ast(ast: ItemFn) {
  let ast_clone = ast.clone();

  let ItemFn {
    attrs,
    vis,
    sig,
    block,
  } = ast;

  eprintln!(
    "{} ast_item_fn {{ attrs.len:{}, vis:{}, sig:'{}' stmt: '{}' }}",
    style_primary("=>"),
    style_prompt(&attrs.len().to_string()),
    style_prompt(match vis {
      syn::Visibility::Public(_) => "public",
      syn::Visibility::Crate(_) => "crate",
      syn::Visibility::Restricted(_) => "restricted",
      syn::Visibility::Inherited => "inherited",
    }),
    style_prompt(&sig.ident.to_string()),
    style_prompt(&match block.stmts.first() {
      Some(stmt) => {
        let expr_str = stmt.to_token_stream().to_string().clone();
        expr_str
      }
      None => "empty".to_string(),
    }),
  );

  eprintln!("{} => {:#?}", style_primary("Debug::ast"), ast_clone);
}

fn viz_token_stream(
  msg: &str,
  token_stream: &TokenStream,
) {
  eprint_header(msg);
  eprintln!("{:#?}", token_stream);
}
