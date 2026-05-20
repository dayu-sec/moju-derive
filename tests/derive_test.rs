use moju_derive::{MoJu, MoJuItem};

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "struct", domain = "Business")]
struct Order {
    #[moju(unique)]
    id: String,
    user: String,
    status: String,
}

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "message", role = "command", domain = "Business")]
struct PlaceOrder {
    items: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "state", domain = "Business")]
enum OrderStatus {
    Created,
    Paid,
    Shipped,
    Cancelled,
}

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "failure", identity = "payment.timeout", tag = "payment")]
enum PaymentError {
    ProviderUnavailable,
    Timeout,
}

#[test]
fn test_struct_kind_and_domain() {
    assert_eq!(Order::moju_kind(), "struct");
    assert_eq!(Order::moju_domain(), "Business");
    assert_eq!(Order::moju_role(), None);
    assert_eq!(Order::moju_unique_fields(), &["id"]);
}

#[test]
fn test_message_role() {
    assert_eq!(PlaceOrder::moju_kind(), "message");
    assert_eq!(PlaceOrder::moju_role(), Some("command"));
    assert_eq!(PlaceOrder::moju_unique_fields(), &[] as &[&str]);
}

#[test]
fn test_state_enum() {
    assert_eq!(OrderStatus::moju_kind(), "state");
    assert_eq!(OrderStatus::moju_domain(), "Business");
}

#[test]
fn test_failure_identity() {
    assert_eq!(PaymentError::moju_kind(), "failure");
    assert_eq!(PaymentError::moju_identity(), Some("payment.timeout"));
    assert_eq!(PaymentError::moju_tag(), Some("payment"));
}
