use std::collections::HashMap;
use strum_macros::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display)]
pub enum IPTCTag {
    Null,
    // 0x0100 blocks
    ModelVersion,
    DateSent,
    TimeSent,
    CodedCharacterSet,
    // 0x0200 blocks
    RecordVersion,
    ObjectTypeReference,
    ObjectAttributeReference,
    ObjectName,
    EditStatus,
    EditorialUpdate,
    Urgency,
    SubjectReference,
    Category,
    SupplementalCategories,
    FixtureId,
    Keywords,
    ContentLocationCode,
    ContentLocationName,
    ReleaseDate,
    ReleaseTime,
    ExpirationDate,
    ExpirationTime,
    SpecialInstructions,
    ActionAdvised,
    ReferenceService,
    ReferenceDate,
    ReferenceNumber,
    DateCreated,
    TimeCreated,
    DigitalDateCreated,
    DigitalTimeCreated,
    OriginatingProgram,
    ProgramVersion,
    ObjectCycle,
    ByLine,
    ByLineTitle,
    City,
    SubLocation,
    ProvinceOrState,
    CountryOrPrimaryLocationCode,
    CountryOrPrimaryLocationName,
    OriginalTransmissionReference,
    Headline,
    Credit,
    Source,
    CopyrightNotice,
    Contact,
    Caption,
    LocalCaption,
    CaptionWriter,
    RasterizedCaption,
    ImageType,
    ImageOrientation,
    LanguageIdentifier,
    AudioType,
    AudioSamplingRate,
    AudioSamplingResolution,
    AudioDuration,
    AudioOutcue,
    JobId,
    MasterDocumentId,
    ShortDocumentId,
    UniqueDocumentId,
    OwnerId,
    ObjectPreviewFileFormat,
    ObjectPreviewFileFormatVersion,
    ObjectPreviewData,
    // 0x0700 blocks
    SizeMode,
}

pub type ParseFn = fn(String) -> String;

// name, repeatable, default parse
pub type TagBlock = (IPTCTag, bool, ParseFn);

pub struct TagsMap {
    map: HashMap<String, TagBlock>,
}

pub fn default_parse(s: String) -> String {
    s.to_string()
}

fn parse_short(s: String) -> String {
    // Convert bytes to number, handle both big and little endian
    let value = s
        .as_bytes()
        .iter()
        .fold(0u16, |acc, &b| (acc << 8) | b as u16);

    value.to_string()
}

const PARSE_FN: ParseFn = default_parse;
const PARSE_SHORT: ParseFn = parse_short;

pub const NULL_BLOCK: TagBlock = (IPTCTag::Null, false, PARSE_FN);

