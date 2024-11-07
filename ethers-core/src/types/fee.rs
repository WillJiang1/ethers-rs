use crate::types::{serde_helpers::deserialize_stringified_numeric, U256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeHistory {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub base_fee_per_gas: Vec<U256>,
    #[serde(deserialize_with = "gas_used_ratio_deser::deserialize")]
    pub gas_used_ratio: Vec<f64>,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    /// oldestBlock is returned as an unsigned integer up to geth v1.10.6. From
    /// geth v1.10.7, this has been updated to return in the hex encoded form.
    /// The custom deserializer allows backward compatibility for those clients
    /// not running v1.10.7 yet.
    pub oldest_block: U256,
    /// An (optional) array of effective priority fee per gas data points from a single block. All
    /// zeroes are returned if the block is empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reward: Vec<Vec<U256>>,
}

mod gas_used_ratio_deser {
    use serde::{Deserialize, Deserializer};

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First try to deserialize as Option<Vec<f64>>
        let opt = Option::<Vec<f64>>::deserialize(deserializer)?;
        // Return empty vec if null, otherwise return the vec
        Ok(opt.unwrap_or_default())
    }
}
