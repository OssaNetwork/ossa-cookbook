
use dioxus::prelude::*;

#[component]
pub fn LoginView() -> Element {
    rsx! (
        div {
            class: "flex flex-col justify-center items-center w-full h-full gap-7",
            h1 {
                class: "text-2xl font-bold",
                "Identities"
            }
            div {
                class: "border border-gray-500 w-2/3 rounded-xl text-gray-800 hover:bg-gray-50 items-center p-10 text-center",
                span {
                    class: "items-center text-xl",
                    "Add identity"
                }
            }
        }
    )
}
