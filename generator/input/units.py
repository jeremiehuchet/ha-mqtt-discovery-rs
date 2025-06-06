
class UnitOfApparentPower(StrEnum):
    """Apparent power units."""
    VOLT_AMPERE = "VA"
 
class UnitOfPower(StrEnum):
    """Power units."""
    MILLIWATT = "mW"
    WATT = "W"
    KILO_WATT = "kW"
    MEGA_WATT = "MW"
    GIGA_WATT = "GW"
    TERA_WATT = "TW"
    BTU_PER_HOUR = "BTU/h"
 
class UnitOfReactivePower(StrEnum):
    """Reactive power units."""
    VOLT_AMPERE_REACTIVE = "var"
    KILO_VOLT_AMPERE_REACTIVE = "kvar"
 
class UnitOfEnergy(StrEnum):
    """Energy units."""
    JOULE = "J"
    KILO_JOULE = "kJ"
    MEGA_JOULE = "MJ"
    GIGA_JOULE = "GJ"
    MILLIWATT_HOUR = "mWh"
    WATT_HOUR = "Wh"
    KILO_WATT_HOUR = "kWh"
    MEGA_WATT_HOUR = "MWh"
    GIGA_WATT_HOUR = "GWh"
    TERA_WATT_HOUR = "TWh"
    CALORIE = "cal"
    KILO_CALORIE = "kcal"
    MEGA_CALORIE = "Mcal"
    GIGA_CALORIE = "Gcal"
 
class UnitOfEnergyDistance(StrEnum):
    """Energy Distance units."""
    KILO_WATT_HOUR_PER_100_KM = "kWh/100km"
    MILES_PER_KILO_WATT_HOUR = "mi/kWh"
    KM_PER_KILO_WATT_HOUR = "km/kWh"
 
class UnitOfElectricCurrent(StrEnum):
    """Electric current units."""
    MILLIAMPERE = "mA"
    AMPERE = "A"
 
class UnitOfElectricPotential(StrEnum):
    """Electric potential units."""
    MICROVOLT = "µV"
    MILLIVOLT = "mV"
    VOLT = "V"
    KILOVOLT = "kV"
    MEGAVOLT = "MV"
 
class UnitOfTemperature(StrEnum):
    """Temperature units."""
    CELSIUS = "°C"
    FAHRENHEIT = "°F"
    KELVIN = "K"
 
class UnitOfTime(StrEnum):
    """Time units."""
    MICROSECONDS = "μs"
    MILLISECONDS = "ms"
    SECONDS = "s"
    MINUTES = "min"
    HOURS = "h"
    DAYS = "d"
    WEEKS = "w"
    MONTHS = "m"
    YEARS = "y"
 
class UnitOfLength(StrEnum):
    """Length units."""
    MILLIMETERS = "mm"
    CENTIMETERS = "cm"
    METERS = "m"
    KILOMETERS = "km"
    INCHES = "in"
    FEET = "ft"
    YARDS = "yd"
    MILES = "mi"
    NAUTICAL_MILES = "nmi"
 
class UnitOfFrequency(StrEnum):
    """Frequency units."""
    HERTZ = "Hz"
    KILOHERTZ = "kHz"
    MEGAHERTZ = "MHz"
    GIGAHERTZ = "GHz"
 
class UnitOfPressure(StrEnum):
    """Pressure units."""
    PA = "Pa"
    HPA = "hPa"
    KPA = "kPa"
    BAR = "bar"
    CBAR = "cbar"
    MBAR = "mbar"
    MMHG = "mmHg"
    INHG = "inHg"
    PSI = "psi"
 
class UnitOfSoundPressure(StrEnum):
    """Sound pressure units."""
    DECIBEL = "dB"
    WEIGHTED_DECIBEL_A = "dBA"
 
class UnitOfVolume(StrEnum):
    """Volume units."""
    CUBIC_FEET = "ft³"
    CENTUM_CUBIC_FEET = "CCF"
    CUBIC_METERS = "m³"
    LITERS = "L"
    MILLILITERS = "mL"
    GALLONS = "gal"
    """Assumed to be US gallons in conversion utilities.
    British/Imperial gallons are not yet supported"""
    FLUID_OUNCES = "fl. oz."
    """Assumed to be US fluid ounces in conversion utilities.
    British/Imperial fluid ounces are not yet supported"""
 
