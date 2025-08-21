use rudof_lib::{
    RDFFormat, ShaclSchemaIR, ShaclValidationMode, ValidationReport,
    shacl_validation::{
        shacl_processor::ShaclProcessor, store::graph::Graph, validate_error::ValidateError,
    },
    srdf::{self, ReaderMode, SRDFGraph},
};

struct Validator {
    location_schema: ShaclSchemaIR,
}

impl Default for Validator {
    fn default() -> Self {
        let location_schema = ShaclSchemaIR::from_str(
            include_str!("testdata/locationOriented_shapes.ttl"),
            &RDFFormat::Turtle,
            None,
            &ReaderMode::default(),
        )
        .expect("Failed to parse location-oriented SHACL schema");
        Self { location_schema }
    }
}

pub fn validate_n_quads(
    schema: &ShaclSchemaIR,
    quads: &str,
) -> Result<ValidationReport, ValidateError> {
    let srdf_graph = SRDFGraph::from_str(
        quads,
        &RDFFormat::NQuads,
        None,
        &srdf::ReaderMode::default(),
    )?;

    let data = Graph::from_graph(srdf_graph)?;

    let endpoint_validation =
        rudof_lib::shacl_validation::shacl_processor::GraphValidation::from_graph(
            data,
            ShaclValidationMode::Native,
        );
    let report = endpoint_validation.validate(schema)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use crate::validate_n_quads;

    #[test]
    fn test_full_location_oriented() {
        // Minimal valid RDF data for the locationOriented.ttl SHACL shape
        let quads = include_str!("testdata/fullLocationOrientedExample.nq");

        let validator = crate::Validator::default();

        let result = validate_n_quads(&validator.location_schema, quads);
        let report = result.unwrap();
        assert!(
            report.conforms(),
            "Report should indicate non conformance for invalid location-oriented data"
        );
    }
}
