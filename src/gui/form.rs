use std::ops::Deref;

use dioxus::{core::AttributeValue, prelude::*};

#[derive(Props, Clone, PartialEq)]
pub struct FieldLabelProps {
    label: String,
    id: String,
    field: Element,
}

pub fn FieldLabel(props: FieldLabelProps) -> Element {
    rsx! (
        div {
            class: "flex flex-col mb-4",
            label {
                class: "font-bold mb-2",
                r#for: props.id,
                { props.label }
            }
            { props.field }
        }
    )
}

// #[derive(PartialEq, Props, Clone)]
// pub struct TextFieldProps {
//     id: String,
//     class: Option<String>,
//     placeholder: String,
//     value: String,
//     oninput: EventHandler<Event<FormData>>,
//     onkeyup: Option<EventHandler<Event<KeyboardData>>>,
//     validation_fn: for<'b> fn(&'b str) -> Result<(), &'static str>,
// }

#[component]
pub fn TextField<A: Clone + ToString + 'static>(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    class: Option<String>,
    placeholder: String,
    value: Signal<Result<A, &'static str>>,
    // value: Option<A>,
    oninput: Option<EventHandler<Event<FormData>>>,
    onkeyup: Option<EventHandler<Event<KeyboardData>>>,
    // parse_fn: for<'b> fn(&'b str) -> Result<A, &'static str>,
    parse_fn: fn(String) -> Result<A, &'static str>,

    // options: Vec<SelectOption<A>>,
    // value: Option<A>,
    // oninput: Option<EventHandler<(A, Event<FormData>)>>,
) -> Element {
    // let mut is_modified = use_signal(|| false);
    // let err: Result<(), &'static str> = if *is_modified.peek() {
    //     (props.validation_fn)(&props.value)
    // } else {
    //     Ok(())
    // };

    rsx! (
        input {
            class: format_args!("{} appearance-none border rounded py-1 px-2 {}", class.unwrap_or("".to_string()), if value.peek().is_err() {"border-red-500"} else {""}),
            // r#id: props.id,
            r#type: "text",
            value: value.peek().deref().as_ref().map_or("".to_string(), |v| v.to_string()),
            placeholder: placeholder,
            oninput: move |evt| {
                let current = parse_fn(evt.value());
                value.set(current.clone());
                if let Some(f) = oninput {
                    f(evt)
                };
            },
            onkeyup: move |evt| onkeyup.as_ref().map_or((), |f| {
                f.call(evt)
            }),
            // value: "{value}"
            ..attributes,
        }
        { value.peek().as_ref().err().map(|e| rsx!(
            p {
                class: format_args!("text-red-500 text-sm"),
                "{e}"
            }
        )) }
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectOption<A> {
    Option { title: String, value: A },
    Separator,
}

#[component]
pub fn SelectField<A: Clone + PartialEq + 'static>(
    #[props(extends=GlobalAttributes)]
    #[props(extends=select)]
    attributes: Vec<Attribute>,
    options: Vec<SelectOption<A>>,
    value: Option<A>,
    oninput: Option<EventHandler<(A, Event<FormData>)>>,
) -> Element {
    let options_view = options.iter().enumerate().map(|(idx, option)| {
        match option {
            SelectOption::Option{ title, value: v } => {
                let is_selected = Some(v) == value.as_ref();

                rsx! {
                    option {
                        selected: is_selected,
                        value: idx,
                        { title.clone() }
                    }
                }
            }
            SelectOption::Separator => {
                rsx! {
                    option {
                        disabled: true,
                        value: idx,
                        "─────"
                    }
                }
            }
        }
    });
    rsx! {
        select {
            oninput: move |evt| {
                let pos = evt.value().parse::<usize>().expect("Parsing values should never fail");
                
                if let SelectOption::Option {value: v, ..} = &options[pos] {
                    if let Some(oninput) = oninput {
                        oninput.call((v.clone(), evt)); // JP: Is it possible to avoid this clone?
                    }
                };
            },
            ..attributes,
            { options_view }
        }
    }
}

fn is_disabled(attributes: &[Attribute]) -> bool {
    // println!("*** is_disabled ***");
    for attr in attributes {
        if attr.name == "disabled" {
            if let AttributeValue::Bool(b) = attr.value {
                return b;
            }
        }
    }

    false
}

#[component]
pub fn Button(
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let disabled = is_disabled(&attributes);
    rsx! {
        button {
            onclick: move |event| {
                // println!("onclick!");
                if let Some(f) = &onclick {
                    if !disabled {
                        f.call(event);
                    } else {
                        // println!("Disabled!");
                    }
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    if !disabled {
                        f.call(event);
                    }
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    if !disabled {
                        f.call(event);
                    }
                }
            },
            ..attributes,
            {children}
        }
    }
}
