use sycamore::prelude::*;
use sycamore_rstml::html;

fn main() {
    let var = create_signal(());
    let _ = html! {
        <h1>"Hello World!"</h1>
        <div>raw string</div>
        <p>{var}</p>
    };
}
