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

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "storage", domain = "Business", storage_kind = "table", durability = "persistent")]
struct OrderStore;

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "actor", domain = "Business", parent = "User")]
struct CustomerActor;

#[allow(dead_code)]
#[derive(Debug, MoJu)]
#[moju(kind = "failure", identity = "db.timeout", tag = "db", description = "database operation timed out")]
enum DbError {
    Timeout,
}

#[test]
fn test_storage_kind_and_durability() {
    assert_eq!(OrderStore::moju_kind(), "storage");
    assert_eq!(OrderStore::moju_domain(), "Business");
    assert_eq!(OrderStore::moju_storage_kind(), Some("table"));
    assert_eq!(OrderStore::moju_durability(), Some("persistent"));
    assert_eq!(OrderStore::moju_role(), None);
    assert_eq!(OrderStore::moju_identity(), None);
}

#[test]
fn test_actor_parent() {
    assert_eq!(CustomerActor::moju_kind(), "actor");
    assert_eq!(CustomerActor::moju_domain(), "Business");
    assert_eq!(CustomerActor::moju_parent(), Some("User"));
}

#[test]
fn test_failure_description() {
    assert_eq!(DbError::moju_kind(), "failure");
    assert_eq!(DbError::moju_identity(), Some("db.timeout"));
    assert_eq!(DbError::moju_tag(), Some("db"));
    assert_eq!(DbError::moju_description(), Some("database operation timed out"));
}
