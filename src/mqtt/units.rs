use serde_derive::Serialize;

/// Units of measurement
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Unit {
    ApparentPower(ApparentPowerUnit),
    Power(PowerUnit),
    ReactivePower(ReactivePowerUnit),
    Energy(EnergyUnit),
    EnergyDistance(EnergyDistanceUnit),
    ElectricCurrent(ElectricCurrentUnit),
    ElectricPotential(ElectricPotentialUnit),
    Temperature(TemperatureUnit),
    Time(TimeUnit),
    Length(LengthUnit),
    Frequency(FrequencyUnit),
    Pressure(PressureUnit),
    SoundPressure(SoundPressureUnit),
    Volume(VolumeUnit),
    VolumeFlowRate(VolumeFlowRateUnit),
    Mass(MassUnit),
    Irradiance(IrradianceUnit),
    PrecipitationDepth(PrecipitationDepthUnit),
    BloodGlucoseConcentration(BloodGlucoseConcentrationUnit),
    Speed(SpeedUnit),
    Information(InformationUnit),
    DataRate(DataRateUnit),
}

/// ApparentPower units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ApparentPowerUnit {
    #[serde(rename = "VA")]
    VoltAmpere,
}

impl Into<Unit> for ApparentPowerUnit {
    fn into(self) -> Unit {
        Unit::ApparentPower(self)
    }
}
/// Power units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PowerUnit {
    #[serde(rename = "mW")]
    MilliWatt,
    #[serde(rename = "W")]
    Watt,
    #[serde(rename = "kW")]
    KiloWatt,
    #[serde(rename = "MW")]
    MegaWatt,
    #[serde(rename = "GW")]
    GigaWatt,
    #[serde(rename = "TW")]
    TeraWatt,
    #[serde(rename = "BTU/h")]
    BtuPerHour,
}

impl Into<Unit> for PowerUnit {
    fn into(self) -> Unit {
        Unit::Power(self)
    }
}
/// ReactivePower units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ReactivePowerUnit {
    #[serde(rename = "var")]
    VoltAmpereReactive,
    #[serde(rename = "kvar")]
    KiloVoltAmpereReactive,
}

impl Into<Unit> for ReactivePowerUnit {
    fn into(self) -> Unit {
        Unit::ReactivePower(self)
    }
}
/// Energy units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EnergyUnit {
    #[serde(rename = "J")]
    Joule,
    #[serde(rename = "kJ")]
    KiloJoule,
    #[serde(rename = "MJ")]
    MegaJoule,
    #[serde(rename = "GJ")]
    GigaJoule,
    #[serde(rename = "mWh")]
    MilliwattHour,
    #[serde(rename = "Wh")]
    WattHour,
    #[serde(rename = "kWh")]
    KiloWattHour,
    #[serde(rename = "MWh")]
    MegaWattHour,
    #[serde(rename = "GWh")]
    GigaWattHour,
    #[serde(rename = "TWh")]
    TeraWattHour,
    #[serde(rename = "cal")]
    Calorie,
    #[serde(rename = "kcal")]
    KiloCalorie,
    #[serde(rename = "Mcal")]
    MegaCalorie,
    #[serde(rename = "Gcal")]
    GigaCalorie,
}

impl Into<Unit> for EnergyUnit {
    fn into(self) -> Unit {
        Unit::Energy(self)
    }
}
/// EnergyDistance units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EnergyDistanceUnit {
    #[serde(rename = "kWh/100km")]
    KiloWattHourPer100Km,
    #[serde(rename = "mi/kWh")]
    MilesPerKiloWattHour,
    #[serde(rename = "km/kWh")]
    KmPerKiloWattHour,
}

impl Into<Unit> for EnergyDistanceUnit {
    fn into(self) -> Unit {
        Unit::EnergyDistance(self)
    }
}
/// ElectricCurrent units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ElectricCurrentUnit {
    #[serde(rename = "mA")]
    Milliampere,
    #[serde(rename = "A")]
    Ampere,
}

impl Into<Unit> for ElectricCurrentUnit {
    fn into(self) -> Unit {
        Unit::ElectricCurrent(self)
    }
}
/// ElectricPotential units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ElectricPotentialUnit {
    #[serde(rename = "µV")]
    Microvolt,
    #[serde(rename = "mV")]
    Millivolt,
    #[serde(rename = "V")]
    Volt,
    #[serde(rename = "kV")]
    Kilovolt,
    #[serde(rename = "MV")]
    Megavolt,
}

