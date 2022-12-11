//
//  av.h
//  av
//
//  Created by Yury Korolev on 02.05.2022.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - AVCaptureSystemPressureState

#if TARGET_OS_IPHONE
//@property(atomic, readonly) AVCaptureSystemPressureLevel level;
rsel(, AVCaptureSystemPressureState *, level, AVCaptureSystemPressureLevel)
rsel(, AVCaptureSystemPressureState *, factors, AVCaptureSystemPressureFactors)

#endif

#pragma mark - AVCaptureDevice

NS_RETURNS_RETAINED
csel_abc(, AVCaptureDevice, defaultDeviceWithDeviceType, AVCaptureDeviceType, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDevice * _Nullable)

NS_RETURNS_RETAINED
csel_a(, AVCaptureDevice, deviceWithUniqueID, NSString *, AVCaptureDevice * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel(, AVCaptureDevice *, uniqueID, NSString *)

//- (BOOL)lockForConfiguration:(NSError * _Nullable * _Nullable)outError;
rsel_a(, id, lockForConfiguration, NSError * _Nullable * _Nullable, BOOL)
//- (void)unlockForConfiguration;
wsel(, id, unlockForConfiguration)

//- (BOOL)supportsAVCaptureSessionPreset:(AVCaptureSessionPreset)preset;
rsel_a(, id, supportsAVCaptureSessionPreset, AVCaptureSessionPreset, BOOL)

//@property(nonatomic, readonly) NSArray<AVCaptureDeviceFormat *> *formats
NS_RETURNS_NOT_RETAINED
rsel(, id, formats, NSArray<AVCaptureDeviceFormat *> *)

//@property(nonatomic, retain) AVCaptureDeviceFormat *activeFormat
NS_RETURNS_NOT_RETAINED
rsel(, id, activeFormat, AVCaptureDeviceFormat *)
wsel_a(, id, setActiveFormat, AVCaptureDeviceFormat* )

//@property(nonatomic) CMTime activeVideoMinFrameDuration API_AVAILABLE(ios(7.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);
rsel(, id, activeVideoMinFrameDuration, CMTime)
wsel_a(, id, setActiveVideoMinFrameDuration, CMTime)
rsel(, id, activeVideoMaxFrameDuration, CMTime)
wsel_a(, id, setActiveVideoMaxFrameDuration, CMTime)

// @property(nonatomic, readonly) BOOL hasTorch;
rsel(, id, hasTorch, BOOL)

//@property(nonatomic, readonly, getter=isVideoBinned) BOOL videoBinned API_UNAVAILABLE(macos);
#if TARGET_OS_IPHONE
rsel(, id, isVideoBinned, BOOL)
#endif

//@property(nonatomic, readonly) NSArray<AVFrameRateRange *> *videoSupportedFrameRateRanges;
rsel(, id, videoSupportedFrameRateRanges, NSArray<AVFrameRateRange *> *)

//@property(nonatomic, readonly) CMFormatDescriptionRef formatDescription;
rsel(, id, formatDescription, CMFormatDescriptionRef)

//@property(nonatomic, readonly) AVCaptureAutoFocusSystem autoFocusSystem API_AVAILABLE(macos(10.15), ios(8.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);
rsel(, id, autoFocusSystem, AVCaptureAutoFocusSystem)

//@property(nonatomic, readonly, getter=isMultiCamSupported) BOOL multiCamSupported API_AVAILABLE(ios(13.0), macCatalyst(14.0)) API_UNAVAILABLE(macos, tvos) API_UNAVAILABLE(watchos);
#if TARGET_OS_IPHONE
rsel(, id, isMultiCamSupported, BOOL)
#endif

//@property(nonatomic, readonly, getter=isCenterStageSupported) BOOL centerStageSupported API_AVAILABLE(macos(12.3), ios(14.5), macCatalyst(14.5)) API_UNAVAILABLE(tvos) API_UNAVAILABLE(watchos);

rsel(, id, isCenterStageSupported, BOOL)

//@property(nonatomic, readonly, nullable) AVFrameRateRange *videoFrameRateRangeForCenterStage API_AVAILABLE(macos(12.3), ios(14.5), macCatalyst(14.5)) API_UNAVAILABLE(tvos) API_UNAVAILABLE(watchos);
NS_RETURNS_NOT_RETAINED
rsel(, id, videoFrameRateRangeForCenterStage, AVFrameRateRange* _Nullable)

//+ (nullable AVCaptureDevice *)deviceWithUniqueID:(NSString *)deviceUniqueID;
// + (nullable AVCaptureDevice *)defaultDeviceWithDeviceType:(AVCaptureDeviceType)deviceType mediaType:(nullable AVMediaType)mediaType position:(AVCaptureDevicePosition)position API_AVAILABLE(macos(10.15), ios(10.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);

#pragma mark - AVFrameRateRange

rsel(,id, minFrameRate, Float64)
rsel(,id, maxFrameRate, Float64)

rsel(,id,  minFrameDuration, CMTime)
rsel(,id,  maxFrameDuration, CMTime)

#pragma mark - AVCaptureInput

//@property(nonatomic, readonly) NSArray<AVCaptureInputPort *> *ports;
NS_RETURNS_NOT_RETAINED
rsel(, id, ports, NSArray<AVCaptureInputPort *> *);


//@property(nonatomic, readonly) AVCaptureInput *input;
NS_RETURNS_NOT_RETAINED
rsel(, id, input, AVCaptureInput *)

#pragma mark - AVCaptureOutput

// @property(nonatomic, readonly) NSArray<AVCaptureConnection *> *connections;
NS_RETURNS_NOT_RETAINED
rsel(, id, connections, NSArray<AVCaptureConnection *> *)


NS_RETURNS_NOT_RETAINED
rsel_a(, id, connectionWithMediaType, AVMediaType, AVCaptureConnection * _Nullable)

bool is_mutlicam_supported(void) {
#if TARGET_OS_OSX
  return NO;
#else
  return [AVCaptureMultiCamSession isMultiCamSupported];
#endif
}

#if TARGET_OS_OSX

#else
//@property(nonatomic, readonly) float hardwareCost;
rsel(, id, hardwareCost, float)
rsel(, id, systemPressureCost, float)
#endif

#pragma mark - AVCaptureSession

NS_RETURNS_RETAINED
csel(, AVCaptureSession, new, AVCaptureSession *)
//- (BOOL)canSetSessionPreset:(AVCaptureSessionPreset)preset;
rsel_a(, id, canSetSessionPreset, AVCaptureSessionPreset, BOOL)

NS_RETURNS_NOT_RETAINED
rsel(, id, sessionPreset, AVCaptureSessionPreset)

wsel_a(, id, setSessionPreset, AVCaptureSessionPreset)

NS_RETURNS_NOT_RETAINED
rsel(, AVCaptureSession *, inputs, NSArray<__kindof AVCaptureInput *> *)

//- (BOOL)canAddInput:(AVCaptureInput *)input;
rsel_a(, id, canAddInput, AVCaptureInput *, BOOL)
//- (void)addInput:(AVCaptureInput *)input;
wsel_a(, id, addInput, AVCaptureInput *)
//- (void)removeInput:(AVCaptureInput *)input;
wsel_a(, id, removeInput, AVCaptureInput *)

//@property(nonatomic, readonly) NSArray<__kindof AVCaptureOutput *> *outputs;
NS_RETURNS_NOT_RETAINED
rsel(, AVCaptureSession *, outputs, NSArray<__kindof AVCaptureOutput *> *)

rsel_a(, id, canAddOutput, AVCaptureOutput *, BOOL)
wsel_a(, id, addOutput, AVCaptureOutput *)
wsel_a(, id, removeOutput, AVCaptureOutput *)

wsel_a(, id, addInputWithNoConnections, AVCaptureInput *)
wsel_a(, id, addOutputWithNoConnections, AVCaptureOutput *)

rsel_a(, id, canAddConnection, AVCaptureConnection *, BOOL)
wsel_a(, id, addConnection, AVCaptureConnection *)
wsel_a(, id, removeConnection, AVCaptureConnection *)

//- (void)beginConfiguration;
wsel(, id, beginConfiguration)
//- (void)commitConfiguration;
wsel(, id, commitConfiguration)

wsel(, id, startRunning)
wsel(, id, stopRunning)

rwsel(, id, usesApplicationAudioSession, setUsesApplicationAudioSession, BOOL)

#pragma mark - AVCaptureMultiCamSession

#if TARGET_OS_OSX

#else

NS_RETURNS_RETAINED
csel(, AVCaptureMultiCamSession, new, AVCaptureMultiCamSession *)
#endif

#pragma mark - AVCaptureDeviceDiscoverySession

NS_RETURNS_RETAINED
csel_abc(, AVCaptureDeviceDiscoverySession, discoverySessionWithDeviceTypes, NSArray<AVCaptureDeviceType> *, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDeviceDiscoverySession *)

NS_RETURNS_NOT_RETAINED
rsel(, id, devices, NSArray<AVCaptureDevice *> *)

#if TARGET_OS_OSX

#else

NS_RETURNS_NOT_RETAINED
rsel(, id, supportedMultiCamDeviceSets, NSArray<NSSet<AVCaptureDevice *> *> *)

#endif

wsel(av_, id, reset)
NS_RETURNS_NOT_RETAINED
rsel(, id, engine, AVAudioEngine * _Nullable)


#pragma mark - AVAudioNode

//@property (nonatomic, readonly) NSUInteger numberOfInputs;
rsel(, id, numberOfInputs, NSUInteger)
//@property (nonatomic, readonly) NSUInteger numberOfOutputs;
rsel(, id, numberOfOutputs, NSUInteger)


#pragma mark - AVAudioEngine

NS_RETURNS_RETAINED
csel(, AVAudioEngine, new, AVAudioEngine *)

//- (void)attachNode:(AVAudioNode *)node;
wsel_a(, id, attachNode, AVAudioNode *)
wsel_a(, id, detachNode, AVAudioNode *)

wsel_abcde(, id, connect, AVAudioNode *, to, AVAudioNode *, fromBus, AVAudioNodeBus, toBus, AVAudioNodeBus, format, AVAudioFormat * _Nullable)
wsel_abc(, id, connect, AVAudioNode *, to, AVAudioNode *, format, AVAudioFormat * _Nullable)

wsel(, id, prepare)

rsel_a(, id, startAndReturnError, NSError **, BOOL);
wsel_abcd(, id, connect, AVAudioNode *, toConnectionPoints, NSArray<AVAudioConnectionPoint *> *, fromBus, AVAudioNodeBus, format, AVAudioFormat * _Nullable)
wsel_ab(, id, disconnectNodeInput, AVAudioNode *, bus, AVAudioNodeBus)
wsel_a(, id, disconnectNodeInput, AVAudioNode *)

wsel_ab(, id, disconnectNodeOutput, AVAudioNode *, bus, AVAudioNodeBus)
wsel_a(, id, disconnectNodeOutput, AVAudioNode *)

NS_RETURNS_NOT_RETAINED
rsel(, id, inputNode, AVAudioInputNode *)

NS_RETURNS_NOT_RETAINED
rsel(, id, outputNode, AVAudioOutputNode *)

NS_RETURNS_NOT_RETAINED
rsel(, id, mainMixerNode, AVAudioMixerNode *)

#pragma mark AVAudioUnitEffect

NS_RETURNS_RETAINED
asel_a(, AVAudioUnitEffect, initWithAudioComponentDescription, AudioComponentDescription)
rwsel(, id, bypass, setBypass, BOOL)

#pragma mark AVAudioUnitEQFilterParameters

rwsel(, id, filterType, setFilterType, AVAudioUnitEQFilterType)
rwsel(, id, frequency, setFrequency, float)
rwsel(, id, bandwidth, setBandwidth, float)
rwsel(, id, gain, setGain, float)

#pragma mark AVAudioUnitEQ

asel_a(, AVAudioUnitEQ, initWithNumberOfBands, NSUInteger)
rsel(, id, bands, NSArray *)

rwsel(, id, globalGain, setGlobalGain, float)

#pragma mark AVAudioUnitTimeEffect

NS_RETURNS_RETAINED
asel_a(, AVAudioUnitTimeEffect, initWithAudioComponentDescription, AudioComponentDescription)

//- (void)prepare;
//- (void)connect:(AVAudioNode *)node1 to:(AVAudioNode *)node2 format:(AVAudioFormat * __nullable)format;

#pragma mark - AVAudioTime

NS_RETURNS_RETAINED
csel_a(, AVAudioTime, timeWithHostTime, uint64_t, AVAudioTime *)
NS_RETURNS_RETAINED
csel_ab(, AVAudioTime, timeWithAudioTimeStamp, const AudioTimeStamp *, sampleRate, double, AVAudioTime *)
NS_RETURNS_RETAINED
csel_ab(, AVAudioTime, timeWithSampleTime, AVAudioFramePosition, atRate, double, AVAudioTime *)

NS_RETURNS_RETAINED
csel_abc(, AVAudioTime, timeWithHostTime, uint64_t, sampleTime, AVAudioFramePosition, atRate, double, AVAudioTime *)


rsel(, id, hostTime, uint64_t)
rsel(, id, audioTimeStamp, AudioTimeStamp)
rsel(, id, sampleRate, double)
rsel(, id, isSampleTimeValid, BOOL)
rsel(, id, isHostTimeValid, BOOL)
rsel(, id, sampleTime, AVAudioFramePosition)

NS_RETURNS_RETAINED
rsel_a(, id, extrapolateTimeFromAnchor, AVAudioTime *, AVAudioTime * _Nullable)

#pragma mark - AVAudioCommonFormat

NS_RETURNS_RETAINED
asel_a(, AVAudioFormat, initWithStreamDescription, const AudioStreamBasicDescription *)

NS_RETURNS_RETAINED
asel_ab(, AVAudioFormat, initWithStreamDescription, const AudioStreamBasicDescription *, channelLayout, AVAudioChannelLayout * _Nullable)

NS_RETURNS_RETAINED
asel_ab(, AVAudioFormat, initStandardFormatWithSampleRate, double, channels, AVAudioChannelCount)

NS_RETURNS_RETAINED
asel_ab(, AVAudioFormat, initStandardFormatWithSampleRate, double, channelLayout, AVAudioChannelLayout *)

NS_RETURNS_RETAINED
asel_abcd(, AVAudioFormat, initWithCommonFormat, AVAudioCommonFormat, sampleRate, double, interleaved, BOOL, channelLayout, AVAudioChannelLayout *)

NS_RETURNS_RETAINED
asel_a(, AVAudioFormat, initWithSettings, NSDictionary *);

NS_RETURNS_NOT_RETAINED
rsel(, id, settings, NSDictionary *)
rsel(, id, isInterleaved, BOOL)
rsel(, id, commonFormat, AVAudioCommonFormat)
rsel(av_format_, AVAudioFormat *, channelCount, AVAudioChannelCount)
rsel(, id, streamDescription, const AudioStreamBasicDescription *)
NS_RETURNS_NOT_RETAINED
rsel(, id, channelLayout, AVAudioChannelLayout * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel(, id, magicCookie, NSData * _Nullable)

#pragma mark - AVAudioBuffer

rsel(, id, format, AVAudioFormat *)
rsel(, id, audioBufferList, const AudioBufferList *)
rsel(, id, mutableAudioBufferList, AudioBufferList *)

#pragma mark - AVAudioPCMBuffer

//- (nullable instancetype)initWithPCMFormat:(AVAudioFormat *)format frameCapacity:(AVAudioFrameCount)frameCapacity NS_DESIGNATED_INITIALIZER;
asel_ab(, AVAudioPCMBuffer, initWithPCMFormat,AVAudioFormat *, frameCapacity, AVAudioFrameCount)

rsel(, AVAudioPCMBuffer *, frameLength, AVAudioFrameCount)
wsel_a(, AVAudioPCMBuffer *, setFrameLength, AVAudioFrameCount)
rsel(, AVAudioPCMBuffer *, frameCapacity, AVAudioFrameCount)
//@property (nonatomic, readonly) AVAudioFrameCount frameCapacity;
rsel(, id, stride, NSUInteger)

#pragma mark - AVAudioCompressedBuffer

asel_ab(, AVAudioCompressedBuffer, initWithFormat, AVAudioFormat *, packetCapacity, AVAudioPacketCount)

asel_abc(, AVAudioCompressedBuffer, initWithFormat, AVAudioFormat *, packetCapacity, AVAudioPacketCount, maximumPacketSize, NSInteger)


rsel(, id, packetCapacity, AVAudioPacketCount)
rsel(, id, packetCount, AVAudioPacketCount)
wsel_a(, id, setPacketCount, AVAudioPacketCount)

rsel(, id, maximumPacketSize, NSInteger)
rsel(, id, byteCapacity, uint32_t)

rwsel(, id, byteLength, setByteLength, uint32_t)

//@property (nonatomic, readonly, nullable) AudioStreamPacketDescription *packetDescriptions;
rsel(, id, packetDescriptions, AudioStreamPacketDescription * _Nullable)

rsel(, AVAudioCompressedBuffer *, data, void *)
//@property (nonatomic) uint32_t byteLength API_AVAILABLE(macosx(10.13), ios(11.0), watchos(4.0), tvos(11.0));

//- (nullable instancetype)initWithPCMFormat:(AVAudioFormat *)format frameCapacity:(AVAudioFrameCount)frameCapacity

#pragma mark - AVAssetWriterInput

csel_ab(, AVAssetWriterInput, assetWriterInputWithMediaType, AVMediaType, outputSettings, NSDictionary * _Nullable, AVAssetWriterInput *)

csel_abc(, AVAssetWriterInput, assetWriterInputWithMediaType, AVMediaType, outputSettings, NSDictionary * _Nullable, sourceFormatHint, CMFormatDescriptionRef _Nullable, AVAssetWriterInput *)


rsel(, id, mediaType, AVMediaType)
rsel(, id, outputSettings, NSDictionary * _Nullable)

rsel(, id, isReadyForMoreMediaData, BOOL)

rsel(, id, expectsMediaDataInRealTime, BOOL)
wsel_a(, id, setExpectsMediaDataInRealTime, BOOL)

wsel(, id, markAsFinished)

//- (BOOL)appendSampleBuffer:(CMSampleBufferRef)sampleBuffer;
rsel_a(, id, appendSampleBuffer, CMSampleBufferRef, BOOL)

#pragma mark - AVAssetWriter

//+ (nullable instancetype)assetWriterWithURL:(NSURL *)outputURL fileType:(AVFileType)outputFileType error:(NSError * _Nullable * _Nullable)outError;
NS_RETURNS_RETAINED
csel_abc(, AVAssetWriter, assetWriterWithURL, NSURL *, fileType, AVFileType, error, NSError * _Nullable * _Nullable, AVAssetWriter *)

rwsel(, id, shouldOptimizeForNetworkUse, setShouldOptimizeForNetworkUse, BOOL)
//- (BOOL)canAddInput:(AVAssetWriterInput *)input;
rsel_a(AVAssetWriter_, AVAssetWriter *, canAddInput, AVAssetWriterInput *, BOOL)
wsel_a(AVAssetWriter_, AVAssetWriter *, addInput, AVAssetWriterInput *)

wsel(, id, startWriting)
wsel_a(, id, startSessionAtSourceTime, CMTime);
wsel_a(, id, endSessionAtSourceTime, CMTime);
wsel(, id, cancelWriting)
//- (void)endSessionAtSourceTime:(CMTime)endTime;
wsel(, id, finishWriting)

rsel(, id, error, NSError * _Nullable)
//@property (nonatomic, readonly) NSArray<AVAssetWriterInput *> *inputs;
rsel(AVAssetWriter_, id, inputs, NSArray<AVAssetWriterInput *> *)

typedef void (^ VoidBlock)(void);
wsel_a(, id, finishWritingWithCompletionHandler, VoidBlock)
//- (void)finishWritingWithCompletionHandler:(void (^)(void))handler API_AVAILABLE(macos(10.9), ios(6.0), tvos(9.0))

#pragma mark - AVURLAsset

wsel_ab(, id, loadTracksWithMediaType, AVMediaType, completionHandler, id)

NS_RETURNS_RETAINED
csel_ab(, AVURLAsset, URLAssetWithURL, NSURL *, options, NSDictionary * _Nullable, AVURLAsset *)
//+ (instancetype)URLAssetWithURL:(NSURL *)URL options:(nullable NSDictionary<NSString *, id> *)options;

#pragma mark - AVAssetReader

NS_RETURNS_RETAINED
csel_ab(, AVAssetReader, assetReaderWithAsset, AVAsset *, error, NSError **, AVAssetReader *)

wsel_a(AVAssetReader_, id, addOutput, AVAssetReaderOutput *);
rsel(, id, startReading, BOOL)
wsel(, id, cancelReading)

//@property (nonatomic, readonly) NSArray<AVAssetReaderOutput *> *outputs;
NS_RETURNS_NOT_RETAINED
rsel(AVAssetReader_, AVAssetReader *, outputs, NSArray *)

rsel_a(AVAssetReader_, id, canAddOutput, AVAssetReaderOutput *, BOOL);
//- (BOOL)canAddOutput:(AVAssetReaderOutput *)output;

#pragma mark - AVAssetReaderOutput

//+ (instancetype)assetReaderTrackOutputWithTrack:(AVAssetTrack *)track outputSettings:(nullable NSDictionary<NSString *, id> *)outputSettings;

NS_RETURNS_RETAINED
csel_ab(, AVAssetReaderTrackOutput, assetReaderTrackOutputWithTrack, AVAssetTrack *, outputSettings, NSDictionary * _Nullable, AVAssetReaderTrackOutput *)
//AVAssetReaderTrackOutput

rwsel(, id, alwaysCopiesSampleData, setAlwaysCopiesSampleData, BOOL)

#pragma mark - AVAudioConverter

rsel(, id, availableEncodeChannelLayoutTags, NSArray<NSNumber *> * _Nullable)
rsel(, id, applicableEncodeSampleRates, NSArray<NSNumber *> * _Nullable)
rsel(, id, availableEncodeSampleRates, NSArray<NSNumber *> * _Nullable)
rsel(, id, applicableEncodeBitRates, NSArray<NSNumber *> * _Nullable)
rsel(, id, availableEncodeBitRates, NSArray<NSNumber *> * _Nullable)
rsel(, id, maximumOutputPacketSize, NSInteger)

rsel(, id, bitRateStrategy, NSString * _Nullable)
wsel_a(, id, setBitRateStrategy, NSString * _Nullable)

#pragma mark - AVMetadataObject

rsel(AVMetadataObject_, AVMetadataObject *, time, CMTime)
rsel(AVMetadataObject_, AVMetadataObject *, duration, CMTime)
rsel(AVMetadataObject_, AVMetadataObject *, type, AVMetadataObjectType)

#pragma mark - AVMetadataBodyObject

rsel(AVMetadataBodyObject_, AVMetadataBodyObject *, bodyID, NSInteger)

#pragma mark - AVMetadataSalientObject

rsel(AVMetadataSalientObject_, AVMetadataSalientObject *, objectID, NSInteger)

#pragma mark - AVMetadataFaceObject

rsel(AVMetadataFaceObject_, AVMetadataFaceObject *, faceID, NSInteger)
rsel(AVMetadataFaceObject_, AVMetadataFaceObject *, hasRollAngle, BOOL)
rsel(AVMetadataFaceObject_, AVMetadataFaceObject *, hasYawAngle, BOOL)
rsel(AVMetadataFaceObject_, AVMetadataFaceObject *, yawAngle, CGFloat)
rsel(AVMetadataFaceObject_, AVMetadataFaceObject *, rollAngle, CGFloat)

#pragma mark - AVCaptureMetadataOutput

//@property(nonatomic, readonly) NSArray<AVMetadataObjectType> *availableMetadataObjectTypes;
rsel(, id, availableMetadataObjectTypes, NSArray *)
rwsel(, id, rectOfInterest, setRectOfInterest, CGRect)
csel(, AVCaptureMetadataOutput, new, AVCaptureMetadataOutput *)

#pragma mark - AVCaptureVideoDataOutput


NS_RETURNS_RETAINED
csel(, AVCaptureVideoDataOutput, new, AVCaptureVideoDataOutput *)

rwsel(, AVCaptureVideoDataOutput *, alwaysDiscardsLateVideoFrames, setAlwaysDiscardsLateVideoFrames, BOOL)

rsel(, id, availableVideoCVPixelFormatTypes, NSArray *)
rsel(, id, availableVideoCodecTypes, NSArray *)

rsel(, id, sampleBufferCallbackQueue, dispatch_queue_t _Nullable)
rsel(, id, videoSettings, NSDictionary * _Nullable)
rsel_a(, id, recommendedVideoSettingsForAssetWriterWithOutputFileType, AVFileType, NSDictionary * _Nullable)

rsel_ab(, id, recommendedVideoSettingsForVideoCodecType, AVVideoCodecType, assetWriterOutputFileType, AVFileType, NSDictionary * _Nullable)
//- (nullable NSDictionary<NSString *, id> *)recommendedVideoSettingsForVideoCodecType:(AVVideoCodecType)videoCodecType assetWriterOutputFileType:(AVFileType)outputFileType API_AVAILABLE(macos(10.15), ios(11.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);

#pragma mark - AVCaptureDeviceInput

// + (nullable instancetype)deviceInputWithDevice:(AVCaptureDevice *)device error:(NSError * _Nullable * _Nullable)outError;

csel_ab(, AVCaptureDeviceInput, deviceInputWithDevice, AVCaptureDevice *, error,  NSError * _Nullable * _Nullable, AVCaptureDeviceInput * _Nullable)


SEL sel_copyNextSampleBuffer;
SEL sel_status;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    sel_copyNextSampleBuffer = @selector(copyNextSampleBuffer);
    sel_status = @selector(status);
  }
}

NS_ASSUME_NONNULL_END
