mod html;

use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    html::Html::new().impl_html(input.into()).into()
}
