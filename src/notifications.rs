use notify_rust::Notification;
use tracing::{error, info};

/// Sends a desktop notification.
pub fn send_notification(summary: &str, body_content: &str, title: Option<&str>) {
    // Construct the final body, including the title if present
    let final_body = if let Some(t) = title {
        if body_content.is_empty() {
            // If original body is empty, just use title
            t.to_string()
        } else {
            // Combine body and title
            format!("{}\n{}", body_content, t) // Add newline between body and title
        }
    } else {
        // If no title, use the original body content
        body_content.to_string()
    };

    // Use app_name that matches your .desktop file if you create one later
    match Notification::new()
        .appname("twitch-notifier")
        .summary(summary)
        .body(&final_body) // Use the potentially modified body
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
