#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPointEventKind {
    ProperInteriorInteriorCrossing,
    OperandAEndpointOnOperandBInterior,
    OperandBEndpointOnOperandAInterior,
    SharedEndpoint,
}

impl PlanarBooleanPointEventKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ProperInteriorInteriorCrossing => "proper-interior-interior-crossing",
            Self::OperandAEndpointOnOperandBInterior => "operand-a-endpoint-on-operand-b-interior",
            Self::OperandBEndpointOnOperandAInterior => "operand-b-endpoint-on-operand-a-interior",
            Self::SharedEndpoint => "shared-endpoint",
        }
    }
}
