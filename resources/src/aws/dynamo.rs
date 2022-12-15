use aws_sdk_dynamodb::{model::AttributeValue, Client, Error};

pub async fn increment_item(
    table_name: &str,
    key: &str,
    user_id: &str,
    item_name: &str,
) -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    // Increment the value of item_name attribute if it exists.
    let increment_if_exists_request = client
        .update_item()
        .table_name(table_name)
        .key(key, AttributeValue::S(user_id.to_string()))
        .update_expression(format!("set {item_name} = {item_name} + :value"))
        .expression_attribute_values(":value", AttributeValue::N("1".to_string()))
        .condition_expression(format!("attribute_exists({item_name})"));

    let increment_if_exists_response = increment_if_exists_request
        .send()
        .await
        .map_err(|e| Error::from(e));

    // Create a new item with value 1 for the item_name attribute if it does not yet exist.
    if let Err(Error::ConditionalCheckFailedException(_err)) = increment_if_exists_response {
        let create_if_dne_request = client
            .update_item()
            .table_name(table_name)
            .key(key, AttributeValue::S(user_id.to_string()))
            .update_expression(format!("set {item_name} = :value"))
            .expression_attribute_values(":value", AttributeValue::N("1".to_string()));

        create_if_dne_request.send().await?;
    }

    Ok(())
}
