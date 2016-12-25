use libc::{c_float, uint32_t};

pub mod align;
pub mod dimensions;
pub mod direction;
pub mod edge;
pub mod experimental_feature;
pub mod flex_direction;
pub mod flex_wrap;
pub mod justify_content;
pub mod log_level;
pub mod measure_mode;
pub mod overflow;
pub mod position_type;
pub mod print_options;

pub use self::align::Align;
pub use self::dimensions::Dimensions;
pub use self::direction::Direction;
pub use self::edge::Edge;
pub use self::experimental_feature::ExperimentalFeature;
pub use self::flex_direction::FlexDirection;
pub use self::flex_wrap::FlexWrap;
pub use self::justify_content::JustifyContent;
pub use self::log_level::LogLevel;
pub use self::measure_mode::MeasureMode;
pub use self::overflow::Overflow;
pub use self::position_type::PositionType;
pub use self::print_options::PrintOptions;

#[repr(C)]
pub struct Node {}

#[link(name = "yoga")]
extern "C" {
    pub fn YGNodeNew() -> *mut Node;

    pub fn YGNodeStyleSetWidth(node: *mut Node, width: c_float);
    pub fn YGNodeStyleGetWidth(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetHeight(node: *mut Node, height: c_float);
    pub fn YGNodeStyleGetHeight(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetMinWidth(node: *mut Node, min_width: c_float);
    pub fn YGNodeStyleGetMinWidth(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetMinHeight(node: *mut Node, min_height: c_float);
    pub fn YGNodeStyleGetMinHeight(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetMaxWidth(node: *mut Node, max_width: c_float);
    pub fn YGNodeStyleGetMaxWidth(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetMaxHeight(node: *mut Node, max_height: c_float);
    pub fn YGNodeStyleGetMaxHeight(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetJustifyContent(node: *mut Node, justify_content: JustifyContent);
    pub fn YGNodeStyleGetJustifyContent(node: *const Node) -> JustifyContent;

    pub fn YGNodeStyleSetAlignContent(node: *mut Node, align_content: Align);
    pub fn YGNodeStyleGetAlignContent(node: *const Node) -> Align;

    pub fn YGNodeStyleSetAlignItems(node: *mut Node, align_items: Align);
    pub fn YGNodeStyleGetAlignItems(node: *const Node) -> Align;

    pub fn YGNodeStyleSetAlignSelf(node: *mut Node, align_self: Align);
    pub fn YGNodeStyleGetAlignSelf(node: *const Node) -> Align;

    pub fn YGNodeStyleSetPositionType(node: *mut Node, overflow: PositionType);
    pub fn YGNodeStyleGetPositionType(node: *const Node) -> PositionType;

    pub fn YGNodeStyleSetFlexWrap(node: *mut Node, overflow: FlexWrap);
    pub fn YGNodeStyleGetFlexWrap(node: *const Node) -> FlexWrap;

    pub fn YGNodeStyleSetOverflow(node: *mut Node, overflow: Overflow);
    pub fn YGNodeStyleGetOverflow(node: *const Node) -> Overflow;

    pub fn YGNodeStyleSetDirection(node: *mut Node, direction: Direction);
    pub fn YGNodeStyleGetDirection(node: *const Node) -> Direction;

    pub fn YGNodeStyleSetFlexDirection(node: *mut Node, flexDirection: FlexDirection);
    pub fn YGNodeStyleGetFlexDirection(node: *const Node) -> FlexDirection;

    pub fn YGNodeStyleSetFlexBasis(node: *mut Node, flex_basis: c_float);
    pub fn YGNodeStyleGetFlexBasis(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetFlexShrink(node: *mut Node, flex_shrink: c_float);
    pub fn YGNodeStyleGetFlexShrink(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetFlexGrow(node: *mut Node, flex_grow: c_float);
    pub fn YGNodeStyleGetFlexGrow(node: *const Node) -> c_float;

    pub fn YGNodeStyleSetFlex(node: *mut Node, flex: c_float);

    pub fn YGNodeStyleSetPosition(node: *mut Node, edge: Edge, position: c_float);
    pub fn YGNodeStyleGetPosition(node: *const Node, edge: Edge) -> c_float;

    pub fn YGNodeStyleSetBorder(node: *mut Node, edge: Edge, border: c_float);
    pub fn YGNodeStyleGetBorder(node: *const Node, edge: Edge) -> c_float;

    pub fn YGNodeStyleSetMargin(node: *mut Node, edge: Edge, margin: c_float);
    pub fn YGNodeStyleGetMargin(node: *const Node, edge: Edge) -> c_float;

    pub fn YGNodeStyleSetPadding(node: *mut Node, edge: Edge, padding: c_float);
    pub fn YGNodeStyleGetPadding(node: *const Node, edge: Edge) -> c_float;

    pub fn YGNodeGetParent(node: *mut Node) -> Node;

    pub fn YGNodeInsertChild(node: *mut Node, node: *const Node, index: uint32_t);

    pub fn YGNodeGetChild(node: *const Node, index: uint32_t) -> Node;
    pub fn YGNodeGetChildCount(node: *const Node) -> uint32_t;

    pub fn YGNodeRemoveChild(node: *mut Node, child: *const Node);

    pub fn YGNodeCalculateLayout(node: *mut Node,
                                 availableWidth: c_float,
                                 availableHeight: c_float,
                                 parentDirection: Direction);

    pub fn YGNodeLayoutGetLeft(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetRight(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetTop(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetBottom(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetWidth(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetHeight(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetDirection(node: *const Node) -> Direction;
}
