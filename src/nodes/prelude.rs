
use std::fmt::Debug;
use convert_case::{Case, Casing};
use enum_iterator::{Sequence, all};
use indexmap::{IndexMap, indexmap};
use serde::{Deserialize, Deserializer};


pub trait KeyValueEnum: Sequence + Debug {
    fn key(&self) -> String{
        format!("{:?}", self).to_case(Case::Camel)
    }

    fn value(&self) -> String{
        format!("{:?}", self).to_case(Case::Title)
    }

    fn map() -> IndexMap<String, String>{
        let mut indexmap: IndexMap<String, String> = indexmap!{};

        for item in all::<Self>(){
            indexmap.insert(item.key(), item.value());
        }
        indexmap
    }
}


pub fn deserialize_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>
{

    if let Ok(value) = T::deserialize(deserializer){
        Ok(value)
    }else{
        Ok(T::default())
    }
}