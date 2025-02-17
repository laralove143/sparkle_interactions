use twilight_model::{
    application::interaction::modal::{
        ModalInteractionDataActionRow,
        ModalInteractionDataComponent,
    },
    channel::message::component::ComponentType,
};

use crate::extract::modal_component::ExtractModalComponentRef;

#[test]
fn test_extract_component() {
    let custom_id = "custom_id";
    let value = "value";

    assert_eq!(
        [ModalInteractionDataActionRow {
            components: vec![ModalInteractionDataComponent {
                custom_id: custom_id.to_owned(),
                kind: ComponentType::TextInput,
                value: Some(value.to_owned()),
            }],
        }]
        .component(custom_id),
        Some(value)
    );

    assert_eq!(
        [ModalInteractionDataActionRow {
            components: vec![ModalInteractionDataComponent {
                custom_id: "a".to_owned(),
                kind: ComponentType::TextInput,
                value: Some(value.to_owned()),
            }],
        }]
        .component(custom_id),
        None
    );

    assert_eq!(
        [ModalInteractionDataActionRow {
            components: vec![ModalInteractionDataComponent {
                custom_id: custom_id.to_owned(),
                kind: ComponentType::TextInput,
                value: None,
            }],
        }]
        .component(custom_id),
        None
    );
}
