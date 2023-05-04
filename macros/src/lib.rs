extern crate proc_macro;
use proc_macro::TokenStream;
use syn;

#[proc_macro_attribute]
pub fn os_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item.clone()).unwrap();
    let fn_ident = ast.sig.ident.to_string();
    
    // Adds test .. ok print to test
    let decorated = format!("#[test_case]\nfn _os_test_decorated_{fn_ident}() {{\nprint!(\"test '{fn_ident}' ... \");      \n{fn_ident}();\nprintln!(\"\\x1b[32mok\\x1b[0m\");\n}}");
    let mut decorated_ts: TokenStream = decorated.parse().unwrap();
    decorated_ts.extend(item);
    return decorated_ts;
}

