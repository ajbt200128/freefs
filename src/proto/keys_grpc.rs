// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KEYS_KEY_GENERATE: ::grpcio::Method<super::keys::KeyGenerateRequest, super::keys::KeyGenerateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/KeyGenerate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_KEYS: ::grpcio::Method<super::keys::KeysRequest, super::keys::KeysResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Keys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_KEY: ::grpcio::Method<super::keys::KeyRequest, super::keys::KeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Key",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_KEY_IMPORT: ::grpcio::Method<super::keys::KeyImportRequest, super::keys::KeyImportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/KeyImport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_KEY_EXPORT: ::grpcio::Method<super::keys::KeyExportRequest, super::keys::KeyExportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/KeyExport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_KEY_REMOVE: ::grpcio::Method<super::keys::KeyRemoveRequest, super::keys::KeyRemoveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/KeyRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SIGN: ::grpcio::Method<super::keys::SignRequest, super::keys::SignResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Sign",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SIGN_FILE: ::grpcio::Method<super::keys::SignFileInput, super::keys::SignFileOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/SignFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SIGN_STREAM: ::grpcio::Method<super::keys::SignInput, super::keys::SignOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/SignStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY: ::grpcio::Method<super::keys::VerifyRequest, super::keys::VerifyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Verify",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY_FILE: ::grpcio::Method<super::keys::VerifyFileInput, super::keys::VerifyFileOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/VerifyFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY_STREAM: ::grpcio::Method<super::keys::VerifyInput, super::keys::VerifyOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/VerifyStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY_ARMORED_STREAM: ::grpcio::Method<super::keys::VerifyInput, super::keys::VerifyOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/VerifyArmoredStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY_DETACHED: ::grpcio::Method<super::keys::VerifyDetachedRequest, super::keys::VerifyDetachedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/VerifyDetached",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY_DETACHED_FILE: ::grpcio::Method<super::keys::VerifyDetachedFileInput, super::keys::VerifyDetachedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/service.Keys/VerifyDetachedFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_VERIFY_DETACHED_STREAM: ::grpcio::Method<super::keys::VerifyDetachedInput, super::keys::VerifyDetachedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/service.Keys/VerifyDetachedStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ENCRYPT: ::grpcio::Method<super::keys::EncryptRequest, super::keys::EncryptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Encrypt",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ENCRYPT_STREAM: ::grpcio::Method<super::keys::EncryptInput, super::keys::EncryptOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/EncryptStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ENCRYPT_FILE: ::grpcio::Method<super::keys::EncryptFileInput, super::keys::EncryptFileOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/EncryptFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_DECRYPT: ::grpcio::Method<super::keys::DecryptRequest, super::keys::DecryptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Decrypt",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_DECRYPT_FILE: ::grpcio::Method<super::keys::DecryptFileInput, super::keys::DecryptFileOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/DecryptFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_DECRYPT_STREAM: ::grpcio::Method<super::keys::DecryptInput, super::keys::DecryptOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/DecryptStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_DECRYPT_ARMORED_STREAM: ::grpcio::Method<super::keys::DecryptInput, super::keys::DecryptOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/DecryptArmoredStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SIGNCRYPT_OPEN_STREAM: ::grpcio::Method<super::keys::DecryptInput, super::keys::DecryptOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/SigncryptOpenStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SIGNCRYPT_OPEN_ARMORED_STREAM: ::grpcio::Method<super::keys::DecryptInput, super::keys::DecryptOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/SigncryptOpenArmoredStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SIGCHAIN: ::grpcio::Method<super::keys::SigchainRequest, super::keys::SigchainResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Sigchain",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_STATEMENT: ::grpcio::Method<super::keys::StatementRequest, super::keys::StatementResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Statement",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_STATEMENT_CREATE: ::grpcio::Method<super::keys::StatementCreateRequest, super::keys::StatementCreateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/StatementCreate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_STATEMENT_REVOKE: ::grpcio::Method<super::keys::StatementRevokeRequest, super::keys::StatementRevokeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/StatementRevoke",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_USER: ::grpcio::Method<super::keys::UserRequest, super::keys::UserResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/User",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_USER_SEARCH: ::grpcio::Method<super::keys::UserSearchRequest, super::keys::UserSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/UserSearch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_USER_SERVICE: ::grpcio::Method<super::keys::UserServiceRequest, super::keys::UserServiceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/UserService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_USER_SIGN: ::grpcio::Method<super::keys::UserSignRequest, super::keys::UserSignResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/UserSign",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_USER_ADD: ::grpcio::Method<super::keys::UserAddRequest, super::keys::UserAddResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/UserAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SEARCH: ::grpcio::Method<super::keys::SearchRequest, super::keys::SearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Search",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SECRET: ::grpcio::Method<super::keys::SecretRequest, super::keys::SecretResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Secret",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SECRET_SAVE: ::grpcio::Method<super::keys::SecretSaveRequest, super::keys::SecretSaveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/SecretSave",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SECRET_REMOVE: ::grpcio::Method<super::keys::SecretRemoveRequest, super::keys::SecretRemoveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/SecretRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_SECRETS: ::grpcio::Method<super::keys::SecretsRequest, super::keys::SecretsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Secrets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ITEM: ::grpcio::Method<super::keys::ItemRequest, super::keys::ItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Item",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ITEMS: ::grpcio::Method<super::keys::ItemsRequest, super::keys::ItemsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Items",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_PULL: ::grpcio::Method<super::keys::PullRequest, super::keys::PullResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Pull",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_PUSH: ::grpcio::Method<super::keys::PushRequest, super::keys::PushResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Push",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_WORMHOLE: ::grpcio::Method<super::keys::WormholeInput, super::keys::WormholeOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/service.Keys/Wormhole",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_PREFERENCES: ::grpcio::Method<super::keys::PreferencesRequest, super::keys::PreferencesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Preferences",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_PREFERENCE_SET: ::grpcio::Method<super::keys::PreferenceSetRequest, super::keys::PreferenceSetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/PreferenceSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_AUTH_SETUP: ::grpcio::Method<super::keys::AuthSetupRequest, super::keys::AuthSetupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/AuthSetup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_AUTH_UNLOCK: ::grpcio::Method<super::keys::AuthUnlockRequest, super::keys::AuthUnlockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/AuthUnlock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_AUTH_LOCK: ::grpcio::Method<super::keys::AuthLockRequest, super::keys::AuthLockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/AuthLock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_RUNTIME_STATUS: ::grpcio::Method<super::keys::RuntimeStatusRequest, super::keys::RuntimeStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/RuntimeStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_RAND: ::grpcio::Method<super::keys::RandRequest, super::keys::RandResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Rand",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_RAND_PASSWORD: ::grpcio::Method<super::keys::RandPasswordRequest, super::keys::RandPasswordResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/RandPassword",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_COLLECTIONS: ::grpcio::Method<super::keys::CollectionsRequest, super::keys::CollectionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Collections",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_DOCUMENTS: ::grpcio::Method<super::keys::DocumentsRequest, super::keys::DocumentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Documents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_DOCUMENT_DELETE: ::grpcio::Method<super::keys::DocumentDeleteRequest, super::keys::DocumentDeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/DocumentDelete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ADMIN_SIGN_URL: ::grpcio::Method<super::keys::AdminSignURLRequest, super::keys::AdminSignURLResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/AdminSignURL",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_ADMIN_CHECK: ::grpcio::Method<super::keys::AdminCheckRequest, super::keys::AdminCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/AdminCheck",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_MESSAGE_PREPARE: ::grpcio::Method<super::keys::MessagePrepareRequest, super::keys::MessagePrepareResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/MessagePrepare",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_MESSAGE_CREATE: ::grpcio::Method<super::keys::MessageCreateRequest, super::keys::MessageCreateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/MessageCreate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_MESSAGES: ::grpcio::Method<super::keys::MessagesRequest, super::keys::MessagesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/service.Keys/Messages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYS_WATCH: ::grpcio::Method<super::keys::WatchRequest, super::keys::WatchEvent> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/service.Keys/Watch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct KeysClient {
    client: ::grpcio::Client,
}

