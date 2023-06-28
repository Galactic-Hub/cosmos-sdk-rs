pub mod hash;
pub mod store_key;

use crate::errors::Error;
use anyhow::Result;
use lazy_static::lazy_static;
use lru::LruCache;
use serde_derive::{Deserialize, Serialize};

// const (
// 	// Constants defined here are the defaults value for address.
// 	// You can use the specific values for your project.
// 	// Add the follow lines to the `main()` of your server.
// 	//
// 	//	config := sdk.GetConfig()
// 	//	config.SetBech32PrefixForAccount(yourBech32PrefixAccAddr, yourBech32PrefixAccPub)
// 	//	config.SetBech32PrefixForValidator(yourBech32PrefixValAddr, yourBech32PrefixValPub)
// 	//	config.SetBech32PrefixForConsensusNode(yourBech32PrefixConsAddr, yourBech32PrefixConsPub)
// 	//	config.SetPurpose(yourPurpose)
// 	//	config.SetCoinType(yourCoinType)
// 	//	config.Seal()

//// Bech32MainPrefix defines the main SDK Bech32 prefix of an account's address
pub const BECH32_MAIN_PREFIX: &str = "cosmos";

/// Purpose is the ATOM purpose as defined in SLIP44 (`https://github.com/satoshilabs/slips/blob/master/slip-0044.md`)
pub const PURPOSE: u32 = 44;

/// CoinType is the ATOM coin type as defined in SLIP44 (https://github.com/satoshilabs/slips/blob/master/slip-0044.md)
pub const COIN_TYPE: u32 = 118;

/// FullFundraiserPath is the parts of the BIP44 HD path that are fixed by
/// what we used during the ATOM fundraiser.
pub const FULL_FUNDRAISER_PATH: &str = "m/44'/118'/0'/0/0";

/// PrefixAccount is the prefix for account keys
pub const PREFIX_ACCOUNT: &str = "acc";

// PrefixValidator is the prefix for validator keys
pub const PREFIX_VALIDATOR: &str = "val";

/// PrefixConsensus is the prefix for consensus keys
pub const PREFIX_CONSENSUS: &str = "cons";

/// PrefixPublic is the prefix for public keys
pub const PREFIX_PUBLIC: &str = "pub";

/// PrefixOperator is the prefix for operator keys
pub const PREFIX_OPERATOR: &str = "oper";

/// PrefixAddress is the prefix for addresses
pub const PREFIX_ADDRESS: &str = "addr";

/// Bech32PrefixAccAddr defines the Bech32 prefix of an account's address
pub const BECH32_PREFIX_ACC_ADDR: &str = BECH32_MAIN_PREFIX;

lazy_static! {
    /// `BECH32_PREFIX_ACC_PUB` defines the Bech32 prefix of an account's public key
    static ref BECH32_PREFIX_ACC_PUB: String = format!("{}{}", BECH32_MAIN_PREFIX, PREFIX_PUBLIC);
}

lazy_static! {
    /// `BECH32_PREFIX_VAL_ADDR` defines the Bech32 prefix of a validator's operator address
    static ref BECH32_PREFIX_VAL_ADDR: String = format!(
        "{}{}{}",
        BECH32_MAIN_PREFIX, PREFIX_VALIDATOR, PREFIX_OPERATOR
    );
}

lazy_static! {
    /// Bech32PrefixValPub defines the Bech32 prefix of a validator's operator public key
    static ref BECH32_PREFIX_VAL_PUB: String = format!(
        "{}{}{}{}",
        BECH32_MAIN_PREFIX, PREFIX_VALIDATOR, PREFIX_OPERATOR, PREFIX_PUBLIC
    );
}

lazy_static! {
    /// Bech32PrefixConsAddr defines the Bech32 prefix of a consensus node address
    static ref BECH32_PREFIX_CONS_ADDR: String = format!(
        "{}{}{}",
        BECH32_MAIN_PREFIX, PREFIX_VALIDATOR, PREFIX_CONSENSUS
    );
}

lazy_static! {
    /// Bech32PrefixConsPub defines the Bech32 prefix of a consensus node public key
    static ref BECH32_PREFIX_CONS_PUB: String = format!(
        "{}{}{}{}",
        BECH32_MAIN_PREFIX, PREFIX_VALIDATOR, PREFIX_CONSENSUS, PREFIX_PUBLIC
    );
}

