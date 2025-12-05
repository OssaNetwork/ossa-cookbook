use dioxus::prelude::*;
use dioxus_primitives::select::{Select, SelectGroup, SelectList, SelectOption, SelectTrigger, SelectValue};
use ossa_core::auth::group::Role;

use crate::gui::form::{SelectField, SelectOption, TextField};

pub struct ShareForm {
    pub identity: Signal<String>,
    pub permissions: Signal<Role>,
}

fn validate_identity(input: &str) -> Result<(), &'static str> {
    Ok(())
}

pub fn share_form() -> (Element, ShareForm) {
    let mut identity = use_signal(|| "".into());
    let permissions = use_signal(|| Role::Read);
    let select_options = vec![
        SelectOption::Option {title: "Relay".into(), value: Role::Relay},
        SelectOption::Option {title: "Read".into(), value: Role::Read},
        SelectOption::Option {title: "Write".into(), value: Role::Write},
        SelectOption::Option {title: "Admin".into(), value: Role::Admin},
    ];

    let view = rsx! {
        TextField {
            class: "grow",
            placeholder: "Add users or groups", // Invite?
            id: "share_textfield",
            value: identity,
            oninput: move |evt: Event<FormData>| identity.set(evt.value()),
            validation_fn: validate_identity,
        }
        // select {
        //     class: "appearance-none bg-white bg-no-repeat bg-size-[16px_12px] bg-position-[right_0.75rem_center] border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body",
        //     style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3e%3cpath fill='none' stroke='%23343a40' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M2 5l6 6 6-6'/%3e%3c/svg%3e\"); background-position: right .75rem center; background-size: 16px 12px; padding-inline-start: calc(0.75rem - 3px); padding: .4rem 2.25rem .4rem .75rem;",
        //     value: permissions,
        //     oninput: move |evt| {
        //         permissions.set(evt.value());
        //         // is_modified.set(true);
        //         // props.oninput.call(evt);
        //     },
        //     // option {
        //     //     disabled: true,
        //     //     selected: "",
        //     //     "Permissions"
        //     // }
        //     option {
        //         "Read"
        //     }
        //     option {
        //         "Write"
        //     }
        //     option {
        //         "Admin"
        //     }
        // }
        SelectField {
            class: "appearance-none bg-white bg-no-repeat bg-size-[16px_12px] bg-position-[right_0.75rem_center] border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body",
            style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3e%3cpath fill='none' stroke='%23343a40' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M2 5l6 6 6-6'/%3e%3c/svg%3e\"); background-position: right .75rem center; background-size: 16px 12px; padding-inline-start: calc(0.75rem - 3px); padding: .4rem 2.25rem .4rem .75rem;",
            options: select_options,
            value: permissions,
        }
        // Select::<String> {
        //     placeholder: "Select a fruit...",
        //     SelectTrigger{
        //         aria_label: "Select Trigger",
        //         width: "12rem",
        //         SelectValue {}
        //     }
        //     SelectList {
        //         SelectGroup {
        //             SelectOption::<String> {
        //                 index: 0usize,
        //                 value: "test",
        //                 "Test"
        //             }
        //         }
        //     }
        // }
        button { // JP: Move this outside the form?
            class: "border rounded px-2",
            onclick: |_evt| {
                println!("TODO!");
            },
            "Share"
        }
    };

    let form = ShareForm {
        identity,
        permissions,
    };
    (view, form)
}
