use notify_rust::Notification;

pub fn send_notification(
    title: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Notification::new()
        .summary(title)
        .body(message)
        .show()
        .unwrap();
    Ok(())
}
