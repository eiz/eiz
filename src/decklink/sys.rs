#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_parens
)]
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};
use winapi::shared::windef::RECT;
use winapi::shared::{minwindef::BOOL, wtypes::BSTR};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HRESULT, LONGLONG, ULONGLONG};
use winapi::{DEFINE_GUID, ENUM, RIDL};

pub type BMDTimeValue = u64;
pub type BMDTimeScale = u64;
pub type BMDTimecodeBCD = c_uint;
pub type BMDTimecodeUserBits = c_uint;

ENUM! {enum BMDTimecodeFlags {
    bmdTimecodeFlagDefault = 0,
    bmdTimecodeIsDropFrame = 1 << 0,
    bmdTimecodeFieldMark = 1 << 1,
    bmdTimecodeColorFrame = 1 << 2,
    bmdTimecodeEmbedRecordingTrigger = 1 << 3,
    bmdTimecodeRecordingTriggered = 1 << 4,
}}

ENUM! {enum BMDVideoConnection {
    bmdVideoConnectionUnspecified = 0,
    bmdVideoConnectionSDI = 1 << 0,
    bmdVideoConnectionHDMI = 1 << 1,
    bmdVideoConnectionOpticalSDI = 1 << 2,
    bmdVideoConnectionComponent = 1 << 3,
    bmdVideoConnectionComposite = 1 << 4,
    bmdVideoConnectionSVideo = 1 << 5,
}}

ENUM! {enum BMDAudioConnection {
    bmdAudioConnectionEmbedded = (1 << 0),
    bmdAudioConnectionAESEBU = (1 << 1),
    bmdAudioConnectionAnalog = (1 << 2),
    bmdAudioConnectionAnalogXLR = (1 << 3),
    bmdAudioConnectionAnalogRCA = (1 << 4),
    bmdAudioConnectionMicrophone = (1 << 5),
    bmdAudioConnectionHeadphones = (1 << 6),
}}

ENUM! {enum BMDDeckControlConnection {
    bmdDeckControlConnectionRS422Remote1 = (1 << 0),
    bmdDeckControlConnectionRS422Remote2 = (1 << 1),
}}

ENUM! {enum BMDDisplayMode {
    bmdModeNTSC = 0x6e747363,
    bmdModeNTSC2398 = 0x6e743233,
    bmdModePAL = 0x70616c20,
    bmdModeNTSCp = 0x6e747370,
    bmdModePALp = 0x70616c70,
    bmdModeHD1080p2398 = 0x32337073,
    bmdModeHD1080p24 = 0x32347073,
    bmdModeHD1080p25 = 0x48703235,
    bmdModeHD1080p2997 = 0x48703239,
    bmdModeHD1080p30 = 0x48703330,
    bmdModeHD1080p4795 = 0x48703437,
    bmdModeHD1080p48 = 0x48703438,
    bmdModeHD1080p50 = 0x48703530,
    bmdModeHD1080p5994 = 0x48703539,
    bmdModeHD1080p6000 = 0x48703630,
    bmdModeHD1080p9590 = 0x48703935,
    bmdModeHD1080p96 = 0x48703936,
    bmdModeHD1080p100 = 0x48703130,
    bmdModeHD1080p11988 = 0x48703131,
    bmdModeHD1080p120 = 0x48703132,
    bmdModeHD1080i50 = 0x48693530,
    bmdModeHD1080i5994 = 0x48693539,
    bmdModeHD1080i6000 = 0x48693630,
    bmdModeHD720p50 = 0x68703530,
    bmdModeHD720p5994 = 0x68703539,
    bmdModeHD720p60 = 0x68703630,
    bmdMode2k2398 = 0x326b3233,
    bmdMode2k24 = 0x326b3234,
    bmdMode2k25 = 0x326b3235,
    bmdMode2kDCI2398 = 0x32643233,
    bmdMode2kDCI24 = 0x32643234,
    bmdMode2kDCI25 = 0x32643235,
    bmdMode2kDCI2997 = 0x32643239,
    bmdMode2kDCI30 = 0x32643330,
    bmdMode2kDCI4795 = 0x32643437,
    bmdMode2kDCI48 = 0x32643438,
    bmdMode2kDCI50 = 0x32643530,
    bmdMode2kDCI5994 = 0x32643539,
    bmdMode2kDCI60 = 0x32643630,
    bmdMode2kDCI9590 = 0x32643935,
    bmdMode2kDCI96 = 0x32643936,
    bmdMode2kDCI100 = 0x32643130,
    bmdMode2kDCI11988 = 0x32643131,
    bmdMode2kDCI120 = 0x32643132,
    bmdMode4K2160p2398 = 0x346b3233,
    bmdMode4K2160p24 = 0x346b3234,
    bmdMode4K2160p25 = 0x346b3235,
    bmdMode4K2160p2997 = 0x346b3239,
    bmdMode4K2160p30 = 0x346b3330,
    bmdMode4K2160p4795 = 0x346b3437,
    bmdMode4K2160p48 = 0x346b3438,
    bmdMode4K2160p50 = 0x346b3530,
    bmdMode4K2160p5994 = 0x346b3539,
    bmdMode4K2160p60 = 0x346b3630,
    bmdMode4K2160p9590 = 0x346b3935,
    bmdMode4K2160p96 = 0x346b3936,
    bmdMode4K2160p100 = 0x346b3130,
    bmdMode4K2160p11988 = 0x346b3131,
    bmdMode4K2160p120 = 0x346b3132,
    bmdMode4kDCI2398 = 0x34643233,
    bmdMode4kDCI24 = 0x34643234,
    bmdMode4kDCI25 = 0x34643235,
    bmdMode4kDCI2997 = 0x34643239,
    bmdMode4kDCI30 = 0x34643330,
    bmdMode4kDCI4795 = 0x34643437,
    bmdMode4kDCI48 = 0x34643438,
    bmdMode4kDCI50 = 0x34643530,
    bmdMode4kDCI5994 = 0x34643539,
    bmdMode4kDCI60 = 0x34643630,
    bmdMode4kDCI9590 = 0x34643935,
    bmdMode4kDCI96 = 0x34643936,
    bmdMode4kDCI100 = 0x34643130,
    bmdMode4kDCI11988 = 0x34643131,
    bmdMode4kDCI120 = 0x34643132,
    bmdMode8K4320p2398 = 0x386b3233,
    bmdMode8K4320p24 = 0x386b3234,
    bmdMode8K4320p25 = 0x386b3235,
    bmdMode8K4320p2997 = 0x386b3239,
    bmdMode8K4320p30 = 0x386b3330,
    bmdMode8K4320p4795 = 0x386b3437,
    bmdMode8K4320p48 = 0x386b3438,
    bmdMode8K4320p50 = 0x386b3530,
    bmdMode8K4320p5994 = 0x386b3539,
    bmdMode8K4320p60 = 0x386b3630,
    bmdMode8kDCI2398 = 0x38643233,
    bmdMode8kDCI24 = 0x38643234,
    bmdMode8kDCI25 = 0x38643235,
    bmdMode8kDCI2997 = 0x38643239,
    bmdMode8kDCI30 = 0x38643330,
    bmdMode8kDCI4795 = 0x38643437,
    bmdMode8kDCI48 = 0x38643438,
    bmdMode8kDCI50 = 0x38643530,
    bmdMode8kDCI5994 = 0x38643539,
    bmdMode8kDCI60 = 0x38643630,
    bmdMode640x480p60 = 0x76676136,
    bmdMode800x600p60 = 0x73766736,
    bmdMode1440x900p50 = 0x77786735,
    bmdMode1440x900p60 = 0x77786736,
    bmdMode1440x1080p50 = 0x73786735,
    bmdMode1440x1080p60 = 0x73786736,
    bmdMode1600x1200p50 = 0x75786735,
    bmdMode1600x1200p60 = 0x75786736,
    bmdMode1920x1200p50 = 0x77757835,
    bmdMode1920x1200p60 = 0x77757836,
    bmdMode1920x1440p50 = 0x31393435,
    bmdMode1920x1440p60 = 0x31393436,
    bmdMode2560x1440p50 = 0x77716835,
    bmdMode2560x1440p60 = 0x77716836,
    bmdMode2560x1600p50 = 0x77717835,
    bmdMode2560x1600p60 = 0x77717836,
    bmdModeUnknown = 0x69756e6b,
}}

ENUM! {enum BMDFieldDominance {
    bmdUnknownFieldDominance = 0,
    bmdLowerFieldFirst = 0x6c6f7772,
    bmdUpperFieldFirst = 0x75707072,
    bmdProgressiveFrame = 0x70726f67,
    bmdProgressiveSegmentedFrame = 0x70736620,
}}

ENUM! {enum BMDPixelFormat {
    bmdFormatUnspecified = 0,
    bmdFormat8BitYUV = 0x32767579,
    bmdFormat10BitYUV = 0x76323130,
    bmdFormat8BitARGB = 32,
    bmdFormat8BitBGRA = 0x42475241,
    bmdFormat10BitRGB = 0x72323130,
    bmdFormat12BitRGB = 0x52313242,
    bmdFormat12BitRGBLE = 0x5231324c,
    bmdFormat10BitRGBXLE = 0x5231306c,
    bmdFormat10BitRGBX = 0x52313062,
    bmdFormatH265 = 0x68657631,
    bmdFormatDNxHR = 0x41566468,
}}

ENUM! {enum BMDDisplayModeFlags {
    bmdDisplayModeSupports3D = (1 << 0),
    bmdDisplayModeColorspaceRec601 = (1 << 1),
    bmdDisplayModeColorspaceRec709 = (1 << 2),
    bmdDisplayModeColorspaceRec2020 = (1 << 3),
}}

ENUM! {enum BMDDeckLinkConfigurationID {
    bmdDeckLinkConfigSwapSerialRxTx = 0x73737274,
    bmdDeckLinkConfigHDMI3DPackingFormat = 0x33647066,
    bmdDeckLinkConfigBypass = 0x62797073,
    bmdDeckLinkConfigClockTimingAdjustment = 0x63746164,
    bmdDeckLinkConfigAnalogAudioConsumerLevels = 0x6161636c,
    bmdDeckLinkConfigSwapHDMICh3AndCh4OnInput = 0x68693334,
    bmdDeckLinkConfigSwapHDMICh3AndCh4OnOutput = 0x686f3334,
    bmdDeckLinkConfigFieldFlickerRemoval = 0x66646672,
    bmdDeckLinkConfigHD1080p24ToHD1080i5994Conversion = 0x746f3539,
    bmdDeckLinkConfig444SDIVideoOutput = 0x3434346f,
    bmdDeckLinkConfigBlackVideoOutputDuringCapture = 0x62766f63,
    bmdDeckLinkConfigLowLatencyVideoOutput = 0x6c6c766f,
    bmdDeckLinkConfigDownConversionOnAllAnalogOutput = 0x6361616f,
    bmdDeckLinkConfigSMPTELevelAOutput = 0x736d7461,
    bmdDeckLinkConfigRec2020Output = 0x72656332,
    bmdDeckLinkConfigQuadLinkSDIVideoOutputSquareDivisionSplit = 0x53445153,
    bmdDeckLinkConfigOutput1080pAsPsF = 0x70667072,
    bmdDeckLinkConfigVideoOutputConnection = 0x766f636e,
    bmdDeckLinkConfigVideoOutputConversionMode = 0x766f636d,
    bmdDeckLinkConfigAnalogVideoOutputFlags = 0x61766f66,
    bmdDeckLinkConfigReferenceInputTimingOffset = 0x676c6f74,
    bmdDeckLinkConfigVideoOutputIdleOperation = 0x766f696f,
    bmdDeckLinkConfigDefaultVideoOutputMode = 0x64766f6d,
    bmdDeckLinkConfigDefaultVideoOutputModeFlags = 0x64766f66,
    bmdDeckLinkConfigSDIOutputLinkConfiguration = 0x736f6c63,
    bmdDeckLinkConfigHDMITimecodePacking = 0x6874706b,
    bmdDeckLinkConfigPlaybackGroup = 0x706c6772,
    bmdDeckLinkConfigVideoOutputComponentLumaGain = 0x6f636c67,
    bmdDeckLinkConfigVideoOutputComponentChromaBlueGain = 0x6f636362,
    bmdDeckLinkConfigVideoOutputComponentChromaRedGain = 0x6f636372,
    bmdDeckLinkConfigVideoOutputCompositeLumaGain = 0x6f696c67,
    bmdDeckLinkConfigVideoOutputCompositeChromaGain = 0x6f696367,
    bmdDeckLinkConfigVideoOutputSVideoLumaGain = 0x6f736c67,
    bmdDeckLinkConfigVideoOutputSVideoChromaGain = 0x6f736367,
    bmdDeckLinkConfigVideoInputScanning = 0x76697363,
    bmdDeckLinkConfigUseDedicatedLTCInput = 0x646c7463,
    bmdDeckLinkConfigSDIInput3DPayloadOverride = 0x33646473,
    bmdDeckLinkConfigCapture1080pAsPsF = 0x63667072,
    bmdDeckLinkConfigVideoInputConnection = 0x7669636e,
    bmdDeckLinkConfigAnalogVideoInputFlags = 0x61766966,
    bmdDeckLinkConfigVideoInputConversionMode = 0x7669636d,
    bmdDeckLinkConfig32PulldownSequenceInitialTimecodeFrame = 0x70646966,
    bmdDeckLinkConfigVANCSourceLine1Mapping = 0x76736c31,
    bmdDeckLinkConfigVANCSourceLine2Mapping = 0x76736c32,
    bmdDeckLinkConfigVANCSourceLine3Mapping = 0x76736c33,
    bmdDeckLinkConfigCapturePassThroughMode = 0x6370746d,
    bmdDeckLinkConfigCaptureGroup = 0x63706772,
    bmdDeckLinkConfigVideoInputComponentLumaGain = 0x69636c67,
    bmdDeckLinkConfigVideoInputComponentChromaBlueGain = 0x69636362,
    bmdDeckLinkConfigVideoInputComponentChromaRedGain = 0x69636372,
    bmdDeckLinkConfigVideoInputCompositeLumaGain = 0x69696c67,
    bmdDeckLinkConfigVideoInputCompositeChromaGain = 0x69696367,
    bmdDeckLinkConfigVideoInputSVideoLumaGain = 0x69736c67,
    bmdDeckLinkConfigVideoInputSVideoChromaGain = 0x69736367,
    bmdDeckLinkConfigInternalKeyingAncillaryDataSource = 0x696b6173,
    bmdDeckLinkConfigMicrophonePhantomPower = 0x6d706870,
    bmdDeckLinkConfigAudioInputConnection = 0x6169636e,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel1 = 0x61697331,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel2 = 0x61697332,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel3 = 0x61697333,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel4 = 0x61697334,
    bmdDeckLinkConfigDigitalAudioInputScale = 0x64616973,
    bmdDeckLinkConfigMicrophoneInputGain = 0x6d696367,
    bmdDeckLinkConfigAudioOutputAESAnalogSwitch = 0x616f6161,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel1 = 0x616f7331,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel2 = 0x616f7332,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel3 = 0x616f7333,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel4 = 0x616f7334,
    bmdDeckLinkConfigDigitalAudioOutputScale = 0x64616f73,
    bmdDeckLinkConfigHeadphoneVolume = 0x68766f6c,
    bmdDeckLinkConfigDeviceInformationLabel = 0x64696c61,
    bmdDeckLinkConfigDeviceInformationSerialNumber = 0x6469736e,
    bmdDeckLinkConfigDeviceInformationCompany = 0x6469636f,
    bmdDeckLinkConfigDeviceInformationPhone = 0x64697068,
    bmdDeckLinkConfigDeviceInformationEmail = 0x6469656d,
    bmdDeckLinkConfigDeviceInformationDate = 0x64696461,
    bmdDeckLinkConfigDeckControlConnection = 0x6463636f,
}}

