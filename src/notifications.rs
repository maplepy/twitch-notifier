use notify_rust::Notification;
use tracing::{error, info};

/// Sends a desktop notification.
pub fn send_notification(summary: &str, body: &str) {
    // Make it public
    // let summary = format!("{} is live!", streamer_name);
    // let body = format!("Playing: {}", game_name);

    // Use app_name that matches your .desktop file if you create one later
    match Notification::new()
        .appname("twitch-notifier")
        .summary(summary)
        .body(body)
        // .icon("dialog-information") // Optional: specify an icon
        .timeout(notify_rust::Timeout::Milliseconds(10000)) // Show for 10 seconds
        .show()
    {
        Ok(_) => {
            info!("Sent notification: {}", summary);
        }
        Err(e) => {
            error!("Failed to send notification: {}", e);
        }
    }
}
