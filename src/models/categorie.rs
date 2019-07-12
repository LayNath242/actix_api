use diesel::PgConnection;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::categories;


#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Categorie {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, Associations, PartialEq)]
#[belongs_to(User)]
#[table_name="categories"]
pub struct NewCategorie {
    pub title: String,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

impl NewCategorie {
    pub fn create(&self, param_user_id: i32, connection: &PgConnection) ->
        Result<Categorie, diesel::result::Error> {
            use diesel::RunQueryDsl;

            let new_categorie = NewCategorie {
                user_id: Some(param_user_id),
                ..self.clone()
            };
            let categorie = 
                diesel::insert_into(categories::table)
                        .values(new_categorie)
                        .get_result::<Categorie>(connection)?;
            Ok(categorie)
        }
}

#[derive(Serialize, Deserialize)]
pub struct CategorieList (pub Vec<Categorie>);

impl CategorieList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::categories::dsl::*;

        let result = 
            categories
                .limit(10)
                .load::<Categorie>(connection)
                .expect("Error loading categories");
        CategorieList(result)
    }
}


impl Categorie {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Categorie, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;

        categories::table.find(id)
        .first(connection)
    }
    
    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::categories::dsl;

        diesel::delete(dsl::categories.find(id))
        .execute(connection)?;
        Ok(())
                
    }
    pub fn update(id: &i32,new_categorie : &NewCategorie, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::categories::dsl;

        diesel::update(dsl::categories.find(id))
            .set(new_categorie)
            .execute(connection)?;
            Ok(())
    }

}