ENUM! {enum BMDDeckLinkEncoderConfigurationID {
    bmdDeckLinkEncoderConfigPreferredBitDepth = 0x65706272,
    bmdDeckLinkEncoderConfigFrameCodingMode = 0x6566636d,
    bmdDeckLinkEncoderConfigH265TargetBitrate = 0x68746272,
    bmdDeckLinkEncoderConfigDNxHRCompressionID = 0x64636964,
    bmdDeckLinkEncoderConfigDNxHRLevel = 0x646c6576,
    bmdDeckLinkEncoderConfigMPEG4SampleDescription = 0x73747345,
    bmdDeckLinkEncoderConfigMPEG4CodecSpecificDesc = 0x65736473,
}}

ENUM! {enum BMDDeckControlMode {
    bmdDeckControlNotOpened = 0x6e746f70,
    bmdDeckControlVTRControlMode = 0x76747263,
    bmdDeckControlExportMode = 0x6578706d,
    bmdDeckControlCaptureMode = 0x6361706d,
}}

ENUM! {enum BMDDeckControlEvent {
    bmdDeckControlAbortedEvent = 0x61627465,
    bmdDeckControlPrepareForExportEvent = 0x70666565,
    bmdDeckControlExportCompleteEvent = 0x65786365,
    bmdDeckControlPrepareForCaptureEvent = 0x70666365,
    bmdDeckControlCaptureCompleteEvent = 0x63636576,
}}

ENUM! {enum BMDDeckControlVTRControlState {
    bmdDeckControlNotInVTRControlMode = 0x6e76636d,
    bmdDeckControlVTRControlPlaying = 0x76747270,
    bmdDeckControlVTRControlRecording = 0x76747272,
    bmdDeckControlVTRControlStill = 0x76747261,
    bmdDeckControlVTRControlShuttleForward = 0x76747366,
    bmdDeckControlVTRControlShuttleReverse = 0x76747372,
    bmdDeckControlVTRControlJogForward = 0x76746a66,
    bmdDeckControlVTRControlJogReverse = 0x76746a72,
    bmdDeckControlVTRControlStopped = 0x7674726f,
}}

ENUM! {enum BMDDeckControlStatusFlags {
    bmdDeckControlStatusDeckConnected = (1 << 0),
    bmdDeckControlStatusRemoteMode = (1 << 1),
    bmdDeckControlStatusRecordInhibited = (1 << 2),
    bmdDeckControlStatusCassetteOut = (1 << 3),
}}

ENUM! {enum BMDDeckControlExportModeOpsFlags {
    bmdDeckControlExportModeInsertVideo = (1 << 0),
    bmdDeckControlExportModeInsertAudio1 = (1 << 1),
    bmdDeckControlExportModeInsertAudio2 = (1 << 2),
    bmdDeckControlExportModeInsertAudio3 = (1 << 3),
    bmdDeckControlExportModeInsertAudio4 = (1 << 4),
    bmdDeckControlExportModeInsertAudio5 = (1 << 5),
    bmdDeckControlExportModeInsertAudio6 = (1 << 6),
    bmdDeckControlExportModeInsertAudio7 = (1 << 7),
    bmdDeckControlExportModeInsertAudio8 = (1 << 8),
    bmdDeckControlExportModeInsertAudio9 = (1 << 9),
    bmdDeckControlExportModeInsertAudio10 = (1 << 10),
    bmdDeckControlExportModeInsertAudio11 = (1 << 11),
    bmdDeckControlExportModeInsertAudio12 = (1 << 12),
    bmdDeckControlExportModeInsertTimeCode = (1 << 13),
    bmdDeckControlExportModeInsertAssemble = (1 << 14),
    bmdDeckControlExportModeInsertPreview = (1 << 15),
    bmdDeckControlUseManualExport = (1 << 16),
}}

ENUM! {enum BMDDeckControlError {
    bmdDeckControlNoError = 0x6e6f6572,
    bmdDeckControlModeError = 0x6d6f6572,
    bmdDeckControlMissedInPointError = 0x6d696572,
    bmdDeckControlDeckTimeoutError = 0x64746572,
    bmdDeckControlCommandFailedError = 0x63666572,
    bmdDeckControlDeviceAlreadyOpenedError = 0x64616c6f,
    bmdDeckControlFailedToOpenDeviceError = 0x66646572,
    bmdDeckControlInLocalModeError = 0x6c6d6572,
    bmdDeckControlEndOfTapeError = 0x65746572,
    bmdDeckControlUserAbortError = 0x75616572,
    bmdDeckControlNoTapeInDeckError = 0x6e746572,
    bmdDeckControlNoVideoFromCardError = 0x6e766663,
    bmdDeckControlNoCommunicationError = 0x6e636f6d,
    bmdDeckControlBufferTooSmallError = 0x6274736d,
    bmdDeckControlBadChecksumError = 0x63686b73,
    bmdDeckControlUnknownError = 0x756e6572,
}}

ENUM! {enum BMDStreamingDeviceMode {
    bmdStreamingDeviceIdle = 0x69646c65,
    bmdStreamingDeviceEncoding = 0x656e636f,
    bmdStreamingDeviceStopping = 0x73746f70,
    bmdStreamingDeviceUnknown = 0x6d756e6b,
}}

ENUM! {enum BMDStreamingEncodingFramerate {
    bmdStreamingEncodedFrameRate50i = 0x65353069,
    bmdStreamingEncodedFrameRate5994i = 0x65353969,
    bmdStreamingEncodedFrameRate60i = 0x65363069,
    bmdStreamingEncodedFrameRate2398p = 0x65323370,
    bmdStreamingEncodedFrameRate24p = 0x65323470,
    bmdStreamingEncodedFrameRate25p = 0x65323570,
    bmdStreamingEncodedFrameRate2997p = 0x65323970,
    bmdStreamingEncodedFrameRate30p = 0x65333070,
    bmdStreamingEncodedFrameRate50p = 0x65353070,
    bmdStreamingEncodedFrameRate5994p = 0x65353970,
    bmdStreamingEncodedFrameRate60p = 0x65363070,
}}

ENUM! {enum BMDStreamingEncodingSupport {
    bmdStreamingEncodingModeNotSupported = 0,
    bmdStreamingEncodingModeSupported =
        (bmdStreamingEncodingModeNotSupported + 1),
    bmdStreamingEncodingModeSupportedWithChanges =
        (bmdStreamingEncodingModeSupported + 1),
}}

ENUM! {enum BMDStreamingVideoCodec {
    bmdStreamingVideoCodecH264 = 0x48323634,
}}

ENUM! {enum BMDStreamingH264Profile {
    bmdStreamingH264ProfileHigh = 0x68696768,
    bmdStreamingH264ProfileMain = 0x6d61696e,
    bmdStreamingH264ProfileBaseline = 0x62617365,
}}

ENUM! {enum BMDStreamingH264Level {
    bmdStreamingH264Level12 = 0x6c763132,
    bmdStreamingH264Level13 = 0x6c763133,
    bmdStreamingH264Level2 = 0x6c763220,
    bmdStreamingH264Level21 = 0x6c763231,
    bmdStreamingH264Level22 = 0x6c763232,
    bmdStreamingH264Level3 = 0x6c763320,
    bmdStreamingH264Level31 = 0x6c763331,
    bmdStreamingH264Level32 = 0x6c763332,
    bmdStreamingH264Level4 = 0x6c763420,
    bmdStreamingH264Level41 = 0x6c763431,
    bmdStreamingH264Level42 = 0x6c763432,
}}

ENUM! {enum BMDStreamingH264EntropyCoding {
    bmdStreamingH264EntropyCodingCAVLC = 0x45564c43,
    bmdStreamingH264EntropyCodingCABAC = 0x45424143,
}}

ENUM! {enum BMDStreamingAudioCodec {
    bmdStreamingAudioCodecAAC = 0x41414320,
}}

ENUM! {enum BMDStreamingEncodingModePropertyID {
    bmdStreamingEncodingPropertyVideoFrameRate = 0x76667274,
    bmdStreamingEncodingPropertyVideoBitRateKbps = 0x76627274,
    bmdStreamingEncodingPropertyH264Profile = 0x68707266,
    bmdStreamingEncodingPropertyH264Level = 0x686c766c,
    bmdStreamingEncodingPropertyH264EntropyCoding = 0x68656e74,
    bmdStreamingEncodingPropertyH264HasBFrames = 0x68426672,
    bmdStreamingEncodingPropertyAudioCodec = 0x61636463,
    bmdStreamingEncodingPropertyAudioSampleRate = 0x61737274,
    bmdStreamingEncodingPropertyAudioChannelCount = 0x61636863,
    bmdStreamingEncodingPropertyAudioBitRateKbps = 0x61627274,
}}

ENUM! {enum BMDVideoOutputFlags {
    bmdVideoOutputFlagDefault = 0,
    bmdVideoOutputVANC = (1 << 0),
    bmdVideoOutputVITC = (1 << 1),
    bmdVideoOutputRP188 = (1 << 2),
    bmdVideoOutputDualStream3D = (1 << 4),
    bmdVideoOutputSynchronizeToPlaybackGroup = (1 << 6),
}}

ENUM! {enum BMDSupportedVideoModeFlags {
    bmdSupportedVideoModeDefault = 0,
    bmdSupportedVideoModeKeying = (1 << 0),
    bmdSupportedVideoModeDualStream3D = (1 << 1),
    bmdSupportedVideoModeSDISingleLink = (1 << 2),
    bmdSupportedVideoModeSDIDualLink = (1 << 3),
    bmdSupportedVideoModeSDIQuadLink = (1 << 4),
    bmdSupportedVideoModeInAnyProfile = (1 << 5),
}}

ENUM! {enum BMDPacketType {
    bmdPacketTypeStreamInterruptedMarker = 0x73696e74,
    bmdPacketTypeStreamData = 0x73646174,
}}

ENUM! {enum BMDFrameFlags {
    bmdFrameFlagDefault = 0,
    bmdFrameFlagFlipVertical = (1 << 0),
    bmdFrameContainsHDRMetadata = (1 << 1),
    bmdFrameCapturedAsPsF = (1 << 30),
    bmdFrameHasNoInputSource = (1 << 31),
}}

ENUM! {enum BMDVideoInputFlags {
    bmdVideoInputFlagDefault = 0,
    bmdVideoInputEnableFormatDetection = (1 << 0),
    bmdVideoInputDualStream3D = (1 << 1),
    bmdVideoInputSynchronizeToCaptureGroup = (1 << 2),
}}

ENUM! {enum BMDVideoInputFormatChangedEvents {
    bmdVideoInputDisplayModeChanged = (1 << 0),
    bmdVideoInputFieldDominanceChanged = (1 << 1),
    bmdVideoInputColorspaceChanged = (1 << 2),
}}

ENUM! {enum BMDDetectedVideoInputFormatFlags {
    bmdDetectedVideoInputYCbCr422 = (1 << 0),
    bmdDetectedVideoInputRGB444 = (1 << 1),
    bmdDetectedVideoInputDualStream3D = (1 << 2),
    bmdDetectedVideoInput12BitDepth = (1 << 3),
    bmdDetectedVideoInput10BitDepth = (1 << 4),
    bmdDetectedVideoInput8BitDepth = (1 << 5),
}}

