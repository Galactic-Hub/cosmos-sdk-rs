/// CustomProtobufType defines the interface custom gogo proto types must implement
/// in order to be used as a "customtype" extension.
///
/// `ref: https://github.com/cosmos/gogoproto/blob/master/custom_types.md`
trait CustomProtobufType {
    type Error;

    fn marshal(&self) -> Result<Vec<u8>, Self::Error>;
    fn marshal_to(&self, data: &mut [u8]) -> Result<usize, Self::Error>;
    fn unmarshal(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn size(&self) -> usize;

    fn marshal_json(&self) -> Result<Vec<u8>, Self::Error>;
    fn unmarshal_json(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}