impl KeysClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KeysClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn key_generate_opt(&self, req: &super::keys::KeyGenerateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::KeyGenerateResponse> {
        self.client.unary_call(&METHOD_KEYS_KEY_GENERATE, req, opt)
    }

    pub fn key_generate(&self, req: &super::keys::KeyGenerateRequest) -> ::grpcio::Result<super::keys::KeyGenerateResponse> {
        self.key_generate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_generate_async_opt(&self, req: &super::keys::KeyGenerateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyGenerateResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_KEY_GENERATE, req, opt)
    }

    pub fn key_generate_async(&self, req: &super::keys::KeyGenerateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyGenerateResponse>> {
        self.key_generate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn keys_opt(&self, req: &super::keys::KeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::KeysResponse> {
        self.client.unary_call(&METHOD_KEYS_KEYS, req, opt)
    }

    pub fn keys(&self, req: &super::keys::KeysRequest) -> ::grpcio::Result<super::keys::KeysResponse> {
        self.keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn keys_async_opt(&self, req: &super::keys::KeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeysResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_KEYS, req, opt)
    }

    pub fn keys_async(&self, req: &super::keys::KeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeysResponse>> {
        self.keys_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_opt(&self, req: &super::keys::KeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::KeyResponse> {
        self.client.unary_call(&METHOD_KEYS_KEY, req, opt)
    }

    pub fn key(&self, req: &super::keys::KeyRequest) -> ::grpcio::Result<super::keys::KeyResponse> {
        self.key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_async_opt(&self, req: &super::keys::KeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_KEY, req, opt)
    }

    pub fn key_async(&self, req: &super::keys::KeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyResponse>> {
        self.key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_import_opt(&self, req: &super::keys::KeyImportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::KeyImportResponse> {
        self.client.unary_call(&METHOD_KEYS_KEY_IMPORT, req, opt)
    }

    pub fn key_import(&self, req: &super::keys::KeyImportRequest) -> ::grpcio::Result<super::keys::KeyImportResponse> {
        self.key_import_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_import_async_opt(&self, req: &super::keys::KeyImportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyImportResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_KEY_IMPORT, req, opt)
    }

    pub fn key_import_async(&self, req: &super::keys::KeyImportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyImportResponse>> {
        self.key_import_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_export_opt(&self, req: &super::keys::KeyExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::KeyExportResponse> {
        self.client.unary_call(&METHOD_KEYS_KEY_EXPORT, req, opt)
    }

    pub fn key_export(&self, req: &super::keys::KeyExportRequest) -> ::grpcio::Result<super::keys::KeyExportResponse> {
        self.key_export_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_export_async_opt(&self, req: &super::keys::KeyExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyExportResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_KEY_EXPORT, req, opt)
    }

    pub fn key_export_async(&self, req: &super::keys::KeyExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyExportResponse>> {
        self.key_export_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_remove_opt(&self, req: &super::keys::KeyRemoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::KeyRemoveResponse> {
        self.client.unary_call(&METHOD_KEYS_KEY_REMOVE, req, opt)
    }

    pub fn key_remove(&self, req: &super::keys::KeyRemoveRequest) -> ::grpcio::Result<super::keys::KeyRemoveResponse> {
        self.key_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn key_remove_async_opt(&self, req: &super::keys::KeyRemoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyRemoveResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_KEY_REMOVE, req, opt)
    }

    pub fn key_remove_async(&self, req: &super::keys::KeyRemoveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::KeyRemoveResponse>> {
        self.key_remove_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_opt(&self, req: &super::keys::SignRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SignResponse> {
        self.client.unary_call(&METHOD_KEYS_SIGN, req, opt)
    }

    pub fn sign(&self, req: &super::keys::SignRequest) -> ::grpcio::Result<super::keys::SignResponse> {
        self.sign_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_async_opt(&self, req: &super::keys::SignRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SignResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SIGN, req, opt)
    }

    pub fn sign_async(&self, req: &super::keys::SignRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SignResponse>> {
        self.sign_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::SignFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::SignFileOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_SIGN_FILE, opt)
    }

    pub fn sign_file(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::SignFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::SignFileOutput>)> {
        self.sign_file_opt(::grpcio::CallOption::default())
    }

    pub fn sign_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::SignInput>, ::grpcio::ClientDuplexReceiver<super::keys::SignOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_SIGN_STREAM, opt)
    }

    pub fn sign_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::SignInput>, ::grpcio::ClientDuplexReceiver<super::keys::SignOutput>)> {
        self.sign_stream_opt(::grpcio::CallOption::default())
    }

    pub fn verify_opt(&self, req: &super::keys::VerifyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::VerifyResponse> {
        self.client.unary_call(&METHOD_KEYS_VERIFY, req, opt)
    }

    pub fn verify(&self, req: &super::keys::VerifyRequest) -> ::grpcio::Result<super::keys::VerifyResponse> {
        self.verify_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_async_opt(&self, req: &super::keys::VerifyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::VerifyResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_VERIFY, req, opt)
    }

    pub fn verify_async(&self, req: &super::keys::VerifyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::VerifyResponse>> {
        self.verify_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::VerifyFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::VerifyFileOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_VERIFY_FILE, opt)
    }

    pub fn verify_file(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::VerifyFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::VerifyFileOutput>)> {
        self.verify_file_opt(::grpcio::CallOption::default())
    }

    pub fn verify_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::VerifyInput>, ::grpcio::ClientDuplexReceiver<super::keys::VerifyOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_VERIFY_STREAM, opt)
    }

    pub fn verify_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::VerifyInput>, ::grpcio::ClientDuplexReceiver<super::keys::VerifyOutput>)> {
        self.verify_stream_opt(::grpcio::CallOption::default())
    }

    pub fn verify_armored_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::VerifyInput>, ::grpcio::ClientDuplexReceiver<super::keys::VerifyOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_VERIFY_ARMORED_STREAM, opt)
    }

    pub fn verify_armored_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::VerifyInput>, ::grpcio::ClientDuplexReceiver<super::keys::VerifyOutput>)> {
        self.verify_armored_stream_opt(::grpcio::CallOption::default())
    }

    pub fn verify_detached_opt(&self, req: &super::keys::VerifyDetachedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::VerifyDetachedResponse> {
        self.client.unary_call(&METHOD_KEYS_VERIFY_DETACHED, req, opt)
    }

    pub fn verify_detached(&self, req: &super::keys::VerifyDetachedRequest) -> ::grpcio::Result<super::keys::VerifyDetachedResponse> {
        self.verify_detached_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_detached_async_opt(&self, req: &super::keys::VerifyDetachedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::VerifyDetachedResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_VERIFY_DETACHED, req, opt)
    }

    pub fn verify_detached_async(&self, req: &super::keys::VerifyDetachedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::VerifyDetachedResponse>> {
        self.verify_detached_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_detached_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::keys::VerifyDetachedFileInput>, ::grpcio::ClientCStreamReceiver<super::keys::VerifyDetachedResponse>)> {
        self.client.client_streaming(&METHOD_KEYS_VERIFY_DETACHED_FILE, opt)
    }

    pub fn verify_detached_file(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::keys::VerifyDetachedFileInput>, ::grpcio::ClientCStreamReceiver<super::keys::VerifyDetachedResponse>)> {
        self.verify_detached_file_opt(::grpcio::CallOption::default())
    }

    pub fn verify_detached_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::keys::VerifyDetachedInput>, ::grpcio::ClientCStreamReceiver<super::keys::VerifyDetachedResponse>)> {
        self.client.client_streaming(&METHOD_KEYS_VERIFY_DETACHED_STREAM, opt)
    }

    pub fn verify_detached_stream(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::keys::VerifyDetachedInput>, ::grpcio::ClientCStreamReceiver<super::keys::VerifyDetachedResponse>)> {
        self.verify_detached_stream_opt(::grpcio::CallOption::default())
    }

    pub fn encrypt_opt(&self, req: &super::keys::EncryptRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::EncryptResponse> {
        self.client.unary_call(&METHOD_KEYS_ENCRYPT, req, opt)
    }

    pub fn encrypt(&self, req: &super::keys::EncryptRequest) -> ::grpcio::Result<super::keys::EncryptResponse> {
        self.encrypt_opt(req, ::grpcio::CallOption::default())
    }

    pub fn encrypt_async_opt(&self, req: &super::keys::EncryptRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::EncryptResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_ENCRYPT, req, opt)
    }

    pub fn encrypt_async(&self, req: &super::keys::EncryptRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::EncryptResponse>> {
        self.encrypt_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn encrypt_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::EncryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::EncryptOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_ENCRYPT_STREAM, opt)
    }

    pub fn encrypt_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::EncryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::EncryptOutput>)> {
        self.encrypt_stream_opt(::grpcio::CallOption::default())
    }

    pub fn encrypt_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::EncryptFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::EncryptFileOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_ENCRYPT_FILE, opt)
    }

    pub fn encrypt_file(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::EncryptFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::EncryptFileOutput>)> {
        self.encrypt_file_opt(::grpcio::CallOption::default())
    }

    pub fn decrypt_opt(&self, req: &super::keys::DecryptRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::DecryptResponse> {
        self.client.unary_call(&METHOD_KEYS_DECRYPT, req, opt)
    }

    pub fn decrypt(&self, req: &super::keys::DecryptRequest) -> ::grpcio::Result<super::keys::DecryptResponse> {
        self.decrypt_opt(req, ::grpcio::CallOption::default())
    }

    pub fn decrypt_async_opt(&self, req: &super::keys::DecryptRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::DecryptResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_DECRYPT, req, opt)
    }

    pub fn decrypt_async(&self, req: &super::keys::DecryptRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::DecryptResponse>> {
        self.decrypt_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn decrypt_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptFileOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_DECRYPT_FILE, opt)
    }

    pub fn decrypt_file(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptFileInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptFileOutput>)> {
        self.decrypt_file_opt(::grpcio::CallOption::default())
    }

    pub fn decrypt_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_DECRYPT_STREAM, opt)
    }

    pub fn decrypt_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.decrypt_stream_opt(::grpcio::CallOption::default())
    }

    pub fn decrypt_armored_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_DECRYPT_ARMORED_STREAM, opt)
    }

    pub fn decrypt_armored_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.decrypt_armored_stream_opt(::grpcio::CallOption::default())
    }

    pub fn signcrypt_open_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_SIGNCRYPT_OPEN_STREAM, opt)
    }

    pub fn signcrypt_open_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.signcrypt_open_stream_opt(::grpcio::CallOption::default())
    }

    pub fn signcrypt_open_armored_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_SIGNCRYPT_OPEN_ARMORED_STREAM, opt)
    }

    pub fn signcrypt_open_armored_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::DecryptInput>, ::grpcio::ClientDuplexReceiver<super::keys::DecryptOutput>)> {
        self.signcrypt_open_armored_stream_opt(::grpcio::CallOption::default())
    }

    pub fn sigchain_opt(&self, req: &super::keys::SigchainRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SigchainResponse> {
        self.client.unary_call(&METHOD_KEYS_SIGCHAIN, req, opt)
    }

    pub fn sigchain(&self, req: &super::keys::SigchainRequest) -> ::grpcio::Result<super::keys::SigchainResponse> {
        self.sigchain_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sigchain_async_opt(&self, req: &super::keys::SigchainRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SigchainResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SIGCHAIN, req, opt)
    }

    pub fn sigchain_async(&self, req: &super::keys::SigchainRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SigchainResponse>> {
        self.sigchain_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn statement_opt(&self, req: &super::keys::StatementRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::StatementResponse> {
        self.client.unary_call(&METHOD_KEYS_STATEMENT, req, opt)
    }

    pub fn statement(&self, req: &super::keys::StatementRequest) -> ::grpcio::Result<super::keys::StatementResponse> {
        self.statement_opt(req, ::grpcio::CallOption::default())
    }

    pub fn statement_async_opt(&self, req: &super::keys::StatementRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::StatementResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_STATEMENT, req, opt)
    }

    pub fn statement_async(&self, req: &super::keys::StatementRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::StatementResponse>> {
        self.statement_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn statement_create_opt(&self, req: &super::keys::StatementCreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::StatementCreateResponse> {
        self.client.unary_call(&METHOD_KEYS_STATEMENT_CREATE, req, opt)
    }

    pub fn statement_create(&self, req: &super::keys::StatementCreateRequest) -> ::grpcio::Result<super::keys::StatementCreateResponse> {
        self.statement_create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn statement_create_async_opt(&self, req: &super::keys::StatementCreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::StatementCreateResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_STATEMENT_CREATE, req, opt)
    }

    pub fn statement_create_async(&self, req: &super::keys::StatementCreateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::StatementCreateResponse>> {
        self.statement_create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn statement_revoke_opt(&self, req: &super::keys::StatementRevokeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::StatementRevokeResponse> {
        self.client.unary_call(&METHOD_KEYS_STATEMENT_REVOKE, req, opt)
    }

    pub fn statement_revoke(&self, req: &super::keys::StatementRevokeRequest) -> ::grpcio::Result<super::keys::StatementRevokeResponse> {
        self.statement_revoke_opt(req, ::grpcio::CallOption::default())
    }

    pub fn statement_revoke_async_opt(&self, req: &super::keys::StatementRevokeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::StatementRevokeResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_STATEMENT_REVOKE, req, opt)
    }

    pub fn statement_revoke_async(&self, req: &super::keys::StatementRevokeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::StatementRevokeResponse>> {
        self.statement_revoke_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_opt(&self, req: &super::keys::UserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::UserResponse> {
        self.client.unary_call(&METHOD_KEYS_USER, req, opt)
    }

    pub fn user(&self, req: &super::keys::UserRequest) -> ::grpcio::Result<super::keys::UserResponse> {
        self.user_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_async_opt(&self, req: &super::keys::UserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_USER, req, opt)
    }

    pub fn user_async(&self, req: &super::keys::UserRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserResponse>> {
        self.user_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_search_opt(&self, req: &super::keys::UserSearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::UserSearchResponse> {
        self.client.unary_call(&METHOD_KEYS_USER_SEARCH, req, opt)
    }

    pub fn user_search(&self, req: &super::keys::UserSearchRequest) -> ::grpcio::Result<super::keys::UserSearchResponse> {
        self.user_search_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_search_async_opt(&self, req: &super::keys::UserSearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserSearchResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_USER_SEARCH, req, opt)
    }

    pub fn user_search_async(&self, req: &super::keys::UserSearchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserSearchResponse>> {
        self.user_search_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_service_opt(&self, req: &super::keys::UserServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::UserServiceResponse> {
        self.client.unary_call(&METHOD_KEYS_USER_SERVICE, req, opt)
    }

    pub fn user_service(&self, req: &super::keys::UserServiceRequest) -> ::grpcio::Result<super::keys::UserServiceResponse> {
        self.user_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_service_async_opt(&self, req: &super::keys::UserServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserServiceResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_USER_SERVICE, req, opt)
    }

    pub fn user_service_async(&self, req: &super::keys::UserServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserServiceResponse>> {
        self.user_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_sign_opt(&self, req: &super::keys::UserSignRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::UserSignResponse> {
        self.client.unary_call(&METHOD_KEYS_USER_SIGN, req, opt)
    }

    pub fn user_sign(&self, req: &super::keys::UserSignRequest) -> ::grpcio::Result<super::keys::UserSignResponse> {
        self.user_sign_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_sign_async_opt(&self, req: &super::keys::UserSignRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserSignResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_USER_SIGN, req, opt)
    }

    pub fn user_sign_async(&self, req: &super::keys::UserSignRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserSignResponse>> {
        self.user_sign_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_add_opt(&self, req: &super::keys::UserAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::UserAddResponse> {
        self.client.unary_call(&METHOD_KEYS_USER_ADD, req, opt)
    }

    pub fn user_add(&self, req: &super::keys::UserAddRequest) -> ::grpcio::Result<super::keys::UserAddResponse> {
        self.user_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_add_async_opt(&self, req: &super::keys::UserAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserAddResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_USER_ADD, req, opt)
    }

    pub fn user_add_async(&self, req: &super::keys::UserAddRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::UserAddResponse>> {
        self.user_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_opt(&self, req: &super::keys::SearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SearchResponse> {
        self.client.unary_call(&METHOD_KEYS_SEARCH, req, opt)
    }

    pub fn search(&self, req: &super::keys::SearchRequest) -> ::grpcio::Result<super::keys::SearchResponse> {
        self.search_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_async_opt(&self, req: &super::keys::SearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SearchResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SEARCH, req, opt)
    }

    pub fn search_async(&self, req: &super::keys::SearchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SearchResponse>> {
        self.search_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secret_opt(&self, req: &super::keys::SecretRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SecretResponse> {
        self.client.unary_call(&METHOD_KEYS_SECRET, req, opt)
    }

    pub fn secret(&self, req: &super::keys::SecretRequest) -> ::grpcio::Result<super::keys::SecretResponse> {
        self.secret_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secret_async_opt(&self, req: &super::keys::SecretRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SECRET, req, opt)
    }

    pub fn secret_async(&self, req: &super::keys::SecretRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretResponse>> {
        self.secret_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secret_save_opt(&self, req: &super::keys::SecretSaveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SecretSaveResponse> {
        self.client.unary_call(&METHOD_KEYS_SECRET_SAVE, req, opt)
    }

    pub fn secret_save(&self, req: &super::keys::SecretSaveRequest) -> ::grpcio::Result<super::keys::SecretSaveResponse> {
        self.secret_save_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secret_save_async_opt(&self, req: &super::keys::SecretSaveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretSaveResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SECRET_SAVE, req, opt)
    }

    pub fn secret_save_async(&self, req: &super::keys::SecretSaveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretSaveResponse>> {
        self.secret_save_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secret_remove_opt(&self, req: &super::keys::SecretRemoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SecretRemoveResponse> {
        self.client.unary_call(&METHOD_KEYS_SECRET_REMOVE, req, opt)
    }

    pub fn secret_remove(&self, req: &super::keys::SecretRemoveRequest) -> ::grpcio::Result<super::keys::SecretRemoveResponse> {
        self.secret_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secret_remove_async_opt(&self, req: &super::keys::SecretRemoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretRemoveResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SECRET_REMOVE, req, opt)
    }

    pub fn secret_remove_async(&self, req: &super::keys::SecretRemoveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretRemoveResponse>> {
        self.secret_remove_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secrets_opt(&self, req: &super::keys::SecretsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::SecretsResponse> {
        self.client.unary_call(&METHOD_KEYS_SECRETS, req, opt)
    }

    pub fn secrets(&self, req: &super::keys::SecretsRequest) -> ::grpcio::Result<super::keys::SecretsResponse> {
        self.secrets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn secrets_async_opt(&self, req: &super::keys::SecretsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretsResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_SECRETS, req, opt)
    }

    pub fn secrets_async(&self, req: &super::keys::SecretsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::SecretsResponse>> {
        self.secrets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn item_opt(&self, req: &super::keys::ItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::ItemResponse> {
        self.client.unary_call(&METHOD_KEYS_ITEM, req, opt)
    }

    pub fn item(&self, req: &super::keys::ItemRequest) -> ::grpcio::Result<super::keys::ItemResponse> {
        self.item_opt(req, ::grpcio::CallOption::default())
    }

    pub fn item_async_opt(&self, req: &super::keys::ItemRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::ItemResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_ITEM, req, opt)
    }

    pub fn item_async(&self, req: &super::keys::ItemRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::ItemResponse>> {
        self.item_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn items_opt(&self, req: &super::keys::ItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::ItemsResponse> {
        self.client.unary_call(&METHOD_KEYS_ITEMS, req, opt)
    }

    pub fn items(&self, req: &super::keys::ItemsRequest) -> ::grpcio::Result<super::keys::ItemsResponse> {
        self.items_opt(req, ::grpcio::CallOption::default())
    }

    pub fn items_async_opt(&self, req: &super::keys::ItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::ItemsResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_ITEMS, req, opt)
    }

    pub fn items_async(&self, req: &super::keys::ItemsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::ItemsResponse>> {
        self.items_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pull_opt(&self, req: &super::keys::PullRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::PullResponse> {
        self.client.unary_call(&METHOD_KEYS_PULL, req, opt)
    }

    pub fn pull(&self, req: &super::keys::PullRequest) -> ::grpcio::Result<super::keys::PullResponse> {
        self.pull_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pull_async_opt(&self, req: &super::keys::PullRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PullResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_PULL, req, opt)
    }

    pub fn pull_async(&self, req: &super::keys::PullRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PullResponse>> {
        self.pull_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn push_opt(&self, req: &super::keys::PushRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::PushResponse> {
        self.client.unary_call(&METHOD_KEYS_PUSH, req, opt)
    }

    pub fn push(&self, req: &super::keys::PushRequest) -> ::grpcio::Result<super::keys::PushResponse> {
        self.push_opt(req, ::grpcio::CallOption::default())
    }

    pub fn push_async_opt(&self, req: &super::keys::PushRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PushResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_PUSH, req, opt)
    }

    pub fn push_async(&self, req: &super::keys::PushRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PushResponse>> {
        self.push_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wormhole_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::WormholeInput>, ::grpcio::ClientDuplexReceiver<super::keys::WormholeOutput>)> {
        self.client.duplex_streaming(&METHOD_KEYS_WORMHOLE, opt)
    }

    pub fn wormhole(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::keys::WormholeInput>, ::grpcio::ClientDuplexReceiver<super::keys::WormholeOutput>)> {
        self.wormhole_opt(::grpcio::CallOption::default())
    }

    pub fn preferences_opt(&self, req: &super::keys::PreferencesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::PreferencesResponse> {
        self.client.unary_call(&METHOD_KEYS_PREFERENCES, req, opt)
    }

    pub fn preferences(&self, req: &super::keys::PreferencesRequest) -> ::grpcio::Result<super::keys::PreferencesResponse> {
        self.preferences_opt(req, ::grpcio::CallOption::default())
    }

    pub fn preferences_async_opt(&self, req: &super::keys::PreferencesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PreferencesResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_PREFERENCES, req, opt)
    }

    pub fn preferences_async(&self, req: &super::keys::PreferencesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PreferencesResponse>> {
        self.preferences_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn preference_set_opt(&self, req: &super::keys::PreferenceSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::PreferenceSetResponse> {
        self.client.unary_call(&METHOD_KEYS_PREFERENCE_SET, req, opt)
    }

    pub fn preference_set(&self, req: &super::keys::PreferenceSetRequest) -> ::grpcio::Result<super::keys::PreferenceSetResponse> {
        self.preference_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn preference_set_async_opt(&self, req: &super::keys::PreferenceSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PreferenceSetResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_PREFERENCE_SET, req, opt)
    }

    pub fn preference_set_async(&self, req: &super::keys::PreferenceSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::PreferenceSetResponse>> {
        self.preference_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_setup_opt(&self, req: &super::keys::AuthSetupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::AuthSetupResponse> {
        self.client.unary_call(&METHOD_KEYS_AUTH_SETUP, req, opt)
    }

    pub fn auth_setup(&self, req: &super::keys::AuthSetupRequest) -> ::grpcio::Result<super::keys::AuthSetupResponse> {
        self.auth_setup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_setup_async_opt(&self, req: &super::keys::AuthSetupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AuthSetupResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_AUTH_SETUP, req, opt)
    }

    pub fn auth_setup_async(&self, req: &super::keys::AuthSetupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AuthSetupResponse>> {
        self.auth_setup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_unlock_opt(&self, req: &super::keys::AuthUnlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::AuthUnlockResponse> {
        self.client.unary_call(&METHOD_KEYS_AUTH_UNLOCK, req, opt)
    }

    pub fn auth_unlock(&self, req: &super::keys::AuthUnlockRequest) -> ::grpcio::Result<super::keys::AuthUnlockResponse> {
        self.auth_unlock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_unlock_async_opt(&self, req: &super::keys::AuthUnlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AuthUnlockResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_AUTH_UNLOCK, req, opt)
    }

    pub fn auth_unlock_async(&self, req: &super::keys::AuthUnlockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AuthUnlockResponse>> {
        self.auth_unlock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_lock_opt(&self, req: &super::keys::AuthLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::AuthLockResponse> {
        self.client.unary_call(&METHOD_KEYS_AUTH_LOCK, req, opt)
    }

    pub fn auth_lock(&self, req: &super::keys::AuthLockRequest) -> ::grpcio::Result<super::keys::AuthLockResponse> {
        self.auth_lock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_lock_async_opt(&self, req: &super::keys::AuthLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AuthLockResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_AUTH_LOCK, req, opt)
    }

    pub fn auth_lock_async(&self, req: &super::keys::AuthLockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AuthLockResponse>> {
        self.auth_lock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn runtime_status_opt(&self, req: &super::keys::RuntimeStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::RuntimeStatusResponse> {
        self.client.unary_call(&METHOD_KEYS_RUNTIME_STATUS, req, opt)
    }

    pub fn runtime_status(&self, req: &super::keys::RuntimeStatusRequest) -> ::grpcio::Result<super::keys::RuntimeStatusResponse> {
        self.runtime_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn runtime_status_async_opt(&self, req: &super::keys::RuntimeStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::RuntimeStatusResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_RUNTIME_STATUS, req, opt)
    }

    pub fn runtime_status_async(&self, req: &super::keys::RuntimeStatusRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::RuntimeStatusResponse>> {
        self.runtime_status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rand_opt(&self, req: &super::keys::RandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::RandResponse> {
        self.client.unary_call(&METHOD_KEYS_RAND, req, opt)
    }

    pub fn rand(&self, req: &super::keys::RandRequest) -> ::grpcio::Result<super::keys::RandResponse> {
        self.rand_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rand_async_opt(&self, req: &super::keys::RandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::RandResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_RAND, req, opt)
    }

    pub fn rand_async(&self, req: &super::keys::RandRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::RandResponse>> {
        self.rand_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rand_password_opt(&self, req: &super::keys::RandPasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::RandPasswordResponse> {
        self.client.unary_call(&METHOD_KEYS_RAND_PASSWORD, req, opt)
    }

    pub fn rand_password(&self, req: &super::keys::RandPasswordRequest) -> ::grpcio::Result<super::keys::RandPasswordResponse> {
        self.rand_password_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rand_password_async_opt(&self, req: &super::keys::RandPasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::RandPasswordResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_RAND_PASSWORD, req, opt)
    }

    pub fn rand_password_async(&self, req: &super::keys::RandPasswordRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::RandPasswordResponse>> {
        self.rand_password_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn collections_opt(&self, req: &super::keys::CollectionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::CollectionsResponse> {
        self.client.unary_call(&METHOD_KEYS_COLLECTIONS, req, opt)
    }

    pub fn collections(&self, req: &super::keys::CollectionsRequest) -> ::grpcio::Result<super::keys::CollectionsResponse> {
        self.collections_opt(req, ::grpcio::CallOption::default())
    }

    pub fn collections_async_opt(&self, req: &super::keys::CollectionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::CollectionsResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_COLLECTIONS, req, opt)
    }

    pub fn collections_async(&self, req: &super::keys::CollectionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::CollectionsResponse>> {
        self.collections_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn documents_opt(&self, req: &super::keys::DocumentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::DocumentsResponse> {
        self.client.unary_call(&METHOD_KEYS_DOCUMENTS, req, opt)
    }

    pub fn documents(&self, req: &super::keys::DocumentsRequest) -> ::grpcio::Result<super::keys::DocumentsResponse> {
        self.documents_opt(req, ::grpcio::CallOption::default())
    }

    pub fn documents_async_opt(&self, req: &super::keys::DocumentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::DocumentsResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_DOCUMENTS, req, opt)
    }

    pub fn documents_async(&self, req: &super::keys::DocumentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::DocumentsResponse>> {
        self.documents_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn document_delete_opt(&self, req: &super::keys::DocumentDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::DocumentDeleteResponse> {
        self.client.unary_call(&METHOD_KEYS_DOCUMENT_DELETE, req, opt)
    }

    pub fn document_delete(&self, req: &super::keys::DocumentDeleteRequest) -> ::grpcio::Result<super::keys::DocumentDeleteResponse> {
        self.document_delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn document_delete_async_opt(&self, req: &super::keys::DocumentDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::DocumentDeleteResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_DOCUMENT_DELETE, req, opt)
    }

    pub fn document_delete_async(&self, req: &super::keys::DocumentDeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::DocumentDeleteResponse>> {
        self.document_delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn admin_sign_url_opt(&self, req: &super::keys::AdminSignURLRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::AdminSignURLResponse> {
        self.client.unary_call(&METHOD_KEYS_ADMIN_SIGN_URL, req, opt)
    }

    pub fn admin_sign_url(&self, req: &super::keys::AdminSignURLRequest) -> ::grpcio::Result<super::keys::AdminSignURLResponse> {
        self.admin_sign_url_opt(req, ::grpcio::CallOption::default())
    }

    pub fn admin_sign_url_async_opt(&self, req: &super::keys::AdminSignURLRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AdminSignURLResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_ADMIN_SIGN_URL, req, opt)
    }

    pub fn admin_sign_url_async(&self, req: &super::keys::AdminSignURLRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AdminSignURLResponse>> {
        self.admin_sign_url_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn admin_check_opt(&self, req: &super::keys::AdminCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::AdminCheckResponse> {
        self.client.unary_call(&METHOD_KEYS_ADMIN_CHECK, req, opt)
    }

    pub fn admin_check(&self, req: &super::keys::AdminCheckRequest) -> ::grpcio::Result<super::keys::AdminCheckResponse> {
        self.admin_check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn admin_check_async_opt(&self, req: &super::keys::AdminCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AdminCheckResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_ADMIN_CHECK, req, opt)
    }

    pub fn admin_check_async(&self, req: &super::keys::AdminCheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::AdminCheckResponse>> {
        self.admin_check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn message_prepare_opt(&self, req: &super::keys::MessagePrepareRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::MessagePrepareResponse> {
        self.client.unary_call(&METHOD_KEYS_MESSAGE_PREPARE, req, opt)
    }

    pub fn message_prepare(&self, req: &super::keys::MessagePrepareRequest) -> ::grpcio::Result<super::keys::MessagePrepareResponse> {
        self.message_prepare_opt(req, ::grpcio::CallOption::default())
    }

    pub fn message_prepare_async_opt(&self, req: &super::keys::MessagePrepareRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::MessagePrepareResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_MESSAGE_PREPARE, req, opt)
    }

    pub fn message_prepare_async(&self, req: &super::keys::MessagePrepareRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::MessagePrepareResponse>> {
        self.message_prepare_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn message_create_opt(&self, req: &super::keys::MessageCreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::MessageCreateResponse> {
        self.client.unary_call(&METHOD_KEYS_MESSAGE_CREATE, req, opt)
    }

    pub fn message_create(&self, req: &super::keys::MessageCreateRequest) -> ::grpcio::Result<super::keys::MessageCreateResponse> {
        self.message_create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn message_create_async_opt(&self, req: &super::keys::MessageCreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::MessageCreateResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_MESSAGE_CREATE, req, opt)
    }

    pub fn message_create_async(&self, req: &super::keys::MessageCreateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::MessageCreateResponse>> {
        self.message_create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn messages_opt(&self, req: &super::keys::MessagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keys::MessagesResponse> {
        self.client.unary_call(&METHOD_KEYS_MESSAGES, req, opt)
    }

    pub fn messages(&self, req: &super::keys::MessagesRequest) -> ::grpcio::Result<super::keys::MessagesResponse> {
        self.messages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn messages_async_opt(&self, req: &super::keys::MessagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::MessagesResponse>> {
        self.client.unary_call_async(&METHOD_KEYS_MESSAGES, req, opt)
    }

    pub fn messages_async(&self, req: &super::keys::MessagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keys::MessagesResponse>> {
        self.messages_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn watch_opt(&self, req: &super::keys::WatchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::keys::WatchEvent>> {
        self.client.server_streaming(&METHOD_KEYS_WATCH, req, opt)
    }

    pub fn watch(&self, req: &super::keys::WatchRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::keys::WatchEvent>> {
        self.watch_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Keys {
    fn key_generate(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::KeyGenerateRequest, sink: ::grpcio::UnarySink<super::keys::KeyGenerateResponse>);
    fn keys(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::KeysRequest, sink: ::grpcio::UnarySink<super::keys::KeysResponse>);
    fn key(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::KeyRequest, sink: ::grpcio::UnarySink<super::keys::KeyResponse>);
    fn key_import(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::KeyImportRequest, sink: ::grpcio::UnarySink<super::keys::KeyImportResponse>);
    fn key_export(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::KeyExportRequest, sink: ::grpcio::UnarySink<super::keys::KeyExportResponse>);
    fn key_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::KeyRemoveRequest, sink: ::grpcio::UnarySink<super::keys::KeyRemoveResponse>);
    fn sign(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SignRequest, sink: ::grpcio::UnarySink<super::keys::SignResponse>);
    fn sign_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::SignFileInput>, sink: ::grpcio::DuplexSink<super::keys::SignFileOutput>);
    fn sign_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::SignInput>, sink: ::grpcio::DuplexSink<super::keys::SignOutput>);
    fn verify(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::VerifyRequest, sink: ::grpcio::UnarySink<super::keys::VerifyResponse>);
    fn verify_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::VerifyFileInput>, sink: ::grpcio::DuplexSink<super::keys::VerifyFileOutput>);
    fn verify_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::VerifyInput>, sink: ::grpcio::DuplexSink<super::keys::VerifyOutput>);
    fn verify_armored_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::VerifyInput>, sink: ::grpcio::DuplexSink<super::keys::VerifyOutput>);
    fn verify_detached(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::VerifyDetachedRequest, sink: ::grpcio::UnarySink<super::keys::VerifyDetachedResponse>);
    fn verify_detached_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::VerifyDetachedFileInput>, sink: ::grpcio::ClientStreamingSink<super::keys::VerifyDetachedResponse>);
    fn verify_detached_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::VerifyDetachedInput>, sink: ::grpcio::ClientStreamingSink<super::keys::VerifyDetachedResponse>);
    fn encrypt(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::EncryptRequest, sink: ::grpcio::UnarySink<super::keys::EncryptResponse>);
    fn encrypt_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::EncryptInput>, sink: ::grpcio::DuplexSink<super::keys::EncryptOutput>);
    fn encrypt_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::EncryptFileInput>, sink: ::grpcio::DuplexSink<super::keys::EncryptFileOutput>);
    fn decrypt(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::DecryptRequest, sink: ::grpcio::UnarySink<super::keys::DecryptResponse>);
    fn decrypt_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::DecryptFileInput>, sink: ::grpcio::DuplexSink<super::keys::DecryptFileOutput>);
    fn decrypt_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::DecryptInput>, sink: ::grpcio::DuplexSink<super::keys::DecryptOutput>);
    fn decrypt_armored_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::DecryptInput>, sink: ::grpcio::DuplexSink<super::keys::DecryptOutput>);
    fn signcrypt_open_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::DecryptInput>, sink: ::grpcio::DuplexSink<super::keys::DecryptOutput>);
    fn signcrypt_open_armored_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::DecryptInput>, sink: ::grpcio::DuplexSink<super::keys::DecryptOutput>);
    fn sigchain(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SigchainRequest, sink: ::grpcio::UnarySink<super::keys::SigchainResponse>);
    fn statement(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::StatementRequest, sink: ::grpcio::UnarySink<super::keys::StatementResponse>);
    fn statement_create(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::StatementCreateRequest, sink: ::grpcio::UnarySink<super::keys::StatementCreateResponse>);
    fn statement_revoke(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::StatementRevokeRequest, sink: ::grpcio::UnarySink<super::keys::StatementRevokeResponse>);
    fn user(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::UserRequest, sink: ::grpcio::UnarySink<super::keys::UserResponse>);
    fn user_search(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::UserSearchRequest, sink: ::grpcio::UnarySink<super::keys::UserSearchResponse>);
    fn user_service(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::UserServiceRequest, sink: ::grpcio::UnarySink<super::keys::UserServiceResponse>);
    fn user_sign(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::UserSignRequest, sink: ::grpcio::UnarySink<super::keys::UserSignResponse>);
    fn user_add(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::UserAddRequest, sink: ::grpcio::UnarySink<super::keys::UserAddResponse>);
    fn search(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SearchRequest, sink: ::grpcio::UnarySink<super::keys::SearchResponse>);
    fn secret(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SecretRequest, sink: ::grpcio::UnarySink<super::keys::SecretResponse>);
    fn secret_save(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SecretSaveRequest, sink: ::grpcio::UnarySink<super::keys::SecretSaveResponse>);
    fn secret_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SecretRemoveRequest, sink: ::grpcio::UnarySink<super::keys::SecretRemoveResponse>);
    fn secrets(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::SecretsRequest, sink: ::grpcio::UnarySink<super::keys::SecretsResponse>);
    fn item(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::ItemRequest, sink: ::grpcio::UnarySink<super::keys::ItemResponse>);
    fn items(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::ItemsRequest, sink: ::grpcio::UnarySink<super::keys::ItemsResponse>);
    fn pull(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::PullRequest, sink: ::grpcio::UnarySink<super::keys::PullResponse>);
    fn push(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::PushRequest, sink: ::grpcio::UnarySink<super::keys::PushResponse>);
    fn wormhole(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::keys::WormholeInput>, sink: ::grpcio::DuplexSink<super::keys::WormholeOutput>);
    fn preferences(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::PreferencesRequest, sink: ::grpcio::UnarySink<super::keys::PreferencesResponse>);
    fn preference_set(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::PreferenceSetRequest, sink: ::grpcio::UnarySink<super::keys::PreferenceSetResponse>);
    fn auth_setup(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::AuthSetupRequest, sink: ::grpcio::UnarySink<super::keys::AuthSetupResponse>);
    fn auth_unlock(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::AuthUnlockRequest, sink: ::grpcio::UnarySink<super::keys::AuthUnlockResponse>);
    fn auth_lock(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::AuthLockRequest, sink: ::grpcio::UnarySink<super::keys::AuthLockResponse>);
    fn runtime_status(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::RuntimeStatusRequest, sink: ::grpcio::UnarySink<super::keys::RuntimeStatusResponse>);
    fn rand(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::RandRequest, sink: ::grpcio::UnarySink<super::keys::RandResponse>);
    fn rand_password(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::RandPasswordRequest, sink: ::grpcio::UnarySink<super::keys::RandPasswordResponse>);
    fn collections(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::CollectionsRequest, sink: ::grpcio::UnarySink<super::keys::CollectionsResponse>);
    fn documents(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::DocumentsRequest, sink: ::grpcio::UnarySink<super::keys::DocumentsResponse>);
    fn document_delete(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::DocumentDeleteRequest, sink: ::grpcio::UnarySink<super::keys::DocumentDeleteResponse>);
    fn admin_sign_url(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::AdminSignURLRequest, sink: ::grpcio::UnarySink<super::keys::AdminSignURLResponse>);
    fn admin_check(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::AdminCheckRequest, sink: ::grpcio::UnarySink<super::keys::AdminCheckResponse>);
    fn message_prepare(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::MessagePrepareRequest, sink: ::grpcio::UnarySink<super::keys::MessagePrepareResponse>);
    fn message_create(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::MessageCreateRequest, sink: ::grpcio::UnarySink<super::keys::MessageCreateResponse>);
    fn messages(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::MessagesRequest, sink: ::grpcio::UnarySink<super::keys::MessagesResponse>);
    fn watch(&mut self, ctx: ::grpcio::RpcContext, req: super::keys::WatchRequest, sink: ::grpcio::ServerStreamingSink<super::keys::WatchEvent>);
}

pub fn create_keys<S: Keys + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_KEY_GENERATE, move |ctx, req, resp| {
        instance.key_generate(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_KEYS, move |ctx, req, resp| {
        instance.keys(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_KEY, move |ctx, req, resp| {
        instance.key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_KEY_IMPORT, move |ctx, req, resp| {
        instance.key_import(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_KEY_EXPORT, move |ctx, req, resp| {
        instance.key_export(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_KEY_REMOVE, move |ctx, req, resp| {
        instance.key_remove(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SIGN, move |ctx, req, resp| {
        instance.sign(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_SIGN_FILE, move |ctx, req, resp| {
        instance.sign_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_SIGN_STREAM, move |ctx, req, resp| {
        instance.sign_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_VERIFY, move |ctx, req, resp| {
        instance.verify(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_VERIFY_FILE, move |ctx, req, resp| {
        instance.verify_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_VERIFY_STREAM, move |ctx, req, resp| {
        instance.verify_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_VERIFY_ARMORED_STREAM, move |ctx, req, resp| {
        instance.verify_armored_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_VERIFY_DETACHED, move |ctx, req, resp| {
        instance.verify_detached(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_KEYS_VERIFY_DETACHED_FILE, move |ctx, req, resp| {
        instance.verify_detached_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_KEYS_VERIFY_DETACHED_STREAM, move |ctx, req, resp| {
        instance.verify_detached_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_ENCRYPT, move |ctx, req, resp| {
        instance.encrypt(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_ENCRYPT_STREAM, move |ctx, req, resp| {
        instance.encrypt_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_ENCRYPT_FILE, move |ctx, req, resp| {
        instance.encrypt_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_DECRYPT, move |ctx, req, resp| {
        instance.decrypt(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_DECRYPT_FILE, move |ctx, req, resp| {
        instance.decrypt_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_DECRYPT_STREAM, move |ctx, req, resp| {
        instance.decrypt_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_DECRYPT_ARMORED_STREAM, move |ctx, req, resp| {
        instance.decrypt_armored_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_SIGNCRYPT_OPEN_STREAM, move |ctx, req, resp| {
        instance.signcrypt_open_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_SIGNCRYPT_OPEN_ARMORED_STREAM, move |ctx, req, resp| {
        instance.signcrypt_open_armored_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SIGCHAIN, move |ctx, req, resp| {
        instance.sigchain(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_STATEMENT, move |ctx, req, resp| {
        instance.statement(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_STATEMENT_CREATE, move |ctx, req, resp| {
        instance.statement_create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_STATEMENT_REVOKE, move |ctx, req, resp| {
        instance.statement_revoke(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_USER, move |ctx, req, resp| {
        instance.user(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_USER_SEARCH, move |ctx, req, resp| {
        instance.user_search(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_USER_SERVICE, move |ctx, req, resp| {
        instance.user_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_USER_SIGN, move |ctx, req, resp| {
        instance.user_sign(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_USER_ADD, move |ctx, req, resp| {
        instance.user_add(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SEARCH, move |ctx, req, resp| {
        instance.search(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SECRET, move |ctx, req, resp| {
        instance.secret(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SECRET_SAVE, move |ctx, req, resp| {
        instance.secret_save(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SECRET_REMOVE, move |ctx, req, resp| {
        instance.secret_remove(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_SECRETS, move |ctx, req, resp| {
        instance.secrets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_ITEM, move |ctx, req, resp| {
        instance.item(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_ITEMS, move |ctx, req, resp| {
        instance.items(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_PULL, move |ctx, req, resp| {
        instance.pull(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_PUSH, move |ctx, req, resp| {
        instance.push(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_KEYS_WORMHOLE, move |ctx, req, resp| {
        instance.wormhole(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_PREFERENCES, move |ctx, req, resp| {
        instance.preferences(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_PREFERENCE_SET, move |ctx, req, resp| {
        instance.preference_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_AUTH_SETUP, move |ctx, req, resp| {
        instance.auth_setup(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_AUTH_UNLOCK, move |ctx, req, resp| {
        instance.auth_unlock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_AUTH_LOCK, move |ctx, req, resp| {
        instance.auth_lock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_RUNTIME_STATUS, move |ctx, req, resp| {
        instance.runtime_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_RAND, move |ctx, req, resp| {
        instance.rand(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_RAND_PASSWORD, move |ctx, req, resp| {
        instance.rand_password(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_COLLECTIONS, move |ctx, req, resp| {
        instance.collections(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_DOCUMENTS, move |ctx, req, resp| {
        instance.documents(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_DOCUMENT_DELETE, move |ctx, req, resp| {
        instance.document_delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_ADMIN_SIGN_URL, move |ctx, req, resp| {
        instance.admin_sign_url(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_ADMIN_CHECK, move |ctx, req, resp| {
        instance.admin_check(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_MESSAGE_PREPARE, move |ctx, req, resp| {
        instance.message_prepare(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_MESSAGE_CREATE, move |ctx, req, resp| {
        instance.message_create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYS_MESSAGES, move |ctx, req, resp| {
        instance.messages(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_KEYS_WATCH, move |ctx, req, resp| {
        instance.watch(ctx, req, resp)
    });
    builder.build()
}
