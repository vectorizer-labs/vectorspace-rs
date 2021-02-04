pub enum UnitType
{
    SVG_ANGLETYPE_UNKNOWN = 0,
    SVG_ANGLETYPE_UNSPECIFIED = 1,
    SVG_ANGLETYPE_DEG = 2,
    SVG_ANGLETYPE_RAD = 3,
    SVG_ANGLETYPE_GRAD = 4
}

pub struct SVGAngle
{
    unit_type : UnitType,
    value : f32
}