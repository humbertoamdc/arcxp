use crate::entities::{Ticket, TicketPriority, TicketStatus};
use anyhow::Result;
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, ReturnValue, WriteRequest};
use chrono::Utc;
use serde_dynamo::{from_items, to_item};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TicketRepository {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

impl TicketRepository {
    pub fn new(client: aws_sdk_dynamodb::Client, table_name: String) -> Self {
        Self { client, table_name }
    }

    pub async fn query(&self) -> Result<Vec<Ticket>> {
        let output = self
            .client
            .scan()
            .table_name(&self.table_name)
            .send()
            .await?;

        let items = output.items().to_vec();

        Ok(from_items(items)?)
    }

    pub async fn create(&self, ticket: Ticket) -> Result<()> {
        let item = to_item(ticket)?;

        let _ = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table_name)
            .send()
            .await?;

        Ok(())
    }

    pub async fn update(
        &self,
        id: String,
        assignee_id: Option<String>,
        title: Option<String>,
        description: Option<String>,
        status: Option<TicketStatus>,
        priority: Option<TicketPriority>,
    ) -> Result<()> {
        let mut update_expression = String::from("SET ");
        let mut expression_attribute_names = HashMap::new();
        let mut expression_attribute_values = HashMap::new();

        update_expression.push_str("#updated_at = :updated_at, ");
        expression_attribute_names.insert("#updated_at".to_string(), "updated_at".to_string());
        expression_attribute_values.insert(
            String::from(":updated_at"),
            AttributeValue::S(Utc::now().to_rfc3339()),
        );

        if let Some(assignee_id) = assignee_id {
            update_expression.push_str("#assignee_id = :assignee_id, ");
            expression_attribute_names
                .insert("#assignee_id".to_string(), "assignee_id".to_string());
            expression_attribute_values.insert(
                String::from(":assignee_id"),
                AttributeValue::S(assignee_id.to_string()),
            );
        }

        if let Some(title) = title {
            update_expression.push_str("#title = :title, ");
            expression_attribute_names.insert("#title".to_string(), "title".to_string());
            expression_attribute_values.insert(String::from(":title"), AttributeValue::S(title));
        }

        if let Some(description) = description {
            update_expression.push_str("#description = :description, ");
            expression_attribute_names
                .insert("#description".to_string(), "description".to_string());
            expression_attribute_values
                .insert(String::from(":description"), AttributeValue::S(description));
        }

        if let Some(status) = status {
            update_expression.push_str("#status = :status, ");
            expression_attribute_names.insert("#status".to_string(), "status".to_string());
            expression_attribute_values.insert(
                String::from(":status"),
                AttributeValue::S(format!("{status:?}")),
            );
        }

        if let Some(priority) = priority {
            update_expression.push_str("#priority = :priority, ");
            expression_attribute_names.insert("#priority".to_string(), "priority".to_string());
            expression_attribute_values.insert(
                String::from(":priority"),
                AttributeValue::S(format!("{priority:?}")),
            );
        }

        // Remove trailing comma and space
        if !update_expression.is_empty() {
            update_expression.pop();
            update_expression.pop();
        }

        self.client
            .update_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(id))
            .update_expression(update_expression)
            .set_expression_attribute_names(Some(expression_attribute_names))
            .set_expression_attribute_values(Some(expression_attribute_values))
            .condition_expression("attribute_exists(id)")
            .return_values(ReturnValue::None)
            .send()
            .await?;

        Ok(())
    }

    pub async fn batch_create(&self, tickets: Vec<Ticket>) -> Result<()> {
        let write_requests = tickets
            .into_iter()
            .map(|ticket| {
                let item = to_item(ticket).expect("Failed to create ticket");
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(item))
                            .build()
                            .expect("Failed to create write request"),
                    )
                    .build()
            })
            .collect::<Vec<WriteRequest>>();

        self.client
            .batch_write_item()
            .request_items(&self.table_name, write_requests)
            .send()
            .await?;

        Ok(())
    }
}
