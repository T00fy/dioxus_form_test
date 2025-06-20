
# Dioxus Form Submission Bug Repro

This minimal reproducible example demonstrates a discrepancy in Dioxus form handling on the web: calling `prevent_default()` inside an `EventHandler` returned by a struct method does **not** stop the page from reloading, whereas using an inline closure does.

## Scenarios

### 🔴 Scenario A: Problematic Form

Uses `EventHandler::new()` returned by a struct method, calls `ev.prevent_default()`, but page still reloads.

### 🟢 Scenario B: Working Form

Uses an inline closure (no explicit `prevent_default()`), and it correctly prevents the reload.

### 🟡 Scenario C: Debug Form

Calls `prevent_default()` immediately in an `EventHandler::new()`, reloads.

---

## Observing the Bug

1. Submit **Scenario A**:
    * Enter text in “Problematic Form” and click **Submit**.
    * Notice the console logs confirm `prevent_default()` was called, but the page reloads (`/?data_a=…` appears).
    * Async task does not execute
2. Submit **Scenario B**:
    * Enter text in “Working Form” and click **Submit**.
    * No reload, and the status updates in-place.
    * Async task works correctly
3. Submit **Scenario C**:
    * Enter text in “Debug Form” and click **Submit**.
    * Despite early call to `prevent_default()`, the page reloads.
    * Async task not called here to check for side effect