ENUM! {enum BMDDeckLinkCapturePassthroughMode {
    bmdDeckLinkCapturePassthroughModeDisabled = 0x70646973,
    bmdDeckLinkCapturePassthroughModeDirect = 0x70646972,
    bmdDeckLinkCapturePassthroughModeCleanSwitch = 0x70636c6e,
}}

ENUM! {enum BMDOutputFrameCompletionResult {
    bmdOutputFrameCompleted = 0,
    bmdOutputFrameDisplayedLate = (bmdOutputFrameCompleted + 1),
    bmdOutputFrameDropped = (bmdOutputFrameDisplayedLate + 1),
    bmdOutputFrameFlushed = (bmdOutputFrameDropped + 1),
}}

ENUM! {enum BMDReferenceStatus {
    bmdReferenceUnlocked = 0,
    bmdReferenceNotSupportedByHardware = (1 << 0),
    bmdReferenceLocked = (1 << 1),
}}

ENUM! {enum BMDAudioFormat {
    bmdAudioFormatPCM = 0x6c70636d,
}}

ENUM! {enum BMDAudioSampleRate {
    bmdAudioSampleRate48kHz = 48000,
}}

ENUM! {enum BMDAudioSampleType {
    bmdAudioSampleType16bitInteger = 16,
    bmdAudioSampleType32bitInteger = 32,
}}

ENUM! {enum BMDAudioOutputStreamType {
    bmdAudioOutputStreamContinuous = 0,
    bmdAudioOutputStreamContinuousDontResample =
        (bmdAudioOutputStreamContinuous + 1),
    bmdAudioOutputStreamTimestamped =
        (bmdAudioOutputStreamContinuousDontResample + 1),
}}

ENUM! {enum BMDAncillaryPacketFormat {
    bmdAncillaryPacketFormatUInt8 = 0x75693038,
    bmdAncillaryPacketFormatUInt16 = 0x75693136,
    bmdAncillaryPacketFormatYCbCr10 = 0x76323130,
}}

ENUM! {enum BMDTimecodeFormat {
    bmdTimecodeRP188VITC1 = 0x72707631,
    bmdTimecodeRP188VITC2 = 0x72703132,
    bmdTimecodeRP188LTC = 0x72706c74,
    bmdTimecodeRP188HighFrameRate = 0x72706872,
    bmdTimecodeRP188Any = 0x72703138,
    bmdTimecodeVITC = 0x76697463,
    bmdTimecodeVITCField2 = 0x76697432,
    bmdTimecodeSerial = 0x73657269,
}}

ENUM! {enum BMDAnalogVideoFlags {
    bmdAnalogVideoFlagCompositeSetup75 = (1 << 0),
    bmdAnalogVideoFlagComponentBetacamLevels = (1 << 1),
}}

ENUM! {enum BMDAudioOutputAnalogAESSwitch {
    bmdAudioOutputSwitchAESEBU = 0x61657320,
    bmdAudioOutputSwitchAnalog = 0x616e6c67,
}}

ENUM! {enum BMDVideoOutputConversionMode {
    bmdNoVideoOutputConversion = 0x6e6f6e65,
    bmdVideoOutputLetterboxDownconversion = 0x6c746278,
    bmdVideoOutputAnamorphicDownconversion = 0x616d7068,
    bmdVideoOutputHD720toHD1080Conversion = 0x37323063,
    bmdVideoOutputHardwareLetterboxDownconversion = 0x48576c62,
    bmdVideoOutputHardwareAnamorphicDownconversion = 0x4857616d,
    bmdVideoOutputHardwareCenterCutDownconversion = 0x48576363,
    bmdVideoOutputHardware720p1080pCrossconversion = 0x78636170,
    bmdVideoOutputHardwareAnamorphic720pUpconversion = 0x75613770,
    bmdVideoOutputHardwareAnamorphic1080iUpconversion = 0x75613169,
    bmdVideoOutputHardwareAnamorphic149To720pUpconversion = 0x75343770,
    bmdVideoOutputHardwareAnamorphic149To1080iUpconversion = 0x75343169,
    bmdVideoOutputHardwarePillarbox720pUpconversion = 0x75703770,
    bmdVideoOutputHardwarePillarbox1080iUpconversion = 0x75703169,
}}

ENUM! {enum BMDVideoInputConversionMode {
    bmdNoVideoInputConversion = 0x6e6f6e65,
    bmdVideoInputLetterboxDownconversionFromHD1080 = 0x31306c62,
    bmdVideoInputAnamorphicDownconversionFromHD1080 = 0x3130616d,
    bmdVideoInputLetterboxDownconversionFromHD720 = 0x37326c62,
    bmdVideoInputAnamorphicDownconversionFromHD720 = 0x3732616d,
    bmdVideoInputLetterboxUpconversion = 0x6c627570,
    bmdVideoInputAnamorphicUpconversion = 0x616d7570,
}}

ENUM! {enum BMDVideo3DPackingFormat {
    bmdVideo3DPackingSidebySideHalf = 0x73627368,
    bmdVideo3DPackingLinebyLine = 0x6c62796c,
    bmdVideo3DPackingTopAndBottom = 0x7461626f,
    bmdVideo3DPackingFramePacking = 0x6672706b,
    bmdVideo3DPackingLeftOnly = 0x6c656674,
    bmdVideo3DPackingRightOnly = 0x72696768,
}}

ENUM! {enum BMDIdleVideoOutputOperation {
    bmdIdleVideoOutputBlack = 0x626c6163,
    bmdIdleVideoOutputLastFrame = 0x6c616661,
}}

ENUM! {enum BMDVideoEncoderFrameCodingMode {
    bmdVideoEncoderFrameCodingModeInter = 0x696e7465,
    bmdVideoEncoderFrameCodingModeIntra = 0x696e7472,
}}

ENUM! {enum BMDDNxHRLevel {
    bmdDNxHRLevelSQ = 0x646e7371,
    bmdDNxHRLevelLB = 0x646e6c62,
    bmdDNxHRLevelHQ = 0x646e6871,
    bmdDNxHRLevelHQX = 0x64687178,
    bmdDNxHRLevel444 = 0x64343434,
}}

ENUM! {enum BMDLinkConfiguration {
    bmdLinkConfigurationSingleLink = 0x6c63736c,
    bmdLinkConfigurationDualLink = 0x6c63646c,
    bmdLinkConfigurationQuadLink = 0x6c63716c,
}}

ENUM! {enum BMDDeviceInterface {
    bmdDeviceInterfacePCI = 0x70636920,
    bmdDeviceInterfaceUSB = 0x75736220,
    bmdDeviceInterfaceThunderbolt = 0x7468756e,
}}

ENUM! {enum BMDColorspace {
    bmdColorspaceRec601 = 0x72363031,
    bmdColorspaceRec709 = 0x72373039,
    bmdColorspaceRec2020 = 0x32303230,
}}

ENUM! {enum BMDDynamicRange {
    bmdDynamicRangeSDR = 0,
    bmdDynamicRangeHDRStaticPQ = (1 << 29),
    bmdDynamicRangeHDRStaticHLG = (1 << 30),
}}

ENUM! {enum BMDDeckLinkHDMIInputEDIDID {
    bmdDeckLinkHDMIInputEDIDDynamicRange = 0x48494479,
}}

ENUM! {enum BMDDeckLinkFrameMetadataID {
    bmdDeckLinkFrameMetadataColorspace = 0x63737063,
    bmdDeckLinkFrameMetadataHDRElectroOpticalTransferFunc = 0x656f7466,
    bmdDeckLinkFrameMetadataHDRDisplayPrimariesRedX = 0x68647278,
    bmdDeckLinkFrameMetadataHDRDisplayPrimariesRedY = 0x68647279,
    bmdDeckLinkFrameMetadataHDRDisplayPrimariesGreenX = 0x68646778,
    bmdDeckLinkFrameMetadataHDRDisplayPrimariesGreenY = 0x68646779,
    bmdDeckLinkFrameMetadataHDRDisplayPrimariesBlueX = 0x68646278,
    bmdDeckLinkFrameMetadataHDRDisplayPrimariesBlueY = 0x68646279,
    bmdDeckLinkFrameMetadataHDRWhitePointX = 0x68647778,
    bmdDeckLinkFrameMetadataHDRWhitePointY = 0x68647779,
    bmdDeckLinkFrameMetadataHDRMaxDisplayMasteringLuminance = 0x68646d6c,
    bmdDeckLinkFrameMetadataHDRMinDisplayMasteringLuminance = 0x686d696c,
    bmdDeckLinkFrameMetadataHDRMaximumContentLightLevel = 0x6d636c6c,
    bmdDeckLinkFrameMetadataHDRMaximumFrameAverageLightLevel = 0x66616c6c,
}}

ENUM! {enum BMDProfileID {
    bmdProfileOneSubDeviceFullDuplex = 0x31646664,
    bmdProfileOneSubDeviceHalfDuplex = 0x31646864,
    bmdProfileTwoSubDevicesFullDuplex = 0x32646664,
    bmdProfileTwoSubDevicesHalfDuplex = 0x32646864,
    bmdProfileFourSubDevicesHalfDuplex = 0x34646864,
}}

ENUM! {enum BMDHDMITimecodePacking {
    bmdHDMITimecodePackingIEEEOUI000085 = 0x8500,
    bmdHDMITimecodePackingIEEEOUI080046 = 0x8004601,
    bmdHDMITimecodePackingIEEEOUI5CF9F0 = 0x5cf9f003,
}}

ENUM! {enum BMDInternalKeyingAncillaryDataSource {
    bmdInternalKeyingUsesAncillaryDataFromInputSignal = 0x696b6169,
    bmdInternalKeyingUsesAncillaryDataFromKeyFrame = 0x696b616b,
}}

ENUM! {enum BMDDeckLinkAttributeID {
    BMDDeckLinkSupportsInternalKeying = 0x6b657969,
    BMDDeckLinkSupportsExternalKeying = 0x6b657965,
    BMDDeckLinkSupportsInputFormatDetection = 0x696e6664,
    BMDDeckLinkHasReferenceInput = 0x6872696e,
    BMDDeckLinkHasSerialPort = 0x68737074,
    BMDDeckLinkHasAnalogVideoOutputGain = 0x61766f67,
    BMDDeckLinkCanOnlyAdjustOverallVideoOutputGain = 0x6f766f67,
    BMDDeckLinkHasVideoInputAntiAliasingFilter = 0x6161666c,
    BMDDeckLinkHasBypass = 0x62797073,
    BMDDeckLinkSupportsClockTimingAdjustment = 0x63746164,
    BMDDeckLinkSupportsFullFrameReferenceInputTimingOffset = 0x6672696e,
    BMDDeckLinkSupportsSMPTELevelAOutput = 0x6c766c61,
    BMDDeckLinkSupportsAutoSwitchingPPsFOnInput = 0x61707366,
    BMDDeckLinkSupportsDualLinkSDI = 0x73646c73,
    BMDDeckLinkSupportsQuadLinkSDI = 0x73716c73,
    BMDDeckLinkSupportsIdleOutput = 0x69646f75,
    BMDDeckLinkVANCRequires10BitYUVVideoFrames = 0x76696f59,
    BMDDeckLinkHasLTCTimecodeInput = 0x686c7463,
    BMDDeckLinkSupportsHDRMetadata = 0x6864726d,
    BMDDeckLinkSupportsColorspaceMetadata = 0x636d6574,
    BMDDeckLinkSupportsHDMITimecode = 0x6874696d,
    BMDDeckLinkSupportsHighFrameRateTimecode = 0x48465254,
    BMDDeckLinkSupportsSynchronizeToCaptureGroup = 0x73746367,
    BMDDeckLinkSupportsSynchronizeToPlaybackGroup = 0x73747067,
    BMDDeckLinkMaximumAudioChannels = 0x6d616368,
    BMDDeckLinkMaximumAnalogAudioInputChannels = 0x69616368,
    BMDDeckLinkMaximumAnalogAudioOutputChannels = 0x61616368,
    BMDDeckLinkNumberOfSubDevices = 0x6e736264,
    BMDDeckLinkSubDeviceIndex = 0x73756269,
    BMDDeckLinkPersistentID = 0x70656964,
    BMDDeckLinkDeviceGroupID = 0x64676964,
    BMDDeckLinkTopologicalID = 0x746f6964,
    BMDDeckLinkVideoOutputConnections = 0x766f636e,
    BMDDeckLinkVideoInputConnections = 0x7669636e,
    BMDDeckLinkAudioOutputConnections = 0x616f636e,
    BMDDeckLinkAudioInputConnections = 0x6169636e,
    BMDDeckLinkVideoIOSupport = 0x76696f73,
    BMDDeckLinkDeckControlConnections = 0x6463636e,
    BMDDeckLinkDeviceInterface = 0x64627573,
    BMDDeckLinkAudioInputRCAChannelCount = 0x61697263,
    BMDDeckLinkAudioInputXLRChannelCount = 0x61697863,
    BMDDeckLinkAudioOutputRCAChannelCount = 0x616f7263,
    BMDDeckLinkAudioOutputXLRChannelCount = 0x616f7863,
    BMDDeckLinkProfileID = 0x70726964,
    BMDDeckLinkDuplex = 0x64757078,
    BMDDeckLinkMinimumPrerollFrames = 0x6d707266,
    BMDDeckLinkSupportedDynamicRange = 0x73756472,
    BMDDeckLinkVideoInputGainMinimum = 0x7669676d,
    BMDDeckLinkVideoInputGainMaximum = 0x76696778,
    BMDDeckLinkVideoOutputGainMinimum = 0x766f676d,
    BMDDeckLinkVideoOutputGainMaximum = 0x766f6778,
    BMDDeckLinkMicrophoneInputGainMinimum = 0x6d69676d,
    BMDDeckLinkMicrophoneInputGainMaximum = 0x6d696778,
    BMDDeckLinkSerialPortDeviceName = 0x736c706e,
    BMDDeckLinkVendorName = 0x766e6472,
    BMDDeckLinkDisplayName = 0x6473706e,
    BMDDeckLinkModelName = 0x6d646c6e,
    BMDDeckLinkDeviceHandle = 0x64657668,
}}

