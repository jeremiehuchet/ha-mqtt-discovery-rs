use serde_derive::Serialize;

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

    /// Control of shutters, which are linked slats that swing out/in to covering an opening or may be tilted to partially cover an opening, such as indoor or exterior window shutters.
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

    /// Duration in d, h, min, s, or ms
    #[serde(rename = "duration")]
    Duration,

    /// Energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy")]
    Energy,

    /// Energy per distance in kWh/100km, mi/kWh or km/kWh.
    #[serde(rename = "energy_distance")]
    EnergyDistance,

    /// Stored energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy_storage")]
    EnergyStorage,

    /// Frequency in Hz, kHz, MHz, or GHz
    #[serde(rename = "frequency")]
    Frequency,

    /// Gasvolume in m³, ft³ or CCF
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

    /// Reactive power in var
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

    /// Concentration of volatile organic compounds in µg/m³
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

    /// Duration in d, h, min, s, or ms
    #[serde(rename = "duration")]
    Duration,

    /// Energy in J, kJ, MJ, GJ, mWh, Wh, kWh, MWh, GWh, TWh, cal, kcal, Mcal, or Gcal
    #[serde(rename = "energy")]
    Energy,

    /// Energy per distance in kWh/100km, mi/kWh or km/kWh.
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

    /// Gasvolume in m³, ft³ or CCF
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

    /// Reactive power in var
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

    /// Concentration of volatile organic compounds in µg/m³
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
