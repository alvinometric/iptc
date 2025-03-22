use std::collections::HashMap;
use strum_macros::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display)]
pub enum IPTCTag {
    Null,
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
    ApplicationRecordVersion,
}

pub type ParseFn = fn(String) -> String;

// name, repeatable, default parse
pub type TagBlock = (IPTCTag, bool, ParseFn);

pub struct TagsMap {
    map: HashMap<u32, TagBlock>,
}

pub fn default_parse(s: String) -> String {
    s.to_string()
}

const PARSE_FN: ParseFn = default_parse;

pub const NULL_BLOCK: TagBlock = (IPTCTag::Null, false, PARSE_FN);

impl TagsMap {
    pub fn new() -> Self {
        let map: HashMap<u32, TagBlock> = [
            (
                0x020000,
                (IPTCTag::ApplicationRecordVersion, false, PARSE_FN),
            ),
            (0x020003, (IPTCTag::ObjectTypeReference, false, PARSE_FN)),
            (
                0x020004,
                (IPTCTag::ObjectAttributeReference, false, PARSE_FN),
            ),
            (0x020005, (IPTCTag::ObjectName, false, PARSE_FN)),
            (0x020007, (IPTCTag::EditStatus, false, PARSE_FN)),
            (0x020008, (IPTCTag::EditorialUpdate, false, PARSE_FN)),
            (0x02000a, (IPTCTag::Urgency, false, PARSE_FN)),
            (0x02000c, (IPTCTag::SubjectReference, false, PARSE_FN)),
            (0x02000f, (IPTCTag::Category, false, PARSE_FN)),
            (0x020014, (IPTCTag::SupplementalCategories, true, PARSE_FN)),
            (0x020016, (IPTCTag::FixtureId, true, PARSE_FN)),
            (0x020019, (IPTCTag::Keywords, true, PARSE_FN)),
            (0x02001a, (IPTCTag::ContentLocationCode, true, PARSE_FN)),
            (0x02001b, (IPTCTag::ContentLocationName, true, PARSE_FN)),
            (0x02001e, (IPTCTag::ReleaseDate, false, PARSE_FN)),
            (0x020023, (IPTCTag::ReleaseTime, false, PARSE_FN)),
            (0x020025, (IPTCTag::ExpirationDate, false, PARSE_FN)),
            (0x020026, (IPTCTag::ExpirationTime, false, PARSE_FN)),
            (0x020028, (IPTCTag::SpecialInstructions, false, PARSE_FN)),
            (0x02002a, (IPTCTag::ActionAdvised, false, PARSE_FN)),
            (0x02002d, (IPTCTag::ReferenceService, true, PARSE_FN)),
            (0x02002f, (IPTCTag::ReferenceDate, true, PARSE_FN)),
            (0x020032, (IPTCTag::ReferenceNumber, true, PARSE_FN)),
            (0x020037, (IPTCTag::DateCreated, false, PARSE_FN)),
            (0x02003c, (IPTCTag::TimeCreated, false, PARSE_FN)),
            (0x02003e, (IPTCTag::DigitalDateCreated, false, PARSE_FN)),
            (0x02003f, (IPTCTag::DigitalTimeCreated, false, PARSE_FN)),
            (0x020041, (IPTCTag::OriginatingProgram, false, PARSE_FN)),
            (0x020046, (IPTCTag::ProgramVersion, false, PARSE_FN)),
            (0x02004b, (IPTCTag::ObjectCycle, false, PARSE_FN)),
            (0x020050, (IPTCTag::ByLine, true, PARSE_FN)),
            (0x020055, (IPTCTag::ByLineTitle, true, PARSE_FN)),
            (0x02005a, (IPTCTag::City, false, PARSE_FN)),
            (0x02005c, (IPTCTag::SubLocation, false, PARSE_FN)),
            (0x02005f, (IPTCTag::ProvinceOrState, false, PARSE_FN)),
            (
                0x020064,
                (IPTCTag::CountryOrPrimaryLocationCode, false, PARSE_FN),
            ),
            (
                0x020065,
                (IPTCTag::CountryOrPrimaryLocationName, false, PARSE_FN),
            ),
            (
                0x020067,
                (IPTCTag::OriginalTransmissionReference, false, PARSE_FN),
            ),
            (0x020069, (IPTCTag::Headline, false, PARSE_FN)),
            (0x02006e, (IPTCTag::Credit, false, PARSE_FN)),
            (0x020073, (IPTCTag::Source, false, PARSE_FN)),
            (0x020074, (IPTCTag::CopyrightNotice, false, PARSE_FN)),
            (0x020076, (IPTCTag::Contact, false, PARSE_FN)),
            (0x020078, (IPTCTag::Caption, false, PARSE_FN)),
            (0x020079, (IPTCTag::LocalCaption, false, PARSE_FN)),
            (0x02007a, (IPTCTag::CaptionWriter, true, PARSE_FN)),
            (0x02007d, (IPTCTag::RasterizedCaption, false, PARSE_FN)),
            (0x020082, (IPTCTag::ImageType, false, PARSE_FN)),
            (0x020083, (IPTCTag::ImageOrientation, false, PARSE_FN)),
            (0x020087, (IPTCTag::LanguageIdentifier, false, PARSE_FN)),
            (0x020096, (IPTCTag::AudioType, false, PARSE_FN)),
            (0x020097, (IPTCTag::AudioSamplingRate, false, PARSE_FN)),
            (
                0x020098,
                (IPTCTag::AudioSamplingResolution, false, PARSE_FN),
            ),
            (0x020099, (IPTCTag::AudioDuration, false, PARSE_FN)),
            (0x02009a, (IPTCTag::AudioOutcue, false, PARSE_FN)),
            (0x0200b8, (IPTCTag::JobId, false, PARSE_FN)),
            (0x0200b9, (IPTCTag::MasterDocumentId, false, PARSE_FN)),
            (0x0200ba, (IPTCTag::ShortDocumentId, false, PARSE_FN)),
            (0x0200bb, (IPTCTag::UniqueDocumentId, false, PARSE_FN)),
            (0x0200bc, (IPTCTag::OwnerId, false, PARSE_FN)),
            (
                0x0200c8,
                (IPTCTag::ObjectPreviewFileFormat, false, PARSE_FN),
            ),
            (
                0x0200c9,
                (IPTCTag::ObjectPreviewFileFormatVersion, false, PARSE_FN),
            ),
            (0x0200ca, (IPTCTag::ObjectPreviewData, false, PARSE_FN)),
        ]
        .into_iter()
        .collect();

        TagsMap { map }
    }

    pub fn get(&self, tag: u32) -> Option<TagBlock> {
        self.map.get(&tag).copied()
    }
}
