pub enum UnitType
{
    SVG_LENGTHTYPE_UNKNOWN = 0,
    SVG_LENGTHTYPE_NUMBER = 1,
    SVG_LENGTHTYPE_PERCENTAGE = 2,
    SVG_LENGTHTYPE_EMS = 3,
    SVG_LENGTHTYPE_EXS = 4,
    SVG_LENGTHTYPE_PX = 5,
    SVG_LENGTHTYPE_CM = 6,
    SVG_LENGTHTYPE_MM = 7,
    SVG_LENGTHTYPE_IN = 8,
    SVG_LENGTHTYPE_PT = 9,
    SVG_LENGTHTYPE_PC = 10
}

pub struct SVGLength
{
    unit_type : UnitType,
    value : f32

}