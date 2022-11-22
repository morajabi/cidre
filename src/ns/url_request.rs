use crate::{
    cf::{self, Retained},
    define_obj_type, ns,
};

define_obj_type!(URLRequest(ns::Id));
define_obj_type!(MutableURLRequest(URLRequest));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum CachePolicy {
    UseProtocol = 0,
    ReloadIgnoringLocalCacheData = 1,
    ReturnCacheDataElseLoad = 2,
    ReturnCacheDataDontLoad = 3,
    ReloadIgnoringLocalAndRemoteCacheData = 4,
    ReloadRevalidatingCacheData = 5,
}

impl Default for CachePolicy {
    #[inline]
    fn default() -> Self {
        Self::UseProtocol
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum NetworkServiceType {
    // // Standard internet traffic
    Default = 0,

    /// Voice over IP control traffic
    VoIP = 1,

    /// Video traffic
    Video = 2,

    /// Background traffic
    Background = 3,

    /// Voice data
    Voice = 4,

    /// Responsive data
    ResponsiveData = 6,

    /// Multimedia Audio/Video Streaming
    AVStreaming = 8,

    /// Responsive Multimedia Audio/Video
    ResponsiveAV = 9,

    /// Call Signaling
    CallSignaling = 11,
}

impl URLRequest {
    /// ```
    /// use cidre::{cf, ns};
    /// let url = cf::URL::from_str("https://google.com").unwrap();
    /// let request = ns::URLRequest::with_url(&url);
    /// let request_url = request.url().unwrap();
    /// assert!(url.cf_string().equal(request_url.cf_string()));
    /// assert_eq!(request.cache_policy(), ns::URLRequestCachePolicy::UseProtocol);
    /// assert_eq!(request.timeout_interval(), 60f64);
    /// assert_eq!(request.network_service_type(), ns::URLRequestNetworkServiceType::Default);
    /// assert!(request.allows_cellular_access());
    /// assert!(request.allows_expensive_network_access());
    /// assert!(request.allows_constrained_network_access());
    /// assert!(!request.assumes_http3_capable());
    /// assert_eq!(request.attribution(), ns::URLRequestAttribution::Developer);
    /// assert!(!request.requires_dns_sec_validation());
    /// assert!(request.http_method().is_some());
    /// assert!(request.all_http_header_fields().is_none());
    /// assert!(request.http_body().is_none());
    /// ```
    #[inline]
    pub fn with_url(url: &cf::URL) -> Retained<URLRequest> {
        unsafe { NSURLRequest_requestWithURL(url) }
    }

    #[inline]
    pub fn with_url_cache_policy_and_timeout(
        url: &cf::URL,
        cache_policy: CachePolicy,
        timeout_interval: cf::TimeInterval,
    ) -> Retained<URLRequest> {
        unsafe {
            NSURLRequest_requestWithURL_cachePolicy_timeoutInterval(
                url,
                cache_policy,
                timeout_interval,
            )
        }
    }

    #[inline]
    pub fn url(&self) -> Option<&cf::URL> {
        unsafe { NSURLRequest_rsel_URL(self) }
    }

    #[inline]
    pub fn cache_policy(&self) -> CachePolicy {
        unsafe { NSURLRequest_rsel_cachePolicy(self) }
    }

    #[inline]
    pub fn timeout_interval(&self) -> cf::TimeInterval {
        unsafe { NSURLRequest_rsel_timeoutInterval(self) }
    }

    #[inline]
    pub fn network_service_type(&self) -> NetworkServiceType {
        unsafe { NSURLRequest_rsel_networkServiceType(self) }
    }

    #[inline]
    pub fn allows_cellular_access(&self) -> bool {
        unsafe { NSURLRequest_rsel_allowsCellularAccess(self) }
    }

    #[inline]
    pub fn allows_expensive_network_access(&self) -> bool {
        unsafe { NSURLRequest_rsel_allowsExpensiveNetworkAccess(self) }
    }

    #[inline]
    pub fn allows_constrained_network_access(&self) -> bool {
        unsafe { NSURLRequest_rsel_allowsConstrainedNetworkAccess(self) }
    }

    pub fn assumes_http3_capable(&self) -> bool {
        unsafe { NSURLRequest_rsel_assumesHTTP3Capable(self) }
    }

    pub fn attribution(&self) -> Attribution {
        unsafe { NSURLRequest_rsel_attribution(self) }
    }

    pub fn requires_dns_sec_validation(&self) -> bool {
        unsafe { NSURLRequest_rsel_requiresDNSSECValidation(self) }
    }

    pub fn http_method(&self) -> Option<&cf::String> {
        unsafe { NSURLRequest_rsel_HTTPMethod(self) }
    }

    pub fn all_http_header_fields(&self) -> Option<&cf::DictionaryOf<cf::String, cf::String>> {
        unsafe { NSURLRequest_rsel_allHTTPHeaderFields(self) }
    }

    pub fn value_for_http_header_field<'a>(&'a self, field: &cf::String) -> Option<&'a cf::String> {
        unsafe { NSURLRequest_rsel_valueForHTTPHeaderField(self, field) }
    }

    #[inline]
    pub fn http_body(&self) -> Option<&cf::Data> {
        unsafe { NSURLRequest_rsel_HTTPBody(self) }
    }
}

/// enum is used to indicate whether the
/// user or developer specified the URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Attribution {
    /// Indicates that the URL was specified
    /// by the developer. This is the default value for an ns::URLRequest when created.
    Developer = 0,

    /// Indicates that the URL was specified by
    /// the user.
    User = 1,
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLRequest_requestWithURL(url: &cf::URL) -> Retained<URLRequest>;
    fn NSURLRequest_requestWithURL_cachePolicy_timeoutInterval(
        url: &cf::URL,
        cache_policy: CachePolicy,
        timeout_interval: cf::TimeInterval,
    ) -> Retained<URLRequest>;

    fn NSURLRequest_rsel_URL(request: &URLRequest) -> Option<&cf::URL>;
    fn NSURLRequest_rsel_cachePolicy(request: &URLRequest) -> CachePolicy;
    fn NSURLRequest_rsel_timeoutInterval(request: &URLRequest) -> cf::TimeInterval;
    fn NSURLRequest_rsel_networkServiceType(request: &URLRequest) -> NetworkServiceType;
    fn NSURLRequest_rsel_allowsCellularAccess(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_allowsExpensiveNetworkAccess(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_allowsConstrainedNetworkAccess(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_assumesHTTP3Capable(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_attribution(request: &URLRequest) -> Attribution;
    fn NSURLRequest_rsel_requiresDNSSECValidation(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_HTTPMethod(request: &URLRequest) -> Option<&cf::String>;
    fn NSURLRequest_rsel_allHTTPHeaderFields(
        request: &URLRequest,
    ) -> Option<&cf::DictionaryOf<cf::String, cf::String>>;
    fn NSURLRequest_rsel_valueForHTTPHeaderField<'a>(
        request: &'a URLRequest,
        field: &cf::String,
    ) -> Option<&'a cf::String>;
    fn NSURLRequest_rsel_HTTPBody(request: &URLRequest) -> Option<&cf::Data>;
}