class UnitOfVolumeFlowRate(StrEnum):
    """Volume flow rate units."""
    CUBIC_METERS_PER_HOUR = "m³/h"
    CUBIC_METERS_PER_SECOND = "m³/s"
    CUBIC_FEET_PER_MINUTE = "ft³/min"
    LITERS_PER_HOUR = "L/h"
    LITERS_PER_MINUTE = "L/min"
    LITERS_PER_SECOND = "L/s"
    GALLONS_PER_MINUTE = "gal/min"
    MILLILITERS_PER_SECOND = "mL/s"
 
class UnitOfMass(StrEnum):
    """Mass units."""
    GRAMS = "g"
    KILOGRAMS = "kg"
    MILLIGRAMS = "mg"
    MICROGRAMS = "µg"
    OUNCES = "oz"
    POUNDS = "lb"
    STONES = "st"
 
class UnitOfIrradiance(StrEnum):
    """Irradiance units."""
    WATTS_PER_SQUARE_METER = "W/m²"
    BTUS_PER_HOUR_SQUARE_FOOT = "BTU/(h⋅ft²)"
 
class UnitOfPrecipitationDepth(StrEnum):
    """Precipitation depth.
    The derivation of these units is a volume of rain amassing in a container
    with constant cross section
    """
    INCHES = "in"
    """Derived from in³/in²"""
    MILLIMETERS = "mm"
    """Derived from mm³/mm²"""
    CENTIMETERS = "cm"
    """Derived from cm³/cm²"""
 
class UnitOfBloodGlucoseConcentration(StrEnum):
    """Blood glucose concentration units."""
    MILLIGRAMS_PER_DECILITER = "mg/dL"
    MILLIMOLE_PER_LITER = "mmol/L"
 
class UnitOfSpeed(StrEnum):
    """Speed units."""
    BEAUFORT = "Beaufort"
    FEET_PER_SECOND = "ft/s"
    INCHES_PER_SECOND = "in/s"
    METERS_PER_SECOND = "m/s"
    KILOMETERS_PER_HOUR = "km/h"
    KNOTS = "kn"
    MILES_PER_HOUR = "mph"
    MILLIMETERS_PER_SECOND = "mm/s"
 
class UnitOfInformation(StrEnum):
    """Information units."""
    BITS = "bit"
    KILOBITS = "kbit"
    MEGABITS = "Mbit"
    GIGABITS = "Gbit"
    BYTES = "B"
    KILOBYTES = "kB"
    MEGABYTES = "MB"
    GIGABYTES = "GB"
    TERABYTES = "TB"
    PETABYTES = "PB"
    EXABYTES = "EB"
    ZETTABYTES = "ZB"
    YOTTABYTES = "YB"
    KIBIBYTES = "KiB"
    MEBIBYTES = "MiB"
    GIBIBYTES = "GiB"
    TEBIBYTES = "TiB"
    PEBIBYTES = "PiB"
    EXBIBYTES = "EiB"
    ZEBIBYTES = "ZiB"
    YOBIBYTES = "YiB"
 
class UnitOfDataRate(StrEnum):
    """Data rate units."""
    BITS_PER_SECOND = "bit/s"
    KILOBITS_PER_SECOND = "kbit/s"
    MEGABITS_PER_SECOND = "Mbit/s"
    GIGABITS_PER_SECOND = "Gbit/s"
    BYTES_PER_SECOND = "B/s"
    KILOBYTES_PER_SECOND = "kB/s"
    MEGABYTES_PER_SECOND = "MB/s"
    GIGABYTES_PER_SECOND = "GB/s"
    KIBIBYTES_PER_SECOND = "KiB/s"
    MEBIBYTES_PER_SECOND = "MiB/s"
    GIBIBYTES_PER_SECOND = "GiB/s"
 