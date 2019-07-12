use diesel::PgConnection;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::comments;

#[derive(Serialize, Deserialize)]
pub struct Commentlist(pub Vec<Comment>);

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Comment {
    pub id: i32,
    pub descriton: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, Associations ,PartialEq)]
#[belongs_to(User)]
#[table_name="comments"]
pub struct NewComment {
    pub descriton: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

impl Commentlist {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::comments::dsl::*;

        let result = 
            comments
                .limit(10)
                .load::<Comment>(connection)
                .expect("Error loading comment");

        Commentlist(result)
    }
}


impl NewComment {
     pub fn create(&self, param_user_id: i32, connection: &PgConnection) ->
        Result<Comment, diesel::result::Error> {
            use diesel::RunQueryDsl;

            let new_comment = NewComment {
                user_id: Some(param_user_id),
                ..self.clone()
            };

            let comment = 
                diesel::insert_into(comments::table)
                    .values(new_comment)
                    .get_result::<Comment>(connection)?;
            Ok(comment)
        }
}

impl Comment {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Comment, diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;

        comments::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, 
                param_user_id: i32,
                connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::comments::dsl;

        diesel::delete(dsl::comments
        .filter(dsl::user_id.eq(param_user_id))
        .find(id))
        .execute(connection)?;
        Ok(())
    }


    pub fn update(id: &i32,
                  param_user_id: i32,
                  new_comment : &NewComment, 
                  connection: &PgConnection) 
                  -> Result<(), diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::comments::dsl;

        let new_comment_replace = NewComment {
            user_id: Some(param_user_id),
            ..new_comment.clone()
        };


        diesel::update(dsl::comments.find(id))
            .filter(dsl::user_id.eq(param_user_id))
            .set(new_comment_replace)
            .execute(connection)?;
            Ok(())
    }   

}