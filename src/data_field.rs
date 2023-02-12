use crate::subfield::SubField;

struct DataField {
    tag: String,
    indicator1: String,
    indicator2: String,
    subfields: Vec<SubField>,
}
