use crate::{arc, blocks, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSWorkspace")]
    pub Workspace(ns::Id)
);

impl Workspace {
    define_cls!(NS_WORKSPACE);

    #[objc::cls_msg_send(sharedWorkspace)]
    pub fn shared() -> &'static mut Self;

    #[objc::msg_send(notificationCenter)]
    pub fn notification_center(&self) -> &ns::NotificationCenter;
}

#[doc(alias = "NSWorkspaceAuthorizationType")]
#[repr(isize)]
pub enum AuthorizationType {
    #[doc(alias = "NSWorkspaceAuthorizationTypeCreateSymbolicLink")]
    CreateSymbolicLink,

    #[doc(alias = "NSWorkspaceAuthorizationTypeSetAttributes")]
    SetAttributes,

    #[doc(alias = "NSWorkspaceAuthorizationTypeReplaceFile")]
    ReplaceFile,
}

define_obj_type!(
    #[doc(alias = "NSWorkspaceAuthorization")]
    pub Authorization(ns::Id)
);

/// NSWorkspaceAuthorization
impl Workspace {
    #[objc::msg_send(requestAuthorizationOfType:completionHandler:)]
    pub unsafe fn _request_authorization_of_type_ch(
        &self,
        type_: ns::WorkspaceAuthorizationType,
        ch: *mut std::ffi::c_void,
    );

    pub fn request_authorization_of_type_ch_block<'a, F>(
        &self,
        type_: ns::WorkspaceAuthorizationType,
        ch: &mut blocks::Block<F>,
    ) where
        F: FnOnce(Option<&'a ns::WorkspaceAuthorization>, Option<&'a ns::Error>),
    {
        unsafe { self._request_authorization_of_type_ch(type_, ch.as_mut_ptr()) }
    }

    pub fn request_authorization_of_type_ch<F>(&self, type_: ns::WorkspaceAuthorizationType, ch: F)
    where
        F: FnOnce(Option<&ns::WorkspaceAuthorization>, Option<&ns::Error>) + 'static,
    {
        let ch = blocks::once2(ch);
        self.request_authorization_of_type_ch_block(type_, ch.escape())
    }

    pub async fn request_authorization_of_type(
        &self,
        type_: ns::WorkspaceAuthorizationType,
    ) -> Result<arc::R<ns::WorkspaceAuthorization>, arc::R<ns::Error>> {
        let (future, ch) = blocks::result();
        self.request_authorization_of_type_ch_block(type_, ch.escape());
        future.await
    }
}

impl ns::FileManager {
    #[objc::cls_msg_send(fileManagerWithAuthorization:)]
    pub fn with_authorization_ar(authorization: &ns::WorkspaceAuthorization) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_authorization(authorization: &ns::WorkspaceAuthorization) -> arc::R<Self>;
}

extern "C" {
    static NS_WORKSPACE: &'static objc::Class<Workspace>;
}
