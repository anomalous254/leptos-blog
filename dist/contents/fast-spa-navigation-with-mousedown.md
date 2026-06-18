---
title: "Making SPA Navigation Lightning Fast with mousedown Instead of click"
author: "Peter Nyando"
date: "June 18, 2026"
description: "Improve perceived navigation performance in single-page applications by initiating route transitions on mousedown rather than click."
image: "../assets/img/crab.png"
---

### Introduction

In single-page applications (SPAs), navigation performance is influenced not only by the router itself but also by how quickly the navigation process begins after user interaction.

Most custom link components initiate navigation through the `click` event. However, a `click` event is only dispatched after both `mousedown` and `mouseup` have occurred. By triggering navigation on `mousedown`, applications can begin route transitions immediately when the user presses the mouse button, resulting in a more responsive user experience.

### Why Use `mousedown`?

A traditional navigation flow looks like this:

```text
mousedown
    ↓
mouseup
    ↓
click
    ↓
navigation
```

When navigation is triggered on `mousedown`, the process becomes:

```text
mousedown
    ↓
navigation
```

Although the time difference is small, it reduces the delay between user intent and application response. Users often perceive interfaces that react immediately as significantly faster, even when the actual performance improvement is measured in milliseconds.

This technique is particularly useful in applications where users frequently navigate between views, such as dashboards, admin panels, developer tools, and productivity software.

### Why It's Lightning Fast

Users judge application speed based on responsiveness rather than raw execution time. When a route transition starts on `mousedown`, the application reacts at the exact moment the user commits to the action.

Even though only a few milliseconds are saved, the UI feels more responsive because there is no waiting period between the user's action and the application's reaction. This creates a smoother and more polished experience, especially in applications with frequent navigation.

### Creating a Fast Link Component

The following example demonstrates how to implement a custom link component in Rust using Leptos:

```rust
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos::web_sys::MouseEvent;

#[component]
pub fn FastA(
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {

    let navigate = use_navigate();

    let path = href.clone();

    view! {
        <a
            href=href
            on:mousedown=move |ev: MouseEvent| {
                if ev.button() == 0 {
                    ev.prevent_default();
                    navigate(&path, Default::default());
                }
            }
        >
            {children()}
        </a>
    }
}
```

### How It Works

* `ev.button() == 0` ensures that navigation only occurs for the primary (left) mouse button.
* `prevent_default()` prevents the browser from performing a full page navigation.
* `navigate()` triggers the SPA route transition immediately when the mouse button is pressed.

By starting navigation earlier in the interaction lifecycle, the application can begin loading the next view before the user completes the click action.

### Considerations

While this approach can improve perceived responsiveness, it should be used thoughtfully.

* Users may press the mouse button and then move the cursor away before releasing it. Traditional links would not activate in this scenario, but `mousedown` navigation already has.
* Additional handling may be required for keyboard navigation and accessibility to ensure a consistent experience.
* Browser behaviors such as opening links in new tabs using modifier keys (`Ctrl`, `Cmd`, or middle-click) should be preserved when appropriate.

For many internal SPA routes, however, the trade-off can be worthwhile when optimizing for responsiveness.

### Conclusion

Using `mousedown` instead of `click` is a simple optimization that can make SPA navigation feel noticeably faster. Because route transitions begin the moment the user presses the mouse button, the interface responds immediately to user intent rather than waiting for the complete click sequence.

The actual time savings are typically small, but perceived performance often matters more than raw performance. Users experience the application as more responsive, smoother, and more polished because feedback begins instantly.

When applied carefully, this technique is an effective way to deliver a lightning-fast navigation experience with minimal implementation effort.
