use leptos::prelude::*;
use leptos_meta::{ServerMetaContext, provide_meta_context};
use std::sync::Arc;

#[test]
fn metadata_does_not_retain_request_context_after_cleanup() {
    let owner = Owner::new();
    let request = Arc::new(());
    let weak_request = Arc::downgrade(&request);
    owner.with(|| {
        provide_context(request);
        provide_meta_context();
    });

    owner.unset_with_forced_cleanup();

    assert!(weak_request.upgrade().is_none());
}

#[test]
fn server_metadata_output_does_not_retain_the_request_owner() {
    let owner = Owner::new();
    let weak_owner = owner.downgrade();
    let output = owner.with(|| {
        let (context, output) = ServerMetaContext::new();
        provide_context(context);
        output
    });

    owner.unset_with_forced_cleanup();

    assert!(weak_owner.upgrade().is_none());
    drop(output);
}
