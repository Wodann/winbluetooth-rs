#![cfg(all(windows, target_arch = "x86_64"))]
extern crate winbluetooth;
use std::mem::{align_of, size_of};
#[test]
fn shared_bthdef() {
    use winbluetooth::shared::bthdef::*;
    assert_eq!(size_of::<BTH_DEVICE_INFO>(), 272);
    assert_eq!(align_of::<BTH_DEVICE_INFO>(), 8);
    assert_eq!(size_of::<BTH_RADIO_IN_RANGE>(), 280);
    assert_eq!(align_of::<BTH_RADIO_IN_RANGE>(), 8);
    assert_eq!(size_of::<BTH_L2CAP_EVENT_INFO>(), 16);
    assert_eq!(align_of::<BTH_L2CAP_EVENT_INFO>(), 8);
    assert_eq!(size_of::<BTH_HCI_EVENT_INFO>(), 16);
    assert_eq!(align_of::<BTH_HCI_EVENT_INFO>(), 8);
}
#[test]
fn shared_bthioctl() {
    use winbluetooth::shared::bthioctl::*;
    assert_eq!(size_of::<BTH_DEVICE_INFO_LIST>(), 276);
    assert_eq!(align_of::<BTH_DEVICE_INFO_LIST>(), 1);
    assert_eq!(size_of::<BTH_RADIO_INFO>(), 13);
    assert_eq!(align_of::<BTH_RADIO_INFO>(), 1);
    assert_eq!(size_of::<BTH_LOCAL_RADIO_INFO>(), 292);
    assert_eq!(align_of::<BTH_LOCAL_RADIO_INFO>(), 1);
    assert_eq!(size_of::<BTH_SDP_CONNECT>(), 21);
    assert_eq!(align_of::<BTH_SDP_CONNECT>(), 1);
    assert_eq!(size_of::<BTH_SDP_DISCONNECT>(), 8);
    assert_eq!(align_of::<BTH_SDP_DISCONNECT>(), 1);
    assert_eq!(size_of::<BTH_SDP_RECORD>(), 17);
    assert_eq!(align_of::<BTH_SDP_RECORD>(), 1);
    assert_eq!(size_of::<BTH_SDP_SERVICE_SEARCH_REQUEST>(), 248);
    assert_eq!(align_of::<BTH_SDP_SERVICE_SEARCH_REQUEST>(), 1);
    assert_eq!(size_of::<BTH_SDP_ATTRIBUTE_SEARCH_REQUEST>(), 20);
    assert_eq!(align_of::<BTH_SDP_ATTRIBUTE_SEARCH_REQUEST>(), 1);
    assert_eq!(size_of::<BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST>(), 256);
    assert_eq!(align_of::<BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST>(), 1);
    assert_eq!(size_of::<BTH_SDP_STREAM_RESPONSE>(), 9);
    assert_eq!(align_of::<BTH_SDP_STREAM_RESPONSE>(), 1);
    assert_eq!(size_of::<BTH_COMMAND_HEADER>(), 3);
    assert_eq!(align_of::<BTH_COMMAND_HEADER>(), 1);
    assert_eq!(size_of::<BTH_VENDOR_SPECIFIC_COMMAND>(), 10);
    assert_eq!(align_of::<BTH_VENDOR_SPECIFIC_COMMAND>(), 1);
    assert_eq!(size_of::<BTH_VENDOR_PATTERN>(), 3);
    assert_eq!(align_of::<BTH_VENDOR_PATTERN>(), 1);
    assert_eq!(size_of::<BTH_VENDOR_EVENT_INFO>(), 13);
    assert_eq!(align_of::<BTH_VENDOR_EVENT_INFO>(), 1);
    assert_eq!(size_of::<BTH_HOST_FEATURE_MASK>(), 24);
    assert_eq!(align_of::<BTH_HOST_FEATURE_MASK>(), 1);
}
#[test]
fn shared_bthsdpdef() {
    use winbluetooth::shared::bthsdpdef::*;
    assert_eq!(size_of::<SDP_LARGE_INTEGER_16>(), 16);
    assert_eq!(align_of::<SDP_LARGE_INTEGER_16>(), 8);
    assert_eq!(size_of::<SDP_ULARGE_INTEGER_16>(), 16);
    assert_eq!(align_of::<SDP_ULARGE_INTEGER_16>(), 8);
    assert_eq!(size_of::<SdpAttributeRange>(), 4);
    assert_eq!(align_of::<SdpAttributeRange>(), 2);
    assert_eq!(size_of::<SdpQueryUuidUnion>(), 16);
    assert_eq!(align_of::<SdpQueryUuidUnion>(), 4);
    assert_eq!(size_of::<SdpQueryUuid>(), 20);
    assert_eq!(align_of::<SdpQueryUuid>(), 4);
}
#[test]
fn um_bluetoothapis() {
    use winbluetooth::um::bluetoothapis::*;
    assert_eq!(size_of::<BLUETOOTH_LOCAL_SERVICE_INFO>(), 1040);
    assert_eq!(align_of::<BLUETOOTH_LOCAL_SERVICE_INFO>(), 8);
    assert_eq!(size_of::<BLUETOOTH_FIND_RADIO_PARAMS>(), 4);
    assert_eq!(align_of::<BLUETOOTH_FIND_RADIO_PARAMS>(), 4);
    assert_eq!(size_of::<BLUETOOTH_RADIO_INFO>(), 520);
    assert_eq!(align_of::<BLUETOOTH_RADIO_INFO>(), 8);
    assert_eq!(size_of::<BLUETOOTH_DEVICE_INFO>(), 560);
    assert_eq!(align_of::<BLUETOOTH_DEVICE_INFO>(), 8);
    assert_eq!(size_of::<BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS>(), 576);
    assert_eq!(align_of::<BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>(), 40);
    assert_eq!(align_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_COD_PAIRS>(), 16);
    assert_eq!(align_of::<BLUETOOTH_COD_PAIRS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>(), 88);
    assert_eq!(align_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_PIN_INFO>(), 17);
    assert_eq!(align_of::<BLUETOOTH_PIN_INFO>(), 1);
    assert_eq!(size_of::<BLUETOOTH_OOB_DATA_INFO>(), 32);
    assert_eq!(align_of::<BLUETOOTH_OOB_DATA_INFO>(), 1);
    assert_eq!(size_of::<BLUETOOTH_NUMERIC_COMPARISON_INFO>(), 4);
    assert_eq!(align_of::<BLUETOOTH_NUMERIC_COMPARISON_INFO>(), 4);
    assert_eq!(size_of::<BLUETOOTH_PASSKEY_INFO>(), 4);
    assert_eq!(align_of::<BLUETOOTH_PASSKEY_INFO>(), 4);
    assert_eq!(size_of::<BLUETOOTH_AUTHENTICATE_RESPONSE>(), 48);
    assert_eq!(align_of::<BLUETOOTH_AUTHENTICATE_RESPONSE>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_string>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_string>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_url>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_url>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_sequence>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_sequence>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_alternative>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_alternative>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA>(), 24);
    assert_eq!(align_of::<SDP_ELEMENT_DATA>(), 8);
    assert_eq!(size_of::<SDP_STRING_TYPE_DATA>(), 6);
    assert_eq!(align_of::<SDP_STRING_TYPE_DATA>(), 2);
}
#[test]
fn um_bthledef() {
    use winbluetooth::um::bthledef::*;
    assert_eq!(size_of::<BTH_LE_UUID>(), 20);
    assert_eq!(align_of::<BTH_LE_UUID>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_SERVICE>(), 24);
    assert_eq!(align_of::<BTH_LE_GATT_SERVICE>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_CHARACTERISTIC>(), 36);
    assert_eq!(align_of::<BTH_LE_GATT_CHARACTERISTIC>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_CHARACTERISTIC_VALUE>(), 8);
    assert_eq!(align_of::<BTH_LE_GATT_CHARACTERISTIC_VALUE>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_DESCRIPTOR>(), 32);
    assert_eq!(align_of::<BTH_LE_GATT_DESCRIPTOR>(), 4);
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicExtendedProperties>(),
        2
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicExtendedProperties>(),
        1
    );
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ClientCharacteristicConfiguration>(),
        2
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ClientCharacteristicConfiguration>(),
        1
    );
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ServerCharacteristicConfiguration>(),
        1
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ServerCharacteristicConfiguration>(),
        1
    );
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicFormat>(),
        48
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicFormat>(),
        4
    );
    assert_eq!(size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE>(), 80);
    assert_eq!(align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE>(), 4);
    assert_eq!(
        size_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT_REGISTRATION>(),
        40
    );
    assert_eq!(
        align_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT_REGISTRATION>(),
        4
    );
    assert_eq!(size_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT>(), 24);
    assert_eq!(align_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT>(), 8);
}
#[test]
fn um_ws2bth() {
    use winbluetooth::um::ws2bth::*;
    assert_eq!(size_of::<SOCKADDR_BTH>(), 30);
    assert_eq!(align_of::<SOCKADDR_BTH>(), 1);
    assert_eq!(size_of::<BTH_SET_SERVICE>(), 45);
    assert_eq!(align_of::<BTH_SET_SERVICE>(), 1);
    assert_eq!(size_of::<BTH_QUERY_DEVICE>(), 5);
    assert_eq!(align_of::<BTH_QUERY_DEVICE>(), 1);
    assert_eq!(size_of::<BTH_QUERY_SERVICE>(), 256);
    assert_eq!(align_of::<BTH_QUERY_SERVICE>(), 1);
    assert_eq!(size_of::<RFCOMM_MSC_DATA>(), 2);
    assert_eq!(align_of::<RFCOMM_MSC_DATA>(), 1);
    assert_eq!(size_of::<RFCOMM_RLS_DATA>(), 1);
    assert_eq!(align_of::<RFCOMM_RLS_DATA>(), 1);
    assert_eq!(size_of::<RFCOMM_RPN_DATA>(), 7);
    assert_eq!(align_of::<RFCOMM_RPN_DATA>(), 1);
    assert_eq!(size_of::<RFCOMM_COMMAND>(), 11);
    assert_eq!(align_of::<RFCOMM_COMMAND>(), 1);
    assert_eq!(size_of::<BTH_PING_REQ>(), 53);
    assert_eq!(align_of::<BTH_PING_REQ>(), 1);
    assert_eq!(size_of::<BTH_PING_RSP>(), 45);
    assert_eq!(align_of::<BTH_PING_RSP>(), 1);
    assert_eq!(size_of::<BTH_INFO_REQ>(), 10);
    assert_eq!(align_of::<BTH_INFO_REQ>(), 1);
    assert_eq!(size_of::<BTH_INFO_RSP>(), 47);
    assert_eq!(align_of::<BTH_INFO_RSP>(), 1);
}