// // cache variables
// var (
// 	// AccAddress.String() is expensive and if unoptimized dominantly showed up in profiles,
// 	// yet has no mechanisms to trivially cache the result given that AccAddress is a []byte type.
// 	accAddrMu     sync.Mutex
// 	accAddrCache  *simplelru.LRU
// 	consAddrMu    sync.Mutex
// 	consAddrCache *simplelru.LRU
// 	valAddrMu     sync.Mutex
// 	valAddrCache  *simplelru.LRU

// 	isCachingEnabled atomic.Bool
// )

// func init() {
// 	var err error
// 	SetAddrCacheEnabled(true)

// 	// in total the cache size is 61k entries. Key is 32 bytes and value is around 50-70 bytes.
// 	// That will make around 92 * 61k * 2 (LRU) bytes ~ 11 MB
// 	if accAddrCache, err = simplelru.NewLRU(60000, nil); err != nil {
// 		panic(err)
// 	}
// 	if consAddrCache, err = simplelru.NewLRU(500, nil); err != nil {
// 		panic(err)
// 	}
// 	if valAddrCache, err = simplelru.NewLRU(500, nil); err != nil {
// 		panic(err)
// 	}
// }

// // SetAddrCacheEnabled enables or disables accAddrCache, consAddrCache, and valAddrCache. By default, caches are enabled.
// func SetAddrCacheEnabled(enabled bool) {
// 	isCachingEnabled.Store(enabled)
// }

// // IsAddrCacheEnabled returns if the address caches are enabled.
// func IsAddrCacheEnabled() bool {
// 	return isCachingEnabled.Load()
// }

