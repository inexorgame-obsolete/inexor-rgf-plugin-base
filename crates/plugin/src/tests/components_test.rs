use inexor_rgf_model_base::Describable;
use inexor_rgf_model_base::DescribableProperties::DESCRIPTION;
use inexor_rgf_model_base::Licensed;
use inexor_rgf_model_base::LicensedProperties::ATTRIBUTION;
use inexor_rgf_model_base::LicensedProperties::LICENSE;
use inexor_rgf_model_base::Versioned;
use inexor_rgf_model_base::VersionedProperties::VERSION;
use inexor_rgf_model_base::COMPONENT_DESCRIBABLE;
use inexor_rgf_model_base::COMPONENT_LICENSED;
use inexor_rgf_model_base::COMPONENT_VERSIONED;
use serde_json::json;

use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::entity_model;
use crate::model_base::Named;
use crate::model_base::NamedProperties::NAME;
use crate::model_base::COMPONENT_NAMED;
use crate::model_base::NAMESPACE_BASE;

// An empty model
entity_model!(ExampleNamed);
// Extend the empty model with the trait "Named"
// The trait is provided by a component_model!
impl Named for ExampleNamed {}

#[test]
fn component_named_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(NAME, json!("name"))
        .component(COMPONENT_NAMED.clone())
        .build();
    let entity = ExampleNamed::from(reactive_instance);
    assert_eq!("name", entity.get_name().unwrap());
    entity.set_name(String::from("modified_name"));
    assert_eq!("modified_name", entity.get_name().unwrap());
}

entity_model!(ExampleDescribable);
impl Describable for ExampleDescribable {}

#[test]
fn component_describable_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(DESCRIPTION, json!("description"))
        .component(COMPONENT_DESCRIBABLE.clone())
        .build();
    let entity = ExampleDescribable::from(reactive_instance);
    assert_eq!("description", entity.get_description().unwrap());
    entity.set_description(String::from("modified_description"));
    assert_eq!("modified_description", entity.get_description().unwrap());
}

entity_model!(ExampleVersioned);
impl Versioned for ExampleVersioned {}

#[test]
fn component_versioned_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(VERSION, json!("0.8.0"))
        .component(COMPONENT_VERSIONED.clone())
        .build();
    let entity = ExampleVersioned::from(reactive_instance);
    assert_eq!("0.8.0", entity.get_version().unwrap());
    entity.set_version(String::from("0.9.0"));
    assert_eq!("0.9.0", entity.get_version().unwrap());
}

entity_model!(ExampleLicensed);
impl Licensed for ExampleLicensed {}

#[test]
fn component_licensed_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new_from_type(NAMESPACE_BASE, "example")
        .property(LICENSE, json!("MIT"))
        .property(ATTRIBUTION, json!("(c) The Inexor Collective"))
        .component(COMPONENT_LICENSED.clone())
        .build();
    let entity = ExampleLicensed::from(reactive_instance);
    assert_eq!("MIT", entity.get_license().unwrap());
    assert_eq!("(c) The Inexor Collective", entity.get_attribution().unwrap());
    entity.set_license(String::from("GPL3"));
    assert_eq!("GPL3", entity.get_license().unwrap());
    entity.set_attribution(String::from("(c) 2011-2022 by The Inexor Collective"));
    assert_eq!("(c) 2011-2022 by The Inexor Collective", entity.get_attribution().unwrap());
}
