use std::sync::Arc;

use twilight_http::Client;
use twilight_model::id::Id;

use crate::InteractionHandle;

#[test]
fn test_data_integrity() {
    let client = Client::new("a".to_owned());
    let handle = InteractionHandle::new(Arc::new(client), Id::new(1), Id::new(1), "a".to_owned());
    #[expect(clippy::redundant_clone, reason = "cloned to test data integrity")]
    let handle_clone = handle.clone();

    handle.set_is_responded(true);
    assert_eq!(handle.is_responded(), handle_clone.is_responded());
}