impl Into<Unit> for ElectricPotentialUnit {
    fn into(self) -> Unit {
        Unit::ElectricPotential(self)
    }
}
/// Temperature units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TemperatureUnit {
    #[serde(rename = "°C")]
    Celsius,
    #[serde(rename = "°F")]
    Fahrenheit,
    #[serde(rename = "K")]
    Kelvin,
}

impl Into<Unit> for TemperatureUnit {
    fn into(self) -> Unit {
        Unit::Temperature(self)
    }
}
/// Time units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TimeUnit {
    #[serde(rename = "μs")]
    Microseconds,
    #[serde(rename = "ms")]
    Milliseconds,
    #[serde(rename = "s")]
    Seconds,
    #[serde(rename = "min")]
    Minutes,
    #[serde(rename = "h")]
    Hours,
    #[serde(rename = "d")]
    Days,
    #[serde(rename = "w")]
    Weeks,
    #[serde(rename = "m")]
    Months,
    #[serde(rename = "y")]
    Years,
}

impl Into<Unit> for TimeUnit {
    fn into(self) -> Unit {
        Unit::Time(self)
    }
}
/// Length units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum LengthUnit {
    #[serde(rename = "mm")]
    Millimeters,
    #[serde(rename = "cm")]
    Centimeters,
    #[serde(rename = "m")]
    Meters,
    #[serde(rename = "km")]
    Kilometers,
    #[serde(rename = "in")]
    Inches,
    #[serde(rename = "ft")]
    Feet,
    #[serde(rename = "yd")]
    Yards,
    #[serde(rename = "mi")]
    Miles,
    #[serde(rename = "nmi")]
    NauticalMiles,
}

impl Into<Unit> for LengthUnit {
    fn into(self) -> Unit {
        Unit::Length(self)
    }
}
/// Frequency units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum FrequencyUnit {
    #[serde(rename = "Hz")]
    Hertz,
    #[serde(rename = "kHz")]
    Kilohertz,
    #[serde(rename = "MHz")]
    Megahertz,
    #[serde(rename = "GHz")]
    Gigahertz,
}

impl Into<Unit> for FrequencyUnit {
    fn into(self) -> Unit {
        Unit::Frequency(self)
    }
}
/// Pressure units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PressureUnit {
    #[serde(rename = "Pa")]
    Pa,
    #[serde(rename = "hPa")]
    Hpa,
    #[serde(rename = "kPa")]
    Kpa,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "cbar")]
    Cbar,
    #[serde(rename = "mbar")]
    Mbar,
    #[serde(rename = "mmHg")]
    Mmhg,
    #[serde(rename = "inHg")]
    Inhg,
    #[serde(rename = "psi")]
    Psi,
}

impl Into<Unit> for PressureUnit {
    fn into(self) -> Unit {
        Unit::Pressure(self)
    }
}
/// SoundPressure units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SoundPressureUnit {
    #[serde(rename = "dB")]
    Decibel,
    #[serde(rename = "dBA")]
    WeightedDecibelA,
}

impl Into<Unit> for SoundPressureUnit {
    fn into(self) -> Unit {
        Unit::SoundPressure(self)
    }
}
/// Volume units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum VolumeUnit {
    #[serde(rename = "ft³")]
    CubicFeet,
    #[serde(rename = "CCF")]
    CentumCubicFeet,
    #[serde(rename = "m³")]
    CubicMeters,
    #[serde(rename = "L")]
    Liters,
    #[serde(rename = "mL")]
    Milliliters,
    #[serde(rename = "gal")]
    Gallons,
    #[serde(rename = "fl. oz.")]
    FluidOunces,
}

impl Into<Unit> for VolumeUnit {
    fn into(self) -> Unit {
        Unit::Volume(self)
    }
}
/// VolumeFlowRate units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum VolumeFlowRateUnit {
    #[serde(rename = "m³/h")]
    CubicMetersPerHour,
    #[serde(rename = "m³/s")]
    CubicMetersPerSecond,
    #[serde(rename = "ft³/min")]
    CubicFeetPerMinute,
    #[serde(rename = "L/h")]
    LitersPerHour,
    #[serde(rename = "L/min")]
    LitersPerMinute,
    #[serde(rename = "L/s")]
    LitersPerSecond,
    #[serde(rename = "gal/min")]
    GallonsPerMinute,
    #[serde(rename = "mL/s")]
    MillilitersPerSecond,
}

impl Into<Unit> for VolumeFlowRateUnit {
    fn into(self) -> Unit {
        Unit::VolumeFlowRate(self)
    }
}
/// Mass units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum MassUnit {
    #[serde(rename = "g")]
    Grams,
    #[serde(rename = "kg")]
    Kilograms,
    #[serde(rename = "mg")]
    Milligrams,
    #[serde(rename = "µg")]
    Micrograms,
    #[serde(rename = "oz")]
    Ounces,
    #[serde(rename = "lb")]
    Pounds,
    #[serde(rename = "st")]
    Stones,
}

