/// Format the total amount to a readable format
/// 
/// ### Arguments
/// - `amount` - The amount to format
/// 
/// ### Returns
/// The formatted amount
/// 
/// ### Example
/// ```rust
/// let amount: i64 = 1000;
/// 
/// let formatted_amount: f64 = format_total_amount(amount).await;
/// println!("Formatted amount: {:?}", formatted_amount);
/// // Formatted amount: 10.0
/// ```
/// 
pub async fn format_total_amount(
    amount: i64
) -> f64 {
    let amount: f64 = amount as f64 / 100.0;

    amount
}