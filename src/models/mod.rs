pub mod bw_account;
pub mod types;
mod bw_group;
mod bw_policy;
mod bw_template;

// pub trait CRUD<T> {
//     type Error;
//     async fn create(&self,db: &T, item: &Self) -> Result<Self, Self::Error>;
//     fn read(&self,db: &T) -> Result<Vec<Self>, Self::Error>
//     where
//         Self: Sized;
//     fn update(&self, db: &T,item: &Self) -> Result<Self, Self::Error>;
//     fn delete(&self, db: &T ,item: &Self) -> Result<(), Self::Error>;
// }
