use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::auth::security::hash_password;
use crate::model_view::ModelViewSet;
use crate::schema::user as table_user;
use crate::schema::user::dsl::user as users;
use crate::schema::user::id as user_id;
use crate::schema::user::username as user_name;

use super::Nullable;
use super::Repository;

#[derive(Queryable, Serialize, Default, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: Nullable,
    pub first_name: Nullable,
    pub last_name: Nullable,
    pub is_admin: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = table_user)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: Nullable,
    pub first_name: Nullable,
    pub last_name: Nullable,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = table_user)]
pub struct UpdateUser {
    pub username: String,
    pub email: Nullable,
    pub first_name: Nullable,
    pub last_name: Nullable,
}

impl Repository<Self> for User {
    fn save(&self) -> bool {
        diesel::insert_into(users)
            .values(NewUser {
                username: self.username.clone(),
                password: hash_password(&self.password),
                email: self.email.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            })
            .execute(&mut Self::establish_connection())
            .is_ok()
    }

    fn update(&self) -> bool {
        diesel::update(users.filter(user_id.eq(self.id)))
            .set(UpdateUser {
                username: self.username.clone(),
                email: self.email.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            })
            .execute(&mut Self::establish_connection())
            .is_ok()
    }

    fn delete(id: i32) -> bool {
        diesel::delete(users.filter(user_id.eq(id)))
            .execute(&mut Self::establish_connection())
            .is_ok()
    }

    fn get(id: i32) -> Vec<Self> {
        if let Ok(user) = users
            .find(id)
            .limit(1)
            .load::<Self>(&mut Self::establish_connection())
        {
            user
        } else {
            Vec::new()
        }
    }

    fn all() -> Vec<Self> {
        if let Ok(user) = users.load::<Self>(&mut Self::establish_connection()) {
            user
        } else {
            Vec::new()
        }
    }
}

impl User {
    pub fn get_by_username(username: &str) -> Vec<Self> {
        if let Ok(user) = users
            .filter(user_name.eq(username))
            .limit(1)
            .load::<Self>(&mut Self::establish_connection())
        {
            user
        } else {
            Vec::new()
        }
    }
}

impl ModelViewSet<Self> for User {}
