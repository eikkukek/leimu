use super::{*, device::*};

pub struct Features {
    /// Enables the use of the following [`DynamicState`]:
    /// - [`DepthBiasEnable`][1]
    /// - [`PrimitiveRestartEnable`][2]
    /// - [`RasterizerDiscardEnable`][3]
    ///
    /// [1]: DynamicState::DepthBiasEnable
    /// [2]: DynamicState::PrimitiveRestartEnable
    /// [3]: DynamicState::RasterizerDiscardEnable
    pub extended_dynamic_state2: bool,
    /// Enables the use of the [`LogicOpExt`][1] [`DynamicState`].
    ///
    /// [1]: DynamicState::LogicOpExt
    pub logic_op: bool,
    /// Enables the use of the [`PatchControlPointsExt`][1] [`DynamicState`].
    ///
    /// [1]: DynamicState::PatchControlPointsExt
    pub patch_control_points: bool,
}