/// Address is a common interface for different types of addresses used by the SDK
pub trait Address:
    std::cmp::PartialEq + std::cmp::Eq + std::fmt::Debug + std::fmt::Display
{
    fn empty(&self) -> bool;
    fn marshal(&self) -> Result<Vec<u8>>;
    fn marshal_json(&self) -> Result<Vec<u8>>;
    fn bytes(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

// // Ensure that different address types implement the interface
// var (
// 	_ Address = AccAddress{}
// 	_ Address = ValAddress{}
// 	_ Address = ConsAddress{}
// )

// ----------------------------------------------------------------------------
// account
// ----------------------------------------------------------------------------

// AccAddress a wrapper around bytes meant to represent an account address.
// When marshaled to a string or JSON, it uses Bech32.
#[derive(PartialEq, Eq, Serialize, Deserialize, prost::Message)]
pub struct AccAddress {
    #[prost(bytes, tag = "1")]
    pub bytes: Vec<u8>,
}

/// `acc_address_from_hex_unsafe` creates an AccAddress from a HEX-encoded string.
///
/// Note, this function is considered unsafe as it may produce an AccAddress from
/// otherwise invalid input, such as a transaction hash. Please use
/// AccAddressFromBech32.
pub fn acc_address_from_hex_unsafe(address: &str) -> Result<AccAddress> {
    let bytes = address_bytes_from_hex_string(address)?;
    Ok(AccAddress { bytes })
}

// // VerifyAddressFormat verifies that the provided bytes form a valid address
// // according to the default address rules or a custom address verifier set by
// // GetConfig().SetAddressVerifier().
// // TODO make an issue to get rid of global Config
// // ref: https://github.com/cosmos/cosmos-sdk/issues/9690
// func VerifyAddressFormat(bz []byte) error {
// 	verifier := GetConfig().GetAddressVerifier()
// 	if verifier != nil {
// 		return verifier(bz)
// 	}

// 	if len(bz) == 0 {
// 		return errorsmod.Wrap(sdkerrors.ErrUnknownAddress, "addresses cannot be empty")
// 	}

// 	if len(bz) > address.MaxAddrLen {
// 		return errorsmod.Wrapf(sdkerrors.ErrUnknownAddress, "address max length is %d, got %d", address.MaxAddrLen, len(bz))
// 	}

// 	return nil
// }

// // MustAccAddressFromBech32 calls AccAddressFromBech32 and panics on error.
// func MustAccAddressFromBech32(address string) AccAddress {
// 	addr, err := AccAddressFromBech32(address)
// 	if err != nil {
// 		panic(err)
// 	}

// 	return addr
// }

// // AccAddressFromBech32 creates an AccAddress from a Bech32 string.
// func AccAddressFromBech32(address string) (addr AccAddress, err error) {
// 	if len(strings.TrimSpace(address)) == 0 {
// 		return AccAddress{}, errors.New("empty address string is not allowed")
// 	}

// 	bech32PrefixAccAddr := GetConfig().GetBech32AccountAddrPrefix()

// 	bz, err := GetFromBech32(address, bech32PrefixAccAddr)
// 	if err != nil {
// 		return nil, err
// 	}

// 	err = VerifyAddressFormat(bz)
// 	if err != nil {
// 		return nil, err
// 	}

// 	return AccAddress(bz), nil
// }

impl AccAddress {
    /// Returns boolean for whether an AccAddress is empty
    pub fn empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Bytes returns the raw address bytes.
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

// // String implements the Stringer interface.
// func (aa AccAddress) String() string {
// 	if aa.Empty() {
// 		return ""
// 	}

// 	key := conv.UnsafeBytesToStr(aa)

// 	if IsAddrCacheEnabled() {
// 		accAddrMu.Lock()
// 		defer accAddrMu.Unlock()

// 		addr, ok := accAddrCache.Get(key)
// 		if ok {
// 			return addr.(string)
// 		}
// 	}
// 	return cacheBech32Addr(GetConfig().GetBech32AccountAddrPrefix(), aa, accAddrCache, key)
// }

// // Format implements the fmt.Formatter interface.

// func (aa AccAddress) Format(s fmt.State, verb rune) {
// 	switch verb {
// 	case 's':
// 		s.Write([]byte(aa.String()))
// 	case 'p':
// 		s.Write([]byte(fmt.Sprintf("%p", aa)))
// 	default:
// 		s.Write([]byte(fmt.Sprintf("%X", []byte(aa))))
// 	}
// }

// // ----------------------------------------------------------------------------
// // validator operator
// // ----------------------------------------------------------------------------

// // ValAddress defines a wrapper around bytes meant to present a validator's
// // operator. When marshaled to a string or JSON, it uses Bech32.
// type ValAddress []byte

// // ValAddressFromHex creates a ValAddress from a hex string.
// func ValAddressFromHex(address string) (addr ValAddress, err error) {
// 	bz, err := addressBytesFromHexString(address)
// 	return ValAddress(bz), err
// }

// // ValAddressFromBech32 creates a ValAddress from a Bech32 string.
// func ValAddressFromBech32(address string) (addr ValAddress, err error) {
// 	if len(strings.TrimSpace(address)) == 0 {
// 		return ValAddress{}, errors.New("empty address string is not allowed")
// 	}

// 	bech32PrefixValAddr := GetConfig().GetBech32ValidatorAddrPrefix()

// 	bz, err := GetFromBech32(address, bech32PrefixValAddr)
// 	if err != nil {
// 		return nil, err
// 	}

// 	err = VerifyAddressFormat(bz)
// 	if err != nil {
// 		return nil, err
// 	}

// 	return ValAddress(bz), nil
// }

// // Returns boolean for whether two ValAddresses are Equal
// func (va ValAddress) Equals(va2 Address) bool {
// 	if va.Empty() && va2.Empty() {
// 		return true
// 	}

// 	return bytes.Equal(va.Bytes(), va2.Bytes())
// }

// // Returns boolean for whether an ValAddress is empty
// func (va ValAddress) Empty() bool {
// 	return len(va) == 0
// }

// // Marshal returns the raw address bytes. It is needed for protobuf
// // compatibility.
// func (va ValAddress) Marshal() ([]byte, error) {
// 	return va, nil
// }

// // Unmarshal sets the address to the given data. It is needed for protobuf
// // compatibility.
// func (va *ValAddress) Unmarshal(data []byte) error {
// 	*va = data
// 	return nil
// }

// // MarshalJSON marshals to JSON using Bech32.
// func (va ValAddress) MarshalJSON() ([]byte, error) {
// 	return json.Marshal(va.String())
// }

// // MarshalYAML marshals to YAML using Bech32.
// func (va ValAddress) MarshalYAML() (interface{}, error) {
// 	return va.String(), nil
// }

// // UnmarshalJSON unmarshals from JSON assuming Bech32 encoding.
// func (va *ValAddress) UnmarshalJSON(data []byte) error {
// 	var s string

// 	err := json.Unmarshal(data, &s)
// 	if err != nil {
// 		return err
// 	}
// 	if s == "" {
// 		*va = ValAddress{}
// 		return nil
// 	}

// 	va2, err := ValAddressFromBech32(s)
// 	if err != nil {
// 		return err
// 	}

// 	*va = va2
// 	return nil
// }

// // UnmarshalYAML unmarshals from YAML assuming Bech32 encoding.
// func (va *ValAddress) UnmarshalYAML(data []byte) error {
// 	var s string

// 	err := yaml.Unmarshal(data, &s)
// 	if err != nil {
// 		return err
// 	}
// 	if s == "" {
// 		*va = ValAddress{}
// 		return nil
// 	}

// 	va2, err := ValAddressFromBech32(s)
// 	if err != nil {
// 		return err
// 	}

// 	*va = va2
// 	return nil
// }

// // Bytes returns the raw address bytes.
// func (va ValAddress) Bytes() []byte {
// 	return va
// }

// // String implements the Stringer interface.
// func (va ValAddress) String() string {
// 	if va.Empty() {
// 		return ""
// 	}

// 	key := conv.UnsafeBytesToStr(va)

// 	if IsAddrCacheEnabled() {
// 		valAddrMu.Lock()
// 		defer valAddrMu.Unlock()

// 		addr, ok := valAddrCache.Get(key)
// 		if ok {
// 			return addr.(string)
// 		}
// 	}
// 	return cacheBech32Addr(GetConfig().GetBech32ValidatorAddrPrefix(), va, valAddrCache, key)
// }

// // Format implements the fmt.Formatter interface.

// func (va ValAddress) Format(s fmt.State, verb rune) {
// 	switch verb {
// 	case 's':
// 		s.Write([]byte(va.String()))
// 	case 'p':
// 		s.Write([]byte(fmt.Sprintf("%p", va)))
// 	default:
// 		s.Write([]byte(fmt.Sprintf("%X", []byte(va))))
// 	}
// }

// // ----------------------------------------------------------------------------
// // consensus node
// // ----------------------------------------------------------------------------

// // ConsAddress defines a wrapper around bytes meant to present a consensus node.
// // When marshaled to a string or JSON, it uses Bech32.
// type ConsAddress []byte

// // ConsAddressFromHex creates a ConsAddress from a hex string.
// func ConsAddressFromHex(address string) (addr ConsAddress, err error) {
// 	bz, err := addressBytesFromHexString(address)
// 	return ConsAddress(bz), err
// }

// // ConsAddressFromBech32 creates a ConsAddress from a Bech32 string.
// func ConsAddressFromBech32(address string) (addr ConsAddress, err error) {
// 	if len(strings.TrimSpace(address)) == 0 {
// 		return ConsAddress{}, errors.New("empty address string is not allowed")
// 	}

// 	bech32PrefixConsAddr := GetConfig().GetBech32ConsensusAddrPrefix()

// 	bz, err := GetFromBech32(address, bech32PrefixConsAddr)
// 	if err != nil {
// 		return nil, err
// 	}

// 	err = VerifyAddressFormat(bz)
// 	if err != nil {
// 		return nil, err
// 	}

// 	return ConsAddress(bz), nil
// }

// // get ConsAddress from pubkey
// func GetConsAddress(pubkey cryptotypes.PubKey) ConsAddress {
// 	return ConsAddress(pubkey.Address())
// }

// // Returns boolean for whether two ConsAddress are Equal
// func (ca ConsAddress) Equals(ca2 Address) bool {
// 	if ca.Empty() && ca2.Empty() {
// 		return true
// 	}

// 	return bytes.Equal(ca.Bytes(), ca2.Bytes())
// }

// // Returns boolean for whether an ConsAddress is empty
// func (ca ConsAddress) Empty() bool {
// 	return len(ca) == 0
// }

// // Marshal returns the raw address bytes. It is needed for protobuf
// // compatibility.
// func (ca ConsAddress) Marshal() ([]byte, error) {
// 	return ca, nil
// }

// // Unmarshal sets the address to the given data. It is needed for protobuf
// // compatibility.
// func (ca *ConsAddress) Unmarshal(data []byte) error {
// 	*ca = data
// 	return nil
// }

// // MarshalJSON marshals to JSON using Bech32.
// func (ca ConsAddress) MarshalJSON() ([]byte, error) {
// 	return json.Marshal(ca.String())
// }

// // MarshalYAML marshals to YAML using Bech32.
// func (ca ConsAddress) MarshalYAML() (interface{}, error) {
// 	return ca.String(), nil
// }

// // UnmarshalJSON unmarshals from JSON assuming Bech32 encoding.
// func (ca *ConsAddress) UnmarshalJSON(data []byte) error {
// 	var s string

// 	err := json.Unmarshal(data, &s)
// 	if err != nil {
// 		return err
// 	}
// 	if s == "" {
// 		*ca = ConsAddress{}
// 		return nil
// 	}

// 	ca2, err := ConsAddressFromBech32(s)
// 	if err != nil {
// 		return err
// 	}

// 	*ca = ca2
// 	return nil
// }

// // UnmarshalYAML unmarshals from YAML assuming Bech32 encoding.
// func (ca *ConsAddress) UnmarshalYAML(data []byte) error {
// 	var s string

// 	err := yaml.Unmarshal(data, &s)
// 	if err != nil {
// 		return err
// 	}
// 	if s == "" {
// 		*ca = ConsAddress{}
// 		return nil
// 	}

// 	ca2, err := ConsAddressFromBech32(s)
// 	if err != nil {
// 		return err
// 	}

// 	*ca = ca2
// 	return nil
// }

// // Bytes returns the raw address bytes.
// func (ca ConsAddress) Bytes() []byte {
// 	return ca
// }

// // String implements the Stringer interface.
// func (ca ConsAddress) String() string {
// 	if ca.Empty() {
// 		return ""
// 	}

// 	key := conv.UnsafeBytesToStr(ca)

// 	if IsAddrCacheEnabled() {
// 		consAddrMu.Lock()
// 		defer consAddrMu.Unlock()

// 		addr, ok := consAddrCache.Get(key)
// 		if ok {
// 			return addr.(string)
// 		}
// 	}
// 	return cacheBech32Addr(GetConfig().GetBech32ConsensusAddrPrefix(), ca, consAddrCache, key)
// }

// // Bech32ifyAddressBytes returns a bech32 representation of address bytes.
// // Returns an empty sting if the byte slice is 0-length. Returns an error if the bech32 conversion
// // fails or the prefix is empty.
// func Bech32ifyAddressBytes(prefix string, bs []byte) (string, error) {
// 	if len(bs) == 0 {
// 		return "", nil
// 	}
// 	if len(prefix) == 0 {
// 		return "", errors.New("prefix cannot be empty")
// 	}
// 	return bech32.ConvertAndEncode(prefix, bs)
// }

// // MustBech32ifyAddressBytes returns a bech32 representation of address bytes.
// // Returns an empty sting if the byte slice is 0-length. It panics if the bech32 conversion
// // fails or the prefix is empty.
// func MustBech32ifyAddressBytes(prefix string, bs []byte) string {
// 	s, err := Bech32ifyAddressBytes(prefix, bs)
// 	if err != nil {
// 		panic(err)
// 	}
// 	return s
// }

// // Format implements the fmt.Formatter interface.

// func (ca ConsAddress) Format(s fmt.State, verb rune) {
// 	switch verb {
// 	case 's':
// 		s.Write([]byte(ca.String()))
// 	case 'p':
// 		s.Write([]byte(fmt.Sprintf("%p", ca)))
// 	default:
// 		s.Write([]byte(fmt.Sprintf("%X", []byte(ca))))
// 	}
// }

// ----------------------------------------------------------------------------
// auxiliary
// ----------------------------------------------------------------------------

/// GetFromBech32 decodes a bytestring from a Bech32 encoded string.
pub fn get_from_bech32(bech32str: &str, prefix: &str) -> Result<Vec<u8>> {
    if bech32str.is_empty() {
        return Err(Error::EmptyBech32Address.into());
    }

    let (hrp, bz, var) = bech32::decode(bech32str)?;
    tracing::info!("hrp: {}, bz: {:?}, var: {:?}", hrp, bz, var);
    if hrp != prefix {
        return Err(Error::InvalidBech32Prefix.into());
    }

    Ok(bz.into_iter().map(Into::into).collect::<Vec<u8>>())
}

pub fn address_bytes_from_hex_string(address: &str) -> Result<Vec<u8>> {
    if address.is_empty() {
        return Err(Error::EmptyBech32Address.into());
    }

    let result = hex::decode(address)?;
    Ok(result)
}

/// cacheBech32Addr is not concurrency safe. Concurrent access to cache causes race condition.
pub fn cache_bech32_addr(
    prefix: &str,
    addr: &[u8],
    cache: &mut LruCache<String, String>,
    cache_key: &str,
) -> String {
    let bech32_addr = bech32::encode(
        prefix,
        addr.iter()
            .map(|v| bech32::u5::try_from_u8(*v).unwrap())
            .collect::<Vec<_>>(),
        bech32::Variant::Bech32,
    )
    .unwrap();
    cache.put(cache_key.to_string(), bech32_addr.clone());
    bech32_addr
}
