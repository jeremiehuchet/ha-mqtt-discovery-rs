pub mod entities;

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
    ReactiveEnergy(ReactiveEnergyUnit),
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
/// ReactiveEnergy units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ReactiveEnergyUnit {
    #[serde(rename = "varh")]
    VoltAmpereReactiveHour,
    #[serde(rename = "kvarh")]
    KiloVoltAmpereReactiveHour,
}

impl Into<Unit> for ReactiveEnergyUnit {
    fn into(self) -> Unit {
        Unit::ReactiveEnergy(self)
    }
}
/// EnergyDistance units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EnergyDistanceUnit {
    #[serde(rename = "kWh/100km")]
    KiloWattHourPer100Km,
    #[serde(rename = "Wh/km")]
    WattHourPerKm,
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ValveDeviceClass {
    /// Generic valve. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// Valve that controls the flow of water through a system.
    #[serde(rename = "water")]
    Water,

    /// Valve that controls the flow of gas through a system.
    #[serde(rename = "gas")]
    Gas,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum CoverDeviceClass {
    /// Generic cover. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// Control of an awning, such as an exterior retractable window, door, or patio cover.
    #[serde(rename = "awning")]
    Awning,

    /// Control of blinds, which are linked slats that expand or collapse to cover an opening or may be tilted to partially covering an opening, such as window blinds.
    #[serde(rename = "blind")]
    Blind,

    /// Control of curtains or drapes, which is often fabric hung above a window or door that can be drawn open.
    #[serde(rename = "curtain")]
    Curtain,

    /// Control of a mechanical damper that reduces airflow, sound, or light.
    #[serde(rename = "damper")]
    Damper,

    /// Control of a door or gate that provides access to an area.
    #[serde(rename = "door")]
    Door,

    /// Control of a garage door that provides access to a garage.
    #[serde(rename = "garage")]
    Garage,

    /// Control of a gate. Gates are found outside of a structure and are typically part of a fence.
    #[serde(rename = "gate")]
    Gate,

    /// Control of shades, which are a continuous plane of material or connected cells that expanded or collapsed over an opening, such as window shades.
    #[serde(rename = "shade")]
    Shade,

    /// Control of shutters. Shutters are linked slats that can be raised or lowered to cover an opening, such as window or door roller shutters. Some shutters (for example, some indoor or exterior window shutters) swing out/in to cover an opening or may be tilted to provide partial cover.
    #[serde(rename = "shutter")]
    Shutter,

    /// Control of a physical window that opens and closes or may tilt.
    #[serde(rename = "window")]
    Window,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum NumberDeviceClass {
    /// Generic number. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// Absolute humidity in g/m³, mg/m³.
    #[serde(rename = "absolute_humidity")]
    AbsoluteHumidity,

    /// Apparent power in VA.
    #[serde(rename = "apparent_power")]
    ApparentPower,

    /// Air Quality Index (unitless).
    #[serde(rename = "aqi")]
    Aqi,

    /// Area in m², cm², km², mm², in², ft², yd², mi², ac, ha
    #[serde(rename = "area")]
    Area,

    /// Atmospheric pressure in cbar, bar, hPa, mmHg, inHg, kPa, mbar, Pa or psi
    #[serde(rename = "atmospheric_pressure")]
    AtmosphericPressure,

    /// Percentage of battery that is left in %
    #[serde(rename = "battery")]
    Battery,

    /// Blood glucose concentration in mg/dL, mmol/L
    #[serde(rename = "blood_glucose_concentration")]
    BloodGlucoseConcentration,

    /// Carbon Dioxide in CO2 (Smoke) in ppm
    #[serde(rename = "carbon_dioxide")]
    CarbonDioxide,

    /// Carbon Monoxide in CO (Gas CNG/LPG) in ppm
    #[serde(rename = "carbon_monoxide")]
    CarbonMonoxide,

    /// Current in A, mA
    #[serde(rename = "current")]
    Current,

    /// Data rate in bit/s, kbit/s, Mbit/s, Gbit/s, B/s, kB/s, MB/s, GB/s, KiB/s, MiB/s or GiB/s
    #[serde(rename = "data_rate")]
    DataRate,

    /// Data size in bit, kbit, Mbit, Gbit, B, kB, MB, GB, TB, PB, EB, ZB, YB, KiB, MiB, GiB, TiB, PiB, EiB, ZiB or YiB
    #[serde(rename = "data_size")]
    DataSize,

    /// Generic distance in km, m, cm, mm, mi, nmi, yd, or in
    #[serde(rename = "distance")]
    Distance,

    /// Duration in d, h, min, s, ms, or µs
    #[serde(rename = "duration")]
    Duration,

    /// Energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy")]
    Energy,

    /// Energy per distance in kWh/100km, Wh/km, mi/kWh, or km/kWh.
    #[serde(rename = "energy_distance")]
    EnergyDistance,

    /// Stored energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy_storage")]
    EnergyStorage,

    /// Frequency in Hz, kHz, MHz, or GHz
    #[serde(rename = "frequency")]
    Frequency,

    /// Gas volume in L, m³, ft³ or CCF
    #[serde(rename = "gas")]
    Gas,

    /// Percentage of humidity in the air in %
    #[serde(rename = "humidity")]
    Humidity,

    /// The current light level in lx
    #[serde(rename = "illuminance")]
    Illuminance,

    /// Irradiance in W/m² or BTU/(h⋅ft²)
    #[serde(rename = "irradiance")]
    Irradiance,

    /// Percentage of water in a substance in %
    #[serde(rename = "moisture")]
    Moisture,

    /// The monetary value ([ISO 4217](https://en.wikipedia.org/wiki/ISO_4217#Active_codes))
    #[serde(rename = "monetary")]
    Monetary,

    /// Concentration of Nitrogen Dioxide in µg/m³
    #[serde(rename = "nitrogen_dioxide")]
    NitrogenDioxide,

    /// Concentration of Nitrogen Monoxide in µg/m³
    #[serde(rename = "nitrogen_monoxide")]
    NitrogenMonoxide,

    /// Concentration of Nitrous Oxide in µg/m³
    #[serde(rename = "nitrous_oxide")]
    NitrousOxide,

    /// Concentration of Ozone in µg/m³
    #[serde(rename = "ozone")]
    Ozone,

    /// Potential hydrogen (pH) value of a water solution
    #[serde(rename = "ph")]
    Ph,

    /// Concentration of particulate matter less than 1 micrometer in µg/m³
    #[serde(rename = "pm1")]
    Pm1,

    /// Concentration of particulate matter less than 2.5 micrometers in µg/m³
    #[serde(rename = "pm25")]
    Pm25,

    /// Concentration of particulate matter less than 10 micrometers in µg/m³
    #[serde(rename = "pm10")]
    Pm10,

    /// Power factor (unitless), unit may be `None` or %
    #[serde(rename = "power_factor")]
    PowerFactor,

    /// Power in mW, W, kW, MW, GW or TW
    #[serde(rename = "power")]
    Power,

    /// Accumulated precipitation in cm, in or mm
    #[serde(rename = "precipitation")]
    Precipitation,

    /// Precipitation intensity in in/d, in/h, mm/d or mm/h
    #[serde(rename = "precipitation_intensity")]
    PrecipitationIntensity,

    /// Pressure in Pa, kPa, hPa, bar, cbar, mbar, mmHg, inHg or psi
    #[serde(rename = "pressure")]
    Pressure,

    /// Reactive energy in varh or kvarh
    #[serde(rename = "reactive_energy")]
    ReactiveEnergy,

    /// Reactive power in var or kvar
    #[serde(rename = "reactive_power")]
    ReactivePower,

    /// Signal strength in dB or dBm
    #[serde(rename = "signal_strength")]
    SignalStrength,

    /// Sound pressure in dB or dBA
    #[serde(rename = "sound_pressure")]
    SoundPressure,

    /// Generic speed in ft/s, in/d, in/h, in/s, km/h, kn, m/s, mph, mm/d, or mm/s
    #[serde(rename = "speed")]
    Speed,

    /// Concentration of sulphur dioxide in µg/m³
    #[serde(rename = "sulphur_dioxide")]
    SulphurDioxide,

    /// Temperature in °C, °F or K
    #[serde(rename = "temperature")]
    Temperature,

    /// Concentration of volatile organic compounds in µg/m³ or mg/m³
    #[serde(rename = "volatile_organic_compounds")]
    VolatileOrganicCompounds,

    /// Ratio of volatile organic compounds in ppm or ppb
    #[serde(rename = "volatile_organic_compounds_parts")]
    VolatileOrganicCompoundsParts,

    /// Voltage in V, mV, µV, kV, MV
    #[serde(rename = "voltage")]
    Voltage,

    /// Generic volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    #[serde(rename = "volume")]
    Volume,

    /// Volume flow rate in m³/h, ft³/min, L/min, gal/min, or mL/s
    #[serde(rename = "volume_flow_rate")]
    VolumeFlowRate,

    /// Generic stored volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    #[serde(rename = "volume_storage")]
    VolumeStorage,

    /// Water consumption in L, gal, m³, ft³, or CCF
    #[serde(rename = "water")]
    Water,

    /// Generic mass in kg, g, mg, µg, oz, lb, or st
    #[serde(rename = "weight")]
    Weight,

    /// Wind direction in °
    #[serde(rename = "wind_direction")]
    WindDirection,

    /// Wind speed in Beaufort, ft/s, km/h, kn, m/s, or mph
    #[serde(rename = "wind_speed")]
    WindSpeed,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum MediaPlayerDeviceClass {
    /// Device is a television type device.
    #[serde(rename = "tv")]
    Tv,

    /// Device is a speaker or stereo type device.
    #[serde(rename = "speaker")]
    Speaker,

    /// Device is an audio/video receiver type device taking audio and outputting to speakers and video to displays.
    #[serde(rename = "receiver")]
    Receiver,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BinarySensorDeviceClass {
    /// Generic on/off. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// `on` means low, `off` means normal
    #[serde(rename = "battery")]
    Battery,

    /// `on` means charging, `off` means not charging
    #[serde(rename = "battery_charging")]
    BatteryCharging,

    /// `on` means carbon monoxide detected, `off` no carbon monoxide (clear)
    #[serde(rename = "carbon_monoxide")]
    CarbonMonoxide,

    /// `on` means cold, `off` means normal
    #[serde(rename = "cold")]
    Cold,

    /// `on` means connected, `off` means disconnected
    #[serde(rename = "connectivity")]
    Connectivity,

    /// `on` means open, `off` means closed
    #[serde(rename = "door")]
    Door,

    /// `on` means open, `off` means closed
    #[serde(rename = "garage_door")]
    GarageDoor,

    /// `on` means gas detected, `off` means no gas (clear)
    #[serde(rename = "gas")]
    Gas,

    /// `on` means hot, `off` means normal
    #[serde(rename = "heat")]
    Heat,

    /// `on` means light detected, `off` means no light
    #[serde(rename = "light")]
    Light,

    /// `on` means open (unlocked), `off` means closed (locked)
    #[serde(rename = "lock")]
    Lock,

    /// `on` means moisture detected (wet), `off` means no moisture (dry)
    #[serde(rename = "moisture")]
    Moisture,

    /// `on` means motion detected, `off` means no motion (clear)
    #[serde(rename = "motion")]
    Motion,

    /// `on` means moving, `off` means not moving (stopped)
    #[serde(rename = "moving")]
    Moving,

    /// `on` means occupied (detected), `off` means not occupied (clear)
    #[serde(rename = "occupancy")]
    Occupancy,

    /// `on` means open, `off` means closed
    #[serde(rename = "opening")]
    Opening,

    /// `on` means device is plugged in, `off` means device is unplugged
    #[serde(rename = "plug")]
    Plug,

    /// `on` means power detected, `off` means no power
    #[serde(rename = "power")]
    Power,

    /// `on` means home, `off` means away
    #[serde(rename = "presence")]
    Presence,

    /// `on` means problem detected, `off` means no problem (OK)
    #[serde(rename = "problem")]
    Problem,

    /// `on` means running, `off` means not running
    #[serde(rename = "running")]
    Running,

    /// `on` means unsafe, `off` means safe
    #[serde(rename = "safety")]
    Safety,

    /// `on` means smoke detected, `off` means no smoke (clear)
    #[serde(rename = "smoke")]
    Smoke,

    /// `on` means sound detected, `off` means no sound (clear)
    #[serde(rename = "sound")]
    Sound,

    /// `on` means tampering detected, `off` means no tampering (clear)
    #[serde(rename = "tamper")]
    Tamper,

    /// `on` means update available, `off` means up-to-date
    #[serde(rename = "update")]
    Update,

    /// `on` means vibration detected, `off` means no vibration (clear)
    #[serde(rename = "vibration")]
    Vibration,

    /// `on` means open, `off` means closed
    #[serde(rename = "window")]
    Window,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum UpdateDeviceClass {
    /// A generic software update. This is the default and doesn't need
    #[serde(rename = "None")]
    None,

    /// This update {% term integration %} provides firmwares.
    #[serde(rename = "firmware")]
    Firmware,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum HumidifierDeviceClass {
    /// Adds humidity to the air around it.
    #[serde(rename = "Humidifier")]
    Humidifier,

    /// Removes humidity from the air around it.
    #[serde(rename = "Dehumidifier")]
    Dehumidifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SwitchDeviceClass {
    /// Generic switch. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// A switch for a power outlet.
    #[serde(rename = "outlet")]
    Outlet,

    /// A generic switch.
    #[serde(rename = "switch")]
    Switch,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum HomeassistantDeviceClass {}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EventDeviceClass {
    /// Generic event. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// For remote control buttons.
    #[serde(rename = "button")]
    Button,

    /// Specifically for buttons that are used as a doorbell.
    #[serde(rename = "doorbell")]
    Doorbell,

    /// For motion events detected by a motion sensor.
    #[serde(rename = "motion")]
    Motion,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SensorDeviceClass {
    /// Generic sensor. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// Absolute humidity in g/m³, mg/m³.
    #[serde(rename = "absolute_humidity")]
    AbsoluteHumidity,

    /// Apparent power in VA.
    #[serde(rename = "apparent_power")]
    ApparentPower,

    /// Air Quality Index (unitless).
    #[serde(rename = "aqi")]
    Aqi,

    /// Area in m², cm², km², mm², in², ft², yd², mi², ac, ha
    #[serde(rename = "area")]
    Area,

    /// Atmospheric pressure in cbar, bar, hPa, mmHg, inHg, kPa, mbar, Pa or psi
    #[serde(rename = "atmospheric_pressure")]
    AtmosphericPressure,

    /// Percentage of battery that is left in %
    #[serde(rename = "battery")]
    Battery,

    /// Blood glucose concentration in mg/dL, mmol/L
    #[serde(rename = "blood_glucose_concentration")]
    BloodGlucoseConcentration,

    /// Carbon Dioxide in CO2 (Smoke) in ppm
    #[serde(rename = "carbon_dioxide")]
    CarbonDioxide,

    /// Carbon Monoxide in CO (Gas CNG/LPG) in ppm
    #[serde(rename = "carbon_monoxide")]
    CarbonMonoxide,

    /// Current in A, mA
    #[serde(rename = "current")]
    Current,

    /// Data rate in bit/s, kbit/s, Mbit/s, Gbit/s, B/s, kB/s, MB/s, GB/s, KiB/s, MiB/s or GiB/s
    #[serde(rename = "data_rate")]
    DataRate,

    /// Data size in bit, kbit, Mbit, Gbit, B, kB, MB, GB, TB, PB, EB, ZB, YB, KiB, MiB, GiB, TiB, PiB, EiB, ZiB or YiB
    #[serde(rename = "data_size")]
    DataSize,

    /// Date string (ISO 8601)
    #[serde(rename = "date")]
    Date,

    /// Generic distance in km, m, cm, mm, mi, nmi, yd, or in
    #[serde(rename = "distance")]
    Distance,

    /// Duration in d, h, min, s, ms, or µs
    #[serde(rename = "duration")]
    Duration,

    /// Energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy")]
    Energy,

    /// Energy per distance in kWh/100km, Wh/km, mi/kWh, or km/kWh.
    #[serde(rename = "energy_distance")]
    EnergyDistance,

    /// Stored energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy_storage")]
    EnergyStorage,

    /// Has a limited set of (non-numeric) states
    #[serde(rename = "enum")]
    Enum,

    /// Frequency in Hz, kHz, MHz, or GHz
    #[serde(rename = "frequency")]
    Frequency,

    /// Gas volume in L, m³, ft³ or CCF
    #[serde(rename = "gas")]
    Gas,

    /// Percentage of humidity in the air in %
    #[serde(rename = "humidity")]
    Humidity,

    /// The current light level in lx
    #[serde(rename = "illuminance")]
    Illuminance,

    /// Irradiance in W/m² or BTU/(h⋅ft²)
    #[serde(rename = "irradiance")]
    Irradiance,

    /// Percentage of water in a substance in %
    #[serde(rename = "moisture")]
    Moisture,

    /// The monetary value ([ISO 4217](https://en.wikipedia.org/wiki/ISO_4217#Active_codes))
    #[serde(rename = "monetary")]
    Monetary,

    /// Concentration of Nitrogen Dioxide in µg/m³
    #[serde(rename = "nitrogen_dioxide")]
    NitrogenDioxide,

    /// Concentration of Nitrogen Monoxide in µg/m³
    #[serde(rename = "nitrogen_monoxide")]
    NitrogenMonoxide,

    /// Concentration of Nitrous Oxide in µg/m³
    #[serde(rename = "nitrous_oxide")]
    NitrousOxide,

    /// Concentration of Ozone in µg/m³
    #[serde(rename = "ozone")]
    Ozone,

    /// Potential hydrogen (pH) value of a water solution
    #[serde(rename = "ph")]
    Ph,

    /// Concentration of particulate matter less than 1 micrometer in µg/m³
    #[serde(rename = "pm1")]
    Pm1,

    /// Concentration of particulate matter less than 2.5 micrometers in µg/m³
    #[serde(rename = "pm25")]
    Pm25,

    /// Concentration of particulate matter less than 10 micrometers in µg/m³
    #[serde(rename = "pm10")]
    Pm10,

    /// Power factor (unitless), unit may be `None` or %
    #[serde(rename = "power_factor")]
    PowerFactor,

    /// Power in mW, W, kW, MW, GW or TW
    #[serde(rename = "power")]
    Power,

    /// Accumulated precipitation in cm, in or mm
    #[serde(rename = "precipitation")]
    Precipitation,

    /// Precipitation intensity in in/d, in/h, mm/d or mm/h
    #[serde(rename = "precipitation_intensity")]
    PrecipitationIntensity,

    /// Pressure in Pa, kPa, hPa, bar, cbar, mbar, mmHg, inHg or psi
    #[serde(rename = "pressure")]
    Pressure,

    /// Reactive energy in varh or kvarh
    #[serde(rename = "reactive_energy")]
    ReactiveEnergy,

    /// Reactive power in var or kvar
    #[serde(rename = "reactive_power")]
    ReactivePower,

    /// Signal strength in dB or dBm
    #[serde(rename = "signal_strength")]
    SignalStrength,

    /// Sound pressure in dB or dBA
    #[serde(rename = "sound_pressure")]
    SoundPressure,

    /// Generic speed in ft/s, in/d, in/h, in/s, km/h, kn, m/s, mph, mm/d, or mm/s
    #[serde(rename = "speed")]
    Speed,

    /// Concentration of sulphur dioxide in µg/m³
    #[serde(rename = "sulphur_dioxide")]
    SulphurDioxide,

    /// Temperature in °C, °F or K
    #[serde(rename = "temperature")]
    Temperature,

    /// Datetime object or timestamp string (ISO 8601)
    #[serde(rename = "timestamp")]
    Timestamp,

    /// Concentration of volatile organic compounds in µg/m³ or mg/m³
    #[serde(rename = "volatile_organic_compounds")]
    VolatileOrganicCompounds,

    /// Ratio of volatile organic compounds in ppm or ppb
    #[serde(rename = "volatile_organic_compounds_parts")]
    VolatileOrganicCompoundsParts,

    /// Voltage in V, mV, µV, kV, MV
    #[serde(rename = "voltage")]
    Voltage,

    /// Generic volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    #[serde(rename = "volume")]
    Volume,

    /// Volume flow rate in m³/h, m³/s, ft³/min, L/h, L/min, L/s, gal/min, or mL/s
    #[serde(rename = "volume_flow_rate")]
    VolumeFlowRate,

    /// Generic stored volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    #[serde(rename = "volume_storage")]
    VolumeStorage,

    /// Water consumption in L, gal, m³, ft³, or CCF
    #[serde(rename = "water")]
    Water,

    /// Generic mass in kg, g, mg, µg, oz, lb, or st
    #[serde(rename = "weight")]
    Weight,

    /// Wind direction in °
    #[serde(rename = "wind_direction")]
    WindDirection,

    /// Wind speed in Beaufort, ft/s, km/h, kn, m/s, or mph
    #[serde(rename = "wind_speed")]
    WindSpeed,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ButtonDeviceClass {
    /// Generic button. This is the default and doesn't need to be set.
    #[serde(rename = "None")]
    None,

    /// The button is used to identify a device.
    #[serde(rename = "identify")]
    Identify,

    /// The button restarts the device.
    #[serde(rename = "restart")]
    Restart,

    /// The button updates the software of the device.
    #[serde(rename = "update")]
    Update,
}
