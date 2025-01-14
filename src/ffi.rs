use core_foundation::{
    base::{CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeID, CFTypeRef, OSStatus, TCFType},
    data::CFDataRef,
    dictionary::CFDictionaryRef,
    error::CFErrorRef,
    string::{CFString, CFStringRef},
};
use std::{
    borrow::Cow,
    fmt::{self, Debug},
    os::raw::{c_char, c_void},
    ptr, slice, str,
};

/// Four character codes used as identifiers. See:
/// <https://developer.apple.com/documentation/kernel/fourcharcode>
#[repr(transparent)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct FourCharacterCode(u32);

impl FourCharacterCode {
    fn as_bytes(&self) -> &[u8; 4] {
        unsafe { &*(self as *const FourCharacterCode as *const [u8; 4]) }
    }

    fn as_str(&self) -> &str {
        str::from_utf8(self.as_bytes()).unwrap()
    }
}

impl AsRef<str> for FourCharacterCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for FourCharacterCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FourCharacterCode({})", self.as_str())
    }
}

impl From<u32> for FourCharacterCode {
    fn from(num: u32) -> FourCharacterCode {
        FourCharacterCode(num)
    }
}

impl From<[u8; 4]> for FourCharacterCode {
    fn from(bytes: [u8; 4]) -> FourCharacterCode {
        Self::from(&bytes)
    }
}

impl<'a> From<&'a [u8; 4]> for FourCharacterCode {
    fn from(bytes: &[u8; 4]) -> FourCharacterCode {
        let mut result: u32 = 0;

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), (&mut result as *mut u32) as *mut u8, 4);
        }

        FourCharacterCode(result)
    }
}

impl From<CFStringRef> for FourCharacterCode {
    fn from(string_ref: CFStringRef) -> FourCharacterCode {
        Self::from(&unsafe { CFString::wrap_under_get_rule(string_ref) })
    }
}

impl<'a> From<&'a CFString> for FourCharacterCode {
    fn from(string: &'a CFString) -> FourCharacterCode {
        let string = Cow::from(string);
        assert_eq!(string.as_bytes().len(), 4);

        let mut code = [0u8; 4];
        code.copy_from_slice(string.as_bytes());
        code.into()
    }
}

/// Reference to an access control policy.
///
/// See `SecAccessControlRef` documentation:
/// <https://developer.apple.com/documentation/security/secaccesscontrolref>
pub(crate) type AccessControlRef = CFTypeRef;

/// Reference to a `Key`
///
/// See `SecKeyRef` documentation:
/// <https://developer.apple.com/documentation/security/seckeyref>
pub(crate) type KeyRef = CFTypeRef;

/// Reference to a `Keychain`
///
/// See `SecKeychainRef` documentation:
/// <https://developer.apple.com/documentation/security/seckeychainref>
pub(crate) type KeychainRef = CFTypeRef;

/// Reference to a `keychain::Item`
///
/// See `SecKeychainItemRef` documentation:
/// <https://developer.apple.com/documentation/security/seckeychainitemref>
pub(crate) type ItemRef = CFTypeRef;

/// Attribute type codes.
///
/// Wrapper for `SecKeychainAttrType`. See:
/// <https://developer.apple.com/documentation/security/seckeychainattrtype>
pub(crate) type SecKeychainAttrType = FourCharacterCode;

/// Individual keychain attribute.
///
/// Wrapper for the `SecKeychainAttribute` struct. See:
/// <https://developer.apple.com/documentation/security/seckeychainattribute>
#[repr(C)]
pub(super) struct SecKeychainAttribute {
    tag: SecKeychainAttrType,
    length: u32,
    data: *mut u8,
}

impl SecKeychainAttribute {
    /// Get the `FourCharacterCode` tag identifying this attribute's type
    pub(crate) fn tag(&self) -> SecKeychainAttrType {
        self.tag
    }

    /// Get the data associated with this attribute as a byte slice.
    pub(crate) fn data(&self) -> Option<&[u8]> {
        if self.data.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(self.data, self.length as usize) })
        }
    }
}

