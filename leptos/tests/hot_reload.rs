#![cfg(feature = "ssr")]

use futures::StreamExt;
use leptos::{html, prelude::*};

fn marked_view() -> impl IntoView {
    View::new(html::p().child("Ready")).with_view_marker("child")
}

fn expected_html() -> String {
    let inner = html::p().child("Ready").to_html();
    if cfg!(debug_assertions) && option_env!("LEPTOS_WATCH").is_some() {
        format!(
            "<!--hot-reload|child|open-->{inner}<!--hot-reload|child|close-->"
        )
    } else {
        inner
    }
}

#[test]
fn synchronous_view_markers() {
    assert_eq!(marked_view().to_html(), expected_html());
}

#[tokio::test]
async fn resolved_view_markers() {
    assert_eq!(marked_view().resolve().await.to_html(), expected_html());
}

#[tokio::test]
async fn resolved_erased_view_markers() {
    assert_eq!(
        marked_view().into_any().resolve().await.to_html(),
        expected_html()
    );
}

#[tokio::test]
async fn resolved_view_markers_in_order() {
    let html = marked_view()
        .resolve()
        .await
        .to_html_stream_in_order()
        .collect::<String>()
        .await;

    assert_eq!(html, expected_html());
}

#[tokio::test]
async fn resolved_view_markers_out_of_order() {
    let html = marked_view()
        .resolve()
        .await
        .to_html_stream_out_of_order()
        .collect::<String>()
        .await;

    assert_eq!(html, expected_html());
}

#[component]
fn MarkedChild() -> impl IntoView {
    marked_view()
}

#[component]
fn MarkedParent() -> impl IntoView {
    View::new(html::section().child(MarkedChild())).with_view_marker("parent")
}

#[tokio::test]
async fn suspense_preserves_ready_component_markers() {
    _ = any_spawner::Executor::init_tokio();
    let owner = Owner::new();
    owner.set();

    let app = view! { <Suspense><MarkedParent/></Suspense> };
    let html = app.to_html_stream_in_order().collect::<String>().await;

    assert!(html.contains(&expected_html()), "{html}");
    if cfg!(debug_assertions) && option_env!("LEPTOS_WATCH").is_some() {
        assert!(
            html.contains("<!--hot-reload|parent|open--><section>"),
            "{html}"
        );
        assert!(
            html.contains("</section><!--hot-reload|parent|close-->"),
            "{html}"
        );
    } else {
        assert!(!html.contains("hot-reload|"), "{html}");
    }
}

#[tokio::test]
async fn suspense_preserves_nested_component_markers() {
    _ = any_spawner::Executor::init_tokio();
    let owner = Owner::new();
    owner.set();

    let app = view! {
        <Suspense fallback=|| "Loading">
            {Suspend::new(async {
                tokio::task::yield_now().await;
                MarkedParent()
            })}
        </Suspense>
    };
    let html = app.to_html_stream_in_order().collect::<String>().await;

    assert!(html.contains(&expected_html()), "{html}");
    if cfg!(debug_assertions) && option_env!("LEPTOS_WATCH").is_some() {
        assert!(
            html.contains("<!--hot-reload|parent|open--><section>"),
            "{html}"
        );
        assert!(
            html.contains("</section><!--hot-reload|parent|close-->"),
            "{html}"
        );
    } else {
        assert!(!html.contains("hot-reload|"), "{html}");
    }
}
