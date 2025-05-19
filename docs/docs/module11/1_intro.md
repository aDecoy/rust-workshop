---
sidebar_position: 1
---

# Intro

Up until this point, the application you've built has been concerned with dealing with synchronous requests. A web application to manage user authentication. But modern applications aren't just made up of synchronous request handlers—they often need to perform work in the background, outside the direct flow of a request/response cycle.

In Rust, running background tasks is a common requirement for things like sending emails, processing jobs, or cleaning up resources. Unlike .NET, which has built-in background services and hosted worker patterns, Rust (especially with async frameworks like Axum or Tokio) gives you explicit control over how and when background tasks are spawned.

## Running Background Tasks in Rust

### The Basics: Spawning Tasks

Rust's async ecosystem, using Tokio, allows you to spawn background tasks using the `tokio::spawn` function. This is similar to starting a new thread or background worker in .NET, but is much lighter-weight because it uses async tasks, not OS threads.

```rust showLineNumbers
// Spawning a background task
use tokio::task;

task::spawn(async move {
    // Your background work here
    do_some_work().await;
});
```

### Example: Sending Emails in the Background

Suppose you want to send a welcome email after a user registers, but you don't want to block the HTTP response. You can spawn a background task to handle the email:

```rust showLineNumbers
async fn register_user(/* ... */) -> impl IntoResponse {
    // ... create user ...
    let email = user.email.clone();
    
    // Spawn a background task
    tokio::spawn(async move {
        send_welcome_email(email).await;
    });
    
    // Respond immediately
    (StatusCode::OK, Json(user))
}
```

### Sharing State with Background Tasks

If your background task needs access to application state (like a database pool or shared cache), you can clone an `Arc` (atomic reference counted pointer) and move it into the task:

```rust showLineNumbers
let state = app_state.clone();
tokio::spawn(async move {
    // Use state inside the task
    state.do_background_work().await;
});
```

### Handling Errors

Background tasks run independently, so errors inside them won't affect the main application. If you need to handle errors, you can log them or send them to a monitoring system:

```rust showLineNumbers
tokio::spawn(async move {
    if let Err(e) = do_work().await {
        eprintln!("Background task failed: {e}");
    }
});
```

### Long-Running and Periodic Tasks

For tasks that need to run periodically (like scheduled jobs), you can use a loop with a delay:

```rust showLineNumbers
use tokio::time::{sleep, Duration};

tokio::spawn(async move {
    loop {
        do_periodic_work().await;
        sleep(Duration::from_secs(60)).await; // Wait 60 seconds
    }
});
```

### Graceful Shutdown

If you need to coordinate shutdown of background tasks (for example, to finish processing before exiting), you can use channels or cancellation tokens. Tokio provides utilities for this, but for most web apps, simply spawning tasks is sufficient.

## Summary

- Use `tokio::spawn` to run background tasks without blocking requests
- Move any needed data or state into the task using `clone` or `Arc`
- Handle errors inside the task, as they won't bubble up to the main thread
- For periodic work, use a loop with `tokio::time::sleep`
- For more advanced scenarios, explore Tokio's synchronization and shutdown primitives

This approach gives you fine-grained control over background processing in your Rust applications, similar to but more explicit than .NET's background services.

