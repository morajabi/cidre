//
//  av.h
//  av
//
//  Created by Yury Korolev on 02.05.2022.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

//#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

Class AV_CAPTURE_DEVICE;
Class AV_CAPTURE_SESSION;
Class AV_CAPTURE_MULTI_CAM_SESSION;
Class AV_CAPTURE_METADATA_OUTPUT;
Class AV_CAPTURE_DEVICE_DISCOVERY_SESSION;
Class AV_CAPTURE_VIDEO_DATA_OUTPUT;
Class AV_CAPTURE_AUDIO_DATA_OUTPUT;
Class AV_CAPTURE_DEVICE_INPUT;
Class AV_CAPTURE_CONNECTION;
Class AV_CAPTURE_METADATA_INPUT;
Class AV_CAPTURE_DEVICE_ROTATION_COORDINATOR;
Class AV_CAPTURE_PHOTO_OUTPUT;
Class AV_CAPTURE_VIDEO_PREVIEW_LAYER;

Class AV_AUDIO_PLAYER_NODE;
Class AV_AUDIO_PLAYER;

Class AV_AUDIO_ENGINE;
Class AV_AUDIO_SESSION;

Class AV_AUDIO_CONNECTION_POINT;

Class AV_URL_ASSET;
Class AV_ASSET_WRITER;
Class AV_ASSET_READER;
Class AV_ASSET_WRITER_INPUT;
Class AV_ASSET_READER_TRACK_OUTPUT;

Class AV_OUTPUT_SETTINGS_ASSISTANT;

Class AV_AUDIO_TIME;

Class AV_AUDIO_UNIT_EQ;
Class AV_AUDIO_UNIT_EFFECT;
Class AV_AUDIO_UNIT_TIME_EFFECT;

Class AV_AUDIO_PCM_BUFFER;
Class AV_AUDIO_COMPRESSED_BUFFER;
Class AV_AUDIO_FORMAT;

Class AV_PLAYER;

Class AV_SAMPLE_BUFFER_DISPLAY_LAYER;
Class AV_SAMPLE_BUFFER_VIDEO_RENDERER;

Class AV_SPEECH_SYNTHESIS_VOICE;
Class AV_SPEECH_SYNTHESIZER;
Class AV_SPEECH_UTTERANCE;

__attribute__((constructor))
static void av_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
#if TARGET_OS_WATCH
#else
        AV_CAPTURE_DEVICE = [AVCaptureDevice class];
        AV_CAPTURE_METADATA_OUTPUT = [AVCaptureMetadataOutput class];
        AV_CAPTURE_SESSION = [AVCaptureSession class];
        AV_CAPTURE_CONNECTION = [AVCaptureConnection class];
        AV_CAPTURE_DEVICE_DISCOVERY_SESSION = [AVCaptureDeviceDiscoverySession class];
        AV_CAPTURE_VIDEO_DATA_OUTPUT = [AVCaptureVideoDataOutput class];
        AV_CAPTURE_AUDIO_DATA_OUTPUT = [AVCaptureAudioDataOutput class];
        AV_CAPTURE_DEVICE_INPUT = [AVCaptureDeviceInput class];
        AV_CAPTURE_PHOTO_OUTPUT = [AVCapturePhotoOutput class];
        AV_CAPTURE_VIDEO_PREVIEW_LAYER = [AVCaptureVideoPreviewLayer class];
#endif
        if (@available(iOS 17.0, *)) {
#if TARGET_OS_WATCH
#else
            AV_CAPTURE_DEVICE_ROTATION_COORDINATOR = [AVCaptureDeviceRotationCoordinator class];
#endif
        } else {
            // Fallback on earlier versions
        }
        
#if TARGET_OS_OSX
#else
        
#if TARGET_OS_WATCH
#else
        AV_CAPTURE_MULTI_CAM_SESSION = [AVCaptureMultiCamSession class];
        AV_CAPTURE_METADATA_INPUT = [AVCaptureMetadataInput class];
#endif
        AV_AUDIO_SESSION = [AVAudioSession class];
#endif

#if TARGET_OS_WATCH
#else
        AV_OUTPUT_SETTINGS_ASSISTANT = [AVOutputSettingsAssistant class];
#endif
        
        AV_AUDIO_PLAYER_NODE = [AVAudioPlayerNode class];
        AV_AUDIO_PLAYER = [AVAudioPlayer class];
        
        AV_AUDIO_ENGINE = [AVAudioEngine class];
        AV_AUDIO_TIME = [AVAudioTime class];
        
#if TARGET_OS_WATCH
#else
        AV_AUDIO_UNIT_EFFECT = [AVAudioUnitEffect class];
        AV_AUDIO_UNIT_EQ = [AVAudioUnitEQ class];
        AV_AUDIO_UNIT_TIME_EFFECT = [AVAudioUnitTimeEffect class];
#endif
        AV_AUDIO_CONNECTION_POINT = [AVAudioConnectionPoint class];
        
        
        AV_URL_ASSET = [AVURLAsset class];
        
        
#if TARGET_OS_WATCH
#else
        AV_ASSET_WRITER = [AVAssetWriter class];
        AV_ASSET_WRITER_INPUT = [AVAssetWriterInput class];
        AV_ASSET_READER_TRACK_OUTPUT = [AVAssetReaderTrackOutput class];
        AV_ASSET_READER = [AVAssetReader class];
        
        AV_SAMPLE_BUFFER_DISPLAY_LAYER = [AVSampleBufferDisplayLayer class];
        AV_SAMPLE_BUFFER_VIDEO_RENDERER = [AVSampleBufferVideoRenderer class];
#endif
        
        AV_AUDIO_FORMAT = [AVAudioFormat class];
        
        AV_AUDIO_PCM_BUFFER = [AVAudioPCMBuffer class];
        AV_AUDIO_COMPRESSED_BUFFER = [AVAudioCompressedBuffer class];
        
        AV_PLAYER = [AVPlayer class];

        AV_SPEECH_SYNTHESIS_VOICE = [AVSpeechSynthesisVoice class];
        AV_SPEECH_SYNTHESIZER = [AVSpeechSynthesizer class];
        AV_SPEECH_UTTERANCE = [AVSpeechUtterance class];

        initialized = 1;
    }
}

NS_ASSUME_NONNULL_END