/// List of attributes (as returned from e.g. `SecKeychainItemCopyContent`).
///
/// NOTE: This type does not implement `Drop` as there are various ways it can
/// be allocated/deallocated. The caller must take care to free it!
///
/// Wrapper for the `SecKeychainAttributeList` struct. See:
/// <https://developer.apple.com/documentation/security/seckeychainattributelist>
#[repr(C)]
pub(super) struct SecKeychainAttributeList {
    count: u32,
    attr: *mut SecKeychainAttribute,
}

impl SecKeychainAttributeList {
    /// Get an iterator over this attribute list.
    pub(crate) fn iter(&self) -> slice::Iter<SecKeychainAttribute> {
        self.as_slice().iter()
    }

    /// Get a slice of `Attribute` values
    pub(crate) fn as_slice(&self) -> &[SecKeychainAttribute] {
        unsafe { slice::from_raw_parts(self.attr, self.count as usize) }
    }
}

#[link(name = "Security", kind = "framework")]
extern "C" {
    pub(crate) static kSecAttrAccessControl: CFStringRef;
    pub(crate) static kSecAttrAccessible: CFStringRef;
    pub(crate) static kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly: CFStringRef;
    pub(crate) static kSecAttrAccessibleWhenUnlockedThisDeviceOnly: CFStringRef;
    pub(crate) static kSecAttrAccessibleWhenUnlocked: CFStringRef;
    pub(crate) static kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly: CFStringRef;
    pub(crate) static kSecAttrAccessibleAfterFirstUnlock: CFStringRef;
    pub(crate) static kSecAttrAccessibleAlwaysThisDeviceOnly: CFStringRef;
    pub(crate) static kSecAttrAccessibleAlways: CFStringRef;
    pub(crate) static kSecAttrAccount: CFStringRef;
    pub(crate) static kSecAttrApplicationLabel: CFStringRef;
    pub(crate) static kSecAttrApplicationTag: CFStringRef;
    pub(crate) static kSecAttrCanEncrypt: CFStringRef;
    pub(crate) static kSecAttrCanDecrypt: CFStringRef;
    pub(crate) static kSecAttrCanDerive: CFStringRef;
    pub(crate) static kSecAttrCanSign: CFStringRef;
    pub(crate) static kSecAttrCanVerify: CFStringRef;
    pub(crate) static kSecAttrCanWrap: CFStringRef;
    pub(crate) static kSecAttrCanUnwrap: CFStringRef;
    pub(crate) static kSecAttrIsExtractable: CFStringRef;
    pub(crate) static kSecAttrIsPermanent: CFStringRef;
    pub(crate) static kSecAttrIsSensitive: CFStringRef;
    pub(crate) static kSecAttrKeyClass: CFStringRef;
    pub(crate) static kSecAttrKeyClassPublic: CFStringRef;
    pub(crate) static kSecAttrKeyClassPrivate: CFStringRef;
    pub(crate) static kSecAttrKeyClassSymmetric: CFStringRef;
    pub(crate) static kSecAttrKeyType: CFStringRef;
    #[cfg(target_os = "macos")]
    pub(crate) static kSecAttrKeyTypeAES: CFStringRef;
    pub(crate) static kSecAttrKeyTypeRSA: CFStringRef;
    pub(crate) static kSecAttrKeyTypeECSECPrimeRandom: CFStringRef;
    pub(crate) static kSecAttrKeySizeInBits: CFStringRef;
    pub(crate) static kSecAttrLabel: CFStringRef;
    pub(crate) static kSecAttrProtocol: CFStringRef;
    pub(crate) static kSecAttrProtocolFTP: CFStringRef;
    pub(crate) static kSecAttrProtocolFTPAccount: CFStringRef;
    pub(crate) static kSecAttrProtocolHTTP: CFStringRef;
    pub(crate) static kSecAttrProtocolIRC: CFStringRef;
    pub(crate) static kSecAttrProtocolNNTP: CFStringRef;
    pub(crate) static kSecAttrProtocolPOP3: CFStringRef;
    pub(crate) static kSecAttrProtocolSMTP: CFStringRef;
    pub(crate) static kSecAttrProtocolSOCKS: CFStringRef;
    pub(crate) static kSecAttrProtocolIMAP: CFStringRef;
    pub(crate) static kSecAttrProtocolLDAP: CFStringRef;
    pub(crate) static kSecAttrProtocolAppleTalk: CFStringRef;
    pub(crate) static kSecAttrProtocolAFP: CFStringRef;
    pub(crate) static kSecAttrProtocolTelnet: CFStringRef;
    pub(crate) static kSecAttrProtocolSSH: CFStringRef;
    pub(crate) static kSecAttrProtocolFTPS: CFStringRef;
    pub(crate) static kSecAttrProtocolHTTPS: CFStringRef;
    pub(crate) static kSecAttrProtocolHTTPProxy: CFStringRef;
    pub(crate) static kSecAttrProtocolHTTPSProxy: CFStringRef;
    pub(crate) static kSecAttrProtocolFTPProxy: CFStringRef;
    pub(crate) static kSecAttrProtocolSMB: CFStringRef;
    pub(crate) static kSecAttrProtocolRTSP: CFStringRef;
    pub(crate) static kSecAttrProtocolRTSPProxy: CFStringRef;
    pub(crate) static kSecAttrProtocolDAAP: CFStringRef;
    pub(crate) static kSecAttrProtocolEPPC: CFStringRef;
    pub(crate) static kSecAttrProtocolIPP: CFStringRef;
    pub(crate) static kSecAttrProtocolNNTPS: CFStringRef;
    pub(crate) static kSecAttrProtocolLDAPS: CFStringRef;
    pub(crate) static kSecAttrProtocolTelnetS: CFStringRef;
    pub(crate) static kSecAttrProtocolIMAPS: CFStringRef;
    pub(crate) static kSecAttrProtocolIRCS: CFStringRef;
    pub(crate) static kSecAttrProtocolPOP3S: CFStringRef;
    pub(crate) static kSecAttrServer: CFStringRef;
    pub(crate) static kSecAttrService: CFStringRef;
    pub(crate) static kSecAttrSynchronizable: CFStringRef;
    pub(crate) static kSecAttrTokenID: CFStringRef;
    pub(crate) static kSecAttrTokenIDSecureEnclave: CFStringRef;
    pub(crate) static kSecClass: CFStringRef;
    pub(crate) static kSecClassGenericPassword: CFStringRef;
    pub(crate) static kSecClassInternetPassword: CFStringRef;
    pub(crate) static kSecClassCertificate: CFStringRef;
    pub(crate) static kSecClassKey: CFStringRef;
    pub(crate) static kSecClassIdentity: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardX963SHA1AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardX963SHA224AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardX963SHA256AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardX963SHA384AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardX963SHA512AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA224AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA256AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA384AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA512AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA224AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA384AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA512AESGCM:
        CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA1AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA224AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA256AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA384AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA512AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureRFC4754: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureDigestX962: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureDigestX962SHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureDigestX962SHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureDigestX962SHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureDigestX962SHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureDigestX962SHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureMessageX962SHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureMessageX962SHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureMessageX962SHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureMessageX962SHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDSASignatureMessageX962SHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeCofactor: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeStandard: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionRaw: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionPKCS1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA1AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA224AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA256AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA384AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSAEncryptionOAEPSHA512AESGCM: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureRaw: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPKCS1v15Raw: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPSSSHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPSSSHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPSSSHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPSSSHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureDigestPSSSHA512: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePSSSHA1: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePSSSHA224: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePSSSHA256: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePSSSHA384: CFStringRef;
    pub(crate) static kSecKeyAlgorithmRSASignatureMessagePSSSHA512: CFStringRef;
    pub(crate) static kSecKeyAlwaysSensitive: CFStringRef;
    pub(crate) static kSecKeyDecrypt: CFStringRef;
    pub(crate) static kSecKeyDerive: CFStringRef;
    pub(crate) static kSecKeyEffectiveKeySize: CFStringRef;
    pub(crate) static kSecKeyEncrypt: CFStringRef;
    pub(crate) static kSecKeyEndDate: CFStringRef;
    pub(crate) static kSecKeyExtractable: CFStringRef;
    pub(crate) static kSecKeyKeySizeInBits: CFStringRef;
    pub(crate) static kSecKeyKeyType: CFStringRef;
    pub(crate) static kSecKeyModifiable: CFStringRef;
    pub(crate) static kSecKeyNeverExtractable: CFStringRef;
    pub(crate) static kSecKeyPermanent: CFStringRef;
    pub(crate) static kSecKeyPrivate: CFStringRef;
    pub(crate) static kSecKeySensitive: CFStringRef;
    pub(crate) static kSecKeySign: CFStringRef;
    pub(crate) static kSecKeyStartDate: CFStringRef;
    pub(crate) static kSecKeyUnwrap: CFStringRef;
    pub(crate) static kSecKeyVerify: CFStringRef;
    pub(crate) static kSecKeyWrap: CFStringRef;
    pub(crate) static kSecMatchLimit: CFStringRef;
    pub(crate) static kSecMatchLimitOne: CFStringRef;
    pub(crate) static kSecMatchLimitAll: CFStringRef;
    pub(crate) static kSecPrivateKeyAttrs: CFStringRef;
    pub(crate) static kSecReturnRef: CFStringRef;
    pub(crate) static kSecUseKeychain: CFStringRef;
    pub(crate) static kSecUseOperationPrompt: CFStringRef;
    pub(crate) static kSecValueData: CFStringRef;

    pub(crate) fn SecAccessControlCreateWithFlags(
        allocator: CFAllocatorRef,
        protection: CFTypeRef,
        flags: CFOptionFlags,
        error: *mut CFErrorRef,
    ) -> CFTypeRef;
    pub(crate) fn SecAccessControlGetTypeID() -> CFTypeID;
    pub(crate) fn SecCopyErrorMessageString(
        status: OSStatus,
        reserved: *const c_void,
    ) -> CFStringRef;
    pub(crate) fn SecItemAdd(attributes: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    pub(crate) fn SecItemDelete(attributes: CFDictionaryRef) -> OSStatus;
    pub(crate) fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    pub(crate) fn SecKeyCopyAttributes(key: KeyRef) -> CFDictionaryRef;
    pub(crate) fn SecKeyCreateWithData(
        keyData: CFDataRef,
        attributes: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> KeyRef;
    pub(crate) fn SecKeyCopyExternalRepresentation(
        key: KeyRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
    pub(crate) fn SecKeyCreateSignature(
        key: KeyRef,
        algorithm: CFTypeRef,
        data_to_sign: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
    pub(crate) fn SecKeyVerifySignature(
        key: KeyRef,
        algorithm: CFTypeRef,
        data_to_verify: CFDataRef,
        signature: CFDataRef,
        error: *mut CFErrorRef,
    ) -> u8;
    pub(crate) fn SecKeyCreateEncryptedData(
        key: KeyRef,
        algorithm: CFTypeRef,
        plaintext: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
    pub(crate) fn SecKeyCreateDecryptedData(
        key: KeyRef,
        algorithm: CFTypeRef,
        ciphertext: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
    pub(crate) fn SecKeyGeneratePair(
        parameters: CFDictionaryRef,
        publicKey: *mut KeyRef,
        privateKey: *mut KeyRef,
    ) -> OSStatus;
    pub(crate) fn SecKeyCreateRandomKey(
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> KeyRef;
    pub(crate) fn SecKeyIsAlgorithmSupported(
        key: KeyRef,
        operationType: CFIndex,
        algorithm: CFTypeRef,
    ) -> u8;
    pub(crate) fn SecKeyCopyPublicKey(privatekey: KeyRef) -> KeyRef;
    pub(crate) fn SecKeyGetTypeID() -> CFTypeID;
    pub(crate) fn SecKeychainCopyDefault(keychain: *mut KeychainRef) -> OSStatus;
    pub(crate) fn SecKeychainCreate(
        path_name: *const c_char,
        password_length: u32,
        password: *const c_char,
        prompt_user: bool,
        initial_access: CFTypeRef,
        keychain: *mut KeychainRef,
    ) -> OSStatus;
    pub(crate) fn SecKeychainDelete(keychain_or_array: KeychainRef) -> OSStatus;
    pub(crate) fn SecKeychainGetTypeID() -> CFTypeID;
    pub(crate) fn SecKeychainItemGetTypeID() -> CFTypeID;
    pub(crate) fn SecKeychainItemCopyContent(
        item_ref: ItemRef,
        itemClass: *mut FourCharacterCode,
        attr_list: *mut SecKeychainAttributeList,
        data_length: *mut u32,
        data_out: *mut *mut c_void,
    ) -> OSStatus;
    pub(crate) fn SecKeychainItemFreeContent(
        attr_list: *mut SecKeychainAttributeList,
        data: *mut c_void,
    ) -> OSStatus;
}
