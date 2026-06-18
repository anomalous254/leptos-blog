---
title: "Making SPA Navigation Faster with mousedown Instead of click"
author: "Peter Nyando"
date: "June 18, 2026"
description: "Using mousedown instead of click in custom SPA links to make navigation feel faster by starting route changes earlier."
image: "../assets/img/crab.png"
---

### Introduction

In a single-page application (SPA), navigation speed depends not only on the router but also on how quickly navigation starts.

Most custom link components use the `click` event, which fires after `mousedown` and `mouseup`. By using `mousedown`, we can start navigation immediately when the user presses the mouse button.

### Why Use mousedown?

Normal link flow:

```text
mousedown
    ↓
mouseup
    ↓
click
    ↓
navigation
```
### Using `mousedown`:

```text
mousedown
    ↓
navigation
```

This removes a small delay and makes the application feel more responsive.


### Creating a Fast Link Component

Example using Rust and Leptos:

```rust
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_location};
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

- `ev.button() == 0` checks for a left mouse click.

- `prevent_default()` stops the browser's normal page reload.

- `navigate()` starts the SPA route transition immediately.


### Conclusion

Using mousedown instead of click is a small optimization that reduces navigation delay and improves perceived performance in SPA applications.