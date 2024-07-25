use twilight_model::application::interaction::modal::ModalInteractionDataActionRow;

/// Trait implemented on [`Vec<ModalInteractionDataActionRow>`] extract options
#[deprecated(note = "Use `ExtractModalComponentRef` instead")]
pub trait ExtractModalComponent {
    /// Extract the value of the component with the given custom ID
    ///
    /// Returns `None` if not found
    fn component(self, custom_id: &str) -> Option<String>;
}

/// Trait implemented on [`&[ModalInteractionDataActionRow]`] extract options
pub trait ExtractModalComponentRef<'a> {
    /// Extract the value of the component with the given custom ID
    ///
    /// Returns `None` if not found
    fn component(self, custom_id: &str) -> Option<&'a str>;
}

#[allow(deprecated)]
impl ExtractModalComponent for Vec<ModalInteractionDataActionRow> {
    fn component(self, custom_id: &str) -> Option<String> {
        for action_row in self {
            if let Some(value) = action_row
                .components
                .into_iter()
                .find_map(|component| (component.custom_id == custom_id).then_some(component.value))
            {
                return value;
            }
        }

        None
    }
}

impl<'a> ExtractModalComponentRef<'a> for &'a [ModalInteractionDataActionRow] {
    fn component(self, custom_id: &str) -> Option<&'a str> {
        for action_row in self {
            if let Some(value) = action_row.components.iter().find_map(|component| {
                (component.custom_id == custom_id).then_some(&component.value)
            }) {
                return value.as_deref();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use twilight_model::{
        application::interaction::modal::{
            ModalInteractionDataActionRow, ModalInteractionDataComponent,
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
}
