extern crate proc_macro;
use proc_macro::TokenStream;
use syn;

#[proc_macro_attribute]
pub fn os_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item.clone()).unwrap();
    let fn_ident = ast.sig.ident.to_string();
    
    let decorated = format!("#[allow(non_snake_case)]\n#[test_case]\nfn __os_test_decorated__{fn_ident}() {{\nprint!(\"test '{fn_ident}' ... \");      \n{fn_ident}();\nprintln!(\"ok\");\n}}");
    let mut decorated_ts: TokenStream = decorated.parse().unwrap();
    decorated_ts.extend(item);
    return decorated_ts;
}