impl Into<Unit> for MassUnit {
    fn into(self) -> Unit {
        Unit::Mass(self)
    }
}
/// Irradiance units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum IrradianceUnit {
    #[serde(rename = "W/m²")]
    WattsPerSquareMeter,
    #[serde(rename = "BTU/(h⋅ft²)")]
    BtusPerHourSquareFoot,
}

impl Into<Unit> for IrradianceUnit {
    fn into(self) -> Unit {
        Unit::Irradiance(self)
    }
}
/// PrecipitationDepth units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PrecipitationDepthUnit {
    #[serde(rename = "in")]
    Inches,
    #[serde(rename = "mm")]
    Millimeters,
    #[serde(rename = "cm")]
    Centimeters,
}

impl Into<Unit> for PrecipitationDepthUnit {
    fn into(self) -> Unit {
        Unit::PrecipitationDepth(self)
    }
}
/// BloodGlucoseConcentration units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BloodGlucoseConcentrationUnit {
    #[serde(rename = "mg/dL")]
    MilligramsPerDeciliter,
    #[serde(rename = "mmol/L")]
    MillimolePerLiter,
}

impl Into<Unit> for BloodGlucoseConcentrationUnit {
    fn into(self) -> Unit {
        Unit::BloodGlucoseConcentration(self)
    }
}
/// Speed units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SpeedUnit {
    #[serde(rename = "Beaufort")]
    Beaufort,
    #[serde(rename = "ft/s")]
    FeetPerSecond,
    #[serde(rename = "in/s")]
    InchesPerSecond,
    #[serde(rename = "m/s")]
    MetersPerSecond,
    #[serde(rename = "km/h")]
    KilometersPerHour,
    #[serde(rename = "kn")]
    Knots,
    #[serde(rename = "mph")]
    MilesPerHour,
    #[serde(rename = "mm/s")]
    MillimetersPerSecond,
}

impl Into<Unit> for SpeedUnit {
    fn into(self) -> Unit {
        Unit::Speed(self)
    }
}
/// Information units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum InformationUnit {
    #[serde(rename = "bit")]
    Bits,
    #[serde(rename = "kbit")]
    Kilobits,
    #[serde(rename = "Mbit")]
    Megabits,
    #[serde(rename = "Gbit")]
    Gigabits,
    #[serde(rename = "B")]
    Bytes,
    #[serde(rename = "kB")]
    Kilobytes,
    #[serde(rename = "MB")]
    Megabytes,
    #[serde(rename = "GB")]
    Gigabytes,
    #[serde(rename = "TB")]
    Terabytes,
    #[serde(rename = "PB")]
    Petabytes,
    #[serde(rename = "EB")]
    Exabytes,
    #[serde(rename = "ZB")]
    Zettabytes,
    #[serde(rename = "YB")]
    Yottabytes,
    #[serde(rename = "KiB")]
    Kibibytes,
    #[serde(rename = "MiB")]
    Mebibytes,
    #[serde(rename = "GiB")]
    Gibibytes,
    #[serde(rename = "TiB")]
    Tebibytes,
    #[serde(rename = "PiB")]
    Pebibytes,
    #[serde(rename = "EiB")]
    Exbibytes,
    #[serde(rename = "ZiB")]
    Zebibytes,
    #[serde(rename = "YiB")]
    Yobibytes,
}

impl Into<Unit> for InformationUnit {
    fn into(self) -> Unit {
        Unit::Information(self)
    }
}
/// DataRate units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DataRateUnit {
    #[serde(rename = "bit/s")]
    BitsPerSecond,
    #[serde(rename = "kbit/s")]
    KilobitsPerSecond,
    #[serde(rename = "Mbit/s")]
    MegabitsPerSecond,
    #[serde(rename = "Gbit/s")]
    GigabitsPerSecond,
    #[serde(rename = "B/s")]
    BytesPerSecond,
    #[serde(rename = "kB/s")]
    KilobytesPerSecond,
    #[serde(rename = "MB/s")]
    MegabytesPerSecond,
    #[serde(rename = "GB/s")]
    GigabytesPerSecond,
    #[serde(rename = "KiB/s")]
    KibibytesPerSecond,
    #[serde(rename = "MiB/s")]
    MebibytesPerSecond,
    #[serde(rename = "GiB/s")]
    GibibytesPerSecond,
}

impl Into<Unit> for DataRateUnit {
    fn into(self) -> Unit {
        Unit::DataRate(self)
    }
}
