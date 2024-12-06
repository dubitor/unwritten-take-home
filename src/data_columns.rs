use polars::prelude::*;
use tokio_postgres::Row;

use crate::errors::UnwrittenError;

pub const ARRAY_AGG_COLUMNS_QUERY: &str = r"
        SELECT 
            ARRAY_AGG(id) AS ids,
            ARRAY_AGG(name) AS names,
            ARRAY_AGG(value) AS values 
        FROM playing_with_neon";

#[derive(Debug)]
pub struct DataColumns {
    ids: Vec<i32>,
    names: Vec<String>,
    values: Vec<f32>,
}
impl TryFrom<&Row> for DataColumns {
    type Error = UnwrittenError;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            ids: row.try_get("ids")?,
            names: row.try_get("names")?,
            values: row.try_get("values")?,
        })
    }
}
impl TryFrom<DataColumns> for LazyFrame {
    type Error = UnwrittenError;

    fn try_from(cols: DataColumns) -> Result<LazyFrame, Self::Error> {
        let df = DataFrame::new(vec![
            Column::new("id".into(), cols.ids),
            Column::new("name".into(), cols.names),
            Column::new("value".into(), cols.values),
        ])?;
        Ok(df.lazy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe_conversion() {
        // Setup sample data for testing the conversion.
        let columns = DataColumns {
            ids: vec![1, 2, 3],
            names: vec!["Alice".into(), "Bob".into(), "Charlie".into()],
            values: vec![0.2, 0.2, 0.3],
        };

        // Directly test conversion logic.
        let lf_result = LazyFrame::try_from(columns);

        // Make sure the conversion is successful.
        assert!(
            lf_result.is_ok(),
            "DataFrame conversion failed: {:?}",
            lf_result.err()
        );
    }
}
