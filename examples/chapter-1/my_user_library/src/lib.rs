#[cfg(feature = "serde_json_support")]
use serde::{Serialize, Deserialize};


#[cfg(feature = "bincode_support")]
use bincode::{config, Decode, Encode};

#[cfg_attr(feature = "serde_json_support", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode_support", derive(Decode, Encode))]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[cfg(feature = "serde_json_support")]
impl User {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize to JSON")
    }
}

#[cfg(feature = "bincode_support")]
impl User {
    pub fn to_bincode(&self) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).expect("Failed to serialize to bincode")
    }
}

mod tests {
    use super::User;

    #[test]
    fn test_user_struct() {
        let user = User {
            name: "Alice".to_string(),
            age: 30,
        };

        #[cfg(feature = "serde_json_support")]
        assert_eq!(user.to_json(), r#"{"name":"Alice","age":30}"#);

        #[cfg(feature = "bincode_support")]
        assert_eq!(user.to_bincode(), vec![5, 65, 108, 105, 99, 101, 30]);
    }
}