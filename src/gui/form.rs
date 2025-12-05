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

#[derive(PartialEq, Props, Clone)]
pub struct TextFieldProps {
    id: String,
    class: Option<String>,
    placeholder: String,
    value: String,
    oninput: EventHandler<Event<FormData>>,
    onkeyup: Option<EventHandler<Event<KeyboardData>>>,
    validation_fn: for<'b> fn(&'b str) -> Result<(), &'static str>,
}

pub fn TextField(props: TextFieldProps) -> Element {
    let mut is_modified = use_signal(|| false);
    let err: Result<(), &'static str> = if *is_modified.peek() {
        (props.validation_fn)(&props.value)
    } else {
        Ok(())
    };

    rsx! (
        input {
            class: format_args!("{} appearance-none border rounded py-1 px-2 {}", props.class.unwrap_or("".to_string()), if err.is_err() {"border-red-500"} else {""}),
            r#id: props.id,
            r#type: "text",
            placeholder: props.placeholder,
            oninput: move |evt| {
                is_modified.set(true);
                props.oninput.call(evt);
            },
            onkeyup: move |evt| props.onkeyup.as_ref().map_or((), |f| f.call(evt)),
            value: "{props.value}"
        }
        { err.err().map(|e| rsx!(
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
    value: Signal<A>,
) -> Element {
    // TODO: Set initial value.
    let options_view = options.iter().enumerate().map(|(idx, option)| {
        match option {
            SelectOption::Option{ title, value } => {
                rsx! {
                    option {
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
            // value: 0,
            oninput: move |evt| {
                let pos = evt.value().parse::<usize>().expect("Parsing values should never fail");
                
                if let SelectOption::Option {value: v, ..} = &options[pos] {
                    value.set(v.clone());
                };
                // permissions.set(evt.value());
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
