mod html;

use proc_macro::TokenStream;

/// Alternative to sycamore::view! that uses rstml/html style templating
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    html::Html::new().impl_html(input.into()).into()
}