// https://exiftool.org/TagNames/IPTC.html
// In the IPTC standard, tags are identified by a record number and dataset number.
// These are in the map as "record:dataset" -> tag.
// This is because I could only find a mapping of binary -> tag and not of record+dataset -> tag.
impl TagsMap {
    pub fn new() -> Self {
        let map: HashMap<String, TagBlock> = [
            // Record 1 blocks
            ("1:0", (IPTCTag::ModelVersion, false, PARSE_SHORT)),
            ("1:5", (IPTCTag::DateSent, false, PARSE_FN)),
            ("1:80", (IPTCTag::TimeSent, false, PARSE_FN)),
            ("1:90", (IPTCTag::CodedCharacterSet, false, PARSE_FN)),
            // Record 2 blocks
            ("2:0", (IPTCTag::RecordVersion, false, PARSE_FN)),
            ("2:3", (IPTCTag::ObjectTypeReference, false, PARSE_FN)),
            ("2:4", (IPTCTag::ObjectAttributeReference, false, PARSE_FN)),
            ("2:5", (IPTCTag::ObjectName, false, PARSE_FN)),
            ("2:7", (IPTCTag::EditStatus, false, PARSE_FN)),
            ("2:8", (IPTCTag::EditorialUpdate, false, PARSE_FN)),
            ("2:10", (IPTCTag::Urgency, false, PARSE_FN)),
            ("2:12", (IPTCTag::SubjectReference, false, PARSE_FN)),
            ("2:15", (IPTCTag::Category, false, PARSE_FN)),
            ("2:20", (IPTCTag::SupplementalCategories, true, PARSE_FN)),
            ("2:22", (IPTCTag::FixtureId, true, PARSE_FN)),
            ("2:25", (IPTCTag::Keywords, true, PARSE_FN)),
            ("2:26", (IPTCTag::ContentLocationCode, true, PARSE_FN)),
            ("2:27", (IPTCTag::ContentLocationName, true, PARSE_FN)),
            ("2:30", (IPTCTag::ReleaseDate, false, PARSE_FN)),
            ("2:35", (IPTCTag::ReleaseTime, false, PARSE_FN)),
            ("2:37", (IPTCTag::ExpirationDate, false, PARSE_FN)),
            ("2:38", (IPTCTag::ExpirationTime, false, PARSE_FN)),
            ("2:40", (IPTCTag::SpecialInstructions, false, PARSE_FN)),
            ("2:42", (IPTCTag::ActionAdvised, false, PARSE_FN)),
            ("2:45", (IPTCTag::ReferenceService, true, PARSE_FN)),
            ("2:47", (IPTCTag::ReferenceDate, true, PARSE_FN)),
            ("2:50", (IPTCTag::ReferenceNumber, true, PARSE_FN)),
            ("2:55", (IPTCTag::DateCreated, false, PARSE_FN)),
            ("2:60", (IPTCTag::TimeCreated, false, PARSE_FN)),
            ("2:62", (IPTCTag::DigitalDateCreated, false, PARSE_FN)),
            ("2:63", (IPTCTag::DigitalTimeCreated, false, PARSE_FN)),
            ("2:65", (IPTCTag::OriginatingProgram, false, PARSE_FN)),
            ("2:70", (IPTCTag::ProgramVersion, false, PARSE_FN)),
            ("2:75", (IPTCTag::ObjectCycle, false, PARSE_FN)),
            ("2:80", (IPTCTag::ByLine, true, PARSE_FN)),
            ("2:85", (IPTCTag::ByLineTitle, true, PARSE_FN)),
            ("2:90", (IPTCTag::City, false, PARSE_FN)),
            ("2:92", (IPTCTag::SubLocation, false, PARSE_FN)),
            ("2:95", (IPTCTag::ProvinceOrState, false, PARSE_FN)),
            (
                "2:100",
                (IPTCTag::CountryOrPrimaryLocationCode, false, PARSE_FN),
            ),
            (
                "2:101",
                (IPTCTag::CountryOrPrimaryLocationName, false, PARSE_FN),
            ),
            (
                "2:103",
                (IPTCTag::OriginalTransmissionReference, false, PARSE_FN),
            ),
            ("2:105", (IPTCTag::Headline, false, PARSE_FN)),
            ("2:110", (IPTCTag::Credit, false, PARSE_FN)),
            ("2:115", (IPTCTag::Source, false, PARSE_FN)),
            ("2:116", (IPTCTag::CopyrightNotice, false, PARSE_FN)),
            ("2:118", (IPTCTag::Contact, false, PARSE_FN)),
            ("2:120", (IPTCTag::Caption, false, PARSE_FN)),
            ("2:121", (IPTCTag::LocalCaption, false, PARSE_FN)),
            ("2:122", (IPTCTag::CaptionWriter, true, PARSE_FN)),
            ("2:125", (IPTCTag::RasterizedCaption, false, PARSE_FN)),
            ("2:130", (IPTCTag::ImageType, false, PARSE_FN)),
            ("2:131", (IPTCTag::ImageOrientation, false, PARSE_FN)),
            ("2:135", (IPTCTag::LanguageIdentifier, false, PARSE_FN)),
            ("2:150", (IPTCTag::AudioType, false, PARSE_FN)),
            ("2:151", (IPTCTag::AudioSamplingRate, false, PARSE_FN)),
            ("2:152", (IPTCTag::AudioSamplingResolution, false, PARSE_FN)),
            ("2:153", (IPTCTag::AudioDuration, false, PARSE_FN)),
            ("2:154", (IPTCTag::AudioOutcue, false, PARSE_FN)),
            ("2:184", (IPTCTag::JobId, false, PARSE_FN)),
            ("2:185", (IPTCTag::MasterDocumentId, false, PARSE_FN)),
            ("2:186", (IPTCTag::ShortDocumentId, false, PARSE_FN)),
            ("2:187", (IPTCTag::UniqueDocumentId, false, PARSE_FN)),
            ("2:188", (IPTCTag::OwnerId, false, PARSE_FN)),
            ("2:200", (IPTCTag::ObjectPreviewFileFormat, false, PARSE_FN)),
            (
                "2:201",
                (IPTCTag::ObjectPreviewFileFormatVersion, false, PARSE_FN),
            ),
            ("2:202", (IPTCTag::ObjectPreviewData, false, PARSE_FN)),
            // Record 7 blocks
            ("7:10", (IPTCTag::SizeMode, false, PARSE_FN)),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

        TagsMap { map }
    }

    pub fn get(&self, tag: String) -> Option<TagBlock> {
        self.map.get(&tag).copied()
    }
}
