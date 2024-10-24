#[cfg(test)]
mod tests {
    use crate::application::commands::create_variant::handler::{
        CreateVariantCommand, CreateVariantHandler,
    };
    use crate::infrastructure::repositories::variant_repository::MockVariantRepository;

    #[tokio::test]
    async fn test_create_variant_handler_success() {
        let mut mock_repo = MockVariantRepository::new();

        // Подготавливаем ожидаемые данные
        let test_title = "Test Variant".to_string();
        // let test_id = Uuid::now_v7();
        // let test_datetime = Utc::now().naive_utc();

        // let expected_variant = Variant {
        //     id: Some(test_id),
        //     title: test_title.clone(),
        //     confirmed: Some(false),
        //     created_at: Some(test_datetime),
        // };

        mock_repo
            .expect_create_variant()
            .returning(move |variant| Box::pin(async move { Ok(variant) }));

        let handler = CreateVariantHandler::new(mock_repo);
        let command = CreateVariantCommand {
            title: test_title.clone(),
        };

        let result = handler.handle(command).await;
        assert!(result.is_ok());

        let variant = result.unwrap();
        assert_eq!(variant.title, test_title);
        assert_eq!(variant.confirmed, Some(false));
        assert!(variant.id.is_some());
        assert!(variant.created_at.is_some());
    }
}
