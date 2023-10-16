use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{Connection, self};
//use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
// TryStreamExt is necessary for mongodb cursor to collect into vector
use futures::stream::TryStreamExt;
use crate::ProjectDB;


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectPageInfo {
    skills: Vec<Skill>,
    projects: Vec<Project>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Skill {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // id: Option<ObjectId>,
    name: String,
    icon: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Project {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // id: Option<ObjectId>,
    name: String,
    tags: Vec<String>,
    icon: Option<String>,
}

#[get("/projects", format="json")]
pub async fn projects(db: Connection<ProjectDB>) -> Json<ProjectPageInfo>{
    let skills = match db.database("port").collection::<Skill>("skills")
        .find(None, None).await{
            Ok(cursor) =>{
                //if deserialization doesnt match database types, this panics
                match cursor.try_collect().await {
                    Ok(result)=>result,
                    Err(e)=>{
                        panic!("Deserialization error:
                            {}",e);
                    }
                }
            },
            Err(e) => {
                println!("Database error:
                    {}",e);
                vec![]
            }
        };
    let projects = match db.database("port").collection::<Project>("projects")
        .find(None, None).await{
            Ok(cursor) =>{
                //if deserialization doesnt match database types, this panics
                match cursor.try_collect().await {
                    Ok(result)=>result,
                    Err(e)=>{
                        panic!("Deserialization error:
                            {}",e);
                    }
                }
            },
            Err(e) => {
                println!("Database error:
                    {}",e);
                vec![]
            }
        };

    Json(ProjectPageInfo {skills: skills, projects: projects})
}



#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectInfo {
    name: String,
    img: Option<String>,
    desc: String,
    repo: Option<String>,
    link: Option<String>
}

#[get("/project-info/<project>", format="json")]
pub async fn project_info(db: Connection<ProjectDB>, project: String) -> Option<Json<ProjectInfo>>{
    let filter = doc! {"name": project};

    match db.database("port").collection::<ProjectInfo>("projectInfo").find_one(filter, None).await{
            Ok(result) =>{
                match result {
                    Some(r) => Some(Json(r)),
                    None => None
                }
            },
            Err(e) => {
                println!("Database error:
                    {}",e);
                None
            }
        }
}