ENUM! {enum BMDDeckLinkAPIInformationID {
    BMDDeckLinkAPIVersion = 0x76657273,
}}

ENUM! {enum BMDDeckLinkStatusID {
    bmdDeckLinkStatusDetectedVideoInputMode = 0x6476696d,
    bmdDeckLinkStatusDetectedVideoInputFormatFlags = 0x64766666,
    bmdDeckLinkStatusDetectedVideoInputFieldDominance = 0x64766664,
    bmdDeckLinkStatusDetectedVideoInputColorspace = 0x6473636c,
    bmdDeckLinkStatusDetectedVideoInputDynamicRange = 0x64736472,
    bmdDeckLinkStatusDetectedSDILinkConfiguration = 0x64736c63,
    bmdDeckLinkStatusCurrentVideoInputMode = 0x6376696d,
    bmdDeckLinkStatusCurrentVideoInputPixelFormat = 0x63766970,
    bmdDeckLinkStatusCurrentVideoInputFlags = 0x63766966,
    bmdDeckLinkStatusCurrentVideoOutputMode = 0x63766f6d,
    bmdDeckLinkStatusCurrentVideoOutputFlags = 0x63766f66,
    bmdDeckLinkStatusPCIExpressLinkWidth = 0x70776964,
    bmdDeckLinkStatusPCIExpressLinkSpeed = 0x706c6e6b,
    bmdDeckLinkStatusLastVideoOutputPixelFormat = 0x6f706978,
    bmdDeckLinkStatusReferenceSignalMode = 0x7265666d,
    bmdDeckLinkStatusReferenceSignalFlags = 0x72656666,
    bmdDeckLinkStatusBusy = 0x62757379,
    bmdDeckLinkStatusInterchangeablePanelType = 0x69637074,
    bmdDeckLinkStatusDeviceTemperature = 0x64746d70,
    bmdDeckLinkStatusVideoInputSignalLocked = 0x7669736c,
    bmdDeckLinkStatusReferenceSignalLocked = 0x7265666c,
    bmdDeckLinkStatusReceivedEDID = 0x65646964,
}}

ENUM! {enum BMDDeckLinkVideoStatusFlags {
    bmdDeckLinkVideoStatusPsF = (1 << 0),
    bmdDeckLinkVideoStatusDualStream3D = (1 << 1),
}}

ENUM! {enum BMDDuplexMode {
    bmdDuplexFull = 0x64786675,
    bmdDuplexHalf = 0x64786861,
    bmdDuplexSimplex = 0x64787370,
    bmdDuplexInactive = 0x6478696e,
}}

ENUM! {enum BMDPanelType {
    bmdPanelNotDetected = 0x6e706e6c,
    bmdPanelTeranexMiniSmartPanel = 0x746d736d,
}}

ENUM! {enum BMDDeviceBusyState {
    bmdDeviceCaptureBusy = (1 << 0),
    bmdDevicePlaybackBusy = (1 << 1),
    bmdDeviceSerialPortBusy = (1 << 2),
}}

ENUM! {enum BMDVideoIOSupport {
    bmdDeviceSupportsCapture = (1 << 0),
    bmdDeviceSupportsPlayback = (1 << 1),
}}

ENUM! {enum BMD3DPreviewFormat {
    bmd3DPreviewFormatDefault = 0x64656661,
    bmd3DPreviewFormatLeftOnly = 0x6c656674,
    bmd3DPreviewFormatRightOnly = 0x72696768,
    bmd3DPreviewFormatSideBySide = 0x73696465,
    bmd3DPreviewFormatTopBottom = 0x746f7062,
}}

ENUM! {enum BMDNotifications {
    bmdPreferencesChanged = 0x70726566,
    bmdStatusChanged = 0x73746174,
}}

ENUM! {enum BMDDeckLinkStatusID_v11_5_1 {
    bmdDeckLinkStatusDetectedVideoInputFlags_v11_5_1 = 0x64766966,
}}

ENUM! {enum BMDDisplayModeSupport_v10_11 {
    bmdDisplayModeNotSupported_v10_11 = 0,
    bmdDisplayModeSupported_v10_11 = (bmdDisplayModeNotSupported_v10_11 + 1),
    bmdDisplayModeSupportedWithConversion_v10_11 =
        (bmdDisplayModeSupported_v10_11 + 1),
}}

ENUM! {enum BMDDuplexMode_v10_11 {
    bmdDuplexModeFull_v10_11 = 0x66647570,
    bmdDuplexModeHalf_v10_11 = 0x68647570,
}}

ENUM! {enum BMDDeckLinkConfigurationID_v10_11 {
    bmdDeckLinkConfigDuplexMode_v10_11 = 0x64757078,
}}

ENUM! {enum BMDDeckLinkAttributeID_v10_11 {
    BMDDeckLinkSupportsDuplexModeConfiguration_v10_11 = 0x64757078,
    BMDDeckLinkSupportsHDKeying_v10_11 = 0x6b657968,
    BMDDeckLinkPairedDevicePersistentID_v10_11 = 0x70706964,
    BMDDeckLinkSupportsFullDuplex_v10_11 = 0x66647570,
}}

ENUM! {enum BMDDeckLinkStatusID_v10_11 {
    bmdDeckLinkStatusDuplexMode_v10_11 = 0x64757078,
}}

ENUM! {enum BMDDuplexStatus_v10_11 {
    bmdDuplexFullDuplex_v10_11 = 0x66647570,
    bmdDuplexHalfDuplex_v10_11 = 0x68647570,
    bmdDuplexSimplex_v10_11 = 0x73706c78,
    bmdDuplexInactive_v10_11 = 0x696e6163,
}}

ENUM! {enum BMDDeckLinkConfigurationID_v10_9 {
    bmdDeckLinkConfig1080pNotPsF_v10_9 = 0x6670726f,
}}

ENUM! {enum BMDDeckLinkConfigurationID_v10_4 {
    bmdDeckLinkConfigSingleLinkVideoOutput_v10_4 = 0x73676c6f,
}}

ENUM! {enum BMDDeckLinkConfigurationID_v10_2 {
    bmdDeckLinkConfig3GBpsVideoOutput_v10_2 = 0x33676273,
}}

ENUM! {enum BMDAudioConnection_v10_2 {
    bmdAudioConnectionEmbedded_v10_2 = 0x656d6264,
    bmdAudioConnectionAESEBU_v10_2 = 0x61657320,
    bmdAudioConnectionAnalog_v10_2 = 0x616e6c67,
    bmdAudioConnectionAnalogXLR_v10_2 = 0x61786c72,
    bmdAudioConnectionAnalogRCA_v10_2 = 0x61726361,
}}

ENUM! {enum BMDDeckLinkFrameMetadataID_v11_5 {
    bmdDeckLinkFrameMetadataCintelFilmType_v11_5 = 0x63667479,
    bmdDeckLinkFrameMetadataCintelFilmGauge_v11_5 = 0x63666761,
    bmdDeckLinkFrameMetadataCintelKeykodeLow_v11_5 = 0x636b6b6c,
    bmdDeckLinkFrameMetadataCintelKeykodeHigh_v11_5 = 0x636b6b68,
    bmdDeckLinkFrameMetadataCintelTile1Size_v11_5 = 0x63743173,
    bmdDeckLinkFrameMetadataCintelTile2Size_v11_5 = 0x63743273,
    bmdDeckLinkFrameMetadataCintelTile3Size_v11_5 = 0x63743373,
    bmdDeckLinkFrameMetadataCintelTile4Size_v11_5 = 0x63743473,
    bmdDeckLinkFrameMetadataCintelImageWidth_v11_5 = 0x49575078,
    bmdDeckLinkFrameMetadataCintelImageHeight_v11_5 = 0x49485078,
    bmdDeckLinkFrameMetadataCintelLinearMaskingRedInRed_v11_5 = 0x6d726972,
    bmdDeckLinkFrameMetadataCintelLinearMaskingGreenInRed_v11_5 = 0x6d676972,
    bmdDeckLinkFrameMetadataCintelLinearMaskingBlueInRed_v11_5 = 0x6d626972,
    bmdDeckLinkFrameMetadataCintelLinearMaskingRedInGreen_v11_5 = 0x6d726967,
    bmdDeckLinkFrameMetadataCintelLinearMaskingGreenInGreen_v11_5 =
        0x6d676967,
    bmdDeckLinkFrameMetadataCintelLinearMaskingBlueInGreen_v11_5 = 0x6d626967,
    bmdDeckLinkFrameMetadataCintelLinearMaskingRedInBlue_v11_5 = 0x6d726962,
    bmdDeckLinkFrameMetadataCintelLinearMaskingGreenInBlue_v11_5 = 0x6d676962,
    bmdDeckLinkFrameMetadataCintelLinearMaskingBlueInBlue_v11_5 = 0x6d626962,
    bmdDeckLinkFrameMetadataCintelLogMaskingRedInRed_v11_5 = 0x6d6c7272,
    bmdDeckLinkFrameMetadataCintelLogMaskingGreenInRed_v11_5 = 0x6d6c6772,
    bmdDeckLinkFrameMetadataCintelLogMaskingBlueInRed_v11_5 = 0x6d6c6272,
    bmdDeckLinkFrameMetadataCintelLogMaskingRedInGreen_v11_5 = 0x6d6c7267,
    bmdDeckLinkFrameMetadataCintelLogMaskingGreenInGreen_v11_5 = 0x6d6c6767,
    bmdDeckLinkFrameMetadataCintelLogMaskingBlueInGreen_v11_5 = 0x6d6c6267,
    bmdDeckLinkFrameMetadataCintelLogMaskingRedInBlue_v11_5 = 0x6d6c7262,
    bmdDeckLinkFrameMetadataCintelLogMaskingGreenInBlue_v11_5 = 0x6d6c6762,
    bmdDeckLinkFrameMetadataCintelLogMaskingBlueInBlue_v11_5 = 0x6d6c6262,
    bmdDeckLinkFrameMetadataCintelFilmFrameRate_v11_5 = 0x63666672,
    bmdDeckLinkFrameMetadataCintelOffsetToApplyHorizontal_v11_5 = 0x6f746168,
    bmdDeckLinkFrameMetadataCintelOffsetToApplyVertical_v11_5 = 0x6f746176,
    bmdDeckLinkFrameMetadataCintelGainRed_v11_5 = 0x4c665264,
    bmdDeckLinkFrameMetadataCintelGainGreen_v11_5 = 0x4c664772,
    bmdDeckLinkFrameMetadataCintelGainBlue_v11_5 = 0x4c66426c,
    bmdDeckLinkFrameMetadataCintelLiftRed_v11_5 = 0x476e5264,
    bmdDeckLinkFrameMetadataCintelLiftGreen_v11_5 = 0x476e4772,
    bmdDeckLinkFrameMetadataCintelLiftBlue_v11_5 = 0x476e426c,
    bmdDeckLinkFrameMetadataCintelHDRGainRed_v11_5 = 0x48475264,
    bmdDeckLinkFrameMetadataCintelHDRGainGreen_v11_5 = 0x48474772,
    bmdDeckLinkFrameMetadataCintelHDRGainBlue_v11_5 = 0x4847426c,
    bmdDeckLinkFrameMetadataCintel16mmCropRequired_v11_5 = 0x63313663,
    bmdDeckLinkFrameMetadataCintelInversionRequired_v11_5 = 0x63696e76,
    bmdDeckLinkFrameMetadataCintelFlipRequired_v11_5 = 0x63666c72,
    bmdDeckLinkFrameMetadataCintelFocusAssistEnabled_v11_5 = 0x63666165,
    bmdDeckLinkFrameMetadataCintelKeykodeIsInterpolated_v11_5 = 0x6b6b6969,
}}

ENUM! {enum BMDDeckLinkAttributeID_v10_6 {
    BMDDeckLinkSupportsDesktopDisplay_v10_6 = 0x65787464,
}}

ENUM! {enum BMDIdleVideoOutputDuration_v10_6 {
    bmdIdleVideoOutputDesktop_v10_6 = 0x6465736b,
}}

ENUM! {enum BMDDeckLinkAttributeID_v10_5 {
    BMDDeckLinkDeviceBusyState_v10_5 = 0x64627374,
}}

ENUM! {enum BMDDeckControlVTRControlState_v8_1 {
    bmdDeckControlNotInVTRControlMode_v8_1 = 0x6e76636d,
    bmdDeckControlVTRControlPlaying_v8_1 = 0x76747270,
    bmdDeckControlVTRControlRecording_v8_1 = 0x76747272,
    bmdDeckControlVTRControlStill_v8_1 = 0x76747261,
    bmdDeckControlVTRControlSeeking_v8_1 = 0x76747273,
    bmdDeckControlVTRControlStopped_v8_1 = 0x7674726f,
}}

