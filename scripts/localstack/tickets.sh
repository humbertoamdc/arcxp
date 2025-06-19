#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Tickets \
    --attribute-definitions \
        AttributeName=id,AttributeType=S \
    --key-schema \
        AttributeName=id,KeyType=HASH \
    --billing-mod PAY_PER_REQUEST
