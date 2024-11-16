use sqlx::FromRow;

/// ```sql
/// create table Persona (
///     id int primary key auto_increment,
///     name varchar(50),
///     surname varchar(50),
///     age int
/// );
/// ```
#[derive(FromRow)]
pub struct Persona {
    #[sqlx(rename = "id")]
    id: i32,
    #[sqlx(rename = "name")]
    name: String,
    #[sqlx(rename = "surname")]
    surname: String,
    #[sqlx(rename = "age")]
    age: i32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_persona() -> Result<(), sqlx::Error> {
        let persona = Persona {
            id: -1,
            name: "John".to_string(),
            surname: "Doe".to_string(),
            age: 30,
        };
        
        let connection_pool = crate::init_db_pool().await?;
        
        sqlx::query!(
            "INSERT INTO Persona (name, surname, age) VALUES (?, ?, ?)",
            persona.name,
            persona.surname,
            persona.age
        ).execute(&connection_pool).await?;
        
        let result = sqlx::query!(
            "SELECT id, name, surname, age FROM Persona"
        ).fetch_one(&connection_pool).await?;
        
        assert_ne!(result.id, persona.id);
        assert_eq!(persona.name, result.name.unwrap());
        assert_eq!(persona.surname, result.surname.unwrap());
        assert_eq!(persona.age, result.age.unwrap());
        
        sqlx::query!(
            "DELETE FROM Persona"
        ).execute(&connection_pool).await?;
        
        let result = sqlx::query!(
            "SELECT id, name, surname, age FROM Persona"
        ).fetch_all(&connection_pool).await?;
        
        assert!(result.is_empty());
        
        Ok(())
    }
}