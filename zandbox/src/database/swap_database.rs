use mongodb::{
    bson::{doc, Bson},
    sync::Client,
    error::Result,
    sync::Collection,
};
use lazy_static;
use std::sync::Mutex;
use chrono::prelude::*;

lazy_static! {
    pub static ref Time: Mutex<Vec<u32>> =  Mutex::new(vec![0u32]);
}

pub fn update_time_cord(w:u32){
    Time.lock().unwrap().pop();
    Time.lock().unwrap().push(w);
}

pub fn show_time_cord() -> u32{
    Time.lock().unwrap()[0]
}


#[derive(Clone, Debug)]
pub struct SwapDatabase{
    db: Client
}

impl SwapDatabase {

    pub fn new(connection_uri: &str) -> Result<Self> {
        let client = Client::with_uri_str(connection_uri)?;
        Ok(Self { db:client })
    }

    pub fn get_collection(&self) -> Collection{
        let database = self.db.database("mydb");
        let collection = database.collection("books");
        collection
    }

    pub fn accumlate_amount(&self, pair:&str ,amount:u64 ,count:u64, fee:u64){
        let time = show_time_cord();
        if time != Local::now().hour(){
            // reset data
            self.insert_update_amount(pair,amount,count,fee);
        }else{
            // add data
            match self.find_amount(pair){
                Some((amount_old,count_old,fee_old)) => {
                    self.insert_update_amount(pair,amount_old  as u64 +amount,
                                              count_old  as u64 + count, fee_old  as u64 + fee, );
                },
                None => { self.insert_update_amount(pair,amount,count,fee);}
            }
        }
        update_time_cord(Local::now().hour());
    }

    pub fn insert_update_amount(&self, pair:&str ,amount:u64 ,count:u64, fee:u64){
        let collection = self.get_collection();

        let format = format!("{}_{}",pair,Local::now().hour());

        let res = self.is_exists(&format);

        if res {
            collection.update_one(doc! { "pair":format.clone() }
                                  , doc! { "pair":format, "value":amount, "count":count ,"fee":fee }, None);
        }else{
            collection.insert_one(doc! { "pair":format, "value":amount, "count": 1u64 ,"fee":fee}, None);
        }

    }

    pub fn find_amount(&self, pair:&str ) -> Option<(i64,i64,i64)>{
        let collection = self.get_collection();

        let format = format!("{}_{}",pair,Local::now().hour());

        let res = collection.find_one(doc! { "pair":format }, None).unwrap();

        if res.is_some(){
            let doc = res.unwrap();
            let data = doc.get("value").unwrap().as_i64().unwrap();
            let count = doc.get("count").unwrap().as_i64().unwrap();
            let fee = doc.get("fee").unwrap().as_i64().unwrap();
            Some((data,count,fee))
        }else {
            None
        }
    }

    pub fn find_specific_amount(&self, key:&str ) -> Option<(i64,i64,i64)>{
        let collection = self.get_collection();

        let res = collection.find_one(doc! { "pair":key }, None).unwrap();

        if res.is_some(){
            let doc = res.unwrap();
            let data = doc.get("value").unwrap().as_i64().unwrap();
            let count = doc.get("count").unwrap().as_i64().unwrap();
            let fee = doc.get("fee").unwrap().as_i64().unwrap();
            Some((data,count,fee))
        }else {
            None
        }
    }

    pub fn find_all(&self, pair:&str) -> (i64,i64,i64){
        let collection = self.get_collection();
        let mut all_amount = 0;
        let mut all_count = 0;
        let mut all_fee = 0;
        for i in 0..=24{
            match self.find_specific_amount( &format!("{}_{}",pair,i) ){
                Some((amount,count,fee)) => {
                    all_amount += amount;
                    all_count += count;
                    all_fee += fee;
                },
                None => {},
            }
        }
        (all_amount,all_count,all_fee)
    }

    pub fn is_exists(&self, pair:&str ) -> bool{
        let collection = self.get_collection();

        let res = collection.find_one(doc! { "pair":pair }, None).unwrap();
        if res.is_some(){
            return true;
        }
        false
    }

    pub fn dup_token(&self, tokenname:&str ) -> bool{
        let collection = self.get_collection();

        let res = collection.find_one(doc! { "tokenname":tokenname }, None).unwrap();
        if res.is_some(){
            return true;
        }
        false
    }

    pub fn add_token(&self,address:&str,tokenname:&str){
        let collection = self.get_collection();

        let res = self.dup_token(tokenname);

        if res {
            collection.update_one(doc! { "tokenname":tokenname }
                                  , doc! { "address":address, "tokenname":tokenname }, None);
        }else{
            collection.insert_one(doc! { "address":address, "tokenname":tokenname }, None);
        }
    }

    pub fn tokenname_convert(&self,address:&str) -> Option<String> {
        let collection = self.get_collection();

        let res = collection.find_one(doc! { "address":address }, None).unwrap();

        if res.is_some(){
            let doc = res.unwrap();
            let data = doc.get("tokenname").unwrap().as_str().unwrap();
            Some(data.to_string())
        }else {
            None
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_add(){

    }
}