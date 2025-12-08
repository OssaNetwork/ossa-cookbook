use dioxus::prelude::*;
use dioxus_primitives::select::{Select, SelectGroup, SelectList, SelectOption, SelectTrigger, SelectValue};
use ossa_core::{auth::{group::Role, identity::Identity}, store::StoreRef, util::Sha256Hash};

use crate::gui::form::{Button, SelectField, SelectOption, TextField};

pub struct ShareFormData {
    pub identity: StoreRef<Sha256Hash, Identity, ()>,
    pub permissions: Role,
}

fn validate_identity(input: &str) -> Result<(), &'static str> {
    // TODO: Accept user friendly display names.
    match input.parse::<Sha256Hash>() {
        Ok(_) => {
            Ok(())
        }
        Err(_) => {
            Err("Invalid identity")
        }
    }
}

#[component]
pub fn ShareForm(
    onclick: EventHandler<(ShareFormData, Event<MouseData>)>,
) -> Element {
    let mut identity: Signal<String> = use_signal(|| "".into());
    let mut permissions = use_signal(|| Role::Read);
    let select_options = vec![
        SelectOption::Option {title: "Relay".into(), value: Role::Relay},
        SelectOption::Option {title: "Read".into(), value: Role::Read},
        SelectOption::Option {title: "Write".into(), value: Role::Write},
        SelectOption::Option {title: "Admin".into(), value: Role::Admin},
    ];

    let ident = identity.peek();
    let is_err = validate_identity(&ident).is_err();

    rsx! {
        TextField {
            class: "grow",
            placeholder: "Add users or groups", // Invite?
            id: "share_textfield",
            value: identity,
            oninput: move |evt: Event<FormData>| identity.set(evt.value()),
            validation_fn: validate_identity,
        }
        SelectField {
            class: "appearance-none bg-white bg-no-repeat bg-size-[16px_12px] bg-position-[right_0.75rem_center] border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body",
            style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3e%3cpath fill='none' stroke='%23343a40' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M2 5l6 6 6-6'/%3e%3c/svg%3e\"); background-position: right .75rem center; background-size: 16px 12px; padding-inline-start: calc(0.75rem - 3px); padding: .4rem 2.25rem .4rem .75rem;",
            options: select_options,
            value: Role::Relay,
            oninput: move |(role, _)| {
                permissions.set(role);
            }
        }
        Button { // JP: Move this outside the form?
            class: "border rounded px-2 hover:bg-gray-100",
            disabled: is_err,
            onclick: move |evt| {
                let identity = identity.peek();
                if let Ok(identity) = identity.parse() {
                    let data = ShareFormData {
                        identity: StoreRef::new(identity),
                        permissions: *permissions.peek(),
                    };
                    onclick((data, evt));
                } else {
                    unreachable!("Unreachable: Validation failed.");
                }
            },
            "Share"
        }
    }
}
