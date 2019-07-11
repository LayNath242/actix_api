use diesel::PgConnection;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::question_answer;

#[derive(Serialize, Deserialize)]
pub struct QAlist(pub Vec<QA>);

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct QA {
    pub id: i32,
    pub title: Option<String>,
    pub descriton: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, PartialEq, Associations)]
#[belongs_to(User)]
#[table_name="question_answer"]
pub struct NewQA {
    pub title: Option<String>,
    pub descriton: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

impl QAlist {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::question_answer::dsl::*;

        let result = 
            question_answer
                .limit(10)
                .load::<QA>(connection)
                .expect("Error loading roles");

        QAlist(result)
    }
}


impl NewQA {
     pub fn create(&self, param_user_id: i32, connection: &PgConnection) ->
        Result<QA, diesel::result::Error> {
            use diesel::RunQueryDsl;

            let new_qa = NewQA {
                user_id: Some(param_user_id),
                ..self.clone()
            };

            let qa = 
                diesel::insert_into(question_answer::table)
                    .values(new_qa)
                    .get_result::<QA>(connection)?;
            Ok(qa)
        }
}

impl QA {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<QA, diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;

        question_answer::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::question_answer::dsl;

        diesel::delete(dsl::question_answer.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32,new_qa : &NewQA, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::question_answer::dsl;

        diesel::update(dsl::question_answer.find(id))
            .set(new_qa)
            .execute(connection)?;
            Ok(())
    }   

}
