use diesel::RunQueryDsl;

use app::models::User;
use app::schema::users::dsl::users;

use crate::helpers::spawn_app;

#[tokio::test]
pub async fn create_user_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=ryuma%20taguchi&email=ryuma017%40gmail.com";

    // Act
    let response = app.post_user_data(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
pub async fn create_user_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let body = "";

    // Act
    let response = app.post_user_data(body.into()).await;

    // Assert
    assert_eq!(400, response.status().as_u16())
}

#[tokio::test]
pub async fn create_user_persists_the_new_user() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=ryuma%20taguchi&email=ryuma017%40gmail.com";

    // Act
    app.post_user_data(body.into()).await;

    // Assert
    let pooled_connection = app.db_pool.get().unwrap();
    let saved = users
        .first::<User>(&pooled_connection)
        .expect("Failed to fetch saved user");

    println!("\nemail: {}\nname: {}\n", saved.email, saved.name);

    assert_eq!(saved.email, "ryuma017@gmail.com");
    assert_eq!(saved.name, "ryuma taguchi");
}
