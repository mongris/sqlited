use proc_macro::TokenStream;

mod sql_check_impl;
mod sql_impl;
mod sql_no_quote_impl;
mod sql_params_impl;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    sql_impl::sql(input)
}

#[proc_macro]
pub fn sql_str(input: TokenStream) -> TokenStream {
    sql_no_quote_impl::sql_no_quotes(input)
}
#[proc_macro]
pub fn sql_params(input: TokenStream) -> TokenStream {
    sql_params_impl::sql_params(input)
}
