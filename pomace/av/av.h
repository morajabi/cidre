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

#pragma mark - AVCaptureSession

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

NS_ASSUME_NONNULL_END