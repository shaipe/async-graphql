use async_graphql::*;

#[async_std::test]
pub async fn test_variables() {
    struct QueryRoot;

    #[GQLObject]
    impl QueryRoot {
        pub async fn int_val(&self, value: i32) -> i32 {
            value
        }

        pub async fn int_list_val(&self, value: Vec<i32>) -> Vec<i32> {
            value
        }
    }

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    let query = Request::new(
        r#"
            query QueryWithVariables($intVal: Int!, $intListVal: [Int!]!) {
                intVal(value: $intVal)
                intListVal(value: $intListVal)
            }
        "#,
    )
    .variables(Variables::from_json(serde_json::json!({
        "intVal": 10,
         "intListVal": [1, 2, 3, 4, 5],
    })));

    assert_eq!(
        schema.execute(query).await.data,
        serde_json::json!({
            "intVal": 10,
            "intListVal": [1, 2, 3, 4, 5],
        })
    );
}

#[async_std::test]
pub async fn test_variable_default_value() {
    struct QueryRoot;

    #[GQLObject]
    impl QueryRoot {
        pub async fn int_val(&self, value: i32) -> i32 {
            value
        }
    }

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    assert_eq!(
        schema
            .execute(
                r#"
            query QueryWithVariables($intVal: Int = 10) {
                intVal(value: $intVal)
            }
        "#
            )
            .await
            .data,
        serde_json::json!({
            "intVal": 10,
        })
    );
}

#[async_std::test]
pub async fn test_variable_no_value() {
    struct QueryRoot;

    #[GQLObject]
    impl QueryRoot {
        pub async fn int_val(&self, value: Option<i32>) -> i32 {
            value.unwrap_or(10)
        }
    }

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    let resp = schema
        .execute(Request::new(
            r#"
            query QueryWithVariables($intVal: Int) {
                intVal(value: $intVal)
            }
        "#,
        ))
        .await
        .into_result()
        .unwrap();
    assert_eq!(
        resp.data,
        serde_json::json!({
            "intVal": 10,
        })
    );
}

#[async_std::test]
pub async fn test_variable_null() {
    struct QueryRoot;

    #[GQLObject]
    impl QueryRoot {
        pub async fn int_val(&self, value: Option<i32>) -> i32 {
            value.unwrap_or(10)
        }
    }

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    let query = Request::new(
        r#"
            query QueryWithVariables($intVal: Int) {
                intVal(value: $intVal)
            }
        "#,
    )
    .variables(Variables::from_json(serde_json::json!({
        "intVal": null,
    })));
    let resp = schema.execute(query).await;
    assert_eq!(
        resp.data,
        serde_json::json!({
            "intVal": 10,
        })
    );
}

#[async_std::test]
pub async fn test_variable_in_input_object() {
    #[derive(GQLInputObject)]
    struct MyInput {
        value: i32,
    }

    struct QueryRoot;

    #[GQLObject]
    impl QueryRoot {
        async fn test(&self, input: MyInput) -> i32 {
            input.value
        }

        async fn test2(&self, input: Vec<MyInput>) -> i32 {
            input.iter().map(|item| item.value).sum()
        }
    }

    struct MutationRoot;

    #[GQLObject]
    impl MutationRoot {
        async fn test(&self, input: MyInput) -> i32 {
            input.value
        }
    }

    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);

    // test query
    {
        let query = r#"
        query TestQuery($value: Int!) {
            test(input: {value: $value })
        }"#;
        let resp = schema
            .execute(
                Request::new(query).variables(Variables::from_json(serde_json::json!({
                    "value": 10,
                }))),
            )
            .await;
        assert_eq!(
            resp.data,
            serde_json::json!({
                "test": 10,
            })
        );
    }

    // test query2
    {
        let query = r#"
        query TestQuery($value: Int!) {
            test2(input: [{value: $value }, {value: $value }])
        }"#;
        let resp = schema
            .execute(
                Request::new(query).variables(Variables::from_json(serde_json::json!({
                    "value": 3,
                }))),
            )
            .await;
        assert_eq!(
            resp.data,
            serde_json::json!({
                "test2": 6,
            })
        );
    }

    // test mutation
    {
        let query = r#"
        mutation TestMutation($value: Int!) {
            test(input: {value: $value })
        }"#;
        let resp = schema
            .execute(
                Request::new(query).variables(Variables::from_json(serde_json::json!({
                    "value": 10,
                }))),
            )
            .await;
        assert_eq!(
            resp.data,
            serde_json::json!({
                "test": 10,
            })
        );
    }
}
