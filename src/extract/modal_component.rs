#[cfg(test)]
mod tests;

use twilight_model::application::interaction::modal::ModalInteractionDataActionRow;

/// Trait implemented on
/// [`&[ModalInteractionDataActionRow]`](ModalInteractionDataActionRow) extract
/// options
pub trait ExtractModalComponent<'a> {
    /// Extract the value of the component with the given custom ID
    ///
    /// Returns `None` if not found
    fn component(self, custom_id: &str) -> Option<&'a str>;
}

impl<'a> ExtractModalComponent<'a> for &'a [ModalInteractionDataActionRow] {
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
