# Notification & Feedback Engine

SharpSystem uses a premium, centralized notification system to manage asynchronous feedback across the application.

## ◈ Core Components

The engine is built on the `SystemState` global context and supports two primary feedback channels:

### 1. Stackable Toasts (Notifications)
- **Behavior**: Small, non-blocking alerts that appear in the corner of the screen.
- **Levels**: `Success`, `Warning`, `Error`.
- **Logic**: Toasts are managed as an `Arc<Vec<Notification>>`, allowing for multiple concurrent alerts that auto-clear.

### 2. Modal Alerts
- **Behavior**: High-priority, blocking modals for critical system interactions.
- **Logic**: Managed as an `Option<Arc<Notification>>` in the global state, ensuring only one modal is active at any time.

## ◈ Usage Pattern

To trigger a notification from any component:

1. **Inject Context**:
   ```rust
   let state = use_context::<RwSignal<SystemState>>().expect("Context missing");
   ```

2. **Dispatch Toast**:
   ```rust
   state.update(|s| s.add_toast(Arc::new(Notification::new(
       "System Title", 
       "Detailed message logic.", 
       NotificationLevel::Success
   ))));
   ```

## ◈ Styling Architecture

- **Visuals**: Glassmorphic "Glass-Card-Premium" styling with tailored icons for each notification level.
- **Animations**: Uses `slide-in` and `fade-out` CSS transitions for a fluid, high-end feel.
