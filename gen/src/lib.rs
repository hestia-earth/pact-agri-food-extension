//! iLEAP Data Model Extension data model
use chrono::{DateTime, Utc};
use pact_data_model::{
    CarbonFootprint, CharacterizationFactors, CompanyIdSet, CrossSectoralStandard,
    CrossSectoralStandardSet, DataModelExtension, DeclaredUnit, ExemptedEmissionsPercent,
    GeographicScope, IpccCharacterizationFactorsSource, PfId, PfStatus, ProductFootprint,
    ProductIdSet, SpecVersionString, Urn, VersionInteger, WrappedDecimal,
};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentFootprint {
    pub mass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_items: Option<String>,
    pub shipment_id: String,
    pub tces: NonEmptyVec<Tce>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NonEmptyVec<T>(pub Vec<T>);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", rename = "TCE")]
pub struct Tce {
    pub tce_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_tce_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoc_id: Option<String>,
    pub shipment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignment_id: Option<String>,
    pub mass: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    pub distance: GlecDistance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Location>,
    pub transport_activity: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voyage_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoterms: Option<Incoterms>,
    #[serde(rename = "co2eWTW")]
    pub co2e_wtw: WrappedDecimal,
    #[serde(rename = "co2eTTW")]
    pub co2e_ttw: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none", rename = "noxTTW")]
    pub nox_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "soxTTW")]
    pub sox_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ch4TTW")]
    pub ch4_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pmTTW")]
    pub pm_ttw: Option<WrappedDecimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Incoterms {
    Exw,
    Fca,
    Cpt,
    Cip,
    Dap,
    Dpu,
    Ddp,
    Fas,
    Fob,
    Cfr,
    Cif,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", rename = "TOC")]
pub struct Toc {
    pub toc_id: String,
    pub is_verified: bool,
    pub is_accredited: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mode: TransportMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_distance_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TemperatureControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truck_loading_sequence: Option<TruckLoadingSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_shipping_option: Option<AirShippingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_length: Option<FlightLength>,
    pub energy_carriers: NonEmptyVec<EnergyCarrier>,
    #[serde(rename = "co2eIntensityWTW")]
    pub co2e_intensity_wtw: WrappedDecimal,
    #[serde(rename = "co2eIntensityTTW")]
    pub co2e_intensity_ttw: WrappedDecimal,
    pub co2e_intensity_throughput: TocCo2eIntensityThroughput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glec_data_quality_index: Option<GlecDataQualityIndex>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TocCo2eIntensityThroughput {
    #[serde(rename = "TEUkm")]
    TEUkm,
    Tkm,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TemperatureControl {
    Ambient,
    Refrigerated,
    Mixed,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TruckLoadingSequence {
    Ltl,
    Ftl,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AirShippingOption {
    #[serde(rename = "belly freight")]
    BellyFreight,
    Freighter,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FlightLength {
    #[serde(rename = "short-haul")]
    ShortHaul,
    #[serde(rename = "long-haul")]
    LongHaul,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlecDataQualityIndex(pub u8);

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", rename = "HOC")]
pub struct Hoc {
    pub hoc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_verified: bool,
    pub is_accredited: bool,
    pub hub_type: HubType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TemperatureControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transport_mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transport_mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    pub energy_carriers: NonEmptyVec<EnergyCarrier>,
    #[serde(rename = "co2eIntensityWTW")]
    pub co2e_intensity_wtw: WrappedDecimal,
    #[serde(rename = "co2eIntensityTTW")]
    pub co2e_intensity_ttw: WrappedDecimal,
    pub co2e_intensity_throughput: HocCo2eIntensityThroughput,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HocCo2eIntensityThroughput {
    #[serde(rename = "TEU")]
    TEU,
    Tonnes,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum HubType {
    Transshipment,
    StorageAndTransshipment,
    Warehouse,
    LiquidBulkTerminal,
    MaritimeContainerterminal,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", rename = "TAD")]
/// Data Type "Transport Activity Data" of the iLEAP Technical Specifications
pub struct Tad {
    pub activity_id: ActivityId,              // Unique
    pub consignment_ids: Vec<ConsignementId>, // Unique
    pub distance: GlecDistance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_factor: Option<WrappedDecimal>, // TODO replace with propoer type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_distance_factor: Option<WrappedDecimal>, // TODO replace with propoer type
    pub origin: Location,
    pub destination: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    // TODO: verify whether the absence of this property is intended.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub energy_carrier: EnergyCarrier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstocks: Option<Vec<Feedstock>>,
}

pub type ActivityId = String;
pub type ConsignementId = String;
pub type ShipmentId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GlecDistance {
    Actual(WrappedDecimal),
    Gcd(WrappedDecimal),
    Sfd(WrappedDecimal),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    pub city: String,
    pub country: GeographicScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata: Option<IataCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locode: Option<Locode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uic: Option<UicCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<WrappedDecimal>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum TransportMode {
    Road,
    Rail,
    Air,
    Sea,
    InlandWaterway,
    //Hub,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum PackagingOrTrEqType {
    Box,
    Pallet,
    Container,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnergyCarrier {
    pub energy_carrier: EnergyCarrierType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstocks: Option<Vec<Feedstock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption_unit: Option<String>,
    #[serde(rename = "emissionFactorWTW")]
    pub emission_factor_wtw: WrappedDecimal,
    #[serde(rename = "emissionFactorTTW")]
    pub emission_factor_ttw: WrappedDecimal,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum EnergyCarrierType {
    Diesel,
    #[serde(rename = "HVO")]
    Hvo,
    Petrol,
    #[serde(rename = "CNG")]
    Cng,
    #[serde(rename = "LNG")]
    Lng,
    #[serde(rename = "LPG")]
    Lpg,
    #[serde(rename = "HFO")]
    Hfo,
    #[serde(rename = "MGO")]
    Mgo,
    #[serde(rename = "Aviation fuel")]
    AviationFuel,
    Hydrogen,
    Methanol,
    Electric,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feedstock {
    pub feedstock: FeedstockType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstock_percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_provenance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum FeedstockType {
    Fossil,
    #[serde(rename = "Natural gas")]
    NaturalGas,
    Grid,
    #[serde(rename = "Renewable electricity")]
    RenewableElectricity,
    #[serde(rename = "Cooking oil")]
    CookingOil,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IataCode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for IataCode {
    fn from(s: String) -> Self {
        if s.len() <= 3 {
            IataCode(s)
        } else {
            panic!("IATA code must be 3 characters long")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Locode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for Locode {
    fn from(s: String) -> Self {
        if s.len() == 5 {
            Locode(s)
        } else {
            panic!("LOCODE must be 5 characters long")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UicCode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for UicCode {
    fn from(s: String) -> Self {
        if s.len() == 2 {
            UicCode(s)
        } else {
            panic!("UIC code must be 2 characters long")
        }
    }
}

impl<T> From<Vec<T>> for NonEmptyVec<T> {
    fn from(v: Vec<T>) -> NonEmptyVec<T> {
        if v.is_empty() {
            panic!("Vector must not be empty")
        } else {
            NonEmptyVec(v)
        }
    }
}

impl From<u8> for GlecDataQualityIndex {
    fn from(v: u8) -> GlecDataQualityIndex {
        if v > 4 {
            panic!("Glec Data Quality Index must be between 0 and 4")
        } else {
            GlecDataQualityIndex(v)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum ILeapType {
    ShipmentFootprint(ShipmentFootprint),
    Toc(Toc),
    Hoc(Hoc),
}

pub enum HocTeuContainerSize {
    Normal,
    Light,
    Heavy,
}

pub enum CharFactors {
    Ar5,
    Ar6,
}

pub fn to_pcf(
    ileap_type: ILeapType,
    company_name: &str,
    company_urn: &str,
    hoc_container_size: Option<HocTeuContainerSize>,
    characterization_factors: Option<Vec<CharFactors>>,
) -> ProductFootprint {
    let (characterization_factors, characterization_factors_sources) =
        match characterization_factors {
            None => (
                CharacterizationFactors::Ar5,
                vec![IpccCharacterizationFactorsSource::from("AR5".to_string())],
            ),
            Some(cf) => {
                if cf.is_empty() {
                    (
                        CharacterizationFactors::Ar5,
                        vec![IpccCharacterizationFactorsSource::from("AR5".to_string())],
                    )
                } else {
                    let cf: Vec<IpccCharacterizationFactorsSource> = cf
                        .iter()
                        .map(|cf| match cf {
                            CharFactors::Ar5 => {
                                IpccCharacterizationFactorsSource::from("AR5".to_string())
                            }
                            CharFactors::Ar6 => {
                                IpccCharacterizationFactorsSource::from("AR6".to_string())
                            }
                        })
                        .collect();

                    let characterization_factors = if cf
                        .contains(&IpccCharacterizationFactorsSource::from("AR5".to_string()))
                    {
                        CharacterizationFactors::Ar5
                    } else {
                        CharacterizationFactors::Ar6
                    };

                    (characterization_factors, cf)
                }
            }
        };

    struct MappedFields {
        product_id_type: String,
        id: String,
        product_name_company: String,
        declared_unit: DeclaredUnit,
        unitary_product_amount: Decimal,
        p_cf_excluding_biogenic: Decimal,
    }

    let MappedFields {
        product_id_type,
        id,
        product_name_company,
        declared_unit,
        unitary_product_amount,
        p_cf_excluding_biogenic,
    } = match ileap_type {
        ILeapType::ShipmentFootprint(ref shipment) => MappedFields {
            product_id_type: "shipment".to_string(),
            id: shipment.shipment_id.clone(),
            product_name_company: format!("ShipmentFootprint with id {}", shipment.shipment_id),
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: shipment
                .tces
                .0
                .iter()
                .fold(Decimal::from(0), |acc, tce| acc + tce.co2e_wtw.0),
            p_cf_excluding_biogenic: shipment
                .tces
                .0
                .iter()
                .fold(Decimal::from(0), |acc, tce| acc + tce.co2e_wtw.0),
        },
        ILeapType::Toc(ref toc) => MappedFields {
            product_id_type: "toc".to_string(),
            id: toc.toc_id.clone(),
            product_name_company: format!("TOC with ID {}", toc.toc_id),
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: Decimal::from(1),
            p_cf_excluding_biogenic: toc.co2e_intensity_wtw.0,
        },
        ILeapType::Hoc(ref hoc) => MappedFields {
            product_id_type: "hoc".to_string(),
            id: hoc.hoc_id.clone(),
            product_name_company: format!("HOC with ID {}", hoc.hoc_id),
            declared_unit: DeclaredUnit::Kilogram,
            unitary_product_amount: Decimal::from(1000),
            p_cf_excluding_biogenic: match hoc.co2e_intensity_throughput {
                HocCo2eIntensityThroughput::TEU => {
                    get_teu_co2e_intensity_wtw(hoc.co2e_intensity_wtw.0, hoc_container_size)
                }
                HocCo2eIntensityThroughput::Tonnes => hoc.co2e_intensity_wtw.0,
            },
        },
    };

    fn get_teu_co2e_intensity_wtw(
        hoc_co2e_intensity_wtw: Decimal,
        hoc_container_size: Option<HocTeuContainerSize>,
    ) -> Decimal {
        match hoc_container_size {
            Some(HocTeuContainerSize::Normal) => hoc_co2e_intensity_wtw * Decimal::from(10000),
            Some(HocTeuContainerSize::Light) => hoc_co2e_intensity_wtw * Decimal::from(6000),
            Some(HocTeuContainerSize::Heavy) => hoc_co2e_intensity_wtw * Decimal::from(14050),
            None => {
                println!("Warning: HOC TEU container size not specified, using normal container");
                hoc_co2e_intensity_wtw * Decimal::from(10000)
            }
        }
    }

    let data_schema_id = if product_id_type == "shipment" {
        "shipment-footprint"
    } else {
        product_id_type.as_str()
    };

    ProductFootprint {
        id: PfId(Uuid::new_v4()),
        spec_version: SpecVersionString("2.2.0".to_string()),
        preceding_pf_ids: None,
        version: VersionInteger(1),
        created: Utc::now(),
        updated: None,
        status: PfStatus::Active,
        status_comment: None,
        validity_period_start: None,
        validity_period_end: None,
        company_name: company_name.to_string().into(),
        company_ids: CompanyIdSet(vec![Urn::from(company_urn.to_string())]),
        product_description: "".to_string(),
        product_ids: ProductIdSet(vec![Urn::from(format!(
            "urn:pathfinder:product:customcode:vendor-assigned:{product_id_type}:{id}"
        ))]),
        product_category_cpc: String::from("83117").into(),
        product_name_company: product_name_company.into(),
        comment: "".to_string(),
        pcf: CarbonFootprint {
            declared_unit,
            unitary_product_amount: unitary_product_amount.into(),
            p_cf_excluding_biogenic: p_cf_excluding_biogenic.into(),
            p_cf_including_biogenic: Some(p_cf_excluding_biogenic.into()), // TODO: to be clarified in the Tech Specs
            fossil_ghg_emissions: p_cf_excluding_biogenic.into(),
            fossil_carbon_content: p_cf_excluding_biogenic.into(), // TODO: to be clarified in the Tech Specs
            biogenic_carbon_content: p_cf_excluding_biogenic.into(), // TODO: to be clarified in the Tech Specs
            d_luc_ghg_emissions: None,
            land_management_ghg_emissions: None,
            other_biogenic_ghg_emissions: None,
            i_luc_ghg_emissions: None,
            biogenic_carbon_withdrawal: None,
            aircraft_ghg_emissions: None,
            characterization_factors,
            ipcc_characterization_factors_sources: characterization_factors_sources.into(),
            cross_sectoral_standards_used: CrossSectoralStandardSet(vec![
                CrossSectoralStandard::ISO14083,
            ]),
            product_or_sector_specific_rules: None, // TODO: get clarity on whether GLEC should be specified
            biogenic_accounting_methodology: None,
            boundary_processes_description: "".to_string(),
            reference_period_start: Utc::now(),
            reference_period_end: (Utc::now() + chrono::Duration::days(364)),
            geographic_scope: None,
            secondary_emission_factor_sources: None,
            exempted_emissions_percent: ExemptedEmissionsPercent(0.into()),
            exempted_emissions_description: "".to_string(),
            packaging_emissions_included: false,
            packaging_ghg_emissions: None,
            allocation_rules_description: None,
            uncertainty_assessment_description: None,
            primary_data_share: None,
            dqi: None,
            assurance: None,
        },
        extensions: Some(vec![DataModelExtension {
            spec_version: SpecVersionString::from("0.2.0".to_string()),
            data_schema: format!("https://api.ileap.sine.dev/{data_schema_id}.json"),
            documentation: Some("https://sine-fdn.github.io/ileap-extension/".to_string()),
            data: serde_json::to_value(&ileap_type)
                .unwrap()
                .as_object()
                .unwrap()
                .to_owned(),
        }]),
    }
}