ENUM! {enum BMDVideoConnection_v7_6 {
    bmdVideoConnectionSDI_v7_6 = 0x73646920,
    bmdVideoConnectionHDMI_v7_6 = 0x68646d69,
    bmdVideoConnectionOpticalSDI_v7_6 = 0x6f707469,
    bmdVideoConnectionComponent_v7_6 = 0x63706e74,
    bmdVideoConnectionComposite_v7_6 = 0x636d7374,
    bmdVideoConnectionSVideo_v7_6 = 0x73766964,
}}

RIDL! {#[uuid(0xbc6cfbd3, 0x8317, 0x4325, 0xac, 0x1c, 0x12, 0x16, 0x39, 0x1e, 0x93, 0x40)]
interface IDeckLinkTimecode(IDeckLinkTimecodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetBCD() -> BMDTimecodeBCD,
    fn GetComponents(
        hours: *mut u8,
        minutes: *mut u8,
        seconds: *mut u8,
        frames: *mut u8,
    ) -> HRESULT,
    fn GetString(
        timecode: *mut BSTR,
    ) -> HRESULT,
    fn GetFlags() -> BMDTimecodeFlags,
    fn GetTimecodeUserBits(
        userBits: *mut BMDTimecodeUserBits,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x9c88499f, 0xf601, 0x4021, 0xb8, 0x0b, 0x03, 0x2e, 0x4e, 0xb4, 0x1c, 0x35)]
interface IDeckLinkDisplayModeIterator(IDeckLinkDisplayModeIteratorVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        deckLinkDisplayMode: *mut *mut IDeckLinkDisplayMode,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x3eb2c1ab, 0x0a3d, 0x4523, 0xa3, 0xad, 0xf4, 0x0d, 0x7f, 0xb1, 0x4e, 0x78)]
interface IDeckLinkDisplayMode(IDeckLinkDisplayModeVtbl): IUnknown(IUnknownVtbl) {
    fn GetName(
        name: *mut BSTR,
    ) -> HRESULT,
    fn GetDisplayMode() -> BMDDisplayMode,
    fn GetWidth() -> c_long,
    fn GetHeight() -> c_long,
    fn GetFrameRate(
        frameDuration: *mut BMDTimeValue,
        timeScale: *mut BMDTimeScale,
    ) -> HRESULT,
    fn GetFieldDominance() -> BMDFieldDominance,
    fn GetFlags() -> BMDDisplayModeFlags,
}}

