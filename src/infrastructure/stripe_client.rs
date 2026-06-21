// src/infrastructure/stripe_client.rs
use async_stripe::{
    Client,
    stripe::{
        CreatePaymentIntent,
        Currency,
        PaymentIntent,
        PaymentIntentStatus,
    },
};
use std::env;

pub fn create_stripe_client() -> Client {
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    Client::new(secret_key)
}

pub async fn create_payment_intent(
    client: &Client,
    amount_krw: i64,
) -> Result<PaymentIntent, async_stripe::StripeError> {
    let mut params = CreatePaymentIntent::new(amount_krw, Currency::Krw);
    params.payment_method_types = Some(vec!["card".to_string()]);

    PaymentIntent::create(client, params).await
}

pub fn is_payment_succeeded(intent: &PaymentIntent) -> bool {
    matches!(intent.status, PaymentIntentStatus::Succeeded)
}