RIDL! {#[uuid(0xc418fbdd, 0x0587, 0x48ed, 0x8f, 0xe5, 0x64, 0x0f, 0x0a, 0x14, 0xaf, 0x91)]
interface IDeckLink(IDeckLinkVtbl): IUnknown(IUnknownVtbl) {
    fn GetModelName(
        modelName: *mut BSTR,
    ) -> HRESULT,
    fn GetDisplayName(
        displayName: *mut BSTR,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x912f634b, 0x2d4e, 0x40a4, 0x8a, 0xab, 0x8d, 0x80, 0xb7, 0x3f, 0x12, 0x89)]
interface IDeckLinkConfiguration(IDeckLinkConfigurationVtbl): IUnknown(IUnknownVtbl) {
    fn SetFlag(
        cfgID: BMDDeckLinkConfigurationID,
        value: BOOL,
    ) -> HRESULT,
    fn GetFlag(
        cfgID: BMDDeckLinkConfigurationID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn SetInt(
        cfgID: BMDDeckLinkConfigurationID,
        value: u64,
    ) -> HRESULT,
    fn GetInt(
        cfgID: BMDDeckLinkConfigurationID,
        value: *mut u64,
    ) -> HRESULT,
    fn SetFloat(
        cfgID: BMDDeckLinkConfigurationID,
        value: f64,
    ) -> HRESULT,
    fn GetFloat(
        cfgID: BMDDeckLinkConfigurationID,
        value: *mut f64,
    ) -> HRESULT,
    fn SetString(
        cfgID: BMDDeckLinkConfigurationID,
        value: BSTR,
    ) -> HRESULT,
    fn GetString(
        cfgID: BMDDeckLinkConfigurationID,
        value: *mut BSTR,
    ) -> HRESULT,
    fn WriteConfigurationToPreferences() -> HRESULT,
}}

RIDL! {#[uuid(0x138050e5, 0xc60a, 0x4552, 0xbf, 0x3f, 0x0f, 0x35, 0x80, 0x49, 0x32, 0x7e)]
interface IDeckLinkEncoderConfiguration(IDeckLinkEncoderConfigurationVtbl): IUnknown(IUnknownVtbl) {
    fn SetFlag(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: BOOL,
    ) -> HRESULT,
    fn GetFlag(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn SetInt(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: u64,
    ) -> HRESULT,
    fn GetInt(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: *mut u64,
    ) -> HRESULT,
    fn SetFloat(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: f64,
    ) -> HRESULT,
    fn GetFloat(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: *mut f64,
    ) -> HRESULT,
    fn SetString(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: BSTR,
    ) -> HRESULT,
    fn GetString(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        value: *mut BSTR,
    ) -> HRESULT,
    fn GetBytes(
        cfgID: BMDDeckLinkEncoderConfigurationID,
        buffer: *mut c_void,
        bufferSize: *mut u32,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x53436ffb, 0xb434, 0x4906, 0xba, 0xdc, 0xae, 0x30, 0x60, 0xff, 0xe8, 0xef)]
interface IDeckLinkDeckControlStatusCallback(IDeckLinkDeckControlStatusCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn TimecodeUpdate(
        currentTimecode: BMDTimecodeBCD,
    ) -> HRESULT,
    fn VTRControlStateChanged(
        newState: BMDDeckControlVTRControlState,
        error: BMDDeckControlError,
    ) -> HRESULT,
    fn DeckControlEventReceived(
        event: BMDDeckControlEvent,
        error: BMDDeckControlError,
    ) -> HRESULT,
    fn DeckControlStatusChanged(
        flags: BMDDeckControlStatusFlags,
        mask: u32,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x8e1c3ace, 0x19c7, 0x4e00, 0x8b, 0x92, 0xd8, 0x04, 0x31, 0xd9, 0x58, 0xbe)]
interface IDeckLinkControl(IDeckLinkControlVtbl): IUnknown(IUnknownVtbl) {
    fn Open(
        timeScale: BMDTimeScale,
        timeValue: BMDTimeValue,
        timecodeIsDropFrame: BOOL,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Close(
        standbyOn: BOOL,
    ) -> HRESULT,
    fn GetCurrentState(
        mode: *mut BMDDeckControlMode,
        vtrControlState: *mut BMDDeckControlVTRControlState,
        flags: *mut BMDDeckControlStatusFlags,
    ) -> HRESULT,
    fn SetStandby(
        standbyOn: BOOL,
    ) -> HRESULT,
    fn SendCommand(
        inBuffer: *mut u8,
        inBufferSize: u32,
        outBuffer: *mut u8,
        outDataSize: u32,
        outBufferSize: u32,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Play(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Stop(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn TogglePlayStop(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Eject(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn GoToTimecode(
        timecode: BMDTimecodeBCD,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn FastForward(
        viewTape: BOOL,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Rewind(
        viewTape: BOOL,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn StepForward(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Jog(
        rate: f64,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Shuttle(
        rate: f64,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn GetTimecodeString(
        currentTimeCode: *mut BSTR,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn GetTimecode(
        currentTimecode: *mut *mut IDeckLinkTimecode,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn GetTimecodeBCD(
        currentTimecode: *mut BMDTimecodeBCD,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn SetPreroll(
        prerollSeconds: u32,
    ) -> HRESULT,
    fn GetPreroll(
        prerollSeconds: *mut u32,
    ) -> HRESULT,
    fn SetExportOffset(
        exportOffsetFields: i32,
    ) -> HRESULT,
    fn GetExportOffset(
        exportOffsetFields: *mut i32,
    ) -> HRESULT,
    fn GetManualExportOffset(
        deckManualExportOffsetFields: *mut i32,
    ) -> HRESULT,
    fn SetCaptureOffset(
        captureOffsetFields: i32,
    ) -> HRESULT,
    fn GetCaptureOffset(
        captureOffsetFields: *mut i32,
    ) -> HRESULT,
    fn StartExport(
        inTimecode: BMDTimecodeBCD,
        outTimecode: BMDTimecodeBCD,
        exportModeOps: BMDDeckControlExportModeOpsFlags,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn StartCapture(
        useVITC: BOOL,
        inTimecode: BMDTimecodeBCD,
        outTimecode: BMDTimecodeBCD,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn GetDeviceID(
        deviceId: *mut u16,
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn Abort() -> HRESULT,
    fn CrashRecordStart(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn CrashRecordStop(
        error: *mut BMDDeckControlError,
    ) -> HRESULT,
    fn SetCallback(
        callback: *mut IDeckLinkDeckControlStatusCallback,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xf9531d64, 0x3305, 0x4b29, 0xa3, 0x87, 0x7f, 0x74, 0xbb, 0x0d, 0x0e, 0x84)]
interface IBMDStreamingDeviceNotificationCallback(IBMDStreamingDeviceNotificationCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn StreamingDeviceArrived(
        device: *mut IDeckLink,
    ) -> HRESULT,
    fn StreamingDeviceRemoved(
        device: *mut IDeckLink,
    ) -> HRESULT,
    fn StreamingDeviceModeChanged(
        device: *mut IDeckLink,
        mode: BMDStreamingDeviceMode,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x823c475f, 0x55ae, 0x46f9, 0x89, 0x0c, 0x53, 0x7c, 0xc5, 0xce, 0xdc, 0xca)]
interface IBMDStreamingH264InputCallback(IBMDStreamingH264InputCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn H264NALPacketArrived(
        nalPacket: *mut IBMDStreamingH264NALPacket,
    ) -> HRESULT,
    fn H264AudioPacketArrived(
        audioPacket: *mut IBMDStreamingAudioPacket,
    ) -> HRESULT,
    fn MPEG2TSPacketArrived(
        tsPacket: *mut IBMDStreamingMPEG2TSPacket,
    ) -> HRESULT,
    fn H264VideoInputConnectorScanningChanged() -> HRESULT,
    fn H264VideoInputConnectorChanged() -> HRESULT,
    fn H264VideoInputModeChanged() -> HRESULT,
}}

RIDL! {#[uuid(0x2c837444, 0xf989, 0x4d87, 0x90, 0x1a, 0x47, 0xc8, 0xa3, 0x6d, 0x09, 0x6d)]
interface IBMDStreamingDiscovery(IBMDStreamingDiscoveryVtbl): IUnknown(IUnknownVtbl) {
    fn InstallDeviceNotifications(
        theCallback: *mut IBMDStreamingDeviceNotificationCallback,
    ) -> HRESULT,
    fn UninstallDeviceNotifications() -> HRESULT,
}}

RIDL! {#[uuid(0x1ab8035b, 0xcd13, 0x458d, 0xb6, 0xdf, 0x5e, 0x8f, 0x7c, 0x21, 0x41, 0xd9)]
interface IBMDStreamingVideoEncodingMode(IBMDStreamingVideoEncodingModeVtbl): IUnknown(IUnknownVtbl) {
    fn GetName(
        name: *mut BSTR,
    ) -> HRESULT,
    fn GetPresetID() -> u32,
    fn GetSourcePositionX() -> u32,
    fn GetSourcePositionY() -> u32,
    fn GetSourceWidth() -> u32,
    fn GetSourceHeight() -> u32,
    fn GetDestWidth() -> u32,
    fn GetDestHeight() -> u32,
    fn GetFlag(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn GetInt(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: *mut u64,
    ) -> HRESULT,
    fn GetFloat(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: *mut f64,
    ) -> HRESULT,
    fn GetString(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: *mut BSTR,
    ) -> HRESULT,
    fn CreateMutableVideoEncodingMode(
        newEncodingMode: *mut *mut IBMDStreamingMutableVideoEncodingMode,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x19bf7d90, 0x1e0a, 0x400d, 0xb2, 0xc6, 0xff, 0xc4, 0xe7, 0x8a, 0xd4, 0x9d)]
interface IBMDStreamingMutableVideoEncodingMode(IBMDStreamingMutableVideoEncodingModeVtbl): IBMDStreamingVideoEncodingMode(IBMDStreamingVideoEncodingModeVtbl) {
    fn SetSourceRect(
        posX: u32,
        posY: u32,
        width: u32,
        height: u32,
    ) -> HRESULT,
    fn SetDestSize(
        width: u32,
        height: u32,
    ) -> HRESULT,
    fn SetFlag(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: BOOL,
    ) -> HRESULT,
    fn SetInt(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: u64,
    ) -> HRESULT,
    fn SetFloat(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: f64,
    ) -> HRESULT,
    fn SetString(
        cfgID: BMDStreamingEncodingModePropertyID,
        value: BSTR,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x7ac731a3, 0xc950, 0x4ad0, 0x80, 0x4a, 0x83, 0x77, 0xaa, 0x51, 0xc6, 0xc4)]
interface IBMDStreamingVideoEncodingModePresetIterator(IBMDStreamingVideoEncodingModePresetIteratorVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        videoEncodingMode: *mut *mut IBMDStreamingVideoEncodingMode,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x24b6b6ec, 0x1727, 0x44bb, 0x98, 0x18, 0x34, 0xff, 0x08, 0x6a, 0xcf, 0x98)]
interface IBMDStreamingDeviceInput(IBMDStreamingDeviceInputVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportVideoInputMode(
        inputMode: BMDDisplayMode,
        result: *mut BOOL,
    ) -> HRESULT,
    fn GetVideoInputModeIterator(
        iterator: *mut *mut IDeckLinkDisplayModeIterator,
    ) -> HRESULT,
    fn SetVideoInputMode(
        inputMode: BMDDisplayMode,
    ) -> HRESULT,
    fn GetCurrentDetectedVideoInputMode(
        detectedMode: *mut BMDDisplayMode,
    ) -> HRESULT,
    fn GetVideoEncodingMode(
        encodingMode: *mut *mut IBMDStreamingVideoEncodingMode,
    ) -> HRESULT,
    fn GetVideoEncodingModePresetIterator(
        inputMode: BMDDisplayMode,
        iterator: *mut *mut IBMDStreamingVideoEncodingModePresetIterator,
    ) -> HRESULT,
    fn DoesSupportVideoEncodingMode(
        inputMode: BMDDisplayMode,
        encodingMode: *mut IBMDStreamingVideoEncodingMode,
        result: *mut BMDStreamingEncodingSupport,
        changedEncodingMode: *mut *mut IBMDStreamingVideoEncodingMode,
    ) -> HRESULT,
    fn SetVideoEncodingMode(
        encodingMode: *mut IBMDStreamingVideoEncodingMode,
    ) -> HRESULT,
    fn StartCapture() -> HRESULT,
}}

RIDL! {#[uuid(0xe260e955, 0x14be, 0x4395, 0x97, 0x75, 0x9f, 0x02, 0xcc, 0x0a, 0x9d, 0x89)]
interface IBMDStreamingH264NALPacket(IBMDStreamingH264NALPacketVtbl): IUnknown(IUnknownVtbl) {
    fn GetPayloadSize() -> c_long,
    fn GetBytes(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetBytesWithSizePrefix(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetDisplayTime(
        requestedTimeScale: ULONGLONG,
        displayTime: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetPacketIndex(
        packetIndex: *mut c_uint,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xd9eb5902, 0x1ad2, 0x43f4, 0x9e, 0x2c, 0x3c, 0xfa, 0x50, 0xb5, 0xee, 0x19)]
interface IBMDStreamingAudioPacket(IBMDStreamingAudioPacketVtbl): IUnknown(IUnknownVtbl) {
    fn GetCodec() -> BMDStreamingAudioCodec,
    fn GetPayloadSize() -> c_long,
    fn GetBytes(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPlayTime(
        requestedTimeScale: ULONGLONG,
        playTime: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetPacketIndex(
        packetIndex: *mut c_uint,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x91810d1c, 0x4fb3, 0x4aaa, 0xae, 0x56, 0xfa, 0x30, 0x1d, 0x3d, 0xfa, 0x4c)]
interface IBMDStreamingMPEG2TSPacket(IBMDStreamingMPEG2TSPacketVtbl): IUnknown(IUnknownVtbl) {
    fn GetPayloadSize() -> c_long,
    fn GetBytes(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x5867f18c, 0x5bfa, 0x4ccc, 0xb2, 0xa7, 0x9d, 0xfd, 0x14, 0x04, 0x17, 0xd2)]
interface IBMDStreamingH264NALParser(IBMDStreamingH264NALParserVtbl): IUnknown(IUnknownVtbl) {
    fn IsNALSequenceParameterSet(
        nal: *mut IBMDStreamingH264NALPacket,
    ) -> HRESULT,
    fn ISNALPictureParameterSet(
        nal: *mut IBMDStreamingH264NALPacket,
    ) -> HRESULT,
    fn GetProfileAndLevelFromSPS(
        nal: *mut IBMDStreamingH264NALPacket,
        profileIdc: *mut c_uint,
        profileCompatibility: *mut c_uint,
        levelIdc: *mut c_uint,
    ) -> HRESULT,
}}

DEFINE_GUID! {CLSID_CBMDStreamingDiscovery,
0x23a4edf5, 0xa0e5, 0x432c, 0x94, 0xef, 0x3b, 0xab, 0xb5, 0xf8, 0x1c, 0x82}
RIDL! {#[uuid(0x23a4edf5, 0xa0e5, 0x432c, 0x94, 0xef, 0x3b, 0xab, 0xb5, 0xf8, 0x1c, 0x82)]
class CBMDStreamingDiscovery;}

DEFINE_GUID! {CLSID_CBMDStreamingH264NALParser,
0x7753efbd, 0x951c, 0x407c, 0x97, 0xa5, 0x23, 0xc7, 0x37, 0xb7, 0x3b, 0x52}
RIDL! {#[uuid(0x7753efbd, 0x951c, 0x407c, 0x97, 0xa5, 0x23, 0xc7, 0x37, 0xb7, 0x3b, 0x52)]
class CBMDStreamingH264NALParser;}

RIDL! {#[uuid(0x20aa5225, 0x1958, 0x47cb, 0x82, 0x0b, 0x80, 0xa8, 0xd5, 0x21, 0xa6, 0xee)]
interface IDeckLinkVideoOutputCallback(IDeckLinkVideoOutputCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn ScheduledFrameCompleted(
        completedFrame: *mut IDeckLinkVideoFrame,
        result: BMDOutputFrameCompletionResult,
    ) -> HRESULT,
    fn ScheduledPlaybackHasStopped() -> HRESULT,
}}

RIDL! {#[uuid(0xc6fce4c9, 0xc4e4, 0x4047, 0x82, 0xfb, 0x5d, 0x23, 0x82, 0x32, 0xa9, 0x02)]
interface IDeckLinkInputCallback(IDeckLinkInputCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn VideoInputFormatChanged(
        notificationEvents: BMDVideoInputFormatChangedEvents,
        newDisplayMode: *mut IDeckLinkDisplayMode,
        detectedSignalFlags: BMDDetectedVideoInputFormatFlags,
    ) -> HRESULT,
    fn VideoInputFrameArrived(
        videoFrame: *mut IDeckLinkVideoInputFrame,
        audioPacket: *mut IDeckLinkAudioInputPacket,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xacf13e61, 0xf4a0, 0x4974, 0xa6, 0xa7, 0x59, 0xaf, 0xf6, 0x26, 0x8b, 0x31)]
interface IDeckLinkEncoderInputCallback(IDeckLinkEncoderInputCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn VideoInputSignalChanged(
        notificationEvents: BMDVideoInputFormatChangedEvents,
        newDisplayMode: *mut IDeckLinkDisplayMode,
        detectedSignalFlags: BMDDetectedVideoInputFormatFlags,
    ) -> HRESULT,
    fn VideoPacketArrived(
        videoPacket: *mut IDeckLinkEncoderVideoPacket,
    ) -> HRESULT,
    fn AudioPacketArrived(
        audioPacket: *mut IDeckLinkEncoderAudioPacket,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xb36eb6e7, 0x9d29, 0x4aa8, 0x92, 0xef, 0x84, 0x3b, 0x87, 0xa2, 0x89, 0xe8)]
interface IDeckLinkMemoryAllocator(IDeckLinkMemoryAllocatorVtbl): IUnknown(IUnknownVtbl) {
    fn AllocateBuffer(
        bufferSize: c_uint,
        allocatedBuffer: *mut *mut c_void,
    ) -> HRESULT,
    fn ReleaseBuffer(
        buffer: *mut c_void,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
    fn Decommit() -> HRESULT,
}}

RIDL! {#[uuid(0x403c681b, 0x7f46, 0x4a12, 0xb9, 0x93, 0x2b, 0xb1, 0x27, 0x08, 0x4e, 0xe6)]
interface IDeckLinkAudioOutputCallback(IDeckLinkAudioOutputCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn RenderAudioSamples(
        preroll: BOOL,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x50fb36cd, 0x3063, 0x4b73, 0xbd, 0xbb, 0x95, 0x80, 0x87, 0xf2, 0xd8, 0xba)]
interface IDeckLinkIterator(IDeckLinkIteratorVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        deckLinkInstance: *mut *mut IDeckLink,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x7bea3c68, 0x730d, 0x4322, 0xaf, 0x34, 0x8a, 0x71, 0x52, 0xb5, 0x32, 0xa4)]
interface IDeckLinkAPIInformation(IDeckLinkAPIInformationVtbl): IUnknown(IUnknownVtbl) {
    fn GetFlag(
        cfgID: BMDDeckLinkAPIInformationID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn GetInt(
        cfgID: BMDDeckLinkAPIInformationID,
        value: *mut LONGLONG,
    ) -> HRESULT,
    fn GetFloat(
        cfgID: BMDDeckLinkAPIInformationID,
        value: *mut c_double,
    ) -> HRESULT,
    fn GetString(
        cfgID: BMDDeckLinkAPIInformationID,
        value: *mut BSTR,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xbe2d9020, 0x461e, 0x442f, 0x84, 0xb7, 0xe9, 0x49, 0xcb, 0x95, 0x3b, 0x9d)]
interface IDeckLinkOutput(IDeckLinkOutputVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportVideoMode(
        connection: BMDVideoConnection,
        requestedMode: BMDDisplayMode,
        requestedPixelFormat: BMDPixelFormat,
        conversionMode: BMDVideoOutputConversionMode,
        flags: BMDSupportedVideoModeFlags,
        actualMode: *mut BMDDisplayMode,
        supported: *mut BOOL,
    ) -> HRESULT,
    fn GetDisplayMode(
        displayMode: BMDDisplayMode,
        resultDisplayMode: *mut *mut IDeckLinkDisplayMode,
    ) -> HRESULT,
    fn GetDisplayModeIterator(
        iterator: *mut *mut IDeckLinkDisplayModeIterator,
    ) -> HRESULT,
    fn SetScreenPreviewCallback(
        previewCallback: *mut IDeckLinkScreenPreviewCallback,
    ) -> HRESULT,
    fn EnableVideoOutput(
        displayMode: BMDDisplayMode,
        flags: BMDVideoOutputFlags,
    ) -> HRESULT,
    fn DisableVideoOutput() -> HRESULT,
    fn SetVideoOutputFrameMemoryAllocator(
        theAllocator: *mut IDeckLinkMemoryAllocator,
    ) -> HRESULT,
    fn CreateVideoFrame(
        width: c_int,
        height: c_int,
        rowBytes: c_int,
        pixelFormat: BMDPixelFormat,
        flags: BMDFrameFlags,
        outFrame: *mut *mut IDeckLinkMutableVideoFrame,
    ) -> HRESULT,
    fn CreateAncillaryData(
        pixelFormat: BMDPixelFormat,
        outBuffer: *mut *mut IDeckLinkVideoFrameAncillary,
    ) -> HRESULT,
    fn DisplayVideoFrameSync(
        theFrame: *mut IDeckLinkVideoFrame,
    ) -> HRESULT,
    fn ScheduleVideoFrame(
        theFrame: *mut IDeckLinkVideoFrame,
        displayTime: BMDTimeValue,
        displayDuration: BMDTimeValue,
        timeScale: BMDTimeScale,
    ) -> HRESULT,
    fn SetScheduledFrameCompletionCallback(
        theCallback: *mut IDeckLinkVideoOutputCallback,
    ) -> HRESULT,
    fn GetBufferedVideoFrameCount(
        bufferedFrameCount: *mut c_uint,
    ) -> HRESULT,
    fn EnableAudioOutput(
        sampleRate: BMDAudioSampleRate,
        sampleType: BMDAudioSampleType,
        channelCount: c_uint,
        streamType: BMDAudioOutputStreamType,
    ) -> HRESULT,
    fn DisableAudioOutput() -> HRESULT,
    fn WriteAudioSamplesSync(
        buffer: *mut c_void,
        sampleFrameCount: c_uint,
        sampleFramesWritten: *mut c_uint,
    ) -> HRESULT,
    fn BeginAudioPreroll() -> HRESULT,
    fn EndAudioPreroll() -> HRESULT,
    fn ScheduleAudioSamples(
        buffer: *mut c_void,
        sampleFrameCount: c_uint,
        streamTime: BMDTimeValue,
        timeScale: BMDTimeScale,
        sampleFramesWritten: *mut c_uint,
    ) -> HRESULT,
    fn GetBufferedAudioSampleFrameCount(
        bufferedSampleFrameCount: *mut c_uint,
    ) -> HRESULT,
    fn FlushBufferedAudioSamples() -> HRESULT,
    fn SetAudioCallback(
        theCallback: *mut IDeckLinkAudioOutputCallback,
    ) -> HRESULT,
    fn StartScheduledPlayback(
        playbackStartTime: BMDTimeValue,
        timeScale: BMDTimeScale,
        playbackSpeed: c_double,
    ) -> HRESULT,
    fn StopScheduledPlayback(
        stopPlaybackAtTime: BMDTimeValue,
        actualStopTime: *mut BMDTimeValue,
        timeScale: BMDTimeScale,
    ) -> HRESULT,
    fn IsScheduledPlaybackRunning(
        active: *mut BOOL,
    ) -> HRESULT,
    fn GetScheduledStreamTime(
        desiredTimeScale: BMDTimeScale,
        streamTime: *mut BMDTimeValue,
        playbackSpeed: *mut c_double,
    ) -> HRESULT,
    fn GetReferenceStatus(
        referenceStatus: *mut BMDReferenceStatus,
    ) -> HRESULT,
    fn GetHardwareReferenceClock(
        desiredTimeScale: BMDTimeScale,
        hardwareTime: *mut BMDTimeValue,
        timeInFrame: *mut BMDTimeValue,
        ticksPerFrame: *mut BMDTimeValue,
    ) -> HRESULT,
    fn GetFrameCompletionReferenceTimestamp(
        theFrame: *mut IDeckLinkVideoFrame,
        desiredTimeScale: BMDTimeScale,
        frameCompletionTimestamp: *mut BMDTimeValue,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xc21cdb6e, 0xf414, 0x46e4, 0xa6, 0x36, 0x80, 0xa5, 0x66, 0xe0, 0xed, 0x37)]
interface IDeckLinkInput(IDeckLinkInputVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportVideoMode(
        connection: BMDVideoConnection,
        requestedMode: BMDDisplayMode,
        requestedPixelFormat: BMDPixelFormat,
        conversionMode: BMDVideoInputConversionMode,
        flags: BMDSupportedVideoModeFlags,
        actualMode: *mut BMDDisplayMode,
        supported: *mut BOOL,
    ) -> HRESULT,
    fn GetDisplayMode(
        displayMode: BMDDisplayMode,
        resultDisplayMode: *mut *mut IDeckLinkDisplayMode,
    ) -> HRESULT,
    fn GetDisplayModeIterator(
        iterator: *mut *mut IDeckLinkDisplayModeIterator,
    ) -> HRESULT,
    fn SetScreenPreviewCallback(
        previewCallback: *mut IDeckLinkScreenPreviewCallback,
    ) -> HRESULT,
    fn EnableVideoInput(
        displayMode: BMDDisplayMode,
        pixelFormat: BMDPixelFormat,
        flags: BMDVideoInputFlags,
    ) -> HRESULT,
    fn DisableVideoInput() -> HRESULT,
    fn GetAvailableVideoFrameCount(
        availableFrameCount: *mut c_uint,
    ) -> HRESULT,
    fn SetVideoInputFrameMemoryAllocator(
        theAllocator: *mut IDeckLinkMemoryAllocator,
    ) -> HRESULT,
    fn EnableAudioInput(
        sampleRate: BMDAudioSampleRate,
        sampleType: BMDAudioSampleType,
        channelCount: c_uint,
    ) -> HRESULT,
    fn DisableAudioInput() -> HRESULT,
    fn GetAvailableAudioSampleFrameCount(
        availableSampleFrameCount: *mut c_uint,
    ) -> HRESULT,
    fn StartStreams() -> HRESULT,
    fn StopStreams() -> HRESULT,
    fn PauseStreams() -> HRESULT,
    fn FlushStreams() -> HRESULT,
    fn SetCallback(
        theCallback: *mut IDeckLinkInputCallback,
    ) -> HRESULT,
    fn GetHardwareReferenceClock(
        desiredTimeScale: BMDTimeScale,
        hardwareTime: *mut BMDTimeValue,
        timeInFrame: *mut BMDTimeValue,
        ticksPerFrame: *mut BMDTimeValue,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xabbbacbc, 0x45bc, 0x4665, 0x9d, 0x92, 0xac, 0xe6, 0xe5, 0xa9, 0x79, 0x02)]
interface IDeckLinkHDMIInputEDID(IDeckLinkHDMIInputEDIDVtbl): IUnknown(IUnknownVtbl) {
    fn SetInt(
        cfgID: BMDDeckLinkHDMIInputEDIDID,
        value: LONGLONG,
    ) -> HRESULT,
    fn GetInt(
        cfgID: BMDDeckLinkHDMIInputEDIDID,
        value: *mut LONGLONG,
    ) -> HRESULT,
    fn WriteToEDID() -> HRESULT,
}}

RIDL! {#[uuid(0xf222551d, 0x13df, 0x4fd8, 0xb5, 0x87, 0x9d, 0x4f, 0x19, 0xec, 0x12, 0xc9)]
interface IDeckLinkEncoderInput(IDeckLinkEncoderInputVtbl): IUnknown(IUnknownVtbl) {
    fn DoesSupportVideoMode(
        connection: BMDVideoConnection,
        requestedMode: BMDDisplayMode,
        requestedCodec: BMDPixelFormat,
        requestedCodecProfile: c_uint,
        flags: BMDSupportedVideoModeFlags,
        supported: *mut BOOL,
    ) -> HRESULT,
    fn GetDisplayMode(
        displayMode: BMDDisplayMode,
        resultDisplayMode: *mut *mut IDeckLinkDisplayMode,
    ) -> HRESULT,
    fn GetDisplayModeIterator(
        iterator: *mut *mut IDeckLinkDisplayModeIterator,
    ) -> HRESULT,
    fn EnableVideoInput(
        displayMode: BMDDisplayMode,
        pixelFormat: BMDPixelFormat,
        flags: BMDVideoInputFlags,
    ) -> HRESULT,
    fn DisableVideoInput() -> HRESULT,
    fn GetAvailablePacketsCount(
        availablePacketsCount: *mut c_uint,
    ) -> HRESULT,
    fn SetMemoryAllocator(
        theAllocator: *mut IDeckLinkMemoryAllocator,
    ) -> HRESULT,
    fn EnableAudioInput(
        audioFormat: BMDAudioFormat,
        sampleRate: BMDAudioSampleRate,
        sampleType: BMDAudioSampleType,
        channelCount: c_uint,
    ) -> HRESULT,
    fn DisableAudioInput() -> HRESULT,
    fn GetAvailableAudioSampleFrameCount(
        availableSampleFrameCount: *mut c_uint,
    ) -> HRESULT,
    fn StartStreams() -> HRESULT,
    fn StopStreams() -> HRESULT,
    fn PauseStreams() -> HRESULT,
    fn FlushStreams() -> HRESULT,
    fn SetCallback(
        theCallback: *mut IDeckLinkEncoderInputCallback,
    ) -> HRESULT,
    fn GetHardwareReferenceClock(
        desiredTimeScale: BMDTimeScale,
        hardwareTime: *mut BMDTimeValue,
        timeInFrame: *mut BMDTimeValue,
        ticksPerFrame: *mut BMDTimeValue,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x3f716fe0, 0xf023, 0x4111, 0xbe, 0x5d, 0xef, 0x44, 0x14, 0xc0, 0x5b, 0x17)]
interface IDeckLinkVideoFrame(IDeckLinkVideoFrameVtbl): IUnknown(IUnknownVtbl) {
    fn GetWidth() -> c_long,
    fn GetHeight() -> c_long,
    fn GetRowBytes() -> c_long,
    fn GetPixelFormat() -> BMDPixelFormat,
    fn GetFlags() -> BMDFrameFlags,
    fn GetBytes(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetTimecode(
        format: BMDTimecodeFormat,
        timecode: *mut *mut IDeckLinkTimecode,
    ) -> HRESULT,
    fn GetAncillaryData(
        ancillary: *mut *mut IDeckLinkVideoFrameAncillary,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x69e2639f, 0x40da, 0x4e19, 0xb6, 0xf2, 0x20, 0xac, 0xe8, 0x15, 0xc3, 0x90)]
interface IDeckLinkMutableVideoFrame(IDeckLinkMutableVideoFrameVtbl): IDeckLinkVideoFrame(IDeckLinkVideoFrameVtbl) {
    fn SetFlags(
        newFlags: BMDFrameFlags,
    ) -> HRESULT,
    fn SetTimecode(
        format: BMDTimecodeFormat,
        timecode: *mut IDeckLinkTimecode,
    ) -> HRESULT,
    fn SetTimecodeFromComponents(
        format: BMDTimecodeFormat,
        hours: c_uchar,
        minutes: c_uchar,
        seconds: c_uchar,
        frames: c_uchar,
        flags: BMDTimecodeFlags,
    ) -> HRESULT,
    fn SetAncillaryData(
        ancillary: *mut IDeckLinkVideoFrameAncillary,
    ) -> HRESULT,
    fn SetTimecodeUserBits(
        format: BMDTimecodeFormat,
        userBits: BMDTimecodeUserBits,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xda0f7e4a, 0xedc7, 0x48a8, 0x9c, 0xdd, 0x2d, 0xb5, 0x1c, 0x72, 0x9c, 0xd7)]
interface IDeckLinkVideoFrame3DExtensions(IDeckLinkVideoFrame3DExtensionsVtbl): IUnknown(IUnknownVtbl) {
    fn Get3DPackingFormat() -> BMDVideo3DPackingFormat,
    fn GetFrameForRightEye(
        rightEyeFrame: *mut *mut IDeckLinkVideoFrame,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xe232a5b7, 0x4db4, 0x44c9, 0x91, 0x52, 0xf4, 0x7c, 0x12, 0xe5, 0xf0, 0x51)]
interface IDeckLinkVideoFrameMetadataExtensions(IDeckLinkVideoFrameMetadataExtensionsVtbl): IUnknown(IUnknownVtbl) {
    fn GetInt(
        metadataID: BMDDeckLinkFrameMetadataID,
        value: *mut LONGLONG,
    ) -> HRESULT,
    fn GetFloat(
        metadataID: BMDDeckLinkFrameMetadataID,
        value: *mut c_double,
    ) -> HRESULT,
    fn GetFlag(
        metadataID: BMDDeckLinkFrameMetadataID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn GetString(
        metadataID: BMDDeckLinkFrameMetadataID,
        value: *mut BSTR,
    ) -> HRESULT,
    fn GetBytes(
        metadataID: BMDDeckLinkFrameMetadataID,
        buffer: *mut c_void,
        bufferSize: *mut c_uint,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x05cfe374, 0x537c, 0x4094, 0x9a, 0x57, 0x68, 0x05, 0x25, 0x11, 0x8f, 0x44)]
interface IDeckLinkVideoInputFrame(IDeckLinkVideoInputFrameVtbl): IDeckLinkVideoFrame(IDeckLinkVideoFrameVtbl) {
    fn GetStreamTime(
        frameTime: *mut BMDTimeValue,
        frameDuration: *mut BMDTimeValue,
        timeScale: BMDTimeScale,
    ) -> HRESULT,
    fn GetHardwareReferenceTimestamp(
        timeScale: BMDTimeScale,
        frameTime: *mut BMDTimeValue,
        frameDuration: *mut BMDTimeValue,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xcc5bbf7e, 0x029c, 0x4d3b, 0x91, 0x58, 0x60, 0x00, 0xef, 0x5e, 0x36, 0x70)]
interface IDeckLinkAncillaryPacket(IDeckLinkAncillaryPacketVtbl): IUnknown(IUnknownVtbl) {
    fn GetBytes(
        format: BMDAncillaryPacketFormat,
        data: *mut *const c_void,
        size: *mut c_uint,
    ) -> HRESULT,
    fn GetDID() -> c_uchar,
    fn GetSDID() -> c_uchar,
    fn GetLineNumber() -> c_uint,
    fn GetDataStreamIndex() -> c_uchar,
}}

RIDL! {#[uuid(0x3fc8994b, 0x88fb, 0x4c17, 0x96, 0x8f, 0x9a, 0xab, 0x69, 0xd9, 0x64, 0xa7)]
interface IDeckLinkAncillaryPacketIterator(IDeckLinkAncillaryPacketIteratorVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        packet: *mut *mut IDeckLinkAncillaryPacket,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x6c186c0f, 0x459e, 0x41d8, 0xae, 0xe2, 0x48, 0x12, 0xd8, 0x1a, 0xee, 0x68)]
interface IDeckLinkVideoFrameAncillaryPackets(IDeckLinkVideoFrameAncillaryPacketsVtbl): IUnknown(IUnknownVtbl) {
    fn GetPacketIterator(
        iterator: *mut *mut IDeckLinkAncillaryPacketIterator,
    ) -> HRESULT,
    fn GetFirstPacketByID(
        DID: c_uchar,
        SDID: c_uchar,
        packet: *mut *mut IDeckLinkAncillaryPacket,
    ) -> HRESULT,
    fn AttachPacket(
        packet: *mut IDeckLinkAncillaryPacket,
    ) -> HRESULT,
    fn DetachPacket(
        packet: *mut IDeckLinkAncillaryPacket,
    ) -> HRESULT,
    fn DetachAllPackets() -> HRESULT,
}}

RIDL! {#[uuid(0x732e723c, 0xd1a4, 0x4e29, 0x9e, 0x8e, 0x4a, 0x88, 0x79, 0x7a, 0x00, 0x04)]
interface IDeckLinkVideoFrameAncillary(IDeckLinkVideoFrameAncillaryVtbl): IUnknown(IUnknownVtbl) {
    fn GetBufferForVerticalBlankingLine(
        lineNumber: c_uint,
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPixelFormat() -> BMDPixelFormat,
    fn GetDisplayMode() -> BMDDisplayMode,
}}

RIDL! {#[uuid(0xb693f36c, 0x316e, 0x4af1, 0xb6, 0xc2, 0xf3, 0x89, 0xa4, 0xbc, 0xa6, 0x20)]
interface IDeckLinkEncoderPacket(IDeckLinkEncoderPacketVtbl): IUnknown(IUnknownVtbl) {
    fn GetBytes(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetSize() -> c_long,
    fn GetStreamTime(
        frameTime: *mut BMDTimeValue,
        timeScale: BMDTimeScale,
    ) -> HRESULT,
    fn GetPacketType() -> BMDPacketType,
}}

RIDL! {#[uuid(0x4e7fd944, 0xe8c7, 0x4eac, 0xb8, 0xc0, 0x7b, 0x77, 0xf8, 0x0f, 0x5a, 0xe0)]
interface IDeckLinkEncoderVideoPacket(IDeckLinkEncoderVideoPacketVtbl): IDeckLinkEncoderPacket(IDeckLinkEncoderPacketVtbl) {
    fn GetPixelFormat() -> BMDPixelFormat,
    fn GetHardwareReferenceTimestamp(
        timeScale: BMDTimeScale,
        frameTime: *mut BMDTimeValue,
        frameDuration: *mut BMDTimeValue,
    ) -> HRESULT,
    fn GetTimecode(
        format: BMDTimecodeFormat,
        timecode: *mut *mut IDeckLinkTimecode,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x49e8edc8, 0x693b, 0x4e14, 0x8e, 0xf6, 0x12, 0xc6, 0x58, 0xf5, 0xa0, 0x7a)]
interface IDeckLinkEncoderAudioPacket(IDeckLinkEncoderAudioPacketVtbl): IDeckLinkEncoderPacket(IDeckLinkEncoderPacketVtbl) {
    fn GetAudioFormat() -> BMDAudioFormat,
}}

RIDL! {#[uuid(0x639c8e0b, 0x68d5, 0x4bde, 0xa6, 0xd4, 0x95, 0xf3, 0xae, 0xaf, 0xf2, 0xe7)]
interface IDeckLinkH265NALPacket(IDeckLinkH265NALPacketVtbl): IDeckLinkEncoderVideoPacket(IDeckLinkEncoderVideoPacketVtbl) {
    fn GetUnitType(
        unitType: *mut c_uchar,
    ) -> HRESULT,
    fn GetBytesNoPrefix(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetSizeNoPrefix() -> c_long,
}}

RIDL! {#[uuid(0xe43d5870, 0x2894, 0x11de, 0x8c, 0x30, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66)]
interface IDeckLinkAudioInputPacket(IDeckLinkAudioInputPacketVtbl): IUnknown(IUnknownVtbl) {
    fn GetSampleFrameCount() -> c_long,
    fn GetBytes(
        buffer: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPacketTime(
        packetTime: *mut BMDTimeValue,
        timeScale: BMDTimeScale,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xb1d3f49a, 0x85fe, 0x4c5d, 0x95, 0xc8, 0x0b, 0x5d, 0x5d, 0xcc, 0xd4, 0x38)]
interface IDeckLinkScreenPreviewCallback(IDeckLinkScreenPreviewCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn DrawFrame(
        theFrame: *mut IDeckLinkVideoFrame,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x504e2209, 0xcac7, 0x4c1a, 0x9f, 0xb4, 0xc5, 0xbb, 0x62, 0x74, 0xd2, 0x2f)]
interface IDeckLinkGLScreenPreviewHelper(IDeckLinkGLScreenPreviewHelperVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeGL() -> HRESULT,
    fn PaintGL() -> HRESULT,
    fn SetFrame(
        theFrame: *mut IDeckLinkVideoFrame,
    ) -> HRESULT,
    fn Set3DPreviewFormat(
        previewFormat: BMD3DPreviewFormat,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x2094b522, 0xd1a1, 0x40c0, 0x9a, 0xc7, 0x1c, 0x01, 0x22, 0x18, 0xef, 0x02)]
interface IDeckLinkDX9ScreenPreviewHelper(IDeckLinkDX9ScreenPreviewHelperVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        device: *mut c_void,
    ) -> HRESULT,
    fn Render(
        rc: *mut RECT,
    ) -> HRESULT,
    fn SetFrame(
        theFrame: *mut IDeckLinkVideoFrame,
    ) -> HRESULT,
    fn Set3DPreviewFormat(
        previeFormat: BMD3DPreviewFormat,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xb002a1ec, 0x070d, 0x4288, 0x82, 0x89, 0xbd, 0x5d, 0x36, 0xe5, 0xff, 0x0d)]
interface IDeckLinkNotificationCallback(IDeckLinkNotificationCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Notify(
        topic: BMDNotifications,
        param1: ULONGLONG,
        param2: ULONGLONG,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xb85df4c8, 0xbdf5, 0x47c1, 0x80, 0x64, 0x28, 0x16, 0x2e, 0xbd, 0xd4, 0xeb)]
interface IDeckLinkNotification(IDeckLinkNotificationVtbl): IUnknown(IUnknownVtbl) {
    fn Subscribe(
        topic: BMDNotifications,
        theCallback: *mut IDeckLinkNotificationCallback,
    ) -> HRESULT,
    fn Unsubscribe(
        topic: BMDNotifications,
        theCallback: *mut IDeckLinkNotificationCallback,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x17d4bf8e, 0x4911, 0x473a, 0x80, 0xa0, 0x73, 0x1c, 0xf6, 0xff, 0x34, 0x5b)]
interface IDeckLinkProfileAttributes(IDeckLinkProfileAttributesVtbl): IUnknown(IUnknownVtbl) {
    fn GetFlag(
        cfgID: BMDDeckLinkAttributeID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn GetInt(
        cfgID: BMDDeckLinkAttributeID,
        value: *mut LONGLONG,
    ) -> HRESULT,
    fn GetFloat(
        cfgID: BMDDeckLinkAttributeID,
        value: *mut c_double,
    ) -> HRESULT,
    fn GetString(
        cfgID: BMDDeckLinkAttributeID,
        value: *mut BSTR,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x29e5a8c0, 0x8be4, 0x46eb, 0x93, 0xac, 0x31, 0xda, 0xab, 0x5b, 0x7b, 0xf2)]
interface IDeckLinkProfileIterator(IDeckLinkProfileIteratorVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        profile: *mut *mut IDeckLinkProfile,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x16093466, 0x674a, 0x432b, 0x9d, 0xa0, 0x1a, 0xc2, 0xc5, 0xa8, 0x24, 0x1c)]
interface IDeckLinkProfile(IDeckLinkProfileVtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        device: *mut *mut IDeckLink,
    ) -> HRESULT,
    fn IsActive(
        isActive: *mut BOOL,
    ) -> HRESULT,
    fn SetActive() -> HRESULT,
    fn GetPeers(
        profileIterator: *mut *mut IDeckLinkProfileIterator,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xa4f9341e, 0x97aa, 0x4e04, 0x89, 0x35, 0x15, 0xf8, 0x09, 0x89, 0x8c, 0xea)]
interface IDeckLinkProfileCallback(IDeckLinkProfileCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn ProfileChanging(
        profileToBeActivated: *mut IDeckLinkProfile,
        streamsWillBeForcedToStop: BOOL,
    ) -> HRESULT,
    fn ProfileActivated(
        activatedProfile: *mut IDeckLinkProfile,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x30d41429, 0x3998, 0x4b6d, 0x84, 0xf8, 0x78, 0xc9, 0x4a, 0x79, 0x7c, 0x6e)]
interface IDeckLinkProfileManager(IDeckLinkProfileManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetProfiles(
        profileIterator: *mut *mut IDeckLinkProfileIterator,
    ) -> HRESULT,
    fn GetProfile(
        profileID: BMDProfileID,
        profile: *mut *mut IDeckLinkProfile,
    ) -> HRESULT,
    fn SetCallback(
        callback: *mut IDeckLinkProfileCallback,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x5f558200, 0x4028, 0x49bc, 0xbe, 0xac, 0xdb, 0x3f, 0xa4, 0xa9, 0x6e, 0x46)]
interface IDeckLinkStatus(IDeckLinkStatusVtbl): IUnknown(IUnknownVtbl) {
    fn GetFlag(
        statusID: BMDDeckLinkStatusID,
        value: *mut BOOL,
    ) -> HRESULT,
    fn GetInt(
        statusID: BMDDeckLinkStatusID,
        value: *mut LONGLONG,
    ) -> HRESULT,
    fn GetFloat(
        statusID: BMDDeckLinkStatusID,
        value: *mut c_double,
    ) -> HRESULT,
    fn GetString(
        statusID: BMDDeckLinkStatusID,
        value: *mut BSTR,
    ) -> HRESULT,
    fn GetBytes(
        statusID: BMDDeckLinkStatusID,
        buffer: *mut c_void,
        bufferSize: *mut c_uint,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x89afcaf5, 0x65f8, 0x421e, 0x98, 0xf7, 0x96, 0xfe, 0x5f, 0x5b, 0xfb, 0xa3)]
interface IDeckLinkKeyer(IDeckLinkKeyerVtbl): IUnknown(IUnknownVtbl) {
    fn Enable(
        isExternal: BOOL,
    ) -> HRESULT,
    fn SetLevel(
        level: c_uchar,
    ) -> HRESULT,
    fn RampUp(
        numberOfFrames: c_uint,
    ) -> HRESULT,
    fn RampDown(
        numberOfFrames: c_uint,
    ) -> HRESULT,
    fn Disable() -> HRESULT,
}}

RIDL! {#[uuid(0x3bbcb8a2, 0xda2c, 0x42d9, 0xb5, 0xd8, 0x88, 0x08, 0x36, 0x44, 0xe9, 0x9a)]
interface IDeckLinkVideoConversion(IDeckLinkVideoConversionVtbl): IUnknown(IUnknownVtbl) {
    fn ConvertFrame(
        srcFrame: *mut IDeckLinkVideoFrame,
        dstFrame: *mut IDeckLinkVideoFrame,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x4997053b, 0x0adf, 0x4cc8, 0xac, 0x70, 0x7a, 0x50, 0xc4, 0xbe, 0x72, 0x8f)]
interface IDeckLinkDeviceNotificationCallback(IDeckLinkDeviceNotificationCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn DeckLinkDeviceArrived(
        deckLinkDevice: *mut IDeckLink,
    ) -> HRESULT,
    fn DeckLinkDeviceRemoved(
        deckLinkDevice: *mut IDeckLink,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xcdbf631c, 0xbc76, 0x45fa, 0xb4, 0x4d, 0xc5, 0x50, 0x59, 0xbc, 0x61, 0x01)]
interface IDeckLinkDiscovery(IDeckLinkDiscoveryVtbl): IUnknown(IUnknownVtbl) {
    fn InstallDeviceNotifications(
        deviceNotificationCallback: *mut IDeckLinkDeviceNotificationCallback,
    ) -> HRESULT,
    fn UninstallDeviceNotifications() -> HRESULT,
}}

DEFINE_GUID! {CLSID_CDeckLinkIterator,
0xba6c6f44, 0x6da5, 0x4dce, 0x94, 0xaa, 0xee, 0x2d, 0x13, 0x72, 0xa6, 0x76}
RIDL! {#[uuid(0xba6c6f44, 0x6da5, 0x4dce, 0x94, 0xaa, 0xee, 0x2d, 0x13, 0x72, 0xa6, 0x76)]
class CDeckLinkIterator;}

DEFINE_GUID! {CLSID_CDeckLinkAPIInformation,
0x263ca19f, 0xed09, 0x482e, 0x9f, 0x9d, 0x84, 0x00, 0x57, 0x83, 0xa2, 0x37}
RIDL! {#[uuid(0x263ca19f, 0xed09, 0x482e, 0x9f, 0x9d, 0x84, 0x00, 0x57, 0x83, 0xa2, 0x37)]
class CDeckLinkAPIInformation;}

DEFINE_GUID! {CLSID_CDeckLinkGLScreenPreviewHelper,
0xf63e77c7, 0xb655, 0x4a4a, 0x9a, 0xd0, 0x3c, 0xa8, 0x5d, 0x39, 0x43, 0x43}
RIDL! {#[uuid(0xf63e77c7, 0xb655, 0x4a4a, 0x9a, 0xd0, 0x3c, 0xa8, 0x5d, 0x39, 0x43, 0x43)]
class CDeckLinkGLScreenPreviewHelper;}

DEFINE_GUID! {CLSID_CDeckLinkDX9ScreenPreviewHelper,
0xcc010023, 0xe01d, 0x4525, 0x9d, 0x59, 0x80, 0xc8, 0xab, 0x3d, 0xc7, 0xa0}
RIDL! {#[uuid(0xcc010023, 0xe01d, 0x4525, 0x9d, 0x59, 0x80, 0xc8, 0xab, 0x3d, 0xc7, 0xa0)]
class CDeckLinkDX9ScreenPreviewHelper;}

DEFINE_GUID! {CLSID_DeckLinkVideoConversion,
0x7dbbbb11, 0x5b7b, 0x467d, 0xae, 0xa4, 0xce, 0xa4, 0x68, 0xfd, 0x36, 0x8c}
RIDL! {#[uuid(0x7dbbbb11, 0x5b7b, 0x467d, 0xae, 0xa4, 0xce, 0xa4, 0x68, 0xfd, 0x36, 0x8c)]
class CDeckLinkVideoConversion;}

DEFINE_GUID! {CLSID_CDeckLinkDiscovery,
0x22fbfc33, 0x8d07, 0x495c, 0xa5, 0xbf, 0xda, 0xb5, 0xea, 0x9b, 0x82, 0xdb}
RIDL! {#[uuid(0x22fbfc33, 0x8d07, 0x495c, 0xa5, 0xbf, 0xda, 0xb5, 0xea, 0x9b, 0x82, 0xdb)]
class CDeckLinkDiscovery;}

DEFINE_GUID! {CLSID_CDeckLinkVideoFrameAncillaryPackets,
0xf891ad29, 0xd0c2, 0x46e9, 0xa9, 0x26, 0x4e, 0x2d, 0x0d, 0xd8, 0xcf, 0xad}
RIDL! {#[uuid(0xf891ad29, 0xd0c2, 0x46e9, 0xa9, 0x26, 0x4e, 0x2d, 0x0d, 0xd8, 0xcf, 0xad)]
class CDeckLinkVideoFrameAncillaryPackets;}
