//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!
//! The enum implements [`Into<icondata_core::IconData>`][icondata_core::IconData].
//!
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::AsRefStr))]
pub enum WiIcon {
    #[cfg(feature = "WiAlien")]
    WiAlien,
    #[cfg(feature = "WiBarometer")]
    WiBarometer,
    #[cfg(feature = "WiCelsius")]
    WiCelsius,
    #[cfg(feature = "WiCloud")]
    WiCloud,
    #[cfg(feature = "WiCloudDown")]
    WiCloudDown,
    #[cfg(feature = "WiCloudRefresh")]
    WiCloudRefresh,
    #[cfg(feature = "WiCloudUp")]
    WiCloudUp,
    #[cfg(feature = "WiCloudy")]
    WiCloudy,
    #[cfg(feature = "WiCloudyGusts")]
    WiCloudyGusts,
    #[cfg(feature = "WiCloudyWindy")]
    WiCloudyWindy,
    #[cfg(feature = "WiDayCloudy")]
    WiDayCloudy,
    #[cfg(feature = "WiDayCloudyGusts")]
    WiDayCloudyGusts,
    #[cfg(feature = "WiDayCloudyHigh")]
    WiDayCloudyHigh,
    #[cfg(feature = "WiDayCloudyWindy")]
    WiDayCloudyWindy,
    #[cfg(feature = "WiDayFog")]
    WiDayFog,
    #[cfg(feature = "WiDayHail")]
    WiDayHail,
    #[cfg(feature = "WiDayHaze")]
    WiDayHaze,
    #[cfg(feature = "WiDayLightWind")]
    WiDayLightWind,
    #[cfg(feature = "WiDayLightning")]
    WiDayLightning,
    #[cfg(feature = "WiDayRain")]
    WiDayRain,
    #[cfg(feature = "WiDayRainMix")]
    WiDayRainMix,
    #[cfg(feature = "WiDayRainWind")]
    WiDayRainWind,
    #[cfg(feature = "WiDayShowers")]
    WiDayShowers,
    #[cfg(feature = "WiDaySleet")]
    WiDaySleet,
    #[cfg(feature = "WiDaySleetStorm")]
    WiDaySleetStorm,
    #[cfg(feature = "WiDaySnow")]
    WiDaySnow,
    #[cfg(feature = "WiDaySnowThunderstorm")]
    WiDaySnowThunderstorm,
    #[cfg(feature = "WiDaySnowWind")]
    WiDaySnowWind,
    #[cfg(feature = "WiDaySprinkle")]
    WiDaySprinkle,
    #[cfg(feature = "WiDayStormShowers")]
    WiDayStormShowers,
    #[cfg(feature = "WiDaySunny")]
    WiDaySunny,
    #[cfg(feature = "WiDaySunnyOvercast")]
    WiDaySunnyOvercast,
    #[cfg(feature = "WiDayThunderstorm")]
    WiDayThunderstorm,
    #[cfg(feature = "WiDayWindy")]
    WiDayWindy,
    #[cfg(feature = "WiDegrees")]
    WiDegrees,
    #[cfg(feature = "WiDirectionDown")]
    WiDirectionDown,
    #[cfg(feature = "WiDirectionDownLeft")]
    WiDirectionDownLeft,
    #[cfg(feature = "WiDirectionDownRight")]
    WiDirectionDownRight,
    #[cfg(feature = "WiDirectionLeft")]
    WiDirectionLeft,
    #[cfg(feature = "WiDirectionRight")]
    WiDirectionRight,
    #[cfg(feature = "WiDirectionUp")]
    WiDirectionUp,
    #[cfg(feature = "WiDirectionUpLeft")]
    WiDirectionUpLeft,
    #[cfg(feature = "WiDirectionUpRight")]
    WiDirectionUpRight,
    #[cfg(feature = "WiDust")]
    WiDust,
    #[cfg(feature = "WiEarthquake")]
    WiEarthquake,
    #[cfg(feature = "WiFahrenheit")]
    WiFahrenheit,
    #[cfg(feature = "WiFire")]
    WiFire,
    #[cfg(feature = "WiFlood")]
    WiFlood,
    #[cfg(feature = "WiFog")]
    WiFog,
    #[cfg(feature = "WiGaleWarning")]
    WiGaleWarning,
    #[cfg(feature = "WiHail")]
    WiHail,
    #[cfg(feature = "WiHorizon")]
    WiHorizon,
    #[cfg(feature = "WiHorizonAlt")]
    WiHorizonAlt,
    #[cfg(feature = "WiHot")]
    WiHot,
    #[cfg(feature = "WiHumidity")]
    WiHumidity,
    #[cfg(feature = "WiHurricane")]
    WiHurricane,
    #[cfg(feature = "WiHurricaneWarning")]
    WiHurricaneWarning,
    #[cfg(feature = "WiLightning")]
    WiLightning,
    #[cfg(feature = "WiLunarEclipse")]
    WiLunarEclipse,
    #[cfg(feature = "WiMeteor")]
    WiMeteor,
    #[cfg(feature = "WiMoonAltFirstQuarter")]
    WiMoonAltFirstQuarter,
    #[cfg(feature = "WiMoonAltFull")]
    WiMoonAltFull,
    #[cfg(feature = "WiMoonAltNew")]
    WiMoonAltNew,
    #[cfg(feature = "WiMoonAltThirdQuarter")]
    WiMoonAltThirdQuarter,
    #[cfg(feature = "WiMoonAltWaningCrescent1")]
    WiMoonAltWaningCrescent1,
    #[cfg(feature = "WiMoonAltWaningCrescent2")]
    WiMoonAltWaningCrescent2,
    #[cfg(feature = "WiMoonAltWaningCrescent3")]
    WiMoonAltWaningCrescent3,
    #[cfg(feature = "WiMoonAltWaningCrescent4")]
    WiMoonAltWaningCrescent4,
    #[cfg(feature = "WiMoonAltWaningCrescent5")]
    WiMoonAltWaningCrescent5,
    #[cfg(feature = "WiMoonAltWaningCrescent6")]
    WiMoonAltWaningCrescent6,
    #[cfg(feature = "WiMoonAltWaningGibbous1")]
    WiMoonAltWaningGibbous1,
    #[cfg(feature = "WiMoonAltWaningGibbous2")]
    WiMoonAltWaningGibbous2,
    #[cfg(feature = "WiMoonAltWaningGibbous3")]
    WiMoonAltWaningGibbous3,
    #[cfg(feature = "WiMoonAltWaningGibbous4")]
    WiMoonAltWaningGibbous4,
    #[cfg(feature = "WiMoonAltWaningGibbous5")]
    WiMoonAltWaningGibbous5,
    #[cfg(feature = "WiMoonAltWaningGibbous6")]
    WiMoonAltWaningGibbous6,
    #[cfg(feature = "WiMoonAltWaxingCrescent1")]
    WiMoonAltWaxingCrescent1,
    #[cfg(feature = "WiMoonAltWaxingCrescent2")]
    WiMoonAltWaxingCrescent2,
    #[cfg(feature = "WiMoonAltWaxingCrescent3")]
    WiMoonAltWaxingCrescent3,
    #[cfg(feature = "WiMoonAltWaxingCrescent4")]
    WiMoonAltWaxingCrescent4,
    #[cfg(feature = "WiMoonAltWaxingCrescent5")]
    WiMoonAltWaxingCrescent5,
    #[cfg(feature = "WiMoonAltWaxingCrescent6")]
    WiMoonAltWaxingCrescent6,
    #[cfg(feature = "WiMoonAltWaxingGibbous1")]
    WiMoonAltWaxingGibbous1,
    #[cfg(feature = "WiMoonAltWaxingGibbous2")]
    WiMoonAltWaxingGibbous2,
    #[cfg(feature = "WiMoonAltWaxingGibbous3")]
    WiMoonAltWaxingGibbous3,
    #[cfg(feature = "WiMoonAltWaxingGibbous4")]
    WiMoonAltWaxingGibbous4,
    #[cfg(feature = "WiMoonAltWaxingGibbous5")]
    WiMoonAltWaxingGibbous5,
    #[cfg(feature = "WiMoonAltWaxingGibbous6")]
    WiMoonAltWaxingGibbous6,
    #[cfg(feature = "WiMoonFirstQuarter")]
    WiMoonFirstQuarter,
    #[cfg(feature = "WiMoonFull")]
    WiMoonFull,
    #[cfg(feature = "WiMoonNew")]
    WiMoonNew,
    #[cfg(feature = "WiMoonThirdQuarter")]
    WiMoonThirdQuarter,
    #[cfg(feature = "WiMoonWaningCrescent1")]
    WiMoonWaningCrescent1,
    #[cfg(feature = "WiMoonWaningCrescent2")]
    WiMoonWaningCrescent2,
    #[cfg(feature = "WiMoonWaningCrescent3")]
    WiMoonWaningCrescent3,
    #[cfg(feature = "WiMoonWaningCrescent4")]
    WiMoonWaningCrescent4,
    #[cfg(feature = "WiMoonWaningCrescent5")]
    WiMoonWaningCrescent5,
    #[cfg(feature = "WiMoonWaningCrescent6")]
    WiMoonWaningCrescent6,
    #[cfg(feature = "WiMoonWaningGibbous1")]
    WiMoonWaningGibbous1,
    #[cfg(feature = "WiMoonWaningGibbous2")]
    WiMoonWaningGibbous2,
    #[cfg(feature = "WiMoonWaningGibbous3")]
    WiMoonWaningGibbous3,
    #[cfg(feature = "WiMoonWaningGibbous4")]
    WiMoonWaningGibbous4,
    #[cfg(feature = "WiMoonWaningGibbous5")]
    WiMoonWaningGibbous5,
    #[cfg(feature = "WiMoonWaningGibbous6")]
    WiMoonWaningGibbous6,
    #[cfg(feature = "WiMoonWaxingCrescent1")]
    WiMoonWaxingCrescent1,
    #[cfg(feature = "WiMoonWaxingCrescent2")]
    WiMoonWaxingCrescent2,
    #[cfg(feature = "WiMoonWaxingCrescent3")]
    WiMoonWaxingCrescent3,
    #[cfg(feature = "WiMoonWaxingCrescent4")]
    WiMoonWaxingCrescent4,
    #[cfg(feature = "WiMoonWaxingCrescent5")]
    WiMoonWaxingCrescent5,
    #[cfg(feature = "WiMoonWaxingCrescent6")]
    WiMoonWaxingCrescent6,
    #[cfg(feature = "WiMoonWaxingGibbous1")]
    WiMoonWaxingGibbous1,
    #[cfg(feature = "WiMoonWaxingGibbous2")]
    WiMoonWaxingGibbous2,
    #[cfg(feature = "WiMoonWaxingGibbous3")]
    WiMoonWaxingGibbous3,
    #[cfg(feature = "WiMoonWaxingGibbous4")]
    WiMoonWaxingGibbous4,
    #[cfg(feature = "WiMoonWaxingGibbous5")]
    WiMoonWaxingGibbous5,
    #[cfg(feature = "WiMoonWaxingGibbous6")]
    WiMoonWaxingGibbous6,
    #[cfg(feature = "WiMoonrise")]
    WiMoonrise,
    #[cfg(feature = "WiMoonset")]
    WiMoonset,
    #[cfg(feature = "WiNa")]
    WiNa,
    #[cfg(feature = "WiNightAltCloudy")]
    WiNightAltCloudy,
    #[cfg(feature = "WiNightAltCloudyGusts")]
    WiNightAltCloudyGusts,
    #[cfg(feature = "WiNightAltCloudyHigh")]
    WiNightAltCloudyHigh,
    #[cfg(feature = "WiNightAltCloudyWindy")]
    WiNightAltCloudyWindy,
    #[cfg(feature = "WiNightAltHail")]
    WiNightAltHail,
    #[cfg(feature = "WiNightAltLightning")]
    WiNightAltLightning,
    #[cfg(feature = "WiNightAltPartlyCloudy")]
    WiNightAltPartlyCloudy,
    #[cfg(feature = "WiNightAltRain")]
    WiNightAltRain,
    #[cfg(feature = "WiNightAltRainMix")]
    WiNightAltRainMix,
    #[cfg(feature = "WiNightAltRainWind")]
    WiNightAltRainWind,
    #[cfg(feature = "WiNightAltShowers")]
    WiNightAltShowers,
    #[cfg(feature = "WiNightAltSleet")]
    WiNightAltSleet,
    #[cfg(feature = "WiNightAltSleetStorm")]
    WiNightAltSleetStorm,
    #[cfg(feature = "WiNightAltSnow")]
    WiNightAltSnow,
    #[cfg(feature = "WiNightAltSnowThunderstorm")]
    WiNightAltSnowThunderstorm,
    #[cfg(feature = "WiNightAltSnowWind")]
    WiNightAltSnowWind,
    #[cfg(feature = "WiNightAltSprinkle")]
    WiNightAltSprinkle,
    #[cfg(feature = "WiNightAltStormShowers")]
    WiNightAltStormShowers,
    #[cfg(feature = "WiNightAltThunderstorm")]
    WiNightAltThunderstorm,
    #[cfg(feature = "WiNightClear")]
    WiNightClear,
    #[cfg(feature = "WiNightCloudy")]
    WiNightCloudy,
    #[cfg(feature = "WiNightCloudyGusts")]
    WiNightCloudyGusts,
    #[cfg(feature = "WiNightCloudyHigh")]
    WiNightCloudyHigh,
    #[cfg(feature = "WiNightCloudyWindy")]
    WiNightCloudyWindy,
    #[cfg(feature = "WiNightFog")]
    WiNightFog,
    #[cfg(feature = "WiNightHail")]
    WiNightHail,
    #[cfg(feature = "WiNightLightning")]
    WiNightLightning,
    #[cfg(feature = "WiNightPartlyCloudy")]
    WiNightPartlyCloudy,
    #[cfg(feature = "WiNightRain")]
    WiNightRain,
    #[cfg(feature = "WiNightRainMix")]
    WiNightRainMix,
    #[cfg(feature = "WiNightRainWind")]
    WiNightRainWind,
    #[cfg(feature = "WiNightShowers")]
    WiNightShowers,
    #[cfg(feature = "WiNightSleet")]
    WiNightSleet,
    #[cfg(feature = "WiNightSleetStorm")]
    WiNightSleetStorm,
    #[cfg(feature = "WiNightSnow")]
    WiNightSnow,
    #[cfg(feature = "WiNightSnowThunderstorm")]
    WiNightSnowThunderstorm,
    #[cfg(feature = "WiNightSnowWind")]
    WiNightSnowWind,
    #[cfg(feature = "WiNightSprinkle")]
    WiNightSprinkle,
    #[cfg(feature = "WiNightStormShowers")]
    WiNightStormShowers,
    #[cfg(feature = "WiNightThunderstorm")]
    WiNightThunderstorm,
    #[cfg(feature = "WiRain")]
    WiRain,
    #[cfg(feature = "WiRainMix")]
    WiRainMix,
    #[cfg(feature = "WiRainWind")]
    WiRainWind,
    #[cfg(feature = "WiRaindrop")]
    WiRaindrop,
    #[cfg(feature = "WiRaindrops")]
    WiRaindrops,
    #[cfg(feature = "WiRefresh")]
    WiRefresh,
    #[cfg(feature = "WiRefreshAlt")]
    WiRefreshAlt,
    #[cfg(feature = "WiSandstorm")]
    WiSandstorm,
    #[cfg(feature = "WiShowers")]
    WiShowers,
    #[cfg(feature = "WiSleet")]
    WiSleet,
    #[cfg(feature = "WiSmallCraftAdvisory")]
    WiSmallCraftAdvisory,
    #[cfg(feature = "WiSmog")]
    WiSmog,
    #[cfg(feature = "WiSmoke")]
    WiSmoke,
    #[cfg(feature = "WiSnow")]
    WiSnow,
    #[cfg(feature = "WiSnowWind")]
    WiSnowWind,
    #[cfg(feature = "WiSnowflakeCold")]
    WiSnowflakeCold,
    #[cfg(feature = "WiSolarEclipse")]
    WiSolarEclipse,
    #[cfg(feature = "WiSprinkle")]
    WiSprinkle,
    #[cfg(feature = "WiStars")]
    WiStars,
    #[cfg(feature = "WiStormShowers")]
    WiStormShowers,
    #[cfg(feature = "WiStormWarning")]
    WiStormWarning,
    #[cfg(feature = "WiStrongWind")]
    WiStrongWind,
    #[cfg(feature = "WiSunrise")]
    WiSunrise,
    #[cfg(feature = "WiSunset")]
    WiSunset,
    #[cfg(feature = "WiThermometer")]
    WiThermometer,
    #[cfg(feature = "WiThermometerExterior")]
    WiThermometerExterior,
    #[cfg(feature = "WiThermometerInternal")]
    WiThermometerInternal,
    #[cfg(feature = "WiThunderstorm")]
    WiThunderstorm,
    #[cfg(feature = "WiTime1")]
    WiTime1,
    #[cfg(feature = "WiTime10")]
    WiTime10,
    #[cfg(feature = "WiTime11")]
    WiTime11,
    #[cfg(feature = "WiTime12")]
    WiTime12,
    #[cfg(feature = "WiTime2")]
    WiTime2,
    #[cfg(feature = "WiTime3")]
    WiTime3,
    #[cfg(feature = "WiTime4")]
    WiTime4,
    #[cfg(feature = "WiTime5")]
    WiTime5,
    #[cfg(feature = "WiTime6")]
    WiTime6,
    #[cfg(feature = "WiTime7")]
    WiTime7,
    #[cfg(feature = "WiTime8")]
    WiTime8,
    #[cfg(feature = "WiTime9")]
    WiTime9,
    #[cfg(feature = "WiTornado")]
    WiTornado,
    #[cfg(feature = "WiTrain")]
    WiTrain,
    #[cfg(feature = "WiTsunami")]
    WiTsunami,
    #[cfg(feature = "WiUmbrella")]
    WiUmbrella,
    #[cfg(feature = "WiVolcano")]
    WiVolcano,
    #[cfg(feature = "WiWindBeaufort0")]
    WiWindBeaufort0,
    #[cfg(feature = "WiWindBeaufort1")]
    WiWindBeaufort1,
    #[cfg(feature = "WiWindBeaufort10")]
    WiWindBeaufort10,
    #[cfg(feature = "WiWindBeaufort11")]
    WiWindBeaufort11,
    #[cfg(feature = "WiWindBeaufort12")]
    WiWindBeaufort12,
    #[cfg(feature = "WiWindBeaufort2")]
    WiWindBeaufort2,
    #[cfg(feature = "WiWindBeaufort3")]
    WiWindBeaufort3,
    #[cfg(feature = "WiWindBeaufort4")]
    WiWindBeaufort4,
    #[cfg(feature = "WiWindBeaufort5")]
    WiWindBeaufort5,
    #[cfg(feature = "WiWindBeaufort6")]
    WiWindBeaufort6,
    #[cfg(feature = "WiWindBeaufort7")]
    WiWindBeaufort7,
    #[cfg(feature = "WiWindBeaufort8")]
    WiWindBeaufort8,
    #[cfg(feature = "WiWindBeaufort9")]
    WiWindBeaufort9,
    #[cfg(feature = "WiWindDeg")]
    WiWindDeg,
    #[cfg(feature = "WiWindy")]
    WiWindy,
}
#[cfg(feature = "WiAlien")]
const WI_ALIEN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M8.75,15.54c-1.12-2.4-0.95-4.66,0.52-6.79c1.03-1.48,2.6-2.39,4.73-2.72c0.16-0.04,0.34-0.07,0.54-0.08h0.63\n\tc2.91,0.09,5.05,1.38,6.4,3.88c0.64,1.18,0.8,2.48,0.48,3.91c-0.26,1.13-0.68,2.19-1.28,3.17c-1.29,2.01-2.63,3.64-4,4.88\n\tc-0.07,0.07-0.17,0.16-0.3,0.26c-0.46,0.35-0.89,0.53-1.28,0.54s-0.83-0.14-1.31-0.45c-0.29-0.17-0.53-0.37-0.74-0.59\n\tC11.18,19.55,9.71,17.55,8.75,15.54z M8.86,13.33c0.02,0.11,0.05,0.25,0.09,0.44s0.07,0.32,0.09,0.4c0.28,1.26,0.86,2.23,1.73,2.93\n\tc0.88,0.7,1.96,1.11,3.26,1.23c0.29,0.03,0.46,0.02,0.51-0.03s0.08-0.23,0.09-0.52c-0.01-0.08-0.03-0.21-0.05-0.39\n\tc-0.02-0.18-0.04-0.31-0.06-0.39c-0.25-1.34-0.88-2.32-1.9-2.93c-0.18-0.11-0.39-0.22-0.62-0.34s-0.44-0.2-0.61-0.27\n\tc-0.17-0.07-0.4-0.16-0.69-0.27c-0.29-0.11-0.5-0.19-0.63-0.25c-0.16-0.06-0.42-0.1-0.8-0.11C8.95,12.83,8.81,13,8.86,13.33z\n\t M15.66,17.73c-0.02,0.31,0,0.49,0.06,0.56c0.07,0.07,0.25,0.08,0.55,0.04c0.38-0.04,0.78-0.12,1.2-0.22\n\tc1.07-0.27,1.94-0.84,2.62-1.71c0.34-0.41,0.6-0.86,0.77-1.34s0.34-1.05,0.47-1.72c0.05-0.23,0.04-0.38-0.03-0.46\n\tc-0.07-0.08-0.22-0.11-0.44-0.08c-0.59,0.1-1.12,0.23-1.59,0.4c-1.15,0.43-2.02,1.01-2.62,1.74C16.05,15.68,15.72,16.6,15.66,17.73z\n\t\" />",
};
#[cfg(feature = "WiBarometer")]
const WI_BAROMETER: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.69,13.2c0-0.99,0.19-1.94,0.58-2.85c0.39-0.91,0.91-1.68,1.57-2.33s1.44-1.17,2.34-1.56c0.9-0.39,1.85-0.58,2.84-0.58\n\tc0.99,0,1.94,0.19,2.85,0.58c0.9,0.39,1.68,0.91,2.33,1.56c0.65,0.65,1.17,1.43,1.56,2.33s0.58,1.85,0.58,2.85\n\tc0,1.62-0.48,3.06-1.44,4.34c-0.96,1.27-2.2,2.14-3.71,2.61v3.29h-4.24v-3.25c-1.54-0.45-2.81-1.32-3.79-2.61S7.69,14.83,7.69,13.2z\n\t M9.3,13.2c0,1.55,0.56,2.88,1.69,3.99c1.11,1.12,2.45,1.68,4.02,1.68c1.03,0,1.99-0.25,2.86-0.76c0.88-0.51,1.57-1.2,2.09-2.07\n\tc0.51-0.87,0.77-1.82,0.77-2.85c0-0.77-0.15-1.5-0.45-2.21s-0.71-1.31-1.22-1.82c-0.51-0.51-1.12-0.92-1.83-1.22\n\tc-0.71-0.3-1.44-0.45-2.21-0.45c-0.77,0-1.5,0.15-2.21,0.45s-1.31,0.71-1.82,1.22c-0.51,0.51-0.92,1.12-1.22,1.82\n\tC9.45,11.7,9.3,12.43,9.3,13.2z M9.88,13.56v-0.72h2.17v0.72H9.88z M10.97,10.02l0.52-0.52l1.52,1.52l-0.52,0.53L10.97,10.02z\n\t M13.5,14.95c0-0.42,0.15-0.78,0.44-1.09c0.29-0.31,0.65-0.47,1.06-0.48l2.73-4.49l0.66,0.35l-2.02,4.83\n\tc0.18,0.25,0.26,0.54,0.26,0.88c0,0.44-0.15,0.81-0.46,1.11c-0.31,0.3-0.68,0.45-1.12,0.45c-0.43,0-0.8-0.15-1.1-0.45\n\tC13.65,15.76,13.5,15.39,13.5,14.95z M14.81,10.28V8.12h0.69v2.17H14.81z M17.75,13.55v-0.74h2.17v0.74H17.75z\" />",
};
#[cfg(feature = "WiCelsius")]
const WI_CELSIUS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.75,10.98c0-0.5,0.18-0.93,0.53-1.28c0.36-0.36,0.78-0.53,1.28-0.53c0.49,0,0.92,0.18,1.27,0.53\n\tc0.35,0.36,0.53,0.78,0.53,1.28c0,0.5-0.18,0.93-0.53,1.28c-0.35,0.36-0.78,0.53-1.27,0.53c-0.5,0-0.93-0.18-1.28-0.53\n\tS9.75,11.48,9.75,10.98z M10.63,10.98c0,0.26,0.09,0.48,0.27,0.67c0.19,0.19,0.41,0.28,0.67,0.28c0.26,0,0.48-0.09,0.67-0.28\n\ts0.28-0.41,0.28-0.67c0-0.26-0.09-0.48-0.28-0.67s-0.41-0.28-0.67-0.28c-0.26,0-0.48,0.09-0.67,0.28\n\tC10.72,10.49,10.63,10.72,10.63,10.98z M14.52,15.4c0,0.77,0.21,1.45,0.64,2.05c0.22,0.31,0.53,0.56,0.93,0.75\n\tc0.39,0.18,0.84,0.28,1.34,0.28c1.46,0,2.38-0.56,2.75-1.67c0.04-0.14,0.02-0.28-0.06-0.41c-0.08-0.13-0.19-0.2-0.33-0.23\n\tc-0.14-0.04-0.28-0.02-0.4,0.07c-0.12,0.08-0.2,0.19-0.23,0.34c0,0.01,0,0.02-0.01,0.05l-0.02,0.07c-0.11,0.19-0.26,0.34-0.45,0.45\n\tc-0.31,0.19-0.72,0.28-1.23,0.28c-0.31,0-0.59-0.05-0.83-0.16c-0.4-0.17-0.68-0.47-0.85-0.89c-0.11-0.27-0.17-0.6-0.17-0.97v-3.22\n\tc0-0.15,0.01-0.3,0.03-0.45c0.04-0.38,0.19-0.73,0.45-1.04c0.29-0.35,0.75-0.52,1.38-0.52c0.52,0,0.93,0.09,1.23,0.27\n\tc0.2,0.12,0.35,0.27,0.45,0.45c0.01,0.02,0.01,0.05,0.02,0.08c0.01,0.03,0.01,0.05,0.01,0.06c0.04,0.14,0.12,0.24,0.23,0.3\n\tc0.12,0.07,0.25,0.08,0.4,0.05c0.14-0.03,0.25-0.11,0.33-0.23c0.08-0.12,0.1-0.25,0.06-0.4v-0.01l-0.08-0.23\n\tc-0.05-0.11-0.14-0.26-0.28-0.43c-0.13-0.18-0.29-0.32-0.45-0.44c-0.21-0.15-0.48-0.27-0.82-0.38c-0.34-0.1-0.71-0.15-1.11-0.15\n\tc-0.51,0-0.95,0.09-1.35,0.27c-0.39,0.18-0.7,0.42-0.91,0.73c-0.43,0.59-0.65,1.28-0.65,2.07V15.4z\" />",
};
#[cfg(feature = "WiCloud")]
const WI_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.61,16.88c0-1.15,0.36-2.17,1.08-3.07c0.72-0.9,1.63-1.48,2.74-1.73c0.31-1.37,1.02-2.49,2.11-3.37s2.35-1.32,3.76-1.32\n\tc1.38,0,2.61,0.43,3.69,1.28s1.78,1.95,2.1,3.29h0.33c0.9,0,1.73,0.22,2.49,0.65s1.37,1.03,1.81,1.79c0.44,0.76,0.67,1.58,0.67,2.48\n\tc0,0.88-0.21,1.7-0.63,2.45s-1,1.35-1.73,1.8c-0.73,0.45-1.54,0.69-2.41,0.72H9.41c-1.34-0.06-2.47-0.57-3.4-1.53\n\tC5.08,19.37,4.61,18.22,4.61,16.88z M6.32,16.88c0,0.87,0.3,1.62,0.9,2.26s1.33,0.98,2.19,1.03h11.19c0.86-0.04,1.59-0.39,2.19-1.03\n\tc0.61-0.64,0.91-1.4,0.91-2.26c0-0.88-0.33-1.63-0.98-2.27c-0.65-0.64-1.42-0.96-2.32-0.96H18.8c-0.11,0-0.17-0.06-0.17-0.18\n\tl-0.07-0.57c-0.11-1.08-0.58-1.99-1.4-2.72c-0.82-0.73-1.77-1.1-2.86-1.1c-1.09,0-2.05,0.37-2.85,1.1\n\tc-0.81,0.73-1.27,1.64-1.37,2.72l-0.08,0.57c0,0.12-0.07,0.18-0.2,0.18H9.27c-0.84,0.1-1.54,0.46-2.1,1.07S6.32,16.05,6.32,16.88z\" />",
};
#[cfg(feature = "WiCloudDown")]
const WI_CLOUD_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.61,16.88c0,1.34,0.47,2.48,1.4,3.44c0.93,0.96,2.07,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.34\n\tc0-0.12-0.06-0.18-0.17-0.18c-0.86-0.04-1.59-0.38-2.19-1.02c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.85-2.17\n\tc0.57-0.62,1.27-0.97,2.1-1.07L9.8,13.6c0.13,0,0.2-0.06,0.2-0.17l0.08-0.55c0.1-1.08,0.55-1.99,1.36-2.71\n\tc0.81-0.73,1.76-1.09,2.86-1.09c1.09,0,2.04,0.36,2.86,1.09c0.82,0.73,1.28,1.63,1.4,2.71l0.07,0.58c0,0.11,0.06,0.17,0.17,0.17\n\th1.62c0.89,0,1.66,0.32,2.31,0.97c0.65,0.64,0.98,1.4,0.98,2.28c0,0.87-0.3,1.62-0.91,2.26c-0.61,0.64-1.34,0.98-2.19,1.02\n\tc-0.13,0-0.19,0.06-0.19,0.18v1.34c0,0.11,0.06,0.17,0.19,0.17c0.88-0.02,1.68-0.26,2.41-0.72c0.73-0.45,1.31-1.05,1.73-1.8\n\ts0.63-1.57,0.63-2.45c0-0.9-0.22-1.73-0.67-2.48c-0.44-0.76-1.05-1.35-1.81-1.79s-1.59-0.65-2.49-0.65h-0.33\n\tc-0.33-1.34-1.03-2.43-2.1-3.29s-2.31-1.28-3.69-1.28c-1.41,0-2.67,0.44-3.76,1.31s-1.8,2-2.11,3.37c-1.1,0.26-2.01,0.84-2.73,1.74\n\tS4.61,15.73,4.61,16.88z M11.58,18.4c0,0.24,0.08,0.44,0.24,0.6l2.59,2.61c0.12,0.16,0.32,0.23,0.57,0.23\n\tc0.28,0,0.48-0.08,0.61-0.23l2.6-2.61c0.16-0.17,0.24-0.38,0.24-0.6c0-0.23-0.08-0.43-0.24-0.58s-0.36-0.23-0.6-0.23\n\tc-0.24,0-0.44,0.08-0.62,0.23l-1.12,1.11v-3.98c0-0.24-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.23-0.61-0.23s-0.43,0.08-0.59,0.23\n\tc-0.16,0.16-0.23,0.35-0.23,0.59v3.98l-1.1-1.11c-0.18-0.16-0.38-0.23-0.63-0.23c-0.25,0-0.45,0.08-0.61,0.23\n\tC11.66,17.97,11.58,18.17,11.58,18.4z\" />",
};
#[cfg(feature = "WiCloudRefresh")]
const WI_CLOUD_REFRESH: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.63,16.91c0,0.66,0.12,1.28,0.38,1.88c0.25,0.6,0.59,1.11,1.02,1.55c0.43,0.43,0.94,0.79,1.53,1.05\n\tc0.59,0.27,1.21,0.42,1.87,0.45c0.11,0,0.17-0.06,0.17-0.17v-1.33c0-0.12-0.06-0.19-0.17-0.19c-0.87-0.06-1.6-0.41-2.19-1.03\n\ts-0.89-1.36-0.89-2.21c0-0.84,0.28-1.57,0.85-2.19c0.57-0.62,1.26-0.97,2.1-1.04l0.52-0.07c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52\n\tc0.07-0.71,0.3-1.36,0.69-1.95c0.39-0.58,0.9-1.04,1.52-1.37s1.29-0.49,2.01-0.49c1.09,0,2.05,0.36,2.86,1.08\n\tc0.82,0.72,1.28,1.62,1.39,2.7l0.06,0.57c0,0.12,0.06,0.18,0.19,0.18h1.61c0.9,0,1.67,0.32,2.32,0.97c0.64,0.64,0.97,1.41,0.97,2.3\n\tc0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.32,0.97-2.19,1.03c-0.13,0-0.2,0.06-0.2,0.19v1.33c0,0.11,0.07,0.17,0.2,0.17\n\tc1.34-0.06,2.47-0.57,3.39-1.51s1.38-2.09,1.38-3.42c0-0.89-0.22-1.72-0.67-2.48c-0.45-0.76-1.05-1.36-1.81-1.8\n\tc-0.76-0.44-1.59-0.67-2.48-0.67h-0.32c-0.33-1.33-1.04-2.42-2.11-3.28C16.9,7.82,15.67,7.4,14.3,7.4c-1.41,0-2.66,0.44-3.75,1.33\n\ts-1.8,2.01-2.11,3.38c-1.11,0.26-2.02,0.84-2.73,1.74C4.99,14.74,4.63,15.76,4.63,16.91z M10.86,18.18c0,0.74,0.19,1.43,0.56,2.07\n\ts0.88,1.14,1.51,1.51c0.63,0.38,1.32,0.56,2.06,0.56c1.15,0,2.13-0.41,2.95-1.22c0.82-0.82,1.23-1.79,1.23-2.92\n\tc0-0.23-0.08-0.43-0.25-0.6c-0.17-0.17-0.37-0.25-0.61-0.25c-0.24,0-0.44,0.08-0.61,0.25s-0.26,0.37-0.26,0.6\n\tc0,0.67-0.24,1.24-0.72,1.73c-0.48,0.48-1.05,0.72-1.73,0.72c-0.66,0-1.23-0.24-1.71-0.72c-0.48-0.48-0.72-1.06-0.72-1.73\n\tc0-0.6,0.18-1.13,0.53-1.6c0.36-0.47,0.79-0.73,1.31-0.77l-0.41,0.39c-0.15,0.15-0.23,0.34-0.23,0.57c0,0.25,0.07,0.47,0.23,0.66\n\tc0.14,0.15,0.31,0.23,0.53,0.23c0.22,0.01,0.45-0.07,0.7-0.23l1.82-1.87c0.17-0.17,0.25-0.36,0.25-0.58c0-0.25-0.08-0.45-0.25-0.61\n\tl-1.82-1.83c-0.19-0.18-0.39-0.26-0.62-0.26c-0.23,0-0.43,0.08-0.59,0.25c-0.16,0.17-0.24,0.37-0.24,0.61\n\tc0,0.24,0.07,0.43,0.23,0.58l0.35,0.36c-1,0.17-1.83,0.63-2.49,1.4C11.19,16.24,10.86,17.14,10.86,18.18z\" />",
};
#[cfg(feature = "WiCloudUp")]
const WI_CLOUD_UP: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,16.88c0,1.33,0.46,2.48,1.39,3.43c0.93,0.96,2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.33\n\tc0-0.12-0.06-0.19-0.17-0.19c-0.86-0.04-1.58-0.38-2.18-1.02s-0.9-1.39-0.9-2.25c0-0.82,0.28-1.54,0.84-2.16\n\tc0.56-0.61,1.26-0.97,2.1-1.07h0.53c0.13,0,0.2-0.06,0.2-0.18l0.06-0.57c0.11-1.08,0.57-1.99,1.38-2.72s1.77-1.1,2.86-1.1\n\tc1.08,0,2.03,0.37,2.85,1.1c0.82,0.73,1.28,1.64,1.39,2.72l0.08,0.57c0,0.12,0.06,0.18,0.18,0.18h1.61c0.89,0,1.66,0.32,2.31,0.96\n\ts0.98,1.4,0.98,2.26c0,0.86-0.3,1.61-0.9,2.25c-0.6,0.64-1.33,0.98-2.18,1.02c-0.13,0-0.2,0.06-0.2,0.19v1.33\n\tc0,0.11,0.07,0.17,0.2,0.17c0.87-0.02,1.67-0.26,2.4-0.71c0.73-0.45,1.31-1.05,1.73-1.8c0.42-0.75,0.63-1.57,0.63-2.44\n\tc0-0.67-0.13-1.31-0.39-1.91c-0.26-0.61-0.62-1.13-1.06-1.57c-0.44-0.44-0.97-0.79-1.58-1.05c-0.61-0.26-1.25-0.39-1.92-0.39h-0.32\n\tc-0.33-1.34-1.03-2.43-2.11-3.29c-1.07-0.85-2.3-1.28-3.68-1.28c-1.41,0-2.67,0.44-3.76,1.32s-1.79,2-2.1,3.36\n\tc-1.11,0.26-2.02,0.83-2.73,1.73C4.99,14.71,4.64,15.73,4.64,16.88z M11.58,17.51c0,0.25,0.08,0.46,0.24,0.64\n\tc0.15,0.15,0.35,0.23,0.61,0.23c0.24,0,0.45-0.08,0.62-0.23l1.11-1.14v3.98c0,0.24,0.08,0.44,0.23,0.61\n\tc0.16,0.17,0.35,0.25,0.59,0.25c0.23,0,0.43-0.08,0.6-0.25c0.17-0.17,0.25-0.37,0.25-0.61v-3.94l1.12,1.11\n\tc0.4,0.31,0.81,0.31,1.22,0c0.16-0.15,0.24-0.36,0.24-0.62c0-0.24-0.08-0.44-0.24-0.62l-2.59-2.57c-0.16-0.16-0.36-0.24-0.6-0.24\n\tc-0.24,0-0.44,0.08-0.59,0.24l-2.58,2.57C11.66,17.08,11.58,17.27,11.58,17.51z\" />",
};
#[cfg(feature = "WiCloudy")]
const WI_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.89,17.6c0-0.99,0.31-1.88,0.93-2.65s1.41-1.27,2.38-1.49c0.26-1.17,0.85-2.14,1.78-2.88c0.93-0.75,2-1.12,3.22-1.12\n\tc1.18,0,2.24,0.36,3.16,1.09c0.93,0.73,1.53,1.66,1.8,2.8h0.27c1.18,0,2.18,0.41,3.01,1.24s1.25,1.83,1.25,3\n\tc0,1.18-0.42,2.18-1.25,3.01s-1.83,1.25-3.01,1.25H8.16c-0.58,0-1.13-0.11-1.65-0.34S5.52,21,5.14,20.62\n\tc-0.38-0.38-0.68-0.84-0.91-1.36S3.89,18.17,3.89,17.6z M5.34,17.6c0,0.76,0.28,1.42,0.82,1.96s1.21,0.82,1.99,0.82h9.28\n\tc0.77,0,1.44-0.27,1.99-0.82c0.55-0.55,0.83-1.2,0.83-1.96c0-0.76-0.27-1.42-0.83-1.96c-0.55-0.54-1.21-0.82-1.99-0.82h-1.39\n\tc-0.1,0-0.15-0.05-0.15-0.15l-0.07-0.49c-0.1-0.94-0.5-1.73-1.19-2.35s-1.51-0.93-2.45-0.93c-0.94,0-1.76,0.31-2.46,0.94\n\tc-0.7,0.62-1.09,1.41-1.18,2.34l-0.07,0.42c0,0.1-0.05,0.15-0.16,0.15l-0.45,0.07c-0.72,0.06-1.32,0.36-1.81,0.89\n\tC5.59,16.24,5.34,16.87,5.34,17.6z M14.19,8.88c-0.1,0.09-0.08,0.16,0.07,0.21c0.43,0.19,0.79,0.37,1.08,0.55\n\tc0.11,0.03,0.19,0.02,0.22-0.03c0.61-0.57,1.31-0.86,2.12-0.86c0.81,0,1.5,0.27,2.1,0.81c0.59,0.54,0.92,1.21,0.99,2l0.09,0.64h1.42\n\tc0.65,0,1.21,0.23,1.68,0.7c0.47,0.47,0.7,1.02,0.7,1.66c0,0.6-0.21,1.12-0.62,1.57s-0.92,0.7-1.53,0.77c-0.1,0-0.15,0.05-0.15,0.16\n\tv1.13c0,0.11,0.05,0.16,0.15,0.16c1.01-0.06,1.86-0.46,2.55-1.19s1.04-1.6,1.04-2.6c0-1.06-0.37-1.96-1.12-2.7\n\tc-0.75-0.75-1.65-1.12-2.7-1.12h-0.15c-0.26-1-0.81-1.82-1.65-2.47c-0.83-0.65-1.77-0.97-2.8-0.97C16.28,7.29,15.11,7.82,14.19,8.88\n\tz\" />",
};
#[cfg(feature = "WiCloudyGusts")]
const WI_CLOUDY_GUSTS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.62,21.01c0-0.25,0.08-0.46,0.25-0.63c0.17-0.16,0.37-0.24,0.6-0.24h5.42c0.74,0,1.37,0.26,1.89,0.79\n\tc0.52,0.53,0.78,1.16,0.78,1.9c0,0.74-0.26,1.38-0.78,1.9c-0.52,0.52-1.15,0.78-1.89,0.78s-1.38-0.26-1.9-0.79\n\tc-0.16-0.16-0.23-0.36-0.23-0.6c0-0.24,0.08-0.45,0.23-0.6c0.15-0.16,0.35-0.24,0.6-0.24c0.23,0,0.43,0.08,0.61,0.24\n\tc0.2,0.19,0.43,0.29,0.69,0.29s0.49-0.1,0.68-0.29c0.19-0.19,0.29-0.42,0.29-0.7c0-0.26-0.1-0.49-0.29-0.68s-0.42-0.29-0.68-0.29\n\tH4.47c-0.23,0-0.43-0.08-0.6-0.25S3.62,21.25,3.62,21.01z M3.62,17.97c0-0.24,0.08-0.45,0.25-0.62c0.17-0.16,0.37-0.24,0.6-0.24\n\th10.55c0.26,0,0.49-0.1,0.68-0.29c0.19-0.19,0.29-0.43,0.29-0.69s-0.1-0.5-0.29-0.69c-0.19-0.19-0.42-0.29-0.68-0.29\n\tc-0.28,0-0.5,0.09-0.68,0.28c-0.18,0.15-0.39,0.23-0.64,0.23c-0.24,0-0.44-0.08-0.6-0.23c-0.15-0.15-0.23-0.35-0.23-0.6\n\tc0-0.25,0.07-0.45,0.23-0.61c0.51-0.51,1.15-0.76,1.92-0.76c0.74,0,1.38,0.26,1.9,0.78c0.52,0.52,0.78,1.15,0.78,1.88\n\ts-0.26,1.37-0.78,1.89c-0.52,0.52-1.15,0.78-1.9,0.78H4.47c-0.24,0-0.44-0.08-0.6-0.24C3.7,18.4,3.62,18.2,3.62,17.97z M5.77,15.61\n\tc0,0.08,0.05,0.12,0.16,0.12h1.44c0.08,0,0.15-0.05,0.22-0.15c0.22-0.54,0.58-0.99,1.05-1.35c0.48-0.36,1.01-0.56,1.59-0.6\n\tl0.53-0.08c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1c1.11,0,2.07,0.36,2.88,1.09\n\tc0.81,0.73,1.27,1.64,1.39,2.73l0.07,0.59c0,0.11,0.06,0.17,0.17,0.17h1.62c0.91,0,1.68,0.32,2.33,0.96c0.65,0.64,0.97,1.4,0.97,2.3\n\tc0,0.89-0.32,1.66-0.97,2.3c-0.65,0.64-1.42,0.96-2.33,0.96h-6.91c-0.12,0-0.19,0.06-0.19,0.17v1.39c0,0.11,0.06,0.17,0.19,0.17\n\th6.91c0.91,0,1.74-0.22,2.51-0.67c0.77-0.44,1.37-1.05,1.82-1.81c0.45-0.77,0.67-1.6,0.67-2.5c0-0.91-0.22-1.74-0.67-2.5\n\tc-0.45-0.76-1.05-1.37-1.82-1.81c-0.77-0.44-1.6-0.67-2.51-0.67h-0.31c-0.31-1.33-1.01-2.42-2.1-3.27\n\tc-1.08-0.85-2.33-1.27-3.73-1.27c-1.41,0-2.66,0.44-3.75,1.32s-1.78,2-2.07,3.37c-0.86,0.2-1.62,0.61-2.28,1.23\n\ts-1.12,1.36-1.38,2.21v0.04C5.77,15.56,5.77,15.58,5.77,15.61z\" />",
};
#[cfg(feature = "WiCloudyWindy")]
const WI_CLOUDY_WINDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.1,21.04c0-0.24,0.08-0.45,0.25-0.61s0.38-0.24,0.63-0.24h8.97c0.24,0,0.43,0.08,0.59,0.24c0.16,0.16,0.23,0.36,0.23,0.61\n\tc0,0.24-0.08,0.44-0.24,0.6c-0.16,0.16-0.35,0.24-0.59,0.24H3.98c-0.25,0-0.46-0.08-0.63-0.24S3.1,21.27,3.1,21.04z M5.73,17.98\n\tc0-0.24,0.09-0.44,0.27-0.6c0.14-0.15,0.34-0.23,0.59-0.23h9c0.23,0,0.42,0.08,0.58,0.23s0.23,0.35,0.23,0.59\n\tc0,0.24-0.08,0.44-0.23,0.61c-0.15,0.17-0.35,0.25-0.58,0.25h-9c-0.23,0-0.43-0.09-0.6-0.26S5.73,18.21,5.73,17.98z M6.35,15.65\n\tc0,0.09,0.06,0.14,0.17,0.14h1.43c0.09,0,0.17-0.05,0.23-0.14c0.23-0.54,0.57-0.99,1.04-1.35s0.99-0.56,1.58-0.6l0.54-0.07\n\tc0.11,0,0.17-0.06,0.17-0.18l0.07-0.52c0.11-1.09,0.58-1.99,1.39-2.72c0.82-0.73,1.77-1.09,2.87-1.09c1.09,0,2.03,0.36,2.83,1.07\n\tc0.8,0.72,1.27,1.62,1.41,2.7l0.07,0.59c0,0.11,0.06,0.16,0.18,0.16h1.6c0.91,0,1.68,0.32,2.32,0.96c0.64,0.64,0.96,1.41,0.96,2.32\n\tc0,0.88-0.33,1.64-0.97,2.28c-0.65,0.65-1.42,0.97-2.31,0.97h-6.89c-0.12,0-0.18,0.06-0.18,0.17v1.34c0,0.12,0.06,0.18,0.18,0.18\n\th6.89c0.68,0,1.32-0.13,1.94-0.39s1.14-0.61,1.58-1.05s0.79-0.97,1.05-1.58s0.39-1.25,0.39-1.92c0-0.9-0.22-1.73-0.66-2.49\n\tc-0.44-0.76-1.04-1.36-1.8-1.8c-0.76-0.44-1.6-0.66-2.5-0.66h-0.31c-0.33-1.34-1.03-2.44-2.1-3.3c-1.08-0.85-2.3-1.28-3.68-1.28\n\tc-1.42,0-2.67,0.44-3.76,1.33c-1.09,0.88-1.78,2.01-2.08,3.39c-0.86,0.19-1.62,0.6-2.27,1.21s-1.1,1.35-1.36,2.22v0.02\n\tC6.36,15.6,6.35,15.62,6.35,15.65z M7.5,24.13c0-0.24,0.09-0.44,0.26-0.6c0.15-0.16,0.35-0.23,0.59-0.23h8.99\n\tc0.24,0,0.45,0.08,0.61,0.24c0.17,0.16,0.25,0.36,0.25,0.6c0,0.24-0.08,0.44-0.25,0.61c-0.17,0.17-0.37,0.25-0.61,0.25H8.35\n\tc-0.23,0-0.43-0.08-0.6-0.25C7.58,24.57,7.5,24.37,7.5,24.13z\" />",
};
#[cfg(feature = "WiDayCloudy")]
const WI_DAY_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.56,16.9c0,0.9,0.22,1.73,0.66,2.49s1.04,1.36,1.8,1.8c0.76,0.44,1.58,0.66,2.47,0.66h10.83c0.89,0,1.72-0.22,2.48-0.66\n\tc0.76-0.44,1.37-1.04,1.81-1.8c0.44-0.76,0.67-1.59,0.67-2.49c0-0.66-0.14-1.33-0.42-2C22.62,13.98,23,12.87,23,11.6\n\tc0-0.71-0.14-1.39-0.41-2.04c-0.27-0.65-0.65-1.2-1.12-1.67C21,7.42,20.45,7.04,19.8,6.77c-0.65-0.28-1.33-0.41-2.04-0.41\n\tc-1.48,0-2.77,0.58-3.88,1.74c-0.77-0.44-1.67-0.66-2.7-0.66c-1.41,0-2.65,0.44-3.73,1.31c-1.08,0.87-1.78,1.99-2.08,3.35\n\tc-1.12,0.26-2.03,0.83-2.74,1.73S1.56,15.75,1.56,16.9z M3.27,16.9c0-0.84,0.28-1.56,0.84-2.17c0.56-0.61,1.26-0.96,2.1-1.06\n\tl0.5-0.03c0.12,0,0.19-0.06,0.19-0.18l0.07-0.54c0.14-1.08,0.61-1.99,1.41-2.71c0.8-0.73,1.74-1.09,2.81-1.09\n\tc1.1,0,2.06,0.37,2.87,1.1c0.82,0.73,1.27,1.63,1.37,2.71l0.07,0.58c0.02,0.11,0.09,0.17,0.21,0.17h1.61c0.88,0,1.64,0.32,2.28,0.96\n\tc0.64,0.64,0.96,1.39,0.96,2.27c0,0.91-0.32,1.68-0.95,2.32c-0.63,0.64-1.4,0.96-2.28,0.96H6.49c-0.88,0-1.63-0.32-2.27-0.97\n\tC3.59,18.57,3.27,17.8,3.27,16.9z M9.97,4.63c0,0.24,0.08,0.45,0.24,0.63l0.66,0.64c0.25,0.19,0.46,0.27,0.64,0.25\n\tc0.21,0,0.39-0.09,0.55-0.26s0.24-0.38,0.24-0.62c0-0.24-0.09-0.44-0.26-0.59l-0.59-0.66c-0.18-0.16-0.38-0.24-0.61-0.24\n\tc-0.24,0-0.45,0.08-0.62,0.25C10.05,4.19,9.97,4.39,9.97,4.63z M15.31,9.06c0.69-0.67,1.51-1,2.45-1c0.99,0,1.83,0.34,2.52,1.03\n\tc0.69,0.69,1.04,1.52,1.04,2.51c0,0.62-0.17,1.24-0.51,1.84C19.84,12.48,18.68,12,17.32,12H17C16.75,10.91,16.19,9.93,15.31,9.06z\n\t M16.94,3.78c0,0.26,0.08,0.46,0.23,0.62s0.35,0.23,0.59,0.23c0.26,0,0.46-0.08,0.62-0.23c0.16-0.16,0.23-0.36,0.23-0.62V1.73\n\tc0-0.24-0.08-0.43-0.24-0.59s-0.36-0.23-0.61-0.23c-0.24,0-0.43,0.08-0.59,0.23s-0.23,0.35-0.23,0.59V3.78z M22.46,6.07\n\tc0,0.26,0.07,0.46,0.22,0.62c0.21,0.16,0.42,0.24,0.62,0.24c0.18,0,0.38-0.08,0.59-0.24l1.43-1.43c0.16-0.18,0.24-0.39,0.24-0.64\n\tc0-0.24-0.08-0.44-0.24-0.6c-0.16-0.16-0.36-0.24-0.59-0.24c-0.24,0-0.43,0.08-0.58,0.24l-1.47,1.43\n\tC22.53,5.64,22.46,5.84,22.46,6.07z M23.25,17.91c0,0.24,0.08,0.45,0.25,0.63l0.65,0.63c0.15,0.16,0.34,0.24,0.58,0.24\n\ts0.44-0.08,0.6-0.25c0.16-0.17,0.24-0.37,0.24-0.62c0-0.22-0.08-0.42-0.24-0.58l-0.65-0.65c-0.16-0.16-0.35-0.24-0.57-0.24\n\tc-0.24,0-0.44,0.08-0.6,0.24C23.34,17.47,23.25,17.67,23.25,17.91z M24.72,11.6c0,0.23,0.09,0.42,0.26,0.58\n\tc0.16,0.16,0.37,0.24,0.61,0.24h2.04c0.23,0,0.42-0.08,0.58-0.23s0.23-0.35,0.23-0.59c0-0.24-0.08-0.44-0.23-0.6\n\ts-0.35-0.25-0.58-0.25h-2.04c-0.24,0-0.44,0.08-0.61,0.25C24.8,11.17,24.72,11.37,24.72,11.6z\" />",
};
#[cfg(feature = "WiDayCloudyGusts")]
const WI_DAY_CLOUDY_GUSTS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M0.35,21.01c0,0.24,0.09,0.44,0.27,0.6c0.17,0.17,0.37,0.25,0.61,0.25h5.88c0.26,0,0.48,0.09,0.68,0.28\n\tc0.2,0.19,0.3,0.42,0.3,0.68s-0.1,0.5-0.3,0.69c-0.2,0.19-0.42,0.29-0.68,0.29c-0.26,0-0.48-0.1-0.68-0.3\n\tc-0.18-0.16-0.38-0.24-0.61-0.24c-0.24,0-0.44,0.08-0.6,0.24c-0.16,0.16-0.24,0.36-0.24,0.6c0,0.24,0.08,0.44,0.24,0.6\n\tc0.53,0.53,1.16,0.8,1.89,0.8c0.74,0,1.37-0.26,1.88-0.78c0.52-0.52,0.78-1.15,0.78-1.89s-0.26-1.37-0.78-1.89\n\tc-0.52-0.53-1.15-0.79-1.88-0.79H1.23c-0.24,0-0.44,0.09-0.62,0.26S0.35,20.78,0.35,21.01z M0.35,18c0,0.22,0.09,0.41,0.27,0.57\n\tc0.17,0.17,0.37,0.25,0.61,0.25H12.2c0.74,0,1.37-0.26,1.89-0.78c0.52-0.52,0.78-1.15,0.78-1.89c0-0.74-0.26-1.36-0.78-1.88\n\tc-0.52-0.51-1.15-0.77-1.89-0.77c-0.76,0-1.38,0.25-1.88,0.76c-0.16,0.16-0.23,0.37-0.23,0.61c0,0.24,0.08,0.44,0.23,0.6\n\tc0.15,0.15,0.35,0.23,0.6,0.23c0.24,0,0.44-0.08,0.62-0.23c0.19-0.19,0.41-0.28,0.67-0.28c0.26,0,0.49,0.09,0.68,0.28\n\tc0.19,0.19,0.29,0.42,0.29,0.68c0,0.27-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.68,0.29H1.23c-0.24,0-0.44,0.09-0.62,0.26\n\tC0.43,17.57,0.35,17.77,0.35,18z M3,15.65c0,0.08,0.06,0.12,0.17,0.12H4.6c0.07,0,0.13-0.05,0.2-0.14c0.22-0.54,0.57-0.99,1.05-1.35\n\tc0.47-0.35,1-0.55,1.6-0.6l0.53-0.07c0.12,0,0.19-0.06,0.19-0.17l0.07-0.52c0.11-1.08,0.56-1.98,1.37-2.71\n\tc0.81-0.73,1.76-1.09,2.85-1.09c1.1,0,2.05,0.36,2.86,1.08s1.27,1.63,1.38,2.71l0.07,0.58c0,0.12,0.06,0.18,0.18,0.18h1.63\n\tc0.9,0,1.67,0.31,2.3,0.94c0.63,0.63,0.95,1.39,0.95,2.27c0,0.89-0.32,1.66-0.95,2.29c-0.63,0.63-1.4,0.95-2.3,0.95h-6.9\n\tc-0.13,0-0.19,0.06-0.19,0.18v1.37c0,0.11,0.06,0.17,0.19,0.17h6.9c0.89,0,1.72-0.22,2.48-0.67c0.76-0.44,1.36-1.05,1.8-1.81\n\tc0.44-0.76,0.66-1.59,0.66-2.48c0-0.74-0.14-1.41-0.42-2.03c0.76-0.99,1.13-2.1,1.13-3.31c0-0.94-0.24-1.81-0.71-2.62\n\ts-1.11-1.45-1.92-1.92c-0.81-0.47-1.68-0.71-2.62-0.71c-1.54,0-2.83,0.58-3.86,1.73c-0.8-0.41-1.69-0.61-2.67-0.61\n\tc-1.41,0-2.65,0.44-3.73,1.31s-1.77,1.99-2.06,3.34c-0.85,0.2-1.6,0.61-2.25,1.23c-0.65,0.62-1.11,1.35-1.36,2.19v0.04\n\tC3.01,15.59,3,15.62,3,15.65z M11.18,4.62c0,0.23,0.09,0.43,0.27,0.6l0.61,0.65c0.16,0.16,0.37,0.24,0.61,0.24\n\tc0.25,0,0.45-0.08,0.61-0.23c0.16-0.15,0.24-0.35,0.24-0.6c0-0.24-0.07-0.44-0.23-0.6l-0.66-0.65c-0.16-0.17-0.35-0.26-0.59-0.26\n\ts-0.44,0.08-0.61,0.25C11.26,4.19,11.18,4.39,11.18,4.62z M16.5,9.03c0.72-0.68,1.54-1.02,2.48-1.02c0.97,0,1.8,0.35,2.51,1.05\n\tc0.7,0.7,1.05,1.54,1.05,2.51c0,0.65-0.17,1.26-0.52,1.83c-0.96-0.96-2.11-1.43-3.46-1.43h-0.34C17.99,10.88,17.41,9.9,16.5,9.03z\n\t M18.12,3.79c0,0.23,0.08,0.43,0.25,0.59c0.17,0.16,0.37,0.24,0.6,0.24c0.24,0,0.44-0.08,0.61-0.24c0.17-0.16,0.25-0.35,0.25-0.59\n\tV1.74c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.25-0.61-0.25c-0.24,0-0.44,0.08-0.6,0.25s-0.25,0.37-0.25,0.61V3.79z\n\t M23.67,6.06c0,0.24,0.08,0.44,0.23,0.6c0.17,0.17,0.37,0.25,0.61,0.26s0.43-0.08,0.57-0.26l1.46-1.43\n\tc0.17-0.17,0.25-0.37,0.25-0.61c0-0.23-0.08-0.43-0.25-0.6c-0.17-0.17-0.37-0.25-0.61-0.25S25.49,3.84,25.33,4L23.9,5.47\n\tC23.75,5.63,23.67,5.82,23.67,6.06z M24.44,17.89c0,0.23,0.08,0.43,0.25,0.6l0.64,0.65c0.2,0.16,0.41,0.24,0.62,0.24\n\tc0.19,0,0.39-0.08,0.59-0.24c0.17-0.17,0.25-0.37,0.25-0.6c0-0.22-0.08-0.42-0.25-0.61l-0.64-0.65c-0.16-0.16-0.36-0.24-0.58-0.24\n\tc-0.25,0-0.46,0.08-0.63,0.25C24.52,17.45,24.44,17.65,24.44,17.89z M25.95,11.57c0,0.24,0.08,0.43,0.25,0.59\n\tc0.15,0.18,0.34,0.26,0.57,0.26h2.02c0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6c0-0.23-0.09-0.43-0.26-0.6\n\tc-0.17-0.17-0.37-0.26-0.6-0.26h-2.02c-0.24,0-0.43,0.08-0.59,0.25C26.03,11.13,25.95,11.33,25.95,11.57z\" />",
};
#[cfg(feature = "WiDayCloudyHigh")]
const WI_DAY_CLOUDY_HIGH: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.95,13.05c0-0.93,0.29-1.75,0.87-2.48s1.31-1.2,2.19-1.4c0.26-1.1,0.82-2,1.7-2.71s1.88-1.06,3.01-1.06\n\tc1.1,0,2.08,0.35,2.95,1.04s1.43,1.57,1.68,2.65h0.26c1.1,0,2.04,0.39,2.82,1.16c0.78,0.77,1.17,1.71,1.17,2.81\n\tc0,0.01,0,0.02,0,0.04c0,0.02,0,0.04,0,0.06c0.75,0.8,1.12,1.75,1.12,2.85c0,0.76-0.19,1.46-0.57,2.1\n\tc-0.38,0.65-0.89,1.16-1.53,1.53c-0.64,0.38-1.34,0.56-2.09,0.56c-0.96,0-1.82-0.3-2.56-0.89s-1.24-1.35-1.48-2.26H7.79\n\tC6.72,17,5.81,16.59,5.07,15.82S3.95,14.12,3.95,13.05z M5.31,13.05c0,0.7,0.24,1.31,0.73,1.82s1.07,0.79,1.75,0.82h8.99\n\tc0.68-0.03,1.27-0.3,1.75-0.82c0.49-0.52,0.73-1.12,0.73-1.82c0-0.71-0.26-1.32-0.79-1.83c-0.53-0.52-1.14-0.77-1.86-0.77h-1.29\n\tc-0.09,0-0.14-0.05-0.14-0.14l-0.07-0.47c-0.09-0.87-0.46-1.6-1.12-2.19s-1.42-0.89-2.28-0.89c-0.89,0-1.66,0.29-2.31,0.88\n\tS8.4,8.96,8.31,9.83L8.25,10.3c0,0.09-0.05,0.14-0.16,0.14h-0.4C7.02,10.52,6.45,10.8,6,11.3C5.54,11.79,5.31,12.38,5.31,13.05z\n\t M11.51,22.06c-0.25-0.33-0.25-0.65,0-0.98l1.13-1.15c0.14-0.12,0.31-0.18,0.52-0.18c0.19,0,0.34,0.06,0.46,0.18\n\tc0.12,0.12,0.18,0.28,0.18,0.47c0,0.2-0.06,0.36-0.18,0.48l-1.14,1.18c-0.12,0.12-0.29,0.19-0.49,0.19\n\tC11.79,22.25,11.63,22.18,11.51,22.06z M14.9,17.04c0.21,0.54,0.56,0.97,1.04,1.3c0.48,0.33,1.01,0.5,1.6,0.5\n\tc0.77,0,1.43-0.28,1.97-0.83c0.54-0.56,0.81-1.23,0.81-2.02c0-0.39-0.06-0.74-0.19-1.05c-0.33,0.61-0.8,1.11-1.39,1.49\n\tc-0.6,0.38-1.25,0.58-1.96,0.61H14.9z M16.85,22.23c0-0.19,0.07-0.34,0.2-0.47c0.13-0.12,0.3-0.19,0.48-0.19\n\tc0.18,0,0.35,0.07,0.5,0.21c0.12,0.12,0.19,0.27,0.19,0.45v1.64c0,0.19-0.07,0.35-0.2,0.49c-0.13,0.14-0.3,0.21-0.48,0.21\n\ts-0.35-0.07-0.48-0.21c-0.13-0.14-0.2-0.3-0.2-0.49V22.23z M21.26,20.4c0-0.18,0.06-0.33,0.19-0.46c0.13-0.12,0.29-0.19,0.47-0.19\n\tc0.19,0,0.35,0.06,0.47,0.18l1.18,1.15c0.13,0.14,0.2,0.3,0.2,0.48c0,0.19-0.07,0.35-0.2,0.48c-0.13,0.13-0.3,0.2-0.49,0.2\n\tc-0.21,0-0.37-0.06-0.5-0.19l-1.13-1.18C21.32,20.73,21.26,20.57,21.26,20.4z M21.26,11.59c0-0.19,0.06-0.35,0.19-0.47l1.13-1.18\n\tc0.14-0.12,0.3-0.19,0.5-0.19c0.19,0,0.35,0.06,0.5,0.19c0.13,0.15,0.2,0.32,0.2,0.51c0,0.18-0.07,0.33-0.2,0.48l-1.18,1.15\n\tc-0.12,0.12-0.28,0.19-0.47,0.19s-0.35-0.06-0.47-0.19C21.32,11.94,21.26,11.78,21.26,11.59z M23.08,15.99\n\tc0-0.19,0.06-0.35,0.19-0.48c0.12-0.13,0.28-0.2,0.47-0.2h1.62c0.19,0,0.36,0.07,0.5,0.2s0.21,0.29,0.21,0.48\n\tc0,0.19-0.07,0.36-0.21,0.49c-0.14,0.13-0.3,0.2-0.5,0.2h-1.62c-0.19,0-0.34-0.07-0.47-0.2C23.14,16.35,23.08,16.19,23.08,15.99z\" />",
};
#[cfg(feature = "WiDayCloudyWindy")]
const WI_DAY_CLOUDY_WINDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M0.45,20.97c0,0.24,0.08,0.45,0.24,0.61c0.44,0.18,0.73,0.27,0.88,0.27h7.88c0.24,0,0.44-0.09,0.6-0.26\n\tc0.17-0.17,0.25-0.38,0.25-0.61c0-0.23-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.24-0.6-0.24H1.57c-0.26,0-0.52,0.08-0.76,0.24\n\tC0.57,20.55,0.45,20.75,0.45,20.97z M1.84,17.97c0,0.24,0.08,0.43,0.25,0.59c0.15,0.17,0.34,0.26,0.58,0.26h9.4\n\tc0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.25-0.61-0.25h-9.4\n\tc-0.23,0-0.43,0.08-0.59,0.25C1.92,17.53,1.84,17.73,1.84,17.97z M2.89,15.6c0,0.09,0.06,0.13,0.17,0.13h1.39\n\tc0.12,0,0.19-0.04,0.22-0.13c0.26-0.53,0.62-0.97,1.09-1.32c0.47-0.35,1-0.55,1.58-0.62h0.54c0.11,0,0.16-0.06,0.16-0.19l0.07-0.56\n\tc0.07-0.71,0.3-1.36,0.69-1.95c0.39-0.58,0.9-1.04,1.53-1.37s1.3-0.5,2.02-0.5c1.09,0,2.04,0.37,2.85,1.1s1.27,1.64,1.39,2.72\n\tl0.07,0.56c0,0.12,0.06,0.19,0.18,0.19h1.6c0.89,0,1.65,0.32,2.3,0.96c0.65,0.64,0.97,1.39,0.97,2.27c0,0.9-0.32,1.67-0.97,2.31\n\tc-0.64,0.64-1.41,0.96-2.31,0.96h-6.89c-0.11,0-0.17,0.06-0.17,0.19v1.33c0,0.12,0.06,0.18,0.17,0.18h6.89\n\tc0.9,0,1.73-0.22,2.49-0.67c0.76-0.44,1.37-1.05,1.81-1.81c0.44-0.76,0.67-1.59,0.67-2.49c0-0.73-0.14-1.39-0.43-2.01\n\tc0.78-0.96,1.16-2.06,1.16-3.28c0-0.94-0.24-1.81-0.71-2.62c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.63-0.71\n\tc-0.73,0-1.43,0.15-2.1,0.45c-0.67,0.3-1.25,0.71-1.74,1.25c-0.83-0.43-1.73-0.65-2.7-0.65c-1.41,0-2.67,0.44-3.76,1.31\n\ts-1.79,1.99-2.09,3.36c-0.85,0.21-1.6,0.63-2.25,1.25s-1.1,1.36-1.35,2.21C2.9,15.55,2.89,15.57,2.89,15.6z M3.6,24.16\n\tc0,0.24,0.09,0.43,0.26,0.59C4.02,24.92,4.22,25,4.45,25h9.42c0.23,0,0.43-0.08,0.59-0.25s0.24-0.36,0.24-0.6\n\tc0-0.25-0.08-0.46-0.24-0.62s-0.36-0.25-0.6-0.25H4.45c-0.24,0-0.44,0.08-0.6,0.25S3.6,23.91,3.6,24.16z M11.09,4.65\n\tc0,0.25,0.08,0.45,0.24,0.6l0.64,0.66c0.16,0.16,0.36,0.24,0.6,0.24c0.26,0,0.46-0.08,0.62-0.24c0.16-0.16,0.24-0.36,0.24-0.61\n\tc0-0.23-0.08-0.43-0.24-0.59l-0.65-0.65c-0.17-0.17-0.36-0.25-0.57-0.25c-0.25,0-0.46,0.08-0.63,0.25S11.09,4.42,11.09,4.65z\n\t M16.45,9.03c0.66-0.63,1.48-0.95,2.45-0.95c0.97,0,1.8,0.34,2.49,1.03c0.68,0.68,1.03,1.51,1.03,2.49c0,0.67-0.15,1.27-0.46,1.81\n\tc-0.94-0.95-2.11-1.43-3.5-1.43h-0.3C17.87,10.83,17.3,9.85,16.45,9.03z M18.05,3.81c0,0.24,0.08,0.43,0.25,0.59s0.36,0.23,0.6,0.23\n\tc0.25,0,0.45-0.08,0.6-0.23c0.15-0.15,0.23-0.35,0.23-0.6V1.76c0-0.24-0.08-0.45-0.23-0.61c-0.16-0.17-0.36-0.25-0.6-0.25\n\tc-0.23,0-0.43,0.08-0.6,0.25s-0.25,0.37-0.25,0.61V3.81z M23.57,6.09c0,0.24,0.08,0.44,0.25,0.6c0.12,0.16,0.33,0.24,0.6,0.24\n\tc0.27,0,0.47-0.08,0.59-0.24l1.46-1.44c0.16-0.15,0.24-0.36,0.24-0.62c0-0.23-0.08-0.43-0.25-0.6c-0.17-0.17-0.37-0.25-0.6-0.25\n\ts-0.44,0.08-0.61,0.23L23.83,5.5C23.66,5.67,23.57,5.86,23.57,6.09z M24.37,17.95c0,0.24,0.08,0.44,0.23,0.6l0.66,0.63\n\tc0.24,0.18,0.45,0.27,0.61,0.27c0.16,0,0.37-0.09,0.61-0.27c0.16-0.16,0.24-0.36,0.24-0.6c0-0.23-0.08-0.43-0.24-0.61l-0.64-0.61\n\tc-0.19-0.17-0.4-0.26-0.65-0.26c-0.24,0-0.43,0.08-0.59,0.24C24.45,17.51,24.37,17.71,24.37,17.95z M25.81,11.63\n\tc0,0.24,0.09,0.45,0.27,0.61c0.18,0.17,0.38,0.25,0.6,0.25h2.03c0.23,0,0.42-0.08,0.59-0.25c0.17-0.17,0.25-0.37,0.25-0.61\n\tc0-0.22-0.08-0.41-0.24-0.57c-0.16-0.15-0.36-0.23-0.59-0.23h-2.03c-0.24,0-0.45,0.08-0.62,0.23C25.9,11.22,25.81,11.41,25.81,11.63\n\tz\" />",
};
#[cfg(feature = "WiDayFog")]
const WI_DAY_FOG: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M0.32,21.06c0,0.23,0.08,0.43,0.25,0.59c0.17,0.16,0.38,0.24,0.63,0.24h18.71c0.24,0,0.44-0.08,0.61-0.24\n\tc0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.6c-0.17-0.17-0.37-0.25-0.61-0.25H1.2c-0.25,0-0.46,0.08-0.63,0.25\n\tS0.32,20.82,0.32,21.06z M2.94,17.92c0,0.23,0.08,0.43,0.25,0.58c0.17,0.18,0.37,0.27,0.6,0.27h18.72c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.6c0-0.23-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.24-0.59-0.24H3.8c-0.24,0-0.44,0.08-0.6,0.24\n\tC3.03,17.5,2.94,17.69,2.94,17.92z M3.07,15.52c0,0.09,0.05,0.13,0.16,0.13h1.43c0.07,0,0.14-0.05,0.21-0.16\n\tc0.24-0.52,0.59-0.94,1.06-1.27c0.47-0.33,0.99-0.52,1.56-0.56l0.54-0.07c0.11,0,0.17-0.06,0.17-0.18l0.07-0.51\n\tc0.11-1.08,0.56-1.98,1.37-2.71C10.45,9.47,11.4,9.1,12.5,9.1c1.08,0,2.03,0.36,2.84,1.08c0.81,0.72,1.27,1.61,1.38,2.68l0.07,0.58\n\tc0,0.11,0.06,0.17,0.19,0.17h1.61c0.64,0,1.23,0.17,1.76,0.52c0.53,0.34,0.92,0.8,1.18,1.37c0.07,0.11,0.13,0.16,0.2,0.16h1.44\n\tc0.13,0,0.18-0.07,0.13-0.23l-0.2-0.55c0.76-0.94,1.13-2.04,1.13-3.31c0-0.71-0.14-1.38-0.41-2.03s-0.64-1.2-1.11-1.67\n\tc-0.46-0.47-1.02-0.84-1.67-1.12S19.72,6.35,19,6.35c-1.54,0-2.82,0.56-3.82,1.68C14.33,7.61,13.44,7.4,12.5,7.4\n\tc-1.4,0-2.65,0.44-3.74,1.32s-1.79,2-2.1,3.37c-1.78,0.47-2.98,1.58-3.58,3.35C3.07,15.45,3.07,15.48,3.07,15.52z M4.69,24.13\n\tc0,0.24,0.09,0.44,0.27,0.6c0.16,0.17,0.35,0.26,0.59,0.26h18.74c0.23,0,0.43-0.08,0.6-0.25c0.17-0.17,0.25-0.37,0.25-0.61\n\tc0-0.23-0.08-0.42-0.25-0.58c-0.17-0.15-0.37-0.23-0.6-0.23H5.55c-0.24,0-0.44,0.08-0.61,0.23C4.77,23.71,4.69,23.9,4.69,24.13z\n\t M11.26,4.66c0,0.24,0.08,0.43,0.23,0.59l0.65,0.64c0.17,0.18,0.36,0.27,0.58,0.27c0.22,0,0.42-0.08,0.6-0.25\n\tc0.17-0.17,0.26-0.37,0.26-0.61c0-0.24-0.08-0.45-0.25-0.63l-0.64-0.61c-0.16-0.17-0.36-0.26-0.6-0.26c-0.24,0-0.44,0.08-0.6,0.25\n\tC11.34,4.21,11.26,4.42,11.26,4.66z M16.58,9.04c0.67-0.68,1.48-1.01,2.43-1.01c0.98,0,1.82,0.35,2.51,1.04\n\tc0.69,0.69,1.04,1.53,1.04,2.5c0,0.65-0.16,1.25-0.49,1.8c-0.95-0.95-2.11-1.42-3.47-1.42h-0.34C17.97,10.77,17.41,9.8,16.58,9.04z\n\t M18.18,3.81c0,0.23,0.08,0.43,0.24,0.59c0.16,0.16,0.35,0.24,0.59,0.24c0.25,0,0.46-0.08,0.63-0.24c0.17-0.16,0.25-0.35,0.25-0.59\n\tV1.76c0-0.23-0.09-0.43-0.26-0.6C19.45,0.99,19.24,0.91,19,0.91c-0.23,0-0.43,0.08-0.59,0.25c-0.16,0.17-0.24,0.37-0.24,0.6V3.81z\n\t M23.67,6.08c0,0.22,0.08,0.43,0.24,0.6c0.37,0.36,0.78,0.36,1.23,0l1.43-1.43c0.16-0.18,0.24-0.39,0.24-0.64\n\tc0-0.23-0.08-0.43-0.24-0.59c-0.16-0.16-0.36-0.24-0.59-0.24c-0.24,0-0.44,0.08-0.6,0.24l-1.46,1.47\n\tC23.75,5.67,23.67,5.87,23.67,6.08z M24.48,17.88c0,0.24,0.09,0.44,0.26,0.6l0.64,0.65c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.21,0,0.41-0.08,0.61-0.24c0.16-0.17,0.24-0.39,0.24-0.64c0-0.22-0.08-0.41-0.24-0.56l-0.65-0.66c-0.18-0.16-0.38-0.24-0.6-0.24\n\tc-0.24,0-0.44,0.08-0.6,0.25C24.56,17.44,24.48,17.64,24.48,17.88z M25.96,11.57c0,0.24,0.09,0.44,0.26,0.6\n\tc0.15,0.17,0.35,0.25,0.59,0.25h2.05c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.6c0-0.24-0.08-0.44-0.24-0.6\n\tc-0.16-0.16-0.35-0.24-0.59-0.24h-2.05c-0.24,0-0.44,0.08-0.6,0.25C26.04,11.14,25.96,11.34,25.96,11.57z\" />",
};
#[cfg(feature = "WiDayHail")]
const WI_DAY_HAIL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.48,16.88c0,1.34,0.47,2.49,1.4,3.45s2.07,1.47,3.4,1.53c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.12-0.06-0.18-0.18-0.18\n\tc-0.86-0.04-1.59-0.39-2.19-1.03s-0.9-1.4-0.9-2.26c0-0.83,0.28-1.55,0.85-2.17s1.27-0.97,2.1-1.07l0.53-0.04\n\tc0.13,0,0.2-0.06,0.2-0.18l0.07-0.55c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.86-1.09c1.09,0,2.04,0.36,2.86,1.09\n\tc0.82,0.73,1.29,1.63,1.4,2.71l0.07,0.58c0,0.12,0.06,0.19,0.17,0.19h1.62c0.89,0,1.67,0.32,2.32,0.96c0.65,0.64,0.98,1.4,0.98,2.27\n\tc0,0.87-0.3,1.62-0.91,2.26c-0.61,0.64-1.34,0.98-2.19,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc1.34-0.04,2.47-0.55,3.39-1.51c0.93-0.97,1.39-2.12,1.39-3.46c0-0.74-0.14-1.41-0.41-2.01c0.79-0.96,1.18-2.07,1.18-3.33\n\tc0-0.94-0.24-1.82-0.71-2.63c-0.48-0.81-1.12-1.45-1.93-1.93c-0.81-0.47-1.69-0.71-2.63-0.71c-1.56,0-2.86,0.58-3.9,1.75\n\tc-0.8-0.44-1.7-0.66-2.71-0.66c-1.41,0-2.67,0.44-3.76,1.31s-1.8,2-2.11,3.37c-1.11,0.26-2.02,0.84-2.74,1.74\n\tC1.84,14.7,1.48,15.73,1.48,16.88z M6.82,23.94c0.1,0.22,0.25,0.37,0.46,0.45c0.2,0.1,0.41,0.11,0.63,0.02\n\tc0.22-0.08,0.37-0.23,0.45-0.45c0.1-0.22,0.11-0.43,0.02-0.65c-0.08-0.21-0.23-0.36-0.45-0.44c-0.2-0.11-0.41-0.12-0.62-0.03\n\tc-0.22,0.09-0.37,0.24-0.47,0.47C6.74,23.49,6.73,23.7,6.82,23.94z M7.46,21.1c0,0.14,0.03,0.27,0.09,0.38\n\tc0.19,0.31,0.49,0.46,0.89,0.46c0.32,0,0.55-0.22,0.69-0.65l1.04-3.22c0.08-0.24,0.06-0.47-0.07-0.67s-0.31-0.33-0.55-0.37\n\tC9.34,16.98,9.13,17,8.93,17.1c-0.2,0.11-0.34,0.28-0.41,0.5l-1.03,3.22C7.47,20.92,7.46,21.02,7.46,21.1z M9.33,26.72\n\tc0,0.13,0.02,0.23,0.05,0.29c0.09,0.22,0.24,0.37,0.45,0.45c0.09,0.05,0.21,0.07,0.35,0.07c0.06,0,0.16-0.02,0.3-0.06\n\tc0.22-0.08,0.38-0.23,0.47-0.45s0.1-0.44,0-0.66c-0.1-0.22-0.25-0.37-0.45-0.45s-0.41-0.08-0.62,0c-0.19,0.07-0.33,0.19-0.42,0.35\n\tC9.37,26.42,9.33,26.58,9.33,26.72z M9.94,4.57c0,0.25,0.08,0.45,0.24,0.6l0.65,0.65c0.16,0.16,0.34,0.25,0.54,0.27\n\tc0.21,0.03,0.41-0.05,0.61-0.23c0.2-0.18,0.3-0.4,0.3-0.64c0-0.24-0.08-0.44-0.24-0.6l-0.64-0.64c-0.19-0.17-0.39-0.25-0.62-0.25\n\tc-0.24,0-0.45,0.08-0.61,0.24C10.02,4.14,9.94,4.34,9.94,4.57z M10.06,24.03c0,0.16,0.05,0.32,0.16,0.48s0.27,0.27,0.48,0.33\n\tc0.11,0.02,0.19,0.04,0.24,0.04c0.15,0,0.28-0.03,0.38-0.08c0.2-0.08,0.34-0.27,0.43-0.57l1.8-6.14c0.07-0.24,0.05-0.45-0.06-0.65\n\tc-0.11-0.2-0.27-0.33-0.5-0.39c-0.24-0.07-0.46-0.05-0.66,0.06c-0.2,0.11-0.34,0.27-0.41,0.51l-1.84,6.19\n\tC10.07,23.92,10.06,24,10.06,24.03z M13.51,23.64c0,0.13,0.02,0.23,0.07,0.31c0.09,0.21,0.24,0.35,0.45,0.44\n\tc0.11,0.05,0.22,0.08,0.35,0.08c0.06,0,0.16-0.02,0.3-0.06c0.23-0.09,0.38-0.23,0.46-0.44c0.08-0.22,0.08-0.43,0-0.63\n\tc-0.08-0.2-0.22-0.35-0.42-0.45c-0.23-0.11-0.44-0.12-0.66-0.03c-0.21,0.09-0.37,0.24-0.48,0.47\n\tC13.53,23.41,13.51,23.51,13.51,23.64z M14.23,21.08c0,0.16,0.05,0.31,0.15,0.45c0.1,0.15,0.26,0.25,0.46,0.31\n\tc0.09,0.02,0.17,0.03,0.25,0.03c0.39,0,0.65-0.2,0.79-0.61l1.03-3.18c0.08-0.23,0.05-0.45-0.07-0.65s-0.29-0.33-0.52-0.39\n\tc-0.24-0.07-0.45-0.05-0.64,0.06s-0.32,0.27-0.4,0.51l-1.02,3.2C14.25,20.94,14.23,21.03,14.23,21.08z M15.3,9\n\tc0.67-0.64,1.5-0.97,2.48-0.97c0.98,0,1.81,0.34,2.5,1.03c0.69,0.68,1.04,1.51,1.04,2.49c0,0.62-0.17,1.24-0.52,1.85\n\tc-0.96-0.96-2.12-1.44-3.51-1.44H17C16.7,10.8,16.14,9.81,15.3,9z M16.92,3.73c0,0.24,0.08,0.44,0.25,0.61\n\tc0.17,0.17,0.37,0.25,0.61,0.25c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61V1.67c0-0.24-0.08-0.44-0.24-0.61\n\tc-0.16-0.17-0.35-0.25-0.59-0.25c-0.24,0-0.44,0.08-0.61,0.25c-0.17,0.17-0.25,0.37-0.25,0.61V3.73z M22.47,6.02\n\tc0,0.24,0.08,0.44,0.25,0.6c0.15,0.17,0.34,0.26,0.58,0.26c0.23,0,0.44-0.09,0.6-0.26l1.44-1.44c0.18-0.15,0.27-0.35,0.27-0.6\n\tc0-0.24-0.09-0.44-0.26-0.61c-0.17-0.17-0.38-0.25-0.61-0.25c-0.22,0-0.41,0.09-0.57,0.27l-1.45,1.43\n\tC22.56,5.57,22.47,5.78,22.47,6.02z M23.28,17.92c0,0.23,0.08,0.43,0.24,0.6l0.66,0.63c0.14,0.18,0.34,0.27,0.6,0.27\n\tc0.24,0,0.43-0.09,0.57-0.27c0.18-0.16,0.27-0.36,0.27-0.6c0-0.24-0.09-0.44-0.27-0.61l-0.65-0.62c-0.16-0.18-0.35-0.26-0.58-0.26\n\ts-0.43,0.08-0.6,0.25C23.36,17.48,23.28,17.69,23.28,17.92z M24.74,11.55c0,0.24,0.09,0.44,0.26,0.6c0.18,0.18,0.38,0.26,0.62,0.26\n\th2.03c0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.26-0.37,0.26-0.61c0-0.23-0.08-0.43-0.25-0.59c-0.17-0.16-0.38-0.24-0.62-0.24h-2.03\n\tc-0.25,0-0.46,0.08-0.63,0.24C24.83,11.12,24.74,11.32,24.74,11.55z\" />",
};
#[cfg(feature = "WiDayHaze")]
const WI_DAY_HAZE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.37,15.62c0-0.24,0.08-0.44,0.25-0.61c0.18-0.17,0.38-0.25,0.6-0.25h2.04c0.23,0,0.42,0.08,0.58,0.25\n\tc0.15,0.17,0.23,0.37,0.23,0.61c0,0.24-0.08,0.44-0.23,0.6c-0.15,0.17-0.35,0.25-0.58,0.25H5.23c-0.23,0-0.43-0.08-0.6-0.25\n\tC4.46,16.05,4.37,15.85,4.37,15.62z M7.23,8.71c0-0.23,0.08-0.43,0.23-0.61c0.2-0.17,0.41-0.25,0.64-0.25\n\tc0.21,0,0.41,0.08,0.59,0.25l1.43,1.46c0.16,0.15,0.24,0.35,0.24,0.59c0,0.24-0.08,0.44-0.24,0.6c-0.16,0.16-0.36,0.24-0.6,0.24\n\tc-0.24,0-0.44-0.08-0.59-0.24L7.47,9.32C7.31,9.15,7.23,8.95,7.23,8.71z M7.39,18.02c0-0.22,0.08-0.41,0.23-0.55\n\tc0.16-0.14,0.37-0.22,0.64-0.22h5.71c0.27,0,0.48,0.07,0.64,0.22c0.16,0.14,0.24,0.33,0.24,0.55c0,0.27-0.08,0.48-0.23,0.64\n\tc-0.16,0.16-0.37,0.24-0.65,0.24H8.26c-0.27,0-0.48-0.08-0.64-0.23S7.39,18.29,7.39,18.02z M8.38,20.97c0-0.22,0.09-0.42,0.28-0.6\n\tc0.18-0.18,0.39-0.27,0.6-0.27c0.26,0,0.47,0.09,0.63,0.26c0.16,0.18,0.24,0.38,0.24,0.61c0,0.28-0.08,0.49-0.23,0.65\n\ts-0.37,0.23-0.64,0.23c-0.23,0-0.44-0.08-0.61-0.24C8.47,21.45,8.38,21.23,8.38,20.97z M9.79,15.91v-0.07\n\tc0.03-1.26,0.47-2.35,1.31-3.28c0.84-0.93,1.87-1.49,3.1-1.69h0.05c0.19-0.04,0.45-0.06,0.76-0.06c0.31,0,0.57,0.02,0.76,0.06h0.04\n\tc1.22,0.19,2.26,0.76,3.1,1.69c0.84,0.93,1.28,2.02,1.31,3.28v0.07c0,0.16-0.08,0.24-0.23,0.24h-1.13c-0.12,0-0.2-0.03-0.25-0.09\n\tc-0.05-0.06-0.07-0.12-0.07-0.18c-0.04-0.93-0.4-1.72-1.08-2.37c-0.68-0.65-1.49-0.97-2.44-0.97s-1.76,0.32-2.44,0.97\n\tc-0.68,0.65-1.04,1.44-1.08,2.37c0,0.06-0.03,0.12-0.08,0.18c-0.05,0.06-0.14,0.09-0.26,0.09h-1.13\n\tC9.87,16.15,9.79,16.07,9.79,15.91z M10.8,20.97c0-0.23,0.08-0.43,0.24-0.61c0.16-0.17,0.37-0.26,0.63-0.26h3.83\n\tc0.22,0,0.42,0.09,0.6,0.27c0.18,0.18,0.28,0.38,0.28,0.6c0,0.26-0.09,0.48-0.27,0.64s-0.38,0.24-0.61,0.24h-3.83\n\tc-0.27,0-0.48-0.08-0.64-0.23C10.87,21.46,10.8,21.25,10.8,20.97z M14.14,7.89V5.86c0-0.24,0.08-0.44,0.25-0.61S14.76,5,15,5\n\ts0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.61v2.03c0,0.23-0.08,0.43-0.25,0.58S15.23,8.71,15,8.71s-0.44-0.08-0.6-0.23\n\tS14.14,8.12,14.14,7.89z M15.5,18.02c0-0.21,0.09-0.39,0.27-0.54s0.38-0.23,0.61-0.23s0.43,0.08,0.61,0.23s0.27,0.33,0.27,0.54\n\tc0,0.26-0.09,0.48-0.27,0.64c-0.18,0.16-0.38,0.24-0.61,0.24s-0.44-0.08-0.61-0.24C15.59,18.49,15.5,18.28,15.5,18.02z M17.04,20.97\n\tc0-0.23,0.08-0.43,0.24-0.61c0.16-0.17,0.38-0.26,0.64-0.26h1.86c0.26,0,0.47,0.09,0.63,0.26c0.16,0.18,0.24,0.38,0.24,0.61\n\tc0,0.28-0.08,0.49-0.23,0.65c-0.15,0.16-0.37,0.23-0.64,0.23h-1.86c-0.27,0-0.48-0.08-0.64-0.23\n\tC17.12,21.46,17.04,21.25,17.04,20.97z M17.92,18.02c0-0.21,0.09-0.39,0.27-0.54s0.38-0.23,0.6-0.23h3.07c0.22,0,0.4,0.07,0.54,0.22\n\tc0.14,0.15,0.22,0.33,0.22,0.55c0,0.27-0.07,0.48-0.21,0.64c-0.14,0.16-0.32,0.24-0.55,0.24h-3.07c-0.23,0-0.43-0.08-0.61-0.24\n\tC18.01,18.49,17.92,18.28,17.92,18.02z M19.66,10.15c0-0.25,0.08-0.45,0.23-0.59l1.42-1.46c0.18-0.17,0.38-0.25,0.59-0.25\n\tc0.23,0,0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.61c0,0.24-0.08,0.45-0.24,0.61l-1.46,1.42c-0.18,0.16-0.38,0.24-0.6,0.24\n\tc-0.23,0-0.41-0.08-0.56-0.24C19.74,10.59,19.66,10.39,19.66,10.15z M21.92,15.62c0-0.23,0.08-0.43,0.24-0.61\n\tc0.17-0.17,0.36-0.25,0.57-0.25h2.02c0.23,0,0.43,0.09,0.6,0.26c0.17,0.17,0.26,0.37,0.26,0.6c0,0.23-0.09,0.43-0.26,0.6\n\tc-0.17,0.17-0.37,0.25-0.6,0.25h-2.02c-0.23,0-0.43-0.08-0.58-0.25S21.92,15.86,21.92,15.62z\" />",
};
#[cfg(feature = "WiDayLightWind")]
const WI_DAY_LIGHT_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.32,14.85c0,0.24,0.09,0.44,0.26,0.6c0.16,0.17,0.36,0.25,0.6,0.25h9.42c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.6c0-0.23-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.24-0.59-0.24H3.18c-0.24,0-0.44,0.08-0.61,0.24\n\tC2.4,14.42,2.32,14.62,2.32,14.85z M2.65,21c0,0.24,0.08,0.44,0.25,0.6c0.16,0.17,0.36,0.25,0.6,0.25h9.43\n\tc0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6s-0.08-0.43-0.25-0.59s-0.37-0.24-0.61-0.24H3.51\n\tc-0.24,0-0.44,0.08-0.6,0.24C2.74,20.57,2.65,20.77,2.65,21z M4.02,17.9c0,0.24,0.08,0.43,0.25,0.59c0.17,0.16,0.38,0.23,0.63,0.23\n\th9.4c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.25-0.08-0.45-0.23-0.61c-0.16-0.16-0.35-0.24-0.59-0.24H4.9\n\tc-0.25,0-0.46,0.08-0.63,0.24C4.11,17.45,4.02,17.65,4.02,17.9z M6.45,11.55c0-0.24,0.09-0.44,0.26-0.62\n\tc0.17-0.16,0.38-0.24,0.6-0.24h2.03c0.23,0,0.42,0.08,0.58,0.25c0.16,0.17,0.23,0.37,0.23,0.61c0,0.24-0.08,0.44-0.23,0.6\n\tc-0.16,0.17-0.35,0.25-0.58,0.25H7.31c-0.24,0-0.44-0.08-0.61-0.25C6.53,11.98,6.45,11.78,6.45,11.55z M9.31,4.63\n\tc0-0.22,0.08-0.43,0.24-0.61c0.19-0.16,0.4-0.24,0.63-0.24c0.22,0,0.42,0.08,0.59,0.24l1.42,1.47c0.16,0.15,0.24,0.35,0.24,0.59\n\tc0,0.24-0.08,0.44-0.24,0.6c-0.16,0.16-0.36,0.24-0.6,0.24c-0.24,0-0.44-0.08-0.59-0.24L9.55,5.25C9.39,5.07,9.31,4.87,9.31,4.63z\n\t M11.86,11.43v-0.07c0.02-0.91,0.27-1.75,0.74-2.53c0.47-0.77,1.11-1.38,1.9-1.83s1.65-0.67,2.57-0.67c0.7,0,1.37,0.14,2.02,0.42\n\tc0.64,0.28,1.2,0.65,1.66,1.12c0.47,0.47,0.84,1.02,1.11,1.66c0.27,0.64,0.41,1.32,0.41,2.02c0,0.94-0.23,1.8-0.69,2.6\n\ts-1.09,1.43-1.88,1.89c-0.79,0.47-1.66,0.7-2.6,0.71h-0.2c-0.07,0-0.13-0.02-0.18-0.07c-0.05-0.05-0.08-0.11-0.08-0.18v-1.22\n\tc0-0.13,0.07-0.2,0.22-0.2h0.24c0.96-0.01,1.79-0.35,2.47-1.05c0.68-0.69,1.03-1.52,1.03-2.49c0-0.96-0.35-1.78-1.04-2.47\n\tc-0.69-0.68-1.52-1.02-2.5-1.02c-0.94,0-1.76,0.32-2.44,0.98c-0.68,0.65-1.04,1.44-1.08,2.37c0,0.06-0.02,0.11-0.07,0.17\n\ts-0.13,0.09-0.25,0.09h-1.14C11.93,11.67,11.86,11.59,11.86,11.43z M16.23,21.31v-1.99c0-0.24,0.08-0.44,0.24-0.6\n\ts0.36-0.24,0.6-0.24c0.24,0,0.45,0.08,0.61,0.24s0.24,0.36,0.24,0.6v1.99c0,0.24-0.08,0.45-0.25,0.62c-0.17,0.17-0.37,0.25-0.6,0.25\n\tc-0.24,0-0.44-0.08-0.6-0.25S16.23,21.56,16.23,21.31z M16.23,3.83V1.78c0-0.24,0.08-0.44,0.25-0.6s0.36-0.25,0.6-0.25\n\tc0.23,0,0.43,0.08,0.6,0.25s0.25,0.37,0.25,0.6v2.04c0,0.23-0.08,0.42-0.25,0.58c-0.17,0.15-0.37,0.23-0.6,0.23\n\tc-0.24,0-0.44-0.08-0.6-0.23C16.31,4.25,16.23,4.06,16.23,3.83z M21.74,17.01c0-0.23,0.07-0.42,0.23-0.56\n\tc0.15-0.16,0.34-0.23,0.57-0.23c0.24,0,0.44,0.08,0.6,0.23l1.45,1.42c0.16,0.17,0.24,0.38,0.24,0.61c0,0.23-0.08,0.43-0.24,0.59\n\tc-0.4,0.31-0.8,0.31-1.2,0l-1.42-1.43C21.82,17.48,21.74,17.26,21.74,17.01z M21.74,6.08c0-0.25,0.07-0.45,0.23-0.59l1.42-1.47\n\tc0.18-0.16,0.37-0.24,0.59-0.24c0.24,0,0.44,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6c0,0.25-0.08,0.46-0.24,0.62l-1.45,1.43\n\tc-0.18,0.16-0.38,0.24-0.6,0.24c-0.23,0-0.41-0.08-0.57-0.24S21.74,6.32,21.74,6.08z M24,11.55c0-0.23,0.08-0.44,0.25-0.62\n\tc0.16-0.16,0.35-0.24,0.56-0.24h2.03c0.23,0,0.43,0.09,0.61,0.26c0.17,0.17,0.26,0.37,0.26,0.6c0,0.23-0.09,0.43-0.26,0.6\n\tc-0.18,0.17-0.38,0.25-0.61,0.25h-2.03c-0.23,0-0.42-0.08-0.58-0.25C24.08,11.99,24,11.79,24,11.55z\" />",
};
#[cfg(feature = "WiDayLightning")]
const WI_DAY_LIGHTNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.56,16.9c0,1.33,0.46,2.47,1.39,3.43c0.93,0.96,2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.37\n\tc0-0.12-0.06-0.18-0.17-0.18c-0.87-0.07-1.6-0.41-2.19-1.04c-0.59-0.62-0.89-1.36-0.89-2.21c0-0.84,0.28-1.57,0.85-2.19\n\ts1.26-0.97,2.1-1.04l0.52-0.08c0.13,0,0.2-0.06,0.2-0.17l0.06-0.51c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.86-1.09\n\tc1.09,0,2.04,0.36,2.85,1.09c0.81,0.72,1.27,1.63,1.39,2.72l0.07,0.58c0,0.11,0.06,0.17,0.19,0.17h1.6c0.91,0,1.68,0.32,2.32,0.95\n\tc0.64,0.63,0.96,1.39,0.96,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.32,0.97-2.19,1.04c-0.13,0-0.2,0.06-0.2,0.18v1.37\n\tc0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.38-1.51s1.38-2.11,1.38-3.45c0-0.68-0.16-1.37-0.47-2.07\n\tC22.6,13.89,23,12.8,23,11.56c0-0.71-0.14-1.39-0.42-2.04c-0.28-0.65-0.65-1.2-1.12-1.67s-1.03-0.84-1.67-1.12\n\tc-0.65-0.27-1.32-0.41-2.03-0.41c-1.54,0-2.84,0.58-3.88,1.73c-0.86-0.41-1.74-0.62-2.65-0.62c-1.42,0-2.67,0.43-3.76,1.3\n\ts-1.79,1.99-2.1,3.37c-1.1,0.26-2.01,0.83-2.73,1.73S1.56,15.75,1.56,16.9z M9.06,28.17h0.3l5.32-7.85\n\tc0.04-0.04,0.05-0.09,0.02-0.14s-0.07-0.07-0.14-0.07h-2.18l2.3-4.21c0.07-0.14,0.03-0.22-0.14-0.22h-2.94\n\tc-0.08,0-0.15,0.05-0.22,0.14l-2.16,5.72c-0.02,0.14,0.02,0.21,0.14,0.21h2.17L9.06,28.17z M9.94,4.59c0,0.25,0.08,0.46,0.24,0.62\n\tl0.66,0.65c0.42,0.32,0.82,0.32,1.21,0c0.16-0.18,0.24-0.39,0.24-0.64c0-0.23-0.08-0.43-0.24-0.59l-0.64-0.65\n\tc-0.19-0.17-0.39-0.25-0.62-0.25c-0.23,0-0.43,0.08-0.6,0.25C10.02,4.15,9.94,4.36,9.94,4.59z M15.28,9.02\n\tC15.96,8.34,16.79,8,17.76,8c0.98,0,1.81,0.35,2.49,1.04c0.69,0.69,1.03,1.53,1.03,2.52c0,0.61-0.17,1.21-0.51,1.81\n\tc-0.98-0.94-2.13-1.41-3.46-1.41h-0.31C16.74,10.82,16.16,9.84,15.28,9.02z M16.9,3.73c0,0.25,0.08,0.46,0.24,0.62\n\tc0.16,0.16,0.36,0.24,0.61,0.24c0.24,0,0.43-0.08,0.59-0.24c0.16-0.16,0.23-0.37,0.23-0.62V1.69c0-0.24-0.08-0.43-0.23-0.59\n\tc-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.23S16.9,1.45,16.9,1.69V3.73z M22.42,6.05c0,0.23,0.09,0.43,0.27,0.6\n\tc0.18,0.17,0.37,0.25,0.55,0.25c0.16,0,0.37-0.08,0.62-0.25l1.44-1.43c0.17-0.18,0.25-0.39,0.25-0.63c0-0.24-0.08-0.45-0.25-0.61\n\tc-0.17-0.16-0.37-0.24-0.6-0.24c-0.22,0-0.41,0.08-0.58,0.25l-1.43,1.43C22.51,5.61,22.42,5.82,22.42,6.05z M23.23,17.87\n\tc0,0.23,0.08,0.43,0.24,0.61l0.65,0.66c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.6-0.25c0.17-0.17,0.25-0.38,0.25-0.63\n\tc0-0.23-0.08-0.42-0.25-0.57l-0.62-0.66c-0.19-0.16-0.39-0.23-0.62-0.23c-0.23,0-0.43,0.08-0.59,0.24\n\tC23.31,17.44,23.23,17.64,23.23,17.87z M24.68,11.56c0,0.22,0.09,0.41,0.26,0.57c0.17,0.17,0.37,0.25,0.6,0.25h2.04\n\tc0.24,0,0.44-0.08,0.61-0.24c0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.09-0.44-0.26-0.61s-0.37-0.25-0.6-0.25h-2.04\n\tc-0.23,0-0.43,0.08-0.6,0.25C24.77,11.12,24.68,11.32,24.68,11.56z\" />",
};
#[cfg(feature = "WiDayRain")]
const WI_DAY_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.51,16.89c0,1.33,0.46,2.48,1.39,3.44s2.06,1.47,3.41,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.34c0-0.11-0.06-0.17-0.17-0.17\n\tc-0.86-0.04-1.59-0.39-2.19-1.03s-0.9-1.4-0.9-2.26c0-0.82,0.28-1.54,0.85-2.16s1.27-0.97,2.1-1.07l0.53-0.05\n\tc0.13,0,0.2-0.06,0.2-0.17l0.07-0.54c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.86,1.1\n\ts1.28,1.64,1.4,2.72l0.07,0.57c0,0.12,0.06,0.19,0.17,0.19h1.62c0.89,0,1.65,0.32,2.3,0.96c0.65,0.64,0.97,1.39,0.97,2.27\n\tc0,0.87-0.3,1.62-0.9,2.26c-0.6,0.64-1.33,0.98-2.18,1.03c-0.12,0-0.19,0.06-0.19,0.17v1.34c0,0.11,0.06,0.17,0.19,0.17\n\tc1.33-0.04,2.46-0.55,3.39-1.51c0.93-0.97,1.39-2.12,1.39-3.45c0-0.72-0.14-1.39-0.42-2.01c0.78-0.97,1.17-2.07,1.17-3.31\n\tc0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92s-1.68-0.7-2.62-0.7c-1.56,0-2.85,0.58-3.88,1.74\n\tc-0.82-0.44-1.72-0.66-2.71-0.66c-1.41,0-2.67,0.44-3.76,1.32s-1.79,2-2.1,3.36c-1.11,0.26-2.02,0.84-2.74,1.74\n\tS1.51,15.74,1.51,16.89z M6.91,23.75c0,0.17,0.05,0.33,0.16,0.49c0.11,0.16,0.27,0.27,0.49,0.33c0.11,0.02,0.2,0.04,0.27,0.04\n\tc0.39,0,0.65-0.21,0.77-0.64l1.58-5.88c0.07-0.24,0.04-0.46-0.08-0.67c-0.12-0.21-0.3-0.33-0.53-0.38\n\tc-0.22-0.07-0.43-0.05-0.63,0.07c-0.2,0.11-0.34,0.28-0.41,0.51l-1.58,5.91C6.93,23.66,6.91,23.73,6.91,23.75z M9.52,26.83\n\tc0,0.19,0.05,0.36,0.15,0.52c0.1,0.16,0.27,0.26,0.52,0.3c0.11,0.02,0.2,0.04,0.26,0.04c0.16,0,0.31-0.06,0.45-0.17\n\tc0.14-0.12,0.23-0.28,0.27-0.48l2.4-8.93c0.06-0.23,0.04-0.45-0.07-0.64s-0.28-0.33-0.5-0.4c-0.23-0.07-0.45-0.05-0.65,0.07\n\tc-0.2,0.11-0.34,0.28-0.4,0.51l-2.4,8.93C9.53,26.73,9.52,26.82,9.52,26.83z M9.94,4.6c0,0.24,0.08,0.44,0.25,0.61l0.65,0.66\n\tc0.19,0.15,0.4,0.22,0.62,0.22c0.21,0,0.41-0.08,0.58-0.23c0.17-0.16,0.26-0.35,0.26-0.59c0-0.24-0.08-0.46-0.24-0.64l-0.64-0.65\n\tc-0.18-0.17-0.38-0.25-0.6-0.25c-0.24,0-0.45,0.09-0.62,0.26C10.03,4.16,9.94,4.37,9.94,4.6z M13.67,23.77\n\tc0,0.16,0.05,0.32,0.15,0.47s0.26,0.26,0.46,0.32c0.11,0.02,0.2,0.04,0.25,0.04c0.17,0,0.34-0.05,0.49-0.15\n\tc0.15-0.1,0.25-0.26,0.3-0.49l1.58-5.88c0.06-0.23,0.04-0.45-0.07-0.64c-0.11-0.2-0.28-0.33-0.5-0.4c-0.24-0.07-0.45-0.05-0.65,0.07\n\tc-0.2,0.11-0.33,0.28-0.39,0.51l-1.58,5.91C13.69,23.68,13.67,23.76,13.67,23.77z M15.3,9.03c0.71-0.67,1.53-1,2.48-1\n\tc0.98,0,1.82,0.35,2.5,1.04c0.69,0.69,1.03,1.53,1.03,2.52c0,0.62-0.17,1.24-0.52,1.85c-0.97-0.97-2.13-1.45-3.49-1.45h-0.33\n\tC16.7,10.81,16.14,9.83,15.3,9.03z M16.92,3.78c0,0.23,0.08,0.43,0.25,0.59c0.17,0.16,0.37,0.24,0.61,0.24\n\tc0.23,0,0.43-0.08,0.59-0.23c0.16-0.16,0.24-0.35,0.24-0.59V1.73c0-0.26-0.08-0.47-0.23-0.63c-0.16-0.16-0.35-0.24-0.59-0.24\n\tc-0.25,0-0.46,0.08-0.62,0.25s-0.24,0.37-0.24,0.62V3.78z M22.45,6.06c0,0.24,0.09,0.44,0.27,0.59c0.14,0.16,0.32,0.24,0.55,0.26\n\tc0.23,0.02,0.44-0.07,0.62-0.26l1.44-1.43c0.18-0.17,0.26-0.38,0.26-0.63c0-0.24-0.08-0.45-0.25-0.61\n\tc-0.17-0.16-0.37-0.24-0.61-0.24c-0.21,0-0.4,0.08-0.58,0.25l-1.43,1.44C22.54,5.6,22.45,5.81,22.45,6.06z M23.26,17.91\n\tc0,0.24,0.08,0.45,0.24,0.63l0.65,0.63c0.18,0.14,0.38,0.21,0.6,0.21l0.02,0.02c0.23,0,0.42-0.08,0.58-0.24\n\tc0.16-0.16,0.24-0.37,0.24-0.61c0-0.24-0.09-0.43-0.26-0.58l-0.62-0.66c-0.18-0.16-0.39-0.24-0.62-0.24s-0.43,0.08-0.59,0.25\n\tS23.26,17.67,23.26,17.91z M24.72,11.58c0,0.24,0.09,0.43,0.26,0.59c0.18,0.18,0.38,0.26,0.62,0.26h2.03c0.24,0,0.44-0.08,0.61-0.25\n\tc0.17-0.17,0.25-0.37,0.25-0.6c0-0.24-0.08-0.44-0.25-0.61s-0.37-0.26-0.61-0.26H25.6c-0.24,0-0.44,0.09-0.62,0.26\n\tC24.8,11.14,24.72,11.34,24.72,11.58z\" />",
};
#[cfg(feature = "WiDayRainMix")]
const WI_DAY_RAIN_MIX: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.48,16.95c0,1.32,0.46,2.46,1.37,3.4c0.91,0.94,2.04,1.45,3.38,1.51c0.12,0,0.18-0.06,0.18-0.17v-1.33\n\tc0-0.12-0.06-0.18-0.18-0.18c-0.86-0.04-1.58-0.38-2.17-1s-0.88-1.37-0.88-2.24c0-0.84,0.28-1.58,0.84-2.19\n\tc0.56-0.62,1.26-0.96,2.1-1.03l0.53-0.08c0.11,0,0.16-0.05,0.16-0.14l0.08-0.55c0.12-1.09,0.59-2,1.38-2.72S10,9.16,11.1,9.16\n\ts2.05,0.36,2.86,1.08c0.82,0.72,1.28,1.62,1.38,2.69l0.07,0.58c0.02,0.11,0.1,0.17,0.22,0.17h1.6c0.89,0,1.65,0.32,2.29,0.96\n\tc0.64,0.64,0.96,1.41,0.96,2.31c0,0.87-0.29,1.61-0.88,2.24s-1.31,0.95-2.17,1c-0.13,0-0.2,0.06-0.2,0.18v1.33\n\tc0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.45-0.54,3.37-1.49c0.91-0.95,1.37-2.09,1.37-3.42c0-0.68-0.13-1.34-0.38-2.01\n\tc0.78-0.96,1.16-2.08,1.16-3.35c0-0.71-0.14-1.38-0.41-2.03c-0.27-0.65-0.65-1.2-1.12-1.67s-1.03-0.84-1.67-1.12\n\ts-1.33-0.42-2.04-0.42c-1.54,0-2.83,0.58-3.86,1.74c-0.89-0.44-1.81-0.66-2.74-0.66c-1.41,0-2.66,0.44-3.74,1.31s-1.77,2-2.08,3.38\n\tc-1.12,0.26-2.04,0.83-2.75,1.73C1.83,14.76,1.48,15.79,1.48,16.95z M6.83,23.98c0,0.17,0.05,0.34,0.16,0.51\n\tc0.11,0.17,0.27,0.28,0.47,0.35c0.23,0.07,0.45,0.06,0.64-0.04c0.2-0.09,0.33-0.28,0.4-0.56l0.14-0.61c0.05-0.23,0.02-0.44-0.1-0.63\n\tc-0.12-0.2-0.29-0.33-0.52-0.4c-0.23-0.07-0.44-0.04-0.64,0.08S7.06,22.97,7,23.2l-0.14,0.59C6.84,23.85,6.83,23.91,6.83,23.98z\n\t M7.6,21.08c0,0.22,0.08,0.41,0.24,0.57C8,21.83,8.19,21.91,8.4,21.91c0.24,0,0.44-0.08,0.6-0.24c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.24-0.6-0.24c-0.23,0-0.42,0.08-0.58,0.23S7.6,20.85,7.6,21.08z M8.21,18.81\n\tc-0.01,0.16,0.03,0.31,0.13,0.45c0.1,0.15,0.26,0.25,0.48,0.32c0.21,0.06,0.41,0.04,0.62-0.07C9.65,19.4,9.79,19.23,9.86,19\n\tl0.27-0.9c0.07-0.24,0.05-0.46-0.07-0.65c-0.12-0.19-0.3-0.32-0.54-0.39c-0.22-0.07-0.43-0.05-0.63,0.07\n\tc-0.2,0.11-0.34,0.28-0.41,0.5l-0.24,0.92C8.22,18.71,8.21,18.8,8.21,18.81z M9.36,27.1c0,0.17,0.05,0.33,0.16,0.49\n\tc0.11,0.16,0.27,0.27,0.49,0.33c0.09,0.02,0.17,0.03,0.24,0.03c0.43,0,0.7-0.2,0.8-0.61l0.13-0.59c0.06-0.26,0.03-0.48-0.08-0.68\n\ts-0.29-0.32-0.52-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51L9.4,26.88C9.37,26.99,9.36,27.07,9.36,27.1z\n\t M9.92,4.66c0,0.24,0.08,0.44,0.24,0.6l0.66,0.64c0.14,0.16,0.32,0.24,0.54,0.26c0.22,0.02,0.43-0.07,0.62-0.26\n\tc0.16-0.16,0.24-0.36,0.24-0.59c0-0.24-0.08-0.44-0.24-0.6l-0.63-0.65C11.2,3.9,11,3.82,10.77,3.8c-0.23,0-0.43,0.08-0.6,0.25\n\tC10.01,4.22,9.92,4.42,9.92,4.66z M10.15,24.2c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23\n\tc-0.24,0-0.43,0.08-0.59,0.23S10.15,23.97,10.15,24.2z M10.77,21.93c-0.01,0.15,0.03,0.31,0.14,0.47c0.1,0.16,0.25,0.26,0.45,0.3\n\tc0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56l0.26-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4\n\tc-0.22-0.07-0.43-0.04-0.64,0.08s-0.34,0.3-0.41,0.53l-0.22,0.9C10.78,21.74,10.77,21.83,10.77,21.93z M13.53,24.08\n\tc0,0.17,0.05,0.33,0.15,0.48c0.1,0.15,0.25,0.26,0.46,0.32c0.03,0,0.08,0.01,0.14,0.02c0.06,0.01,0.11,0.02,0.14,0.02\n\tc0.41,0,0.66-0.22,0.76-0.66l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.12-0.21-0.29-0.34-0.51-0.41\n\tc-0.25-0.06-0.48-0.04-0.68,0.08s-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.01,0,0.05-0.01,0.11C13.54,24,13.53,24.04,13.53,24.08z\n\t M14.27,21.12c0,0.23,0.08,0.42,0.24,0.57c0.15,0.16,0.34,0.24,0.58,0.24s0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23s-0.43,0.08-0.59,0.23S14.27,20.88,14.27,21.12z M14.88,18.81\n\tc0,0.17,0.05,0.33,0.16,0.48c0.11,0.15,0.27,0.26,0.49,0.32c0.02,0,0.06,0.01,0.12,0.02c0.06,0.01,0.11,0.02,0.14,0.02\n\tc0.11,0,0.23-0.03,0.37-0.09c0.21-0.11,0.34-0.28,0.4-0.52l0.24-0.9c0.06-0.23,0.04-0.44-0.07-0.63s-0.28-0.33-0.5-0.4\n\tc-0.23-0.07-0.45-0.05-0.64,0.06c-0.2,0.11-0.33,0.27-0.4,0.51l-0.28,0.91c0,0.02,0,0.05-0.01,0.11\n\tC14.89,18.73,14.88,18.77,14.88,18.81z M15.23,9.09c0.66-0.66,1.48-0.99,2.47-0.99c0.99,0,1.83,0.34,2.52,1.02s1.04,1.5,1.04,2.48\n\tc0,0.66-0.18,1.29-0.53,1.88c-0.98-0.98-2.15-1.47-3.5-1.47h-0.31C16.64,10.91,16.07,9.94,15.23,9.09z M16.88,3.83\n\tc0,0.23,0.08,0.42,0.23,0.58c0.15,0.15,0.35,0.23,0.59,0.23c0.24,0,0.45-0.08,0.62-0.23c0.17-0.15,0.25-0.35,0.25-0.58V1.76\n\tc0-0.23-0.09-0.43-0.26-0.6c-0.17-0.17-0.38-0.25-0.61-0.25c-0.23,0-0.43,0.08-0.58,0.25c-0.16,0.17-0.23,0.37-0.23,0.6V3.83z\n\t M22.4,6.09c0,0.25,0.08,0.45,0.23,0.6c0.36,0.36,0.76,0.36,1.21,0l1.43-1.43c0.17-0.17,0.25-0.38,0.25-0.63\n\tc0-0.24-0.08-0.44-0.25-0.6c-0.17-0.17-0.37-0.25-0.6-0.25c-0.23,0-0.43,0.08-0.61,0.24L22.63,5.5C22.48,5.65,22.4,5.84,22.4,6.09z\n\t M23.18,17.94c0,0.23,0.09,0.43,0.27,0.59l0.61,0.63c0.2,0.16,0.4,0.24,0.61,0.24c0.2,0,0.4-0.08,0.6-0.24\n\tc0.17-0.16,0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.62l-0.65-0.61c-0.15-0.17-0.34-0.25-0.57-0.25s-0.43,0.08-0.6,0.25\n\tC23.27,17.51,23.18,17.71,23.18,17.94z M24.66,11.6c0,0.24,0.09,0.43,0.26,0.59c0.18,0.18,0.39,0.27,0.62,0.27h2.03\n\tc0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61c0-0.24-0.08-0.44-0.24-0.6c-0.16-0.17-0.35-0.25-0.59-0.25h-2.03\n\tc-0.24,0-0.44,0.08-0.62,0.25S24.66,11.37,24.66,11.6z\" />",
};
#[cfg(feature = "WiDayRainWind")]
const WI_DAY_RAIN_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.48,16.91c0,1.12,0.33,2.13,1,3.01c0.67,0.88,1.53,1.47,2.58,1.77c0.09,0.01,0.17-0.01,0.24-0.08l1.17-1.41\n\tc-0.89,0-1.66-0.32-2.3-0.97s-0.96-1.42-0.96-2.33c0-0.83,0.28-1.55,0.85-2.17c0.57-0.61,1.27-0.97,2.11-1.07l0.49-0.03\n\tc0.13,0,0.2-0.06,0.2-0.19l0.07-0.54c0.11-1.08,0.57-1.99,1.38-2.72s1.77-1.1,2.86-1.1c1.1,0,2.06,0.37,2.88,1.1\n\ts1.28,1.64,1.39,2.72l0.07,0.59c0.04,0.11,0.12,0.17,0.22,0.17h1.64c0.88,0,1.64,0.32,2.28,0.95c0.64,0.63,0.96,1.4,0.96,2.28\n\tc0,0.84-0.28,1.58-0.85,2.21c-0.57,0.63-1.25,0.98-2.07,1.05c-0.49,0.06-0.79,0.15-0.9,0.28l-2.19,2.82\n\tc-0.14,0.19-0.19,0.4-0.16,0.63c0.03,0.23,0.13,0.42,0.31,0.56c0.15,0.17,0.35,0.24,0.6,0.21c0.25-0.03,0.45-0.15,0.6-0.36\n\tl1.92-2.46c0.82-0.08,1.57-0.35,2.25-0.82c0.68-0.46,1.21-1.06,1.61-1.79c0.39-0.73,0.59-1.51,0.59-2.34c0-0.72-0.14-1.39-0.42-2.01\n\tc0.79-0.96,1.18-2.07,1.18-3.33c0-0.71-0.14-1.38-0.42-2.03c-0.28-0.65-0.66-1.21-1.13-1.68c-0.47-0.47-1.03-0.84-1.68-1.12\n\tc-0.65-0.28-1.33-0.42-2.04-0.42c-1.5,0-2.79,0.58-3.88,1.74c-0.79-0.44-1.7-0.66-2.72-0.66c-1.41,0-2.66,0.44-3.75,1.32\n\ts-1.78,2-2.07,3.37c-1.13,0.26-2.06,0.83-2.79,1.73S1.48,15.76,1.48,16.91z M5.38,25.15c0.07,0.22,0.23,0.38,0.46,0.48\n\tc0.22,0.1,0.44,0.1,0.67,0.01s0.38-0.25,0.46-0.47c0.09-0.21,0.1-0.41,0-0.62c-0.09-0.21-0.24-0.36-0.45-0.46\n\tc-0.22-0.1-0.44-0.11-0.66-0.02c-0.22,0.09-0.37,0.24-0.45,0.46C5.32,24.78,5.31,24.99,5.38,25.15z M7.19,22.41v0.11\n\tc0.02,0.22,0.12,0.4,0.3,0.55c0.18,0.15,0.38,0.22,0.63,0.2c0.24-0.02,0.43-0.12,0.57-0.29l2.18-2.82c0.14-0.18,0.21-0.39,0.19-0.63\n\ts-0.12-0.43-0.29-0.57c-0.18-0.15-0.39-0.21-0.63-0.19c-0.24,0.02-0.44,0.13-0.6,0.31l-2.15,2.8C7.25,22.04,7.19,22.22,7.19,22.41z\n\t M7.7,27.27c0,0.06,0.02,0.16,0.06,0.3c0.09,0.22,0.24,0.37,0.46,0.46c0.11,0.04,0.23,0.07,0.36,0.07c0.09,0,0.2-0.02,0.31-0.05\n\tc0.21-0.08,0.36-0.23,0.44-0.44c0.1-0.22,0.11-0.44,0.02-0.67c-0.08-0.23-0.23-0.38-0.45-0.46c-0.22-0.1-0.44-0.1-0.67-0.01\n\ts-0.38,0.24-0.45,0.45C7.73,27.03,7.7,27.15,7.7,27.27z M9.37,24.9V25c0.02,0.23,0.13,0.42,0.33,0.58c0.15,0.16,0.35,0.23,0.6,0.2\n\tc0.25-0.03,0.45-0.14,0.6-0.34l4.14-5.23c0.14-0.2,0.19-0.41,0.17-0.64s-0.13-0.42-0.31-0.59c-0.18-0.14-0.39-0.2-0.62-0.17\n\tc-0.23,0.03-0.42,0.13-0.56,0.31l-4.16,5.26C9.43,24.53,9.37,24.71,9.37,24.9z M9.95,4.61c0,0.24,0.09,0.44,0.27,0.59l0.61,0.66\n\tc0.16,0.16,0.34,0.24,0.54,0.26c0.21,0.03,0.41-0.04,0.61-0.22c0.2-0.18,0.3-0.39,0.3-0.64c0-0.22-0.08-0.43-0.24-0.6L11.4,4.01\n\tc-0.15-0.16-0.34-0.24-0.58-0.24c-0.24,0-0.45,0.08-0.62,0.24C10.04,4.18,9.95,4.38,9.95,4.61z M12.9,26.04\n\tc0,0.12,0.02,0.22,0.07,0.31c0.09,0.22,0.24,0.37,0.45,0.46c0.11,0.05,0.22,0.08,0.35,0.08c0.06,0,0.16-0.02,0.3-0.06\n\tc0.22-0.09,0.38-0.24,0.46-0.46c0.1-0.22,0.11-0.44,0.02-0.66c-0.08-0.22-0.23-0.37-0.45-0.45c-0.22-0.1-0.44-0.11-0.67-0.02\n\tc-0.23,0.09-0.38,0.24-0.46,0.45C12.92,25.8,12.9,25.92,12.9,26.04z M15.3,9.03c0.71-0.64,1.53-0.97,2.48-0.97\n\tc0.99,0,1.83,0.34,2.53,1.03s1.05,1.51,1.05,2.49c0,0.66-0.17,1.28-0.52,1.86c-0.97-0.97-2.13-1.45-3.48-1.45h-0.34\n\tC16.75,10.83,16.18,9.85,15.3,9.03z M16.93,3.77c0,0.23,0.08,0.43,0.25,0.59c0.17,0.16,0.37,0.24,0.6,0.24\n\tc0.24,0,0.45-0.08,0.62-0.24C18.57,4.2,18.65,4,18.65,3.77V1.7c0-0.23-0.09-0.43-0.26-0.6c-0.17-0.17-0.38-0.25-0.61-0.25\n\tc-0.23,0-0.43,0.08-0.6,0.25s-0.25,0.37-0.25,0.6V3.77z M22.5,6.05c0,0.24,0.07,0.44,0.22,0.6c0.18,0.17,0.38,0.25,0.61,0.26\n\tc0.23,0,0.42-0.08,0.57-0.26L25.4,5.2c0.16-0.16,0.24-0.37,0.24-0.61c0-0.24-0.09-0.44-0.26-0.61c-0.17-0.17-0.37-0.26-0.6-0.26\n\tc-0.22,0-0.42,0.08-0.6,0.25l-1.45,1.47C22.58,5.61,22.5,5.81,22.5,6.05z M23.28,17.95c0,0.24,0.08,0.44,0.24,0.6l0.66,0.63\n\tc0.16,0.16,0.34,0.25,0.54,0.27h0.06c0.18,0,0.38-0.09,0.62-0.27c0.16-0.16,0.24-0.36,0.24-0.58c0-0.24-0.08-0.45-0.24-0.63\n\tl-0.66-0.62c-0.16-0.17-0.36-0.25-0.59-0.25c-0.24,0-0.45,0.08-0.62,0.25C23.37,17.51,23.28,17.72,23.28,17.95z M24.78,11.58\n\tc0,0.24,0.08,0.44,0.24,0.6c0.16,0.18,0.36,0.27,0.6,0.27h2.04c0.24,0,0.44-0.09,0.61-0.26c0.17-0.17,0.25-0.38,0.25-0.61\n\tc0-0.23-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.24-0.61-0.24h-2.04c-0.24,0-0.44,0.08-0.6,0.24C24.86,11.15,24.78,11.35,24.78,11.58\n\tz\" />",
};
#[cfg(feature = "WiDayShowers")]
const WI_DAY_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.56,16.88c0,1.33,0.46,2.47,1.39,3.43s2.06,1.46,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.34c0-0.12-0.06-0.18-0.17-0.18\n\tc-0.85-0.04-1.57-0.38-2.17-1.02s-0.89-1.39-0.89-2.25c0-0.84,0.28-1.56,0.84-2.17s1.26-0.96,2.1-1.06l0.5-0.04\n\tc0.13,0,0.2-0.06,0.2-0.18l0.06-0.53c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.86-1.09c1.09,0,2.05,0.36,2.86,1.09\n\tc0.81,0.73,1.27,1.63,1.37,2.71l0.07,0.57c0,0.12,0.06,0.18,0.18,0.18h1.67c0.88,0,1.63,0.32,2.27,0.96\n\tc0.64,0.64,0.96,1.39,0.96,2.27c0,0.85-0.3,1.59-0.9,2.22s-1.32,0.98-2.16,1.05c-0.11,0-0.17,0.06-0.17,0.18v1.34\n\tc0,0.11,0.06,0.17,0.17,0.17c0.88-0.02,1.67-0.26,2.4-0.72s1.3-1.05,1.71-1.8c0.42-0.75,0.62-1.56,0.62-2.44\n\tc0-0.71-0.14-1.37-0.41-1.96c0.76-0.94,1.13-2.03,1.13-3.28c0-0.71-0.14-1.39-0.41-2.04c-0.27-0.65-0.65-1.2-1.12-1.67\n\tC21,7.46,20.44,7.09,19.8,6.82c-0.65-0.28-1.33-0.41-2.04-0.41c-1.51,0-2.78,0.55-3.81,1.66c-0.79-0.43-1.7-0.64-2.73-0.64\n\tc-1.41,0-2.66,0.44-3.75,1.31s-1.77,1.99-2.07,3.35c-1.12,0.26-2.05,0.83-2.77,1.72C1.92,14.7,1.56,15.73,1.56,16.88z M6.97,23.58\n\tc0,0.18,0.05,0.36,0.16,0.53c0.11,0.18,0.26,0.29,0.45,0.36c0.19,0.07,0.4,0.05,0.61-0.06c0.22-0.11,0.36-0.29,0.44-0.55l0.25-1.05\n\tc0.07-0.21,0.05-0.41-0.07-0.62c-0.12-0.21-0.29-0.35-0.51-0.42c-0.25-0.06-0.47-0.03-0.67,0.08s-0.32,0.3-0.37,0.53l-0.28,0.99\n\tC6.98,23.42,6.97,23.49,6.97,23.58z M8.28,18.86c0,0.38,0.21,0.64,0.64,0.79c0.22,0.08,0.43,0.06,0.64-0.05\n\tc0.21-0.11,0.34-0.29,0.41-0.53l0.24-1.03c0.07-0.21,0.05-0.41-0.07-0.62c-0.11-0.21-0.28-0.35-0.51-0.42\n\tc-0.24-0.06-0.47-0.04-0.67,0.08s-0.32,0.29-0.37,0.52l-0.3,1.02C8.29,18.7,8.28,18.78,8.28,18.86z M9.5,26.75\n\tc0,0.16,0.06,0.33,0.17,0.5c0.11,0.17,0.28,0.29,0.49,0.36c0.01,0,0.04,0,0.1,0.01c0.06,0.01,0.11,0.01,0.15,0.01\n\tc0.14,0,0.26-0.02,0.37-0.07c0.19-0.08,0.33-0.27,0.41-0.58l0.27-0.99c0.07-0.23,0.05-0.45-0.07-0.65c-0.12-0.2-0.29-0.34-0.51-0.4\n\tc-0.23-0.07-0.45-0.05-0.65,0.07c-0.2,0.12-0.34,0.29-0.4,0.51l-0.28,1.02C9.51,26.63,9.5,26.7,9.5,26.75z M9.96,4.68\n\tc0,0.25,0.08,0.46,0.25,0.62l0.66,0.65c0.34,0.34,0.73,0.34,1.17,0c0.16-0.17,0.24-0.38,0.24-0.61c0-0.23-0.08-0.43-0.24-0.61\n\tl-0.63-0.66c-0.16-0.16-0.36-0.24-0.6-0.24c-0.23,0-0.43,0.08-0.6,0.25C10.04,4.24,9.96,4.44,9.96,4.68z M10.85,21.96\n\tc0,0.17,0.05,0.34,0.16,0.51c0.11,0.17,0.26,0.28,0.47,0.35c0.23,0.07,0.44,0.05,0.64-0.05c0.19-0.1,0.33-0.29,0.4-0.56l0.24-1.01\n\tc0.07-0.23,0.05-0.45-0.06-0.65c-0.11-0.2-0.28-0.34-0.5-0.41c-0.25-0.07-0.48-0.04-0.68,0.08c-0.2,0.12-0.33,0.3-0.37,0.53\n\tl-0.28,1.03C10.85,21.81,10.85,21.87,10.85,21.96z M13.63,23.68c0.02,0.38,0.23,0.65,0.63,0.83l0.25,0.04\n\tc0.16,0,0.32-0.05,0.47-0.16c0.15-0.11,0.26-0.27,0.32-0.5l0.29-1.01c0.06-0.24,0.03-0.46-0.09-0.66c-0.12-0.2-0.3-0.33-0.53-0.37\n\tc-0.21-0.07-0.41-0.05-0.62,0.07s-0.34,0.29-0.41,0.51l-0.27,1.02c-0.01,0.02-0.01,0.05-0.02,0.08s-0.01,0.06-0.02,0.08\n\tS13.63,23.66,13.63,23.68z M15.03,18.92c0,0.16,0.05,0.32,0.15,0.48c0.1,0.16,0.25,0.27,0.45,0.32l0.25,0.03\n\tc0.19,0,0.37-0.06,0.52-0.18s0.24-0.28,0.28-0.47l0.27-0.99c0.07-0.24,0.05-0.45-0.07-0.65c-0.11-0.2-0.28-0.33-0.51-0.39\n\tc-0.23-0.07-0.45-0.05-0.64,0.06c-0.2,0.11-0.33,0.28-0.39,0.5l-0.3,1.06C15.04,18.77,15.03,18.85,15.03,18.92z M15.36,9.06\n\tc0.66-0.64,1.46-0.96,2.4-0.96c0.98,0,1.82,0.35,2.51,1.04c0.69,0.69,1.04,1.53,1.04,2.51c0,0.56-0.16,1.15-0.47,1.76\n\tc-0.96-0.96-2.11-1.43-3.47-1.43h-0.34C16.77,10.84,16.21,9.87,15.36,9.06z M16.9,3.83c0,0.25,0.08,0.45,0.24,0.61\n\tc0.16,0.16,0.36,0.24,0.61,0.24s0.45-0.08,0.61-0.24c0.16-0.16,0.24-0.36,0.24-0.61V1.81c0-0.25-0.08-0.46-0.24-0.62\n\tc-0.16-0.16-0.36-0.24-0.61-0.24s-0.45,0.08-0.61,0.24c-0.16,0.16-0.24,0.37-0.24,0.62V3.83z M22.45,6.12\n\tc0,0.25,0.08,0.45,0.23,0.61c0.21,0.17,0.41,0.25,0.62,0.25c0.19,0,0.38-0.08,0.59-0.25l1.43-1.43c0.16-0.18,0.24-0.39,0.24-0.63\n\tc0-0.24-0.08-0.44-0.24-0.6c-0.16-0.16-0.36-0.24-0.59-0.24s-0.43,0.08-0.61,0.24L22.68,5.5C22.53,5.68,22.45,5.88,22.45,6.12z\n\t M23.24,17.95c0,0.23,0.09,0.44,0.26,0.63l0.62,0.64c0.21,0.21,0.41,0.31,0.62,0.31c0.19,0,0.39-0.1,0.58-0.31\n\tc0.18-0.18,0.27-0.39,0.26-0.61c-0.01-0.23-0.09-0.43-0.26-0.6l-0.65-0.66c-0.16-0.16-0.35-0.24-0.57-0.24\n\tc-0.24,0-0.44,0.08-0.61,0.25C23.33,17.52,23.24,17.72,23.24,17.95z M24.71,11.64c0,0.22,0.08,0.42,0.24,0.58\n\tc0.16,0.16,0.36,0.24,0.58,0.24h2.04c0.26,0,0.47-0.08,0.63-0.23c0.16-0.16,0.24-0.35,0.24-0.59c0-0.25-0.08-0.46-0.25-0.62\n\tc-0.17-0.16-0.37-0.24-0.62-0.24h-2.04c-0.23,0-0.43,0.08-0.59,0.25C24.79,11.2,24.71,11.41,24.71,11.64z\" />",
};
#[cfg(feature = "WiDaySleet")]
const WI_DAY_SLEET: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.49,16.92L1.49,16.92c0-1.17,0.36-2.19,1.08-3.09s1.64-1.48,2.74-1.74c0.31-1.37,1.01-2.49,2.1-3.38\n\tc1.1-0.88,2.35-1.32,3.77-1.32c0.99,0,1.9,0.22,2.72,0.66c0.5-0.53,1.09-0.95,1.76-1.25c0.67-0.3,1.37-0.45,2.09-0.45\n\tc0.95,0,1.83,0.24,2.64,0.71c0.81,0.47,1.45,1.11,1.92,1.92c0.47,0.81,0.71,1.69,0.71,2.64c0,1.23-0.38,2.33-1.14,3.29\n\tc0.29,0.61,0.43,1.28,0.43,2.02c0,0.88-0.21,1.7-0.64,2.45c-0.42,0.75-1,1.36-1.74,1.81c-0.73,0.46-1.54,0.7-2.42,0.72\n\tc-0.13,0-0.2-0.06-0.2-0.18v-1.34c0-0.12,0.07-0.18,0.2-0.18c0.86-0.04,1.58-0.39,2.18-1.03c0.6-0.64,0.9-1.4,0.9-2.26\n\tc0-0.89-0.32-1.65-0.97-2.29s-1.41-0.96-2.31-0.96h-1.61c-0.12,0-0.18-0.06-0.18-0.17l-0.08-0.59c-0.11-1.08-0.58-1.99-1.4-2.72\n\tc-0.82-0.73-1.78-1.1-2.86-1.1c-1.1,0-2.05,0.37-2.86,1.1c-0.81,0.73-1.27,1.64-1.37,2.72L6.88,13.5c-0.03,0.09-0.11,0.13-0.22,0.13\n\tl-0.51,0.04c-0.84,0.1-1.55,0.45-2.11,1.06s-0.84,1.34-0.84,2.18v0.05h0.03c0.01,0.98,0.38,1.78,1.11,2.43\n\tc0.22,0.19,0.47,0.36,0.74,0.49v0.02c0.41,0.19,0.82,0.29,1.21,0.31c0.11,0,0.17,0.06,0.17,0.17v1.34c0,0.11-0.06,0.17-0.17,0.17\n\tc-0.52-0.03-1.01-0.13-1.48-0.3v0.02c-0.83-0.29-1.54-0.77-2.11-1.43s-0.95-1.44-1.11-2.31v-0.04c-0.01-0.01-0.01-0.02-0.01-0.03\n\tC1.51,17.54,1.49,17.25,1.49,16.92z M6.99,24.09c0-0.03,0.01-0.07,0.02-0.13c0.01-0.05,0.02-0.09,0.02-0.12l0.09-0.59\n\tc0.07-0.24,0.2-0.41,0.41-0.53c0.2-0.12,0.43-0.14,0.68-0.08c0.23,0.07,0.39,0.2,0.51,0.41c0.11,0.2,0.13,0.41,0.07,0.62l-0.14,0.6\n\tc-0.1,0.44-0.35,0.66-0.76,0.66c-0.03,0-0.08,0-0.15-0.01s-0.11-0.01-0.13-0.01c-0.21-0.06-0.36-0.17-0.46-0.33\n\tC7.04,24.43,6.99,24.26,6.99,24.09z M7.73,21.15c0-0.24,0.08-0.43,0.23-0.59c0.16-0.16,0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23\n\tc0.16,0.16,0.23,0.35,0.23,0.59c0,0.23-0.08,0.42-0.23,0.58s-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.57-0.24\n\tC7.81,21.56,7.73,21.37,7.73,21.15z M9.37,27.13c0-0.04,0.01-0.11,0.04-0.23l0.13-0.58c0.07-0.23,0.21-0.4,0.41-0.51\n\tc0.21-0.12,0.42-0.14,0.63-0.07c0.23,0.04,0.41,0.17,0.53,0.37c0.12,0.2,0.15,0.43,0.08,0.68l-0.13,0.59\n\tc-0.1,0.41-0.37,0.61-0.8,0.61c-0.05,0-0.13-0.01-0.24-0.04c-0.22-0.06-0.38-0.17-0.49-0.33C9.42,27.46,9.37,27.3,9.37,27.13z\n\t M9.9,4.62c0-0.24,0.08-0.44,0.25-0.6c0.17-0.16,0.38-0.24,0.63-0.24c0.24,0,0.44,0.08,0.6,0.24l0.63,0.66\n\tc0.17,0.17,0.25,0.37,0.25,0.6c0,0.24-0.1,0.46-0.3,0.64c-0.2,0.18-0.4,0.26-0.61,0.23c-0.21-0.02-0.39-0.11-0.55-0.27l-0.65-0.66\n\tC9.99,5.05,9.9,4.85,9.9,4.62z M10.16,24.23c0-0.23,0.08-0.43,0.23-0.58c0.16-0.16,0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23\n\tc0.16,0.16,0.23,0.35,0.23,0.58c0,0.24-0.08,0.43-0.23,0.59c-0.16,0.16-0.35,0.23-0.59,0.23c-0.22,0-0.41-0.08-0.58-0.25\n\tC10.24,24.64,10.16,24.45,10.16,24.23z M10.78,21.96c0-0.09,0.01-0.18,0.03-0.26l0.23-0.9c0.07-0.23,0.21-0.41,0.41-0.53\n\tc0.21-0.12,0.42-0.15,0.64-0.08c0.24,0.07,0.41,0.2,0.53,0.4c0.12,0.2,0.14,0.4,0.07,0.62l-0.26,0.9c-0.08,0.27-0.22,0.46-0.41,0.57\n\ts-0.41,0.12-0.64,0.06c-0.2-0.04-0.35-0.14-0.45-0.3C10.81,22.26,10.76,22.11,10.78,21.96z M13.54,24.1c0-0.03,0-0.07,0.01-0.13\n\tc0.01-0.06,0.01-0.09,0.01-0.11l0.09-0.59c0.07-0.24,0.2-0.41,0.41-0.53c0.2-0.12,0.43-0.14,0.68-0.08c0.23,0.07,0.4,0.21,0.51,0.41\n\tc0.12,0.21,0.14,0.42,0.07,0.63l-0.14,0.6c-0.1,0.43-0.35,0.65-0.76,0.65c-0.03,0-0.08,0-0.15-0.01c-0.07-0.01-0.11-0.01-0.13-0.01\n\tc-0.2-0.06-0.35-0.17-0.45-0.33C13.59,24.43,13.54,24.27,13.54,24.1z M14.28,21.16c0-0.24,0.08-0.43,0.23-0.59\n\tc0.16-0.16,0.35-0.23,0.59-0.23c0.24,0,0.43,0.08,0.59,0.23c0.16,0.16,0.23,0.35,0.23,0.59c0,0.23-0.08,0.43-0.23,0.58\n\tc-0.16,0.16-0.35,0.23-0.59,0.23c-0.23,0-0.43-0.08-0.58-0.25C14.36,21.57,14.28,21.38,14.28,21.16z M15.31,9.05\n\tc0.84,0.76,1.4,1.74,1.7,2.93h0.31c1.38,0,2.55,0.48,3.52,1.45c0.31-0.55,0.47-1.16,0.47-1.82c0-0.98-0.35-1.81-1.04-2.5\n\tc-0.69-0.68-1.53-1.03-2.51-1.03C16.8,8.08,15.98,8.4,15.31,9.05z M16.91,3.78V1.73c0-0.24,0.08-0.44,0.25-0.61\n\tc0.17-0.17,0.37-0.26,0.6-0.26c0.24,0,0.44,0.08,0.6,0.25c0.16,0.17,0.24,0.38,0.24,0.62v2.05c0,0.24-0.08,0.45-0.24,0.62\n\tC18.2,4.57,18,4.65,17.76,4.65c-0.23,0-0.43-0.09-0.6-0.26C16.99,4.22,16.91,4.02,16.91,3.78z M22.49,6.07\n\tc0-0.24,0.08-0.44,0.23-0.6l1.44-1.45c0.15-0.17,0.34-0.25,0.58-0.25c0.24,0,0.44,0.08,0.6,0.25c0.18,0.16,0.26,0.36,0.26,0.6\n\tc0,0.24-0.09,0.44-0.26,0.6L23.9,6.68c-0.19,0.19-0.4,0.27-0.63,0.26c-0.23-0.02-0.41-0.1-0.55-0.26\n\tC22.56,6.52,22.49,6.32,22.49,6.07z M23.26,17.98c0-0.24,0.08-0.44,0.25-0.61c0.17-0.17,0.37-0.25,0.6-0.25\n\tc0.23,0,0.43,0.09,0.61,0.26l0.62,0.63c0.18,0.17,0.26,0.38,0.26,0.61c0,0.24-0.09,0.44-0.26,0.6c-0.14,0.17-0.32,0.26-0.54,0.26\n\tl-0.02-0.02c-0.24,0-0.44-0.08-0.62-0.24l-0.65-0.64C23.35,18.41,23.26,18.21,23.26,17.98z M24.73,11.61c0-0.24,0.08-0.44,0.25-0.6\n\tc0.17-0.16,0.37-0.24,0.61-0.24h2.06c0.24,0,0.45,0.08,0.61,0.24s0.25,0.36,0.25,0.6c0,0.24-0.08,0.44-0.25,0.61\n\tc-0.17,0.17-0.37,0.25-0.61,0.25h-2.06c-0.24,0-0.44-0.09-0.6-0.27C24.81,12.05,24.73,11.85,24.73,11.61z\" />",
};
#[cfg(feature = "WiDaySleetStorm")]
const WI_DAY_SLEET_STORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.49,16.88c0,1.12,0.33,2.12,1,3s1.53,1.47,2.58,1.76l-0.66,1.7c-0.05,0.14,0,0.22,0.14,0.22h2.13l-1.43,4.21h0.29\n\tl4.36-5.66c0.04-0.04,0.04-0.09,0.02-0.14C9.9,21.92,9.85,21.9,9.78,21.9H7.59l2.49-4.65c0.07-0.14,0.03-0.22-0.14-0.22H6.98\n\tc-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.88C4.96,19.88,4.36,19.5,3.9,18.9c-0.47-0.59-0.7-1.26-0.7-2.02c0-0.84,0.28-1.57,0.84-2.18\n\tc0.56-0.61,1.27-0.97,2.11-1.07l0.51-0.03c0.12,0,0.19-0.05,0.22-0.14l0.07-0.59c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1\n\tc1.09,0,2.04,0.37,2.86,1.1s1.29,1.64,1.4,2.72l0.08,0.59c0,0.11,0.06,0.17,0.18,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96\n\ts0.97,1.4,0.97,2.29c0,0.87-0.3,1.62-0.9,2.26s-1.32,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc0.88-0.02,1.69-0.26,2.42-0.72c0.73-0.45,1.31-1.06,1.74-1.81s0.64-1.57,0.64-2.45c0-0.73-0.14-1.4-0.43-2.02\n\tc0.76-0.96,1.14-2.06,1.14-3.29c0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71\n\tc-0.72,0-1.42,0.15-2.1,0.45c-0.68,0.3-1.26,0.72-1.76,1.25c-0.81-0.43-1.71-0.65-2.72-0.65c-1.42,0-2.68,0.44-3.77,1.32\n\ts-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74C1.85,14.7,1.49,15.73,1.49,16.88z M9.37,27.1c0,0.17,0.05,0.33,0.16,0.49\n\tc0.11,0.16,0.27,0.27,0.49,0.33c0.09,0.02,0.17,0.03,0.24,0.03c0.43,0,0.7-0.2,0.8-0.61l0.13-0.59c0.06-0.26,0.03-0.48-0.08-0.68\n\tc-0.12-0.2-0.29-0.32-0.53-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.13,0.59\n\tC9.38,26.99,9.37,27.07,9.37,27.1z M9.9,4.59c0,0.23,0.08,0.43,0.25,0.6l0.65,0.66c0.16,0.16,0.34,0.24,0.55,0.26\n\tc0.21,0.03,0.41-0.04,0.61-0.23c0.2-0.18,0.3-0.39,0.3-0.64c0-0.23-0.08-0.43-0.25-0.6l-0.63-0.66c-0.16-0.16-0.36-0.24-0.6-0.24\n\tc-0.25,0-0.46,0.08-0.63,0.24C9.99,4.16,9.9,4.36,9.9,4.59z M10.16,24.2c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23\n\ts-0.43,0.08-0.59,0.23C10.24,23.77,10.16,23.97,10.16,24.2z M10.78,21.93c-0.01,0.15,0.03,0.31,0.14,0.47\n\tc0.1,0.16,0.25,0.26,0.45,0.3c0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56l0.26-0.9c0.07-0.22,0.05-0.43-0.07-0.63\n\tc-0.12-0.2-0.29-0.33-0.53-0.4c-0.22-0.07-0.43-0.04-0.64,0.08s-0.34,0.3-0.41,0.53l-0.23,0.9C10.79,21.74,10.78,21.83,10.78,21.93z\n\t M13.54,24.08c0,0.17,0.05,0.33,0.15,0.48c0.1,0.15,0.25,0.26,0.46,0.32c0.03,0,0.08,0.01,0.14,0.02c0.06,0.01,0.11,0.02,0.14,0.02\n\tc0.41,0,0.66-0.22,0.76-0.66l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.12-0.21-0.29-0.34-0.51-0.41\n\tc-0.25-0.06-0.48-0.04-0.68,0.08c-0.2,0.12-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.01,0,0.05-0.01,0.11\n\tC13.55,24,13.54,24.04,13.54,24.08z M14.28,21.12c0,0.23,0.08,0.42,0.24,0.57c0.15,0.16,0.34,0.24,0.58,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23\n\tc-0.24,0-0.43,0.08-0.59,0.23C14.36,20.69,14.28,20.88,14.28,21.12z M15.31,9.02c0.67-0.64,1.48-0.97,2.45-0.97\n\tc0.98,0,1.82,0.34,2.51,1.03c0.69,0.68,1.04,1.52,1.04,2.5c0,0.66-0.16,1.26-0.47,1.81c-0.96-0.96-2.13-1.44-3.52-1.44h-0.31\n\tC16.72,10.76,16.15,9.78,15.31,9.02z M16.91,3.75c0,0.24,0.08,0.44,0.25,0.61s0.37,0.25,0.6,0.25c0.24,0,0.44-0.08,0.6-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.61V1.69c0-0.24-0.08-0.45-0.24-0.61C18.2,0.91,18,0.82,17.76,0.82c-0.24,0-0.44,0.08-0.6,0.25\n\ts-0.25,0.37-0.25,0.61V3.75z M22.49,6.04c0,0.24,0.08,0.44,0.23,0.6c0.14,0.16,0.32,0.24,0.55,0.26c0.23,0.02,0.44-0.07,0.63-0.26\n\tl1.44-1.44c0.18-0.16,0.26-0.36,0.26-0.6s-0.09-0.44-0.26-0.6c-0.16-0.18-0.36-0.26-0.6-0.26c-0.23,0-0.42,0.09-0.58,0.26\n\tl-1.44,1.44C22.56,5.59,22.49,5.79,22.49,6.04z M23.26,17.95c0,0.23,0.08,0.43,0.25,0.6l0.65,0.63c0.18,0.17,0.39,0.25,0.62,0.25\n\tl0.02,0.02c0.22,0,0.4-0.09,0.54-0.27c0.18-0.16,0.26-0.36,0.26-0.6c0-0.23-0.09-0.43-0.26-0.61l-0.62-0.62\n\tc-0.18-0.18-0.38-0.27-0.61-0.27c-0.24,0-0.44,0.09-0.6,0.26C23.35,17.51,23.26,17.72,23.26,17.95z M24.73,11.58\n\tc0,0.24,0.09,0.44,0.26,0.59c0.16,0.18,0.36,0.26,0.6,0.26h2.06c0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6\n\ts-0.08-0.44-0.25-0.6c-0.17-0.16-0.37-0.24-0.61-0.24h-2.06c-0.24,0-0.45,0.08-0.61,0.24C24.81,11.14,24.73,11.34,24.73,11.58z\" />",
};
#[cfg(feature = "WiDaySnow")]
const WI_DAY_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.58,16.93c0,0.86,0.21,1.67,0.64,2.41c0.42,0.74,1,1.34,1.74,1.79c0.73,0.45,1.54,0.69,2.4,0.71\n\tc0.11,0,0.17-0.06,0.17-0.17v-1.33c0-0.12-0.06-0.19-0.17-0.19c-0.85-0.04-1.58-0.38-2.18-1.02s-0.9-1.37-0.9-2.21\n\tc0-0.82,0.28-1.54,0.85-2.16c0.57-0.61,1.26-0.97,2.1-1.07l0.53-0.06c0.12,0,0.18-0.06,0.18-0.19l0.08-0.51\n\tc0.11-1.09,0.56-2,1.36-2.73c0.8-0.73,1.75-1.09,2.85-1.09c1.09,0,2.04,0.36,2.85,1.09c0.82,0.73,1.28,1.63,1.38,2.7l0.07,0.58\n\tc0,0.11,0.06,0.17,0.17,0.17h1.61c0.9,0,1.67,0.32,2.31,0.96c0.64,0.64,0.96,1.4,0.96,2.29c0,0.84-0.3,1.57-0.9,2.21\n\tc-0.6,0.63-1.33,0.97-2.17,1.02c-0.12,0-0.19,0.06-0.19,0.19v1.33c0,0.11,0.06,0.17,0.19,0.17c1.33-0.04,2.45-0.54,3.38-1.5\n\tc0.93-0.96,1.39-2.09,1.39-3.41c0-0.76-0.14-1.43-0.43-2.03C22.6,13.95,23,12.85,23,11.6c0-0.94-0.23-1.81-0.7-2.61\n\tc-0.47-0.8-1.11-1.44-1.91-1.91s-1.68-0.7-2.62-0.7c-1.54,0-2.83,0.58-3.87,1.73c-0.81-0.44-1.71-0.66-2.69-0.66\n\tc-1.41,0-2.65,0.44-3.74,1.31s-1.78,1.99-2.09,3.34c-1.12,0.28-2.03,0.86-2.74,1.75C1.93,14.75,1.58,15.77,1.58,16.93z M7.92,20.98\n\tc0,0.24,0.08,0.44,0.24,0.61c0.16,0.17,0.35,0.25,0.59,0.25c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61\n\tc0-0.23-0.08-0.42-0.24-0.58s-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.59,0.24S7.92,20.76,7.92,20.98z M7.92,24.61\n\tc0,0.21,0.08,0.4,0.24,0.57c0.18,0.16,0.37,0.24,0.58,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.24-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.23,0-0.43,0.08-0.59,0.23C8,24.17,7.92,24.37,7.92,24.61z\n\t M9.97,4.68c0,0.24,0.08,0.44,0.24,0.59l0.66,0.66c0.16,0.16,0.34,0.25,0.53,0.25c0.21,0.03,0.41-0.04,0.61-0.22\n\tc0.2-0.18,0.3-0.39,0.3-0.63c0-0.24-0.08-0.46-0.24-0.64l-0.64-0.61c-0.15-0.17-0.34-0.25-0.58-0.25c-0.25,0-0.46,0.08-0.63,0.25\n\tC10.05,4.24,9.97,4.44,9.97,4.68z M11.1,22.9c0,0.22,0.08,0.42,0.24,0.6c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.6-0.24\n\ts0.25-0.36,0.25-0.6c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.6-0.25c-0.23,0-0.42,0.08-0.58,0.25S11.1,22.67,11.1,22.9z M11.1,19.3\n\tc0,0.23,0.08,0.42,0.24,0.58s0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.6-0.24c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.08-0.43-0.25-0.59s-0.37-0.24-0.6-0.24c-0.23,0-0.42,0.08-0.58,0.24S11.1,19.07,11.1,19.3z M11.1,26.56\n\tc0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.6-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\ts-0.08-0.44-0.25-0.6c-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25C11.18,26.13,11.1,26.33,11.1,26.56z M14.32,20.98\n\tc0,0.24,0.08,0.44,0.24,0.61c0.16,0.17,0.36,0.25,0.59,0.25s0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61\n\tc0-0.23-0.08-0.42-0.24-0.58s-0.35-0.24-0.59-0.24s-0.43,0.08-0.59,0.24S14.32,20.76,14.32,20.98z M14.32,24.61\n\tc0,0.21,0.08,0.4,0.23,0.57c0.18,0.16,0.38,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.24-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.24C14.4,24.18,14.32,24.38,14.32,24.61z\n\t M15.3,9.06c0.69-0.66,1.51-0.99,2.47-0.99c0.97,0,1.8,0.35,2.48,1.04c0.69,0.69,1.03,1.53,1.03,2.49c0,0.62-0.17,1.24-0.51,1.84\n\tC19.82,12.48,18.66,12,17.3,12h-0.32C16.68,10.83,16.12,9.85,15.3,9.06z M16.9,3.84c0,0.23,0.08,0.43,0.25,0.58s0.37,0.23,0.61,0.23\n\ts0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58V1.8c0-0.24-0.08-0.44-0.24-0.61S18,0.94,17.77,0.94s-0.43,0.09-0.6,0.26\n\tc-0.17,0.17-0.26,0.37-0.26,0.6V3.84z M22.42,6.11c0,0.23,0.08,0.43,0.25,0.59c0.15,0.16,0.34,0.24,0.56,0.26s0.43-0.07,0.62-0.26\n\tl1.43-1.43c0.18-0.18,0.26-0.38,0.26-0.61c0-0.24-0.09-0.44-0.26-0.61c-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25\n\tl-1.43,1.46C22.5,5.67,22.42,5.87,22.42,6.11z M23.22,17.91c0,0.25,0.08,0.46,0.24,0.62l0.64,0.63c0.24,0.16,0.46,0.24,0.64,0.24\n\tc0.21,0,0.39-0.09,0.56-0.26c0.17-0.17,0.25-0.38,0.25-0.61c0-0.23-0.09-0.42-0.26-0.58l-0.62-0.65c-0.18-0.16-0.38-0.24-0.61-0.24\n\ts-0.43,0.08-0.59,0.25C23.3,17.47,23.22,17.67,23.22,17.91z M24.67,11.6c0,0.24,0.09,0.43,0.26,0.59c0.17,0.18,0.38,0.27,0.62,0.27\n\th2.02c0.23,0,0.43-0.08,0.6-0.25s0.25-0.37,0.25-0.61c0-0.24-0.08-0.44-0.25-0.6s-0.37-0.25-0.6-0.25h-2.02\n\tc-0.24,0-0.44,0.08-0.62,0.25S24.67,11.37,24.67,11.6z\" />",
};
#[cfg(feature = "WiDaySnowThunderstorm")]
const WI_DAY_SNOW_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.49,16.88c0,1.12,0.33,2.12,1,3s1.53,1.47,2.58,1.76l-0.66,1.7c-0.05,0.14,0,0.22,0.14,0.22h2.13l-1.43,4.21h0.29\n\tl4.36-5.66c0.04-0.04,0.04-0.09,0.02-0.14C9.9,21.92,9.85,21.9,9.78,21.9H7.59l2.49-4.65c0.07-0.14,0.03-0.22-0.14-0.22H6.98\n\tc-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.88C4.96,19.88,4.36,19.5,3.9,18.9c-0.47-0.59-0.7-1.26-0.7-2.02c0-0.84,0.28-1.57,0.84-2.18\n\tc0.56-0.61,1.27-0.97,2.11-1.07l0.51-0.03c0.12,0,0.19-0.05,0.22-0.14l0.07-0.59c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1\n\tc1.09,0,2.04,0.37,2.86,1.1s1.29,1.64,1.4,2.72l0.08,0.59c0,0.11,0.06,0.17,0.18,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96\n\ts0.97,1.4,0.97,2.29c0,0.87-0.3,1.62-0.9,2.26s-1.32,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc0.88-0.02,1.69-0.26,2.42-0.72c0.73-0.45,1.31-1.06,1.74-1.81s0.64-1.57,0.64-2.45c0-0.73-0.14-1.4-0.43-2.02\n\tc0.76-0.96,1.14-2.06,1.14-3.29c0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71\n\tc-0.72,0-1.42,0.15-2.1,0.45c-0.68,0.3-1.26,0.72-1.76,1.25c-0.81-0.43-1.71-0.65-2.72-0.65c-1.42,0-2.68,0.44-3.77,1.32\n\ts-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74C1.85,14.7,1.49,15.73,1.49,16.88z M9.9,4.59c0,0.23,0.08,0.43,0.25,0.6l0.65,0.66\n\tc0.16,0.16,0.34,0.24,0.55,0.26c0.21,0.03,0.41-0.04,0.61-0.23c0.2-0.18,0.3-0.39,0.3-0.64c0-0.23-0.08-0.43-0.25-0.6l-0.63-0.66\n\tc-0.16-0.16-0.36-0.24-0.6-0.24c-0.25,0-0.46,0.08-0.63,0.24C9.99,4.16,9.9,4.36,9.9,4.59z M11.08,22.96c0,0.24,0.08,0.44,0.24,0.59\n\tc0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.61-0.24s0.25-0.36,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61s-0.37-0.26-0.61-0.26\n\tc-0.22,0-0.41,0.09-0.58,0.26S11.08,22.72,11.08,22.96z M11.08,19.32c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.45-0.08,0.61-0.23s0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25\n\tS11.08,19.09,11.08,19.32z M11.08,26.63c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.61-0.24\n\tc0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26\n\tC11.16,26.19,11.08,26.4,11.08,26.63z M14.31,21.02c0,0.24,0.08,0.44,0.25,0.6s0.36,0.25,0.6,0.25c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58c-0.16-0.16-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.6,0.24\n\tS14.31,20.79,14.31,21.02z M14.31,24.66c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.24\n\tc0.16-0.16,0.23-0.35,0.23-0.59c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.23\n\tC14.39,24.22,14.31,24.42,14.31,24.66z M15.31,9.02c0.67-0.64,1.48-0.97,2.45-0.97c0.98,0,1.82,0.34,2.51,1.03\n\tc0.69,0.68,1.04,1.52,1.04,2.5c0,0.66-0.16,1.26-0.47,1.81c-0.96-0.96-2.13-1.44-3.52-1.44h-0.31C16.72,10.76,16.15,9.78,15.31,9.02\n\tz M16.91,3.75c0,0.24,0.08,0.44,0.25,0.61s0.37,0.25,0.6,0.25c0.24,0,0.44-0.08,0.6-0.25c0.16-0.17,0.24-0.37,0.24-0.61V1.69\n\tc0-0.24-0.08-0.45-0.24-0.61C18.2,0.91,18,0.82,17.76,0.82c-0.24,0-0.44,0.08-0.6,0.25s-0.25,0.37-0.25,0.61V3.75z M22.49,6.04\n\tc0,0.24,0.08,0.44,0.23,0.6c0.14,0.16,0.32,0.24,0.55,0.26c0.23,0.02,0.44-0.07,0.63-0.26l1.44-1.44c0.18-0.16,0.26-0.36,0.26-0.6\n\ts-0.09-0.44-0.26-0.6c-0.16-0.18-0.36-0.26-0.6-0.26c-0.23,0-0.42,0.09-0.58,0.26l-1.44,1.44C22.56,5.59,22.49,5.79,22.49,6.04z\n\t M23.26,17.95c0,0.23,0.08,0.43,0.25,0.6l0.65,0.63c0.18,0.17,0.39,0.25,0.62,0.25l0.02,0.02c0.22,0,0.4-0.09,0.54-0.27\n\tc0.18-0.16,0.26-0.36,0.26-0.6c0-0.23-0.09-0.43-0.26-0.61l-0.62-0.62c-0.18-0.18-0.38-0.27-0.61-0.27c-0.24,0-0.44,0.09-0.6,0.26\n\tC23.35,17.51,23.26,17.72,23.26,17.95z M24.73,11.58c0,0.24,0.09,0.44,0.26,0.59c0.16,0.18,0.36,0.26,0.6,0.26h2.06\n\tc0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6s-0.08-0.44-0.25-0.6c-0.17-0.16-0.37-0.24-0.61-0.24h-2.06\n\tc-0.24,0-0.45,0.08-0.61,0.24C24.81,11.14,24.73,11.34,24.73,11.58z\" />",
};
#[cfg(feature = "WiDaySnowWind")]
const WI_DAY_SNOW_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.58,16.93c0,0.86,0.21,1.67,0.64,2.41c0.42,0.74,1,1.34,1.74,1.79c0.73,0.45,1.54,0.69,2.4,0.71\n\tc0.11,0,0.17-0.06,0.17-0.17v-1.33c0-0.12-0.06-0.19-0.17-0.19c-0.85-0.04-1.58-0.38-2.18-1.02s-0.9-1.37-0.9-2.21\n\tc0-0.82,0.28-1.54,0.85-2.16c0.57-0.61,1.26-0.97,2.1-1.07l0.53-0.06c0.12,0,0.18-0.06,0.18-0.19l0.08-0.51\n\tc0.11-1.09,0.56-2,1.36-2.73c0.8-0.73,1.75-1.09,2.85-1.09c1.09,0,2.04,0.36,2.85,1.09c0.82,0.73,1.28,1.63,1.38,2.7l0.07,0.58\n\tc0,0.11,0.06,0.17,0.17,0.17h1.61c0.9,0,1.67,0.32,2.31,0.96c0.64,0.64,0.96,1.4,0.96,2.29c0,0.84-0.3,1.57-0.9,2.21\n\tc-0.6,0.63-1.33,0.97-2.17,1.02c-0.12,0-0.19,0.06-0.19,0.19v1.33c0,0.11,0.06,0.17,0.19,0.17c1.33-0.04,2.45-0.54,3.38-1.5\n\tc0.93-0.96,1.39-2.09,1.39-3.41c0-0.76-0.14-1.43-0.43-2.03C22.6,13.95,23,12.85,23,11.6c0-0.94-0.23-1.81-0.7-2.61\n\tc-0.47-0.8-1.11-1.44-1.91-1.91s-1.68-0.7-2.62-0.7c-1.54,0-2.83,0.58-3.87,1.73c-0.81-0.44-1.71-0.66-2.69-0.66\n\tc-1.41,0-2.65,0.44-3.74,1.31s-1.78,1.99-2.09,3.34c-1.12,0.28-2.03,0.86-2.74,1.75C1.93,14.75,1.58,15.77,1.58,16.93z M7.06,24.61\n\tc0,0.21,0.08,0.4,0.24,0.57c0.18,0.16,0.37,0.24,0.58,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.24-0.59c-0.16-0.16-0.35-0.23-0.59-0.23s-0.43,0.08-0.59,0.23C7.14,24.17,7.06,24.37,7.06,24.61z M7.92,20.98\n\tc0,0.24,0.08,0.44,0.24,0.61c0.16,0.17,0.35,0.25,0.59,0.25c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61\n\tc0-0.23-0.08-0.42-0.24-0.58s-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.59,0.24S7.92,20.76,7.92,20.98z M9.82,26.56\n\tc0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.6-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\ts-0.08-0.44-0.25-0.6c-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25C9.9,26.13,9.82,26.33,9.82,26.56z M9.97,4.68\n\tc0,0.24,0.08,0.44,0.24,0.59l0.66,0.66c0.16,0.16,0.34,0.25,0.53,0.25c0.21,0.03,0.41-0.04,0.61-0.22c0.2-0.18,0.3-0.39,0.3-0.63\n\tc0-0.24-0.08-0.46-0.24-0.64l-0.64-0.61c-0.15-0.17-0.34-0.25-0.58-0.25c-0.25,0-0.46,0.08-0.63,0.25\n\tC10.05,4.24,9.97,4.44,9.97,4.68z M10.67,22.9c0,0.22,0.08,0.42,0.25,0.6c0.16,0.16,0.35,0.24,0.57,0.24c0.24,0,0.44-0.08,0.61-0.24\n\tc0.17-0.16,0.25-0.36,0.25-0.6c0-0.23-0.08-0.43-0.25-0.6c-0.17-0.17-0.37-0.25-0.61-0.25c-0.22,0-0.42,0.08-0.58,0.25\n\tC10.75,22.47,10.67,22.67,10.67,22.9z M11.1,19.3c0,0.23,0.08,0.42,0.24,0.58s0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.6-0.24\n\tc0.17-0.16,0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.59s-0.37-0.24-0.6-0.24c-0.23,0-0.42,0.08-0.58,0.24S11.1,19.07,11.1,19.3z\n\t M13.47,24.61c0,0.21,0.08,0.4,0.23,0.57c0.17,0.16,0.38,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.24S13.47,24.38,13.47,24.61z M14.32,20.98\n\tc0,0.24,0.08,0.44,0.24,0.61c0.16,0.17,0.36,0.25,0.59,0.25s0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61\n\tc0-0.23-0.08-0.42-0.24-0.58s-0.35-0.24-0.59-0.24s-0.43,0.08-0.59,0.24S14.32,20.76,14.32,20.98z M15.3,9.06\n\tc0.69-0.66,1.51-0.99,2.47-0.99c0.97,0,1.8,0.35,2.48,1.04c0.69,0.69,1.03,1.53,1.03,2.49c0,0.62-0.17,1.24-0.51,1.84\n\tC19.82,12.48,18.66,12,17.3,12h-0.32C16.68,10.83,16.12,9.85,15.3,9.06z M16.9,3.84c0,0.23,0.08,0.43,0.25,0.58s0.37,0.23,0.61,0.23\n\ts0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58V1.8c0-0.24-0.08-0.44-0.24-0.61S18,0.94,17.77,0.94s-0.43,0.09-0.6,0.26\n\tc-0.17,0.17-0.26,0.37-0.26,0.6V3.84z M22.42,6.11c0,0.23,0.08,0.43,0.25,0.59c0.15,0.16,0.34,0.24,0.56,0.26s0.43-0.07,0.62-0.26\n\tl1.43-1.43c0.18-0.18,0.26-0.38,0.26-0.61c0-0.24-0.09-0.44-0.26-0.61c-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25\n\tl-1.43,1.46C22.5,5.67,22.42,5.87,22.42,6.11z M23.22,17.91c0,0.25,0.08,0.46,0.24,0.62l0.64,0.63c0.24,0.16,0.46,0.24,0.64,0.24\n\tc0.21,0,0.39-0.09,0.56-0.26c0.17-0.17,0.25-0.38,0.25-0.61c0-0.23-0.09-0.42-0.26-0.58l-0.62-0.65c-0.18-0.16-0.38-0.24-0.61-0.24\n\ts-0.43,0.08-0.59,0.25C23.3,17.47,23.22,17.67,23.22,17.91z M24.67,11.6c0,0.24,0.09,0.43,0.26,0.59c0.17,0.18,0.38,0.27,0.62,0.27\n\th2.02c0.23,0,0.43-0.08,0.6-0.25s0.25-0.37,0.25-0.61c0-0.24-0.08-0.44-0.25-0.6s-0.37-0.25-0.6-0.25h-2.02\n\tc-0.24,0-0.44,0.08-0.62,0.25S24.67,11.37,24.67,11.6z\" />",
};
#[cfg(feature = "WiDaySprinkle")]
const WI_DAY_SPRINKLE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.58,16.89c0,0.87,0.21,1.68,0.64,2.42c0.42,0.75,1,1.35,1.73,1.8c0.73,0.45,1.53,0.69,2.4,0.73\n\tc0.12,0,0.18-0.06,0.18-0.17v-1.33c0-0.12-0.06-0.19-0.18-0.19c-0.85-0.04-1.58-0.38-2.18-1.02s-0.9-1.38-0.9-2.25\n\tc0-0.82,0.28-1.54,0.84-2.15s1.26-0.96,2.09-1.06l0.52-0.03c0.12,0,0.19-0.06,0.19-0.18l0.08-0.54c0.11-1.08,0.56-1.98,1.36-2.71\n\tc0.8-0.73,1.75-1.09,2.85-1.09c1.07,0,2.02,0.36,2.84,1.09c0.82,0.73,1.28,1.63,1.4,2.7l0.07,0.58c0,0.11,0.06,0.17,0.17,0.17h1.62\n\tc0.88,0,1.65,0.32,2.29,0.96c0.65,0.64,0.97,1.39,0.97,2.26c0,0.86-0.3,1.61-0.9,2.25c-0.6,0.63-1.33,0.97-2.18,1.02\n\tc-0.12,0-0.18,0.06-0.18,0.19v1.33c0,0.11,0.06,0.17,0.18,0.17c0.87-0.02,1.67-0.26,2.4-0.72s1.31-1.05,1.73-1.8\n\ts0.63-1.56,0.63-2.43c0-0.74-0.14-1.4-0.41-2c0.78-0.95,1.16-2.05,1.16-3.3c0-0.94-0.23-1.82-0.7-2.62s-1.1-1.44-1.9-1.9\n\tc-0.8-0.47-1.67-0.7-2.61-0.7c-1.55,0-2.84,0.58-3.87,1.73c-0.82-0.44-1.72-0.66-2.69-0.66c-1.41,0-2.65,0.44-3.74,1.31\n\ts-1.78,1.99-2.09,3.34c-1.1,0.26-2.01,0.84-2.72,1.73C1.93,14.73,1.58,15.75,1.58,16.89z M7.48,17.77c0,0.38,0.14,0.71,0.42,0.98\n\ts0.62,0.4,1.01,0.4c0.4,0,0.73-0.13,1-0.4c0.27-0.27,0.4-0.59,0.4-0.98c0-0.24-0.12-0.58-0.35-1c-0.23-0.43-0.45-0.76-0.65-0.99\n\tc-0.21-0.22-0.35-0.37-0.4-0.42l-0.36,0.4c-0.27,0.29-0.52,0.63-0.74,1.03S7.48,17.52,7.48,17.77z M9.97,4.66\n\tc0,0.26,0.08,0.46,0.24,0.61l0.65,0.66c0.42,0.31,0.82,0.31,1.21,0c0.16-0.19,0.24-0.41,0.24-0.64c0-0.23-0.08-0.43-0.24-0.59\n\tl-0.64-0.65c-0.19-0.17-0.39-0.25-0.61-0.25c-0.24,0-0.45,0.08-0.61,0.25C10.05,4.22,9.97,4.42,9.97,4.66z M10.45,21.73\n\tc0,0.66,0.23,1.21,0.68,1.65s1,0.67,1.65,0.67c0.65,0,1.2-0.23,1.66-0.68c0.46-0.46,0.68-1,0.68-1.64c0-0.54-0.27-1.19-0.81-1.97\n\tc-0.46-0.61-0.89-1.1-1.28-1.49c-0.08-0.06-0.17-0.13-0.26-0.23l-0.23,0.23c-0.36,0.32-0.78,0.82-1.27,1.47\n\tC10.73,20.5,10.45,21.16,10.45,21.73z M11.93,15.11c0,0.25,0.1,0.47,0.29,0.65c0.19,0.18,0.42,0.27,0.69,0.27\n\tc0.26,0,0.48-0.09,0.66-0.27c0.18-0.18,0.27-0.4,0.27-0.65c0-0.41-0.31-0.95-0.93-1.6l-0.24,0.25c-0.18,0.2-0.35,0.43-0.5,0.7\n\tC12,14.73,11.93,14.94,11.93,15.11z M15.3,9.06c0.66-0.66,1.48-0.99,2.47-0.99c0.98,0,1.8,0.34,2.49,1.03\n\tc0.68,0.69,1.03,1.52,1.03,2.5c0,0.59-0.17,1.2-0.52,1.84C19.8,12.48,18.64,12,17.3,12h-0.33C16.68,10.84,16.12,9.86,15.3,9.06z\n\t M16.9,3.8c0,0.24,0.09,0.44,0.26,0.61c0.17,0.17,0.37,0.25,0.6,0.25s0.43-0.08,0.58-0.25c0.16-0.17,0.23-0.37,0.23-0.61V1.76\n\tc0-0.24-0.08-0.43-0.23-0.59S18,0.94,17.77,0.94s-0.44,0.08-0.61,0.23S16.9,1.52,16.9,1.76V3.8z M22.42,6.11\n\tc0,0.24,0.08,0.44,0.25,0.59c0.17,0.16,0.36,0.24,0.56,0.24c0.17,0,0.38-0.08,0.61-0.24l1.43-1.43c0.18-0.18,0.27-0.39,0.27-0.62\n\tc0-0.24-0.08-0.45-0.25-0.61c-0.17-0.16-0.37-0.24-0.61-0.24c-0.22,0-0.41,0.08-0.58,0.25l-1.43,1.43\n\tC22.5,5.65,22.42,5.86,22.42,6.11z M23.22,17.89c0,0.25,0.08,0.46,0.23,0.64l0.65,0.58c0.14,0.18,0.34,0.27,0.59,0.27\n\tc0.24,0,0.44-0.09,0.58-0.27c0.18-0.16,0.27-0.36,0.27-0.58c0-0.22-0.09-0.41-0.27-0.58l-0.61-0.65c-0.18-0.16-0.38-0.24-0.61-0.24\n\tc-0.24,0-0.44,0.08-0.6,0.24S23.22,17.66,23.22,17.89z M24.67,11.6c0,0.22,0.08,0.42,0.25,0.58c0.17,0.16,0.38,0.24,0.63,0.24h2.02\n\tc0.24,0,0.44-0.08,0.6-0.24c0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.6s-0.37-0.25-0.6-0.25h-2.02\n\tc-0.24,0-0.44,0.08-0.62,0.25S24.67,11.37,24.67,11.6z\" />",
};
#[cfg(feature = "WiDayStormShowers")]
const WI_DAY_STORM_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.49,16.88c0,1.12,0.33,2.12,1,3s1.53,1.47,2.58,1.76l-0.66,1.7c-0.05,0.14,0,0.22,0.14,0.22h2.13l-1.43,4.21h0.29\n\tl4.36-5.66c0.04-0.04,0.04-0.09,0.02-0.14C9.9,21.92,9.85,21.9,9.78,21.9H7.59l2.49-4.65c0.07-0.14,0.03-0.22-0.14-0.22H6.98\n\tc-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.88C4.96,19.88,4.36,19.5,3.9,18.9c-0.47-0.59-0.7-1.26-0.7-2.02c0-0.84,0.28-1.57,0.84-2.18\n\tc0.56-0.61,1.27-0.97,2.11-1.07l0.51-0.03c0.12,0,0.19-0.05,0.22-0.14l0.07-0.59c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1\n\tc1.09,0,2.04,0.37,2.86,1.1s1.29,1.64,1.4,2.72l0.08,0.59c0,0.11,0.06,0.17,0.18,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96\n\ts0.97,1.4,0.97,2.29c0,0.87-0.3,1.62-0.9,2.26s-1.32,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc0.88-0.02,1.69-0.26,2.42-0.72c0.73-0.45,1.31-1.06,1.74-1.81s0.64-1.57,0.64-2.45c0-0.73-0.14-1.4-0.43-2.02\n\tc0.76-0.96,1.14-2.06,1.14-3.29c0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71\n\tc-0.72,0-1.42,0.15-2.1,0.45c-0.68,0.3-1.26,0.72-1.76,1.25c-0.81-0.43-1.71-0.65-2.72-0.65c-1.42,0-2.68,0.44-3.77,1.32\n\ts-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74C1.85,14.7,1.49,15.73,1.49,16.88z M9.67,26.8c0,0.15,0.05,0.31,0.16,0.47\n\tc0.11,0.16,0.26,0.27,0.46,0.34c0.11,0.03,0.2,0.04,0.25,0.04c0.15,0,0.28-0.03,0.38-0.08c0.21-0.08,0.36-0.27,0.43-0.57l0.27-1.03\n\tc0.06-0.25,0.03-0.47-0.08-0.67s-0.3-0.32-0.53-0.37c-0.21-0.07-0.41-0.04-0.62,0.07c-0.21,0.12-0.35,0.29-0.42,0.52l-0.25,1.04\n\tC9.69,26.7,9.67,26.78,9.67,26.8z M9.9,4.59c0,0.23,0.08,0.43,0.25,0.6l0.65,0.66c0.16,0.16,0.34,0.24,0.55,0.26\n\tc0.21,0.03,0.41-0.04,0.61-0.23c0.2-0.18,0.3-0.39,0.3-0.64c0-0.23-0.08-0.43-0.25-0.6l-0.63-0.66c-0.16-0.16-0.36-0.24-0.6-0.24\n\tc-0.25,0-0.46,0.08-0.63,0.24C9.99,4.16,9.9,4.36,9.9,4.59z M11.01,22c-0.01,0.16,0.04,0.32,0.14,0.47c0.1,0.15,0.26,0.26,0.48,0.32\n\tc0.21,0.07,0.42,0.05,0.62-0.06c0.2-0.11,0.34-0.3,0.42-0.56l0.3-1.03c0.07-0.22,0.04-0.43-0.08-0.63s-0.3-0.34-0.54-0.41\n\tc-0.23-0.07-0.44-0.05-0.64,0.07c-0.2,0.12-0.34,0.29-0.41,0.53l-0.24,1.05C11.03,21.9,11.01,21.98,11.01,22z M13.84,23.68\n\tc0,0.14,0.03,0.28,0.1,0.39c0.13,0.21,0.31,0.36,0.54,0.43c0.11,0.04,0.21,0.06,0.28,0.06c0.13,0,0.23-0.02,0.31-0.08\n\tc0.2-0.07,0.35-0.27,0.45-0.6l0.25-1.01c0.07-0.24,0.05-0.45-0.07-0.65c-0.11-0.2-0.28-0.33-0.51-0.39\n\tc-0.23-0.07-0.45-0.05-0.65,0.07c-0.2,0.11-0.34,0.28-0.41,0.51l-0.28,1.04C13.85,23.53,13.84,23.61,13.84,23.68z M15.21,18.86\n\tc0,0.18,0.05,0.34,0.16,0.5c0.11,0.16,0.27,0.27,0.49,0.33c0.17,0.06,0.37,0.04,0.61-0.05c0.2-0.09,0.34-0.28,0.43-0.57l0.27-1\n\tc0.06-0.25,0.04-0.47-0.08-0.67s-0.29-0.32-0.53-0.37c-0.23-0.07-0.44-0.05-0.64,0.06c-0.2,0.11-0.33,0.28-0.4,0.5l-0.29,1.06\n\tC15.22,18.79,15.21,18.86,15.21,18.86z M15.31,9.02c0.67-0.64,1.48-0.97,2.45-0.97c0.98,0,1.82,0.34,2.51,1.03\n\tc0.69,0.68,1.04,1.52,1.04,2.5c0,0.66-0.16,1.26-0.47,1.81c-0.96-0.96-2.13-1.44-3.52-1.44h-0.31C16.72,10.76,16.15,9.78,15.31,9.02\n\tz M16.91,3.75c0,0.24,0.08,0.44,0.25,0.61s0.37,0.25,0.6,0.25c0.24,0,0.44-0.08,0.6-0.25c0.16-0.17,0.24-0.37,0.24-0.61V1.69\n\tc0-0.24-0.08-0.45-0.24-0.61C18.2,0.91,18,0.82,17.76,0.82c-0.24,0-0.44,0.08-0.6,0.25s-0.25,0.37-0.25,0.61V3.75z M22.49,6.04\n\tc0,0.24,0.08,0.44,0.23,0.6c0.14,0.16,0.32,0.24,0.55,0.26c0.23,0.02,0.44-0.07,0.63-0.26l1.44-1.44c0.18-0.16,0.26-0.36,0.26-0.6\n\ts-0.09-0.44-0.26-0.6c-0.16-0.18-0.36-0.26-0.6-0.26c-0.23,0-0.42,0.09-0.58,0.26l-1.44,1.44C22.56,5.59,22.49,5.79,22.49,6.04z\n\t M23.26,17.95c0,0.23,0.08,0.43,0.25,0.6l0.65,0.63c0.18,0.17,0.39,0.25,0.62,0.25l0.02,0.02c0.22,0,0.4-0.09,0.54-0.27\n\tc0.18-0.16,0.26-0.36,0.26-0.6c0-0.23-0.09-0.43-0.26-0.61l-0.62-0.62c-0.18-0.18-0.38-0.27-0.61-0.27c-0.24,0-0.44,0.09-0.6,0.26\n\tC23.35,17.51,23.26,17.72,23.26,17.95z M24.73,11.58c0,0.24,0.09,0.44,0.26,0.59c0.16,0.18,0.36,0.26,0.6,0.26h2.06\n\tc0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6s-0.08-0.44-0.25-0.6c-0.17-0.16-0.37-0.24-0.61-0.24h-2.06\n\tc-0.24,0-0.45,0.08-0.61,0.24C24.81,11.14,24.73,11.34,24.73,11.58z\" />",
};
#[cfg(feature = "WiDaySunny")]
const WI_DAY_SUNNY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.37,14.62c0-0.24,0.08-0.45,0.25-0.62c0.17-0.16,0.38-0.24,0.6-0.24h2.04c0.23,0,0.42,0.08,0.58,0.25\n\tc0.15,0.17,0.23,0.37,0.23,0.61S8,15.06,7.85,15.23c-0.15,0.17-0.35,0.25-0.58,0.25H5.23c-0.23,0-0.43-0.08-0.6-0.25\n\tC4.46,15.06,4.37,14.86,4.37,14.62z M7.23,21.55c0-0.23,0.08-0.43,0.23-0.61l1.47-1.43c0.15-0.16,0.35-0.23,0.59-0.23\n\tc0.24,0,0.44,0.08,0.6,0.23s0.24,0.34,0.24,0.57c0,0.24-0.08,0.46-0.24,0.64L8.7,22.14c-0.41,0.32-0.82,0.32-1.23,0\n\tC7.31,21.98,7.23,21.78,7.23,21.55z M7.23,7.71c0-0.23,0.08-0.43,0.23-0.61C7.66,6.93,7.87,6.85,8.1,6.85\n\tc0.22,0,0.42,0.08,0.59,0.24l1.43,1.47c0.16,0.15,0.24,0.35,0.24,0.59c0,0.24-0.08,0.44-0.24,0.6s-0.36,0.24-0.6,0.24\n\tc-0.24,0-0.44-0.08-0.59-0.24L7.47,8.32C7.31,8.16,7.23,7.95,7.23,7.71z M9.78,14.62c0-0.93,0.23-1.8,0.7-2.6s1.1-1.44,1.91-1.91\n\ts1.67-0.7,2.6-0.7c0.7,0,1.37,0.14,2.02,0.42c0.64,0.28,1.2,0.65,1.66,1.12c0.47,0.47,0.84,1.02,1.11,1.66\n\tc0.27,0.64,0.41,1.32,0.41,2.02c0,0.94-0.23,1.81-0.7,2.61c-0.47,0.8-1.1,1.43-1.9,1.9c-0.8,0.47-1.67,0.7-2.61,0.7\n\ts-1.81-0.23-2.61-0.7c-0.8-0.47-1.43-1.1-1.9-1.9C10.02,16.43,9.78,15.56,9.78,14.62z M11.48,14.62c0,0.98,0.34,1.81,1.03,2.5\n\tc0.68,0.69,1.51,1.04,2.49,1.04s1.81-0.35,2.5-1.04s1.04-1.52,1.04-2.5c0-0.96-0.35-1.78-1.04-2.47c-0.69-0.68-1.52-1.02-2.5-1.02\n\tc-0.97,0-1.8,0.34-2.48,1.02C11.82,12.84,11.48,13.66,11.48,14.62z M14.14,22.4c0-0.24,0.08-0.44,0.25-0.6s0.37-0.24,0.6-0.24\n\tc0.24,0,0.45,0.08,0.61,0.24s0.24,0.36,0.24,0.6v1.99c0,0.24-0.08,0.45-0.25,0.62c-0.17,0.17-0.37,0.25-0.6,0.25\n\ts-0.44-0.08-0.6-0.25c-0.17-0.17-0.25-0.38-0.25-0.62V22.4z M14.14,6.9V4.86c0-0.23,0.08-0.43,0.25-0.6C14.56,4.09,14.76,4,15,4\n\ts0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6V6.9c0,0.23-0.08,0.42-0.25,0.58S15.23,7.71,15,7.71s-0.44-0.08-0.6-0.23\n\tS14.14,7.13,14.14,6.9z M19.66,20.08c0-0.23,0.08-0.42,0.23-0.56c0.15-0.16,0.34-0.23,0.56-0.23c0.24,0,0.44,0.08,0.6,0.23\n\tl1.46,1.43c0.16,0.17,0.24,0.38,0.24,0.61c0,0.23-0.08,0.43-0.24,0.59c-0.4,0.31-0.8,0.31-1.2,0l-1.42-1.42\n\tC19.74,20.55,19.66,20.34,19.66,20.08z M19.66,9.16c0-0.25,0.08-0.45,0.23-0.59l1.42-1.47c0.17-0.16,0.37-0.24,0.59-0.24\n\tc0.24,0,0.44,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6c0,0.25-0.08,0.46-0.24,0.62l-1.46,1.43c-0.18,0.16-0.38,0.24-0.6,0.24\n\tc-0.23,0-0.41-0.08-0.56-0.24S19.66,9.4,19.66,9.16z M21.92,14.62c0-0.24,0.08-0.44,0.24-0.62c0.16-0.16,0.35-0.24,0.57-0.24h2.02\n\tc0.23,0,0.43,0.09,0.6,0.26c0.17,0.17,0.26,0.37,0.26,0.6s-0.09,0.43-0.26,0.6c-0.17,0.17-0.37,0.25-0.6,0.25h-2.02\n\tc-0.23,0-0.43-0.08-0.58-0.25S21.92,14.86,21.92,14.62z\" />",
};
#[cfg(feature = "WiDaySunnyOvercast")]
const WI_DAY_SUNNY_OVERCAST: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.09,13.43c0,0.27,0.09,0.49,0.28,0.67s0.43,0.26,0.72,0.26h1.69c0.27,0,0.5-0.09,0.69-0.27s0.29-0.4,0.29-0.66\n\tc0-0.29-0.09-0.53-0.28-0.71s-0.42-0.28-0.69-0.28H4.09c-0.29,0-0.53,0.09-0.72,0.28C3.18,12.89,3.09,13.13,3.09,13.43z M4.86,19.61\n\tc0,0.97,0.35,1.81,1.06,2.52c0.71,0.71,1.54,1.06,2.51,1.06h6.86c0.97,0,1.8-0.35,2.49-1.05c0.69-0.7,1.04-1.54,1.04-2.53\n\tc0-0.48-0.07-0.89-0.21-1.23c0.83-0.53,1.49-1.24,1.97-2.12c0.48-0.88,0.73-1.83,0.73-2.84c0-0.81-0.16-1.59-0.48-2.33\n\tc-0.32-0.74-0.75-1.38-1.28-1.91c-0.53-0.53-1.17-0.96-1.91-1.28c-0.74-0.32-1.51-0.48-2.32-0.48c-1.09,0-2.1,0.27-3.02,0.81\n\ts-1.65,1.27-2.18,2.18c-0.53,0.92-0.79,1.92-0.79,3.01v0.34c-1,0.57-1.66,1.4-1.98,2.49c-0.76,0.24-1.36,0.66-1.81,1.27\n\tC5.09,18.13,4.86,18.83,4.86,19.61z M6.42,5.45c0,0.28,0.09,0.51,0.26,0.67l1.61,1.69c0.47,0.36,0.94,0.36,1.41,0\n\tc0.19-0.19,0.28-0.42,0.28-0.7c0-0.27-0.09-0.5-0.28-0.7L8.05,4.76C7.84,4.57,7.6,4.47,7.35,4.47c-0.28,0-0.51,0.09-0.68,0.28\n\tC6.5,4.94,6.42,5.17,6.42,5.45z M6.84,19.61c0-0.42,0.13-0.78,0.4-1.08c0.27-0.3,0.61-0.47,1.02-0.51l0.62-0.08\n\tc0.13,0,0.2-0.08,0.2-0.23l0.09-0.56c0.07-0.58,0.31-1.06,0.73-1.44c0.42-0.39,0.91-0.58,1.48-0.58c0.58,0,1.09,0.19,1.51,0.58\n\tc0.43,0.39,0.68,0.87,0.75,1.44l0.08,0.65c0.06,0.15,0.14,0.23,0.24,0.23h1.32c0.43,0,0.8,0.16,1.12,0.47\n\tc0.32,0.31,0.47,0.68,0.47,1.12c0,0.45-0.16,0.83-0.47,1.15s-0.69,0.48-1.12,0.48H8.43c-0.45,0-0.83-0.16-1.13-0.48\n\tC6.99,20.45,6.84,20.06,6.84,19.61z M11.26,13.22c0.07-1.09,0.49-2.01,1.27-2.76c0.77-0.74,1.71-1.12,2.79-1.12\n\tc1.11,0,2.06,0.4,2.84,1.19c0.78,0.79,1.17,1.76,1.17,2.89c0,0.7-0.17,1.35-0.51,1.95c-0.34,0.6-0.8,1.08-1.38,1.45\n\tc-0.59-0.49-1.27-0.73-2.03-0.73c-0.29-0.88-0.81-1.57-1.54-2.09c-0.73-0.52-1.56-0.78-2.48-0.78H11.26z M14.35,4.47\n\tc0,0.27,0.1,0.51,0.29,0.7c0.19,0.19,0.42,0.29,0.69,0.29c0.28,0,0.51-0.1,0.7-0.29c0.19-0.19,0.29-0.42,0.29-0.7V2.13\n\tc0-0.26-0.1-0.48-0.29-0.66c-0.19-0.18-0.42-0.27-0.7-0.27c-0.27,0-0.5,0.09-0.69,0.27c-0.19,0.18-0.29,0.4-0.29,0.66V4.47z\n\t M20.67,19.7c0,0.27,0.09,0.5,0.27,0.7l1.64,1.62c0.42,0.42,0.89,0.42,1.41,0c0.18-0.17,0.26-0.39,0.26-0.68\n\tc0-0.27-0.09-0.49-0.26-0.67L22.3,19c-0.18-0.17-0.41-0.25-0.68-0.25c-0.28,0-0.5,0.09-0.68,0.27C20.76,19.2,20.67,19.43,20.67,19.7\n\tz M20.67,7.09c0,0.28,0.09,0.52,0.27,0.72c0.18,0.18,0.41,0.27,0.68,0.27c0.27,0,0.5-0.09,0.68-0.27l1.69-1.69\n\tc0.18-0.17,0.26-0.39,0.26-0.67c0-0.28-0.1-0.51-0.29-0.69s-0.42-0.28-0.7-0.28c-0.26,0-0.49,0.1-0.68,0.29L20.94,6.4\n\tC20.76,6.58,20.67,6.81,20.67,7.09z M23.25,13.43c0,0.27,0.09,0.49,0.28,0.67s0.43,0.26,0.72,0.26h1.69c0.27,0,0.5-0.09,0.69-0.27\n\ts0.29-0.4,0.29-0.66c0-0.29-0.09-0.53-0.28-0.71s-0.42-0.28-0.69-0.28h-1.69c-0.29,0-0.53,0.09-0.72,0.28\n\tC23.35,12.89,23.25,13.13,23.25,13.43z\" />",
};
#[cfg(feature = "WiDayThunderstorm")]
const WI_DAY_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.52,16.9c0,1.11,0.33,2.09,0.98,2.96s1.51,1.46,2.57,1.78l-0.64,1.7c-0.04,0.14,0,0.21,0.14,0.21H6.7L5.45,27.5h0.29\n\tl4.17-5.39c0.04-0.04,0.04-0.09,0.01-0.14C9.9,21.92,9.85,21.9,9.78,21.9H7.61l2.47-4.63c0.07-0.14,0.02-0.22-0.14-0.22H7\n\tc-0.09,0-0.17,0.05-0.23,0.14L5.7,20.07c-0.71-0.18-1.3-0.57-1.77-1.16c-0.47-0.59-0.7-1.26-0.7-2.01c0-0.83,0.28-1.55,0.85-2.17\n\ts1.27-0.97,2.1-1.07L6.7,13.6c0.13,0,0.2-0.06,0.2-0.18l0.06-0.51c0.11-1.08,0.57-1.99,1.38-2.72c0.81-0.73,1.77-1.1,2.86-1.1\n\tc1.09,0,2.04,0.37,2.85,1.1s1.28,1.64,1.4,2.72l0.06,0.58c0,0.11,0.06,0.17,0.18,0.17h1.61c0.91,0,1.68,0.32,2.32,0.95\n\tc0.64,0.63,0.96,1.39,0.96,2.29c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.32,0.97-2.19,1.04c-0.13,0-0.2,0.06-0.2,0.18v1.37\n\tc0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.39-1.51c0.93-0.96,1.39-2.11,1.39-3.45c0-0.74-0.14-1.41-0.43-2.01\n\tc0.79-0.96,1.18-2.06,1.18-3.32c0-0.94-0.24-1.81-0.71-2.62c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.68-0.71-2.62-0.71\n\tc-1.54,0-2.84,0.58-3.88,1.73c-0.81-0.43-1.71-0.65-2.7-0.65c-1.41,0-2.67,0.44-3.76,1.31s-1.79,1.99-2.1,3.36\n\tc-1.11,0.26-2.02,0.83-2.73,1.73S1.52,15.75,1.52,16.9z M9.61,26.48c-0.01,0.15,0.03,0.3,0.14,0.44s0.26,0.25,0.46,0.33\n\tc0.07,0.02,0.14,0.03,0.21,0.03c0.17,0,0.34-0.05,0.51-0.15s0.28-0.26,0.34-0.47l2.29-8.57c0.06-0.23,0.04-0.45-0.07-0.64\n\tc-0.11-0.2-0.27-0.33-0.49-0.4c-0.23-0.07-0.45-0.05-0.65,0.07c-0.2,0.11-0.34,0.28-0.4,0.51l-2.31,8.6\n\tC9.62,26.3,9.61,26.39,9.61,26.48z M9.94,4.63c0,0.24,0.08,0.43,0.25,0.59l0.64,0.66C11,6.05,11.2,6.13,11.44,6.14\n\tc0.24,0,0.43-0.08,0.57-0.26c0.19-0.15,0.28-0.35,0.28-0.6c0-0.24-0.08-0.43-0.25-0.59l-0.63-0.66c-0.17-0.16-0.38-0.24-0.61-0.24\n\tc-0.25,0-0.46,0.08-0.62,0.24C10.02,4.19,9.94,4.39,9.94,4.63z M13.77,23.43c0,0.12,0.04,0.24,0.11,0.38\n\tc0.13,0.2,0.29,0.34,0.5,0.43c0.07,0.03,0.17,0.05,0.3,0.05c0.15,0,0.26-0.02,0.33-0.06c0.2-0.08,0.34-0.28,0.41-0.58l1.49-5.55\n\tc0.06-0.24,0.04-0.45-0.07-0.65c-0.11-0.19-0.28-0.32-0.51-0.39c-0.23-0.07-0.45-0.05-0.64,0.07c-0.2,0.11-0.33,0.28-0.39,0.51\n\tL13.8,23.2c0,0.02-0.01,0.06-0.02,0.11C13.77,23.37,13.77,23.4,13.77,23.43z M15.3,9.04c0.67-0.64,1.49-0.97,2.48-0.97\n\tc0.97,0,1.81,0.34,2.5,1.02c0.69,0.68,1.04,1.51,1.04,2.48c0,0.62-0.17,1.24-0.52,1.85c-0.99-0.98-2.16-1.47-3.5-1.47h-0.31\n\tC16.68,10.78,16.11,9.81,15.3,9.04z M16.91,3.79c0,0.23,0.09,0.43,0.26,0.6s0.37,0.26,0.6,0.26c0.24,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.23-0.37,0.23-0.61V1.73c0-0.24-0.08-0.44-0.23-0.61s-0.35-0.25-0.59-0.25c-0.23,0-0.43,0.08-0.6,0.25\n\ts-0.26,0.37-0.26,0.61V3.79z M22.44,6.07c0,0.24,0.09,0.44,0.26,0.6c0.14,0.17,0.33,0.25,0.57,0.25s0.44-0.08,0.6-0.25l1.44-1.45\n\tc0.17-0.16,0.26-0.35,0.26-0.59c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.25-0.61-0.25c-0.22,0-0.41,0.09-0.57,0.26L22.7,5.47\n\tC22.53,5.63,22.44,5.83,22.44,6.07z M23.25,17.93c0,0.22,0.08,0.42,0.24,0.6l0.66,0.63c0.12,0.14,0.31,0.23,0.54,0.24l0.01,0.01\n\tc0.01,0,0.03,0,0.05,0c0.02,0,0.03,0,0.05,0c0.19,0,0.36-0.09,0.53-0.26c0.17-0.16,0.26-0.36,0.26-0.6c0-0.23-0.09-0.43-0.26-0.61\n\tl-0.65-0.61c-0.16-0.18-0.36-0.27-0.58-0.27c-0.23,0-0.43,0.08-0.6,0.25C23.33,17.49,23.25,17.7,23.25,17.93z M24.7,11.58\n\tc0,0.23,0.09,0.43,0.27,0.6c0.18,0.18,0.38,0.27,0.61,0.27h2.03c0.23,0,0.43-0.09,0.6-0.26s0.26-0.38,0.26-0.61\n\tc0-0.23-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.24-0.61-0.24h-2.03c-0.25,0-0.46,0.08-0.63,0.24C24.78,11.15,24.7,11.35,24.7,11.58z\n\t\" />",
};
#[cfg(feature = "WiDayWindy")]
const WI_DAY_WINDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M1.48,21.1c0,0.24,0.09,0.44,0.27,0.6c0.17,0.17,0.37,0.25,0.61,0.25h5.88c0.26,0,0.48,0.09,0.68,0.28\n\tc0.2,0.19,0.3,0.42,0.3,0.68s-0.1,0.5-0.3,0.69c-0.2,0.19-0.42,0.29-0.68,0.29c-0.26,0-0.48-0.1-0.68-0.3\n\tc-0.18-0.16-0.38-0.24-0.61-0.24c-0.24,0-0.44,0.08-0.6,0.24c-0.16,0.16-0.24,0.36-0.24,0.6c0,0.24,0.08,0.44,0.24,0.6\n\tc0.53,0.53,1.16,0.8,1.89,0.8c0.74,0,1.37-0.26,1.88-0.78s0.78-1.15,0.78-1.89s-0.26-1.37-0.78-1.89c-0.52-0.52-1.15-0.78-1.88-0.78\n\tH2.36c-0.24,0-0.44,0.08-0.62,0.25C1.57,20.67,1.48,20.87,1.48,21.1z M1.48,18.09c0,0.23,0.09,0.42,0.27,0.58\n\tc0.16,0.16,0.37,0.24,0.61,0.24h10.97c0.74,0,1.37-0.26,1.89-0.78c0.52-0.52,0.78-1.15,0.78-1.89c0-0.74-0.26-1.36-0.78-1.88\n\tc-0.52-0.51-1.15-0.77-1.89-0.77c-0.76,0-1.38,0.25-1.88,0.76c-0.16,0.16-0.23,0.37-0.23,0.61c0,0.24,0.08,0.44,0.23,0.6\n\tc0.15,0.15,0.35,0.23,0.6,0.23c0.24,0,0.44-0.08,0.62-0.23c0.19-0.19,0.41-0.28,0.68-0.28s0.49,0.09,0.68,0.28s0.29,0.42,0.29,0.68\n\tc0,0.27-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.68,0.29H2.36c-0.24,0-0.44,0.09-0.62,0.26C1.57,17.66,1.48,17.86,1.48,18.09z\n\t M7.27,11.55c0-0.24,0.09-0.44,0.26-0.62c0.18-0.16,0.38-0.24,0.6-0.24h2.03c0.23,0,0.42,0.08,0.58,0.25\n\tc0.16,0.17,0.23,0.37,0.23,0.61c0,0.24-0.08,0.44-0.23,0.6c-0.16,0.17-0.35,0.25-0.58,0.25H8.13c-0.24,0-0.44-0.08-0.61-0.25\n\tC7.35,11.98,7.27,11.78,7.27,11.55z M10.12,4.63c0-0.23,0.08-0.43,0.23-0.61c0.19-0.16,0.41-0.24,0.64-0.24\n\tc0.22,0,0.42,0.08,0.59,0.24l1.43,1.47c0.16,0.15,0.24,0.35,0.24,0.59c0,0.24-0.08,0.44-0.24,0.6s-0.36,0.24-0.59,0.24\n\tc-0.24,0-0.44-0.08-0.59-0.24l-1.47-1.43C10.2,5.08,10.12,4.88,10.12,4.63z M12.68,11.43v-0.07c0.02-0.91,0.27-1.75,0.74-2.53\n\tc0.48-0.77,1.11-1.38,1.9-1.83c0.79-0.45,1.65-0.67,2.57-0.67c0.7,0,1.37,0.14,2.02,0.42c0.64,0.28,1.2,0.65,1.66,1.12\n\tc0.47,0.47,0.84,1.02,1.11,1.66s0.41,1.32,0.41,2.02c0,0.94-0.23,1.8-0.69,2.6s-1.09,1.43-1.88,1.89c-0.79,0.47-1.66,0.7-2.6,0.71\n\th-0.21c-0.07,0-0.13-0.02-0.17-0.07c-0.05-0.05-0.07-0.11-0.07-0.18v-1.22c0-0.13,0.07-0.2,0.22-0.2h0.24\n\tc0.96-0.01,1.79-0.35,2.47-1.05c0.68-0.69,1.03-1.52,1.03-2.49c0-0.96-0.35-1.78-1.04-2.47c-0.69-0.68-1.52-1.02-2.5-1.02\n\tc-0.94,0-1.76,0.32-2.44,0.98c-0.68,0.65-1.04,1.44-1.08,2.37c0,0.06-0.03,0.11-0.08,0.17s-0.14,0.09-0.26,0.09H12.9\n\tC12.75,11.67,12.68,11.59,12.68,11.43z M17.03,21.31v-1.99c0-0.24,0.08-0.44,0.25-0.6s0.37-0.24,0.6-0.24\n\tc0.25,0,0.45,0.08,0.61,0.24c0.16,0.16,0.24,0.36,0.24,0.6v1.99c0,0.24-0.08,0.45-0.25,0.62c-0.17,0.17-0.37,0.25-0.6,0.25\n\tc-0.24,0-0.44-0.08-0.6-0.25C17.12,21.76,17.03,21.55,17.03,21.31z M17.03,3.83V1.78c0-0.23,0.08-0.43,0.25-0.6s0.37-0.25,0.6-0.25\n\tc0.24,0,0.44,0.08,0.6,0.25s0.25,0.37,0.25,0.6v2.04c0,0.23-0.08,0.42-0.25,0.58c-0.17,0.15-0.37,0.23-0.6,0.23\n\tc-0.24,0-0.44-0.08-0.6-0.23C17.12,4.25,17.03,4.06,17.03,3.83z M22.56,17.01c0-0.23,0.08-0.42,0.23-0.56\n\tc0.15-0.16,0.34-0.23,0.57-0.23c0.24,0,0.44,0.08,0.6,0.23l1.45,1.42c0.16,0.17,0.24,0.38,0.24,0.61c0,0.23-0.08,0.43-0.24,0.59\n\tc-0.4,0.31-0.8,0.31-1.2,0l-1.42-1.43C22.63,17.48,22.56,17.26,22.56,17.01z M22.56,6.08c0-0.25,0.08-0.45,0.23-0.59l1.42-1.47\n\tc0.18-0.16,0.37-0.24,0.59-0.24c0.23,0,0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6c0,0.25-0.08,0.46-0.24,0.62l-1.45,1.43\n\tc-0.18,0.16-0.38,0.24-0.6,0.24c-0.23,0-0.41-0.08-0.57-0.24C22.63,6.52,22.56,6.32,22.56,6.08z M24.82,11.55\n\tc0-0.24,0.08-0.44,0.24-0.62c0.16-0.16,0.35-0.24,0.57-0.24h2.02c0.23,0,0.43,0.09,0.61,0.26s0.26,0.37,0.26,0.6\n\tc0,0.23-0.09,0.43-0.26,0.6c-0.18,0.17-0.38,0.25-0.61,0.25h-2.02c-0.23,0-0.42-0.08-0.58-0.25C24.89,11.99,24.82,11.79,24.82,11.55\n\tz\" />",
};
#[cfg(feature = "WiDegrees")]
const WI_DEGREES: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M13.19,9.21c0-0.5,0.18-0.93,0.53-1.28c0.36-0.36,0.78-0.53,1.28-0.53c0.49,0,0.92,0.18,1.27,0.53\n\tc0.35,0.36,0.53,0.78,0.53,1.28s-0.18,0.93-0.53,1.29c-0.35,0.36-0.78,0.54-1.27,0.54c-0.49,0-0.92-0.18-1.28-0.54\n\tS13.19,9.71,13.19,9.21z M14.07,9.21c0,0.26,0.09,0.48,0.27,0.67c0.19,0.19,0.41,0.28,0.67,0.28c0.26,0,0.48-0.09,0.67-0.28\n\ts0.28-0.41,0.28-0.67c0-0.26-0.09-0.48-0.28-0.66c-0.19-0.18-0.41-0.28-0.67-0.28c-0.26,0-0.48,0.09-0.67,0.27\n\tC14.16,8.72,14.07,8.94,14.07,9.21z\" />",
};
#[cfg(feature = "WiDirectionDown")]
const WI_DIRECTION_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M11.77,16.47c0,0.22,0.08,0.4,0.25,0.55l2.4,2.45c0.16,0.16,0.35,0.23,0.58,0.23c0.24,0,0.43-0.08,0.59-0.23l2.4-2.45\n\tc0.16-0.14,0.24-0.33,0.24-0.55c0-0.22-0.08-0.41-0.23-0.57s-0.34-0.23-0.56-0.23s-0.42,0.08-0.57,0.23l-1.06,1.05v-6.59\n\tc0-0.22-0.08-0.41-0.24-0.56C15.42,9.66,15.23,9.58,15,9.58s-0.42,0.07-0.58,0.22c-0.16,0.15-0.24,0.34-0.24,0.56v6.59l-1.06-1.05\n\tc-0.16-0.16-0.34-0.23-0.55-0.23c-0.22,0-0.42,0.08-0.57,0.23S11.77,16.25,11.77,16.47z\" />",
};
#[cfg(feature = "WiDirectionDownLeft")]
const WI_DIRECTION_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M11.83,16.77c0,0.19,0.06,0.35,0.19,0.48c0.13,0.13,0.29,0.19,0.47,0.19h2.87c0.19,0,0.35-0.06,0.47-0.19\n\tc0.13-0.13,0.19-0.29,0.19-0.48c0-0.19-0.06-0.34-0.19-0.46c-0.13-0.12-0.29-0.18-0.47-0.18h-1.24L18,12.24\n\tc0.12-0.14,0.18-0.3,0.18-0.5c0-0.18-0.06-0.33-0.18-0.46c-0.12-0.12-0.29-0.18-0.5-0.18c-0.2,0-0.36,0.06-0.48,0.18l-3.86,3.87\n\tv-1.25c0-0.19-0.06-0.35-0.19-0.48c-0.13-0.13-0.29-0.19-0.48-0.19c-0.19,0-0.35,0.07-0.47,0.2c-0.13,0.13-0.19,0.29-0.19,0.48\n\tV16.77z\" />",
};
#[cfg(feature = "WiDirectionDownRight")]
const WI_DIRECTION_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M10.04,10.08c0-0.3,0.09-0.55,0.26-0.73c0.2-0.19,0.46-0.28,0.79-0.28c0.3,0,0.55,0.09,0.73,0.28l6.05,6.05v-1.95\n\tc0-0.3,0.1-0.55,0.3-0.75s0.45-0.3,0.75-0.3c0.29,0,0.54,0.1,0.74,0.31s0.3,0.45,0.3,0.75v4.48c0,0.3-0.1,0.55-0.3,0.75\n\ts-0.45,0.3-0.74,0.3h-4.48c-0.29,0-0.54-0.1-0.74-0.3s-0.3-0.45-0.3-0.75c0-0.29,0.1-0.54,0.3-0.73s0.45-0.29,0.74-0.29h1.93\n\tl-6.08-6.06C10.13,10.63,10.04,10.38,10.04,10.08z\" />",
};
#[cfg(feature = "WiDirectionLeft")]
const WI_DIRECTION_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.09,14.96c0,0.37,0.12,0.68,0.37,0.92l3.84,3.75c0.22,0.25,0.51,0.38,0.85,0.38c0.35,0,0.65-0.12,0.89-0.35\n\ts0.37-0.53,0.37-0.88s-0.12-0.65-0.37-0.89l-1.64-1.64h10.3c0.35,0,0.64-0.12,0.87-0.37s0.34-0.55,0.34-0.9s-0.11-0.65-0.34-0.9\n\ts-0.52-0.38-0.87-0.39H11.4l1.64-1.66c0.24-0.24,0.37-0.53,0.37-0.86c0-0.35-0.12-0.65-0.37-0.89S12.5,9.9,12.14,9.9\n\tc-0.32,0-0.61,0.14-0.85,0.41l-3.84,3.75C7.21,14.31,7.09,14.6,7.09,14.96z\" />",
};
#[cfg(feature = "WiDirectionRight")]
const WI_DIRECTION_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.94,14.36c0,0.22,0.08,0.42,0.23,0.57s0.34,0.22,0.56,0.2h6.58l-1.03,1.08c-0.16,0.16-0.24,0.35-0.24,0.55\n\tc0,0.22,0.08,0.42,0.24,0.57c0.16,0.16,0.35,0.23,0.58,0.23c0.21-0.01,0.39-0.1,0.53-0.27l2.45-2.41c0.16-0.16,0.23-0.35,0.23-0.58\n\tc-0.01-0.24-0.09-0.43-0.24-0.58l-2.47-2.39c-0.15-0.16-0.33-0.24-0.54-0.23c-0.23,0-0.42,0.07-0.57,0.22\n\tc-0.16,0.15-0.23,0.34-0.23,0.56c0,0.23,0.08,0.42,0.23,0.57l1.06,1.08h-6.59c-0.23,0.01-0.41,0.09-0.56,0.25\n\tC10.01,13.95,9.94,14.14,9.94,14.36z\" />",
};
#[cfg(feature = "WiDirectionUp")]
const WI_DIRECTION_UP: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.95,10.87c-0.01,0.35,0.1,0.65,0.34,0.9s0.53,0.37,0.89,0.36c0.34,0.02,0.63-0.1,0.88-0.37l1.66-1.64v10.3\n\tc-0.01,0.35,0.11,0.64,0.36,0.88s0.55,0.35,0.92,0.34c0.34,0.02,0.64-0.09,0.89-0.32s0.39-0.53,0.4-0.88v-10.3l1.64,1.64\n\tc0.23,0.24,0.53,0.37,0.88,0.37c0.36,0,0.66-0.12,0.9-0.36s0.36-0.53,0.36-0.89c-0.02-0.36-0.15-0.64-0.4-0.85l-3.74-3.84\n\tc-0.24-0.23-0.55-0.37-0.92-0.4c-0.37,0.02-0.68,0.16-0.92,0.41l-3.75,3.81C10.08,10.25,9.95,10.53,9.95,10.87z\" />",
};
#[cfg(feature = "WiDirectionUpLeft")]
const WI_DIRECTION_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M10.03,14.31V9.84c0-0.3,0.1-0.55,0.3-0.75s0.45-0.3,0.74-0.3h4.48c0.29,0,0.54,0.1,0.74,0.3s0.3,0.45,0.3,0.75\n\tc0,0.29-0.1,0.53-0.3,0.73s-0.45,0.29-0.74,0.29h-1.95l6.06,6.06c0.18,0.21,0.26,0.46,0.26,0.78c0,0.29-0.09,0.53-0.26,0.72\n\tc-0.2,0.19-0.46,0.28-0.79,0.28c-0.3,0-0.55-0.09-0.73-0.28l-6.02-6.05v1.95c0,0.3-0.1,0.55-0.3,0.75c-0.2,0.2-0.45,0.3-0.75,0.3\n\tc-0.29,0-0.54-0.1-0.74-0.31S10.03,14.6,10.03,14.31z\" />",
};
#[cfg(feature = "WiDirectionUpRight")]
const WI_DIRECTION_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M10.05,17.55c0,0.3,0.09,0.55,0.26,0.73c0.2,0.19,0.46,0.28,0.79,0.28c0.3,0,0.55-0.09,0.73-0.28l6.04-6.05v1.95\n\tc0,0.3,0.1,0.55,0.3,0.75c0.2,0.2,0.45,0.3,0.75,0.3c0.29,0,0.54-0.1,0.74-0.31s0.3-0.45,0.3-0.75V9.7c0-0.3-0.1-0.55-0.3-0.75\n\ts-0.45-0.3-0.74-0.3h-4.5c-0.29,0-0.54,0.1-0.73,0.3S13.4,9.39,13.4,9.7c0,0.29,0.1,0.54,0.29,0.73s0.44,0.29,0.73,0.29h1.95\n\tl-6.06,6.06C10.14,16.99,10.05,17.25,10.05,17.55z\" />",
};
#[cfg(feature = "WiDust")]
const WI_DUST: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.33,16.58c0-0.23,0.08-0.41,0.23-0.56c0.16-0.15,0.37-0.22,0.64-0.22h5.71c0.27,0,0.48,0.07,0.64,0.22\n\tc0.16,0.15,0.23,0.33,0.23,0.56c0,0.27-0.08,0.49-0.23,0.64s-0.37,0.23-0.64,0.23H8.2c-0.27,0-0.48-0.08-0.64-0.23\n\tS7.33,16.86,7.33,16.58z M7.33,10.67c0-0.22,0.08-0.41,0.23-0.55C7.72,9.97,7.93,9.9,8.2,9.9h2.96c0.27,0,0.48,0.07,0.64,0.22\n\tc0.16,0.14,0.24,0.33,0.24,0.55c0,0.27-0.08,0.48-0.24,0.64c-0.16,0.16-0.37,0.24-0.64,0.24H8.2c-0.27,0-0.48-0.08-0.64-0.23\n\tS7.33,10.94,7.33,10.67z M8.32,19.54c0-0.22,0.09-0.42,0.28-0.6c0.18-0.18,0.39-0.28,0.6-0.28c0.26,0,0.46,0.09,0.62,0.27\n\ts0.24,0.38,0.24,0.61c0,0.27-0.08,0.49-0.23,0.65c-0.15,0.16-0.36,0.23-0.63,0.23c-0.23,0-0.44-0.08-0.61-0.24\n\tC8.4,20.01,8.32,19.8,8.32,19.54z M9.74,13.61c0-0.23,0.07-0.44,0.22-0.61c0.15-0.18,0.33-0.27,0.54-0.27\n\tc0.26,0,0.48,0.09,0.64,0.27c0.16,0.18,0.24,0.38,0.24,0.61c0,0.27-0.08,0.49-0.23,0.65c-0.16,0.16-0.37,0.23-0.65,0.23\n\tc-0.23,0-0.41-0.08-0.55-0.24S9.74,13.88,9.74,13.61z M10.73,19.54c0-0.23,0.08-0.44,0.24-0.61s0.38-0.27,0.64-0.27h3.83l0.88,0.88\n\tc0,0.26-0.09,0.47-0.27,0.64s-0.38,0.24-0.61,0.24h-3.83c-0.27,0-0.49-0.08-0.65-0.24S10.73,19.81,10.73,19.54z M12.05,13.61\n\tc0-0.22,0.09-0.42,0.28-0.6c0.18-0.18,0.39-0.28,0.6-0.28h3.83c0.26,0,0.47,0.09,0.63,0.27c0.16,0.18,0.24,0.38,0.24,0.61\n\tc0,0.27-0.08,0.49-0.23,0.65c-0.16,0.16-0.37,0.23-0.64,0.23h-3.83c-0.23,0-0.44-0.08-0.61-0.24\n\tC12.14,14.09,12.05,13.88,12.05,13.61z M12.81,10.67c0-0.22,0.08-0.41,0.24-0.55c0.16-0.14,0.37-0.22,0.64-0.22h5.71\n\tc0.23,0,0.43,0.08,0.61,0.23c0.18,0.15,0.27,0.33,0.27,0.54c0,0.26-0.09,0.48-0.27,0.64c-0.18,0.16-0.38,0.24-0.61,0.24h-5.71\n\tc-0.27,0-0.49-0.08-0.65-0.24C12.88,11.15,12.81,10.94,12.81,10.67z M15.44,16.58c0-0.21,0.09-0.4,0.27-0.55\n\tc0.18-0.15,0.38-0.23,0.61-0.23s0.43,0.08,0.61,0.23c0.18,0.15,0.27,0.34,0.27,0.55c0,0.26-0.09,0.47-0.27,0.63\n\tc-0.18,0.16-0.38,0.24-0.61,0.24c-0.23,0-0.44-0.08-0.61-0.24C15.53,17.06,15.44,16.85,15.44,16.58z M16.98,19.54\n\tc0-0.23,0.08-0.44,0.24-0.61c0.16-0.18,0.37-0.27,0.63-0.27h1.87c0.26,0,0.47,0.09,0.63,0.26c0.16,0.17,0.24,0.38,0.24,0.62\n\tc0,0.27-0.08,0.49-0.23,0.65c-0.15,0.16-0.37,0.23-0.64,0.23h-1.87c-0.27,0-0.48-0.08-0.64-0.23\n\tC17.06,20.03,16.98,19.81,16.98,19.54z M17.85,16.58c0-0.21,0.09-0.4,0.27-0.55c0.18-0.15,0.38-0.23,0.61-0.23h3.07\n\tc0.22,0,0.4,0.08,0.54,0.23c0.14,0.15,0.22,0.33,0.22,0.55c0,0.27-0.07,0.48-0.21,0.64c-0.14,0.16-0.32,0.23-0.55,0.23h-3.07\n\tc-0.23,0-0.44-0.08-0.61-0.24C17.94,17.06,17.85,16.85,17.85,16.58z M18.29,13.61c0-0.22,0.09-0.42,0.28-0.6\n\tc0.18-0.18,0.39-0.28,0.6-0.28h1.96c0.21,0,0.39,0.09,0.54,0.27c0.15,0.18,0.23,0.38,0.23,0.61c0,0.27-0.07,0.48-0.22,0.64\n\tc-0.14,0.16-0.33,0.24-0.55,0.24h-1.96c-0.23,0-0.44-0.08-0.61-0.24C18.38,14.09,18.29,13.88,18.29,13.61z M21.03,10.67\n\tc0-0.22,0.07-0.4,0.22-0.55C21.4,9.97,21.58,9.9,21.8,9.9c0.27,0,0.48,0.07,0.64,0.22c0.16,0.14,0.24,0.33,0.24,0.55\n\tc0,0.27-0.08,0.48-0.24,0.64c-0.16,0.16-0.37,0.24-0.64,0.24c-0.23,0-0.41-0.08-0.55-0.24C21.1,11.15,21.03,10.94,21.03,10.67z\" />",
};
#[cfg(feature = "WiEarthquake")]
const WI_EARTHQUAKE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.25,15.3c0,0.16,0.06,0.29,0.17,0.4c0.11,0.11,0.25,0.16,0.4,0.16H8.8c0.14,0,0.27-0.04,0.38-0.13\n\tc0.11-0.09,0.17-0.2,0.2-0.34l0.9-5.27l1.62,13.18c0.02,0.14,0.09,0.26,0.19,0.36c0.1,0.09,0.22,0.14,0.36,0.14\n\tc0.15,0,0.28-0.05,0.38-0.14s0.17-0.21,0.2-0.36l1.25-9.67l1.04,2.8c0.04,0.11,0.1,0.2,0.2,0.27s0.2,0.1,0.32,0.1h0.05\n\tc0.12-0.01,0.23-0.05,0.32-0.13c0.1-0.08,0.16-0.18,0.19-0.31l1.53-6.86l0.71,13.18c0.01,0.14,0.06,0.27,0.15,0.37\n\tc0.09,0.1,0.21,0.16,0.36,0.17c0.14,0.01,0.27-0.02,0.38-0.1c0.11-0.08,0.18-0.19,0.22-0.33l1.65-6.94h2.77\n\tc0.16,0,0.29-0.05,0.4-0.16c0.11-0.11,0.17-0.24,0.17-0.4c0-0.16-0.06-0.29-0.17-0.4c-0.11-0.11-0.25-0.17-0.4-0.17h-3.23\n\tc-0.13,0-0.25,0.04-0.35,0.12s-0.17,0.18-0.2,0.31l-0.83,3.54L18.84,5.33c-0.01-0.14-0.06-0.27-0.16-0.37\n\tc-0.1-0.1-0.22-0.16-0.36-0.16c-0.14-0.01-0.27,0.02-0.39,0.11s-0.19,0.2-0.22,0.34l-2,8.97l-1.16-3.16\n\tc-0.04-0.12-0.12-0.21-0.24-0.28s-0.24-0.1-0.36-0.08c-0.13,0.01-0.24,0.07-0.33,0.16c-0.09,0.09-0.15,0.21-0.17,0.34l-0.98,7.51\n\tL10.94,6.15c-0.03-0.14-0.09-0.26-0.19-0.35c-0.1-0.09-0.22-0.14-0.36-0.15c-0.14-0.01-0.27,0.03-0.38,0.12\n\tc-0.11,0.09-0.18,0.2-0.2,0.35l-1.48,8.61H5.82c-0.16,0-0.29,0.06-0.4,0.17C5.31,15.01,5.25,15.14,5.25,15.3z\" />",
};
#[cfg(feature = "WiFahrenheit")]
const WI_FAHRENHEIT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.67,11.01c0-0.5,0.18-0.93,0.53-1.28s0.78-0.53,1.28-0.53c0.49,0,0.92,0.18,1.27,0.53c0.35,0.36,0.53,0.78,0.53,1.28\n\tc0,0.5-0.18,0.93-0.53,1.29c-0.35,0.36-0.78,0.54-1.27,0.54s-0.92-0.18-1.28-0.54C9.85,11.94,9.67,11.51,9.67,11.01z M10.55,11.01\n\tc0,0.26,0.09,0.48,0.27,0.67c0.19,0.19,0.41,0.28,0.67,0.28s0.48-0.09,0.67-0.28s0.28-0.41,0.28-0.67c0-0.26-0.09-0.48-0.28-0.66\n\tc-0.19-0.18-0.41-0.28-0.67-0.28c-0.26,0-0.48,0.09-0.67,0.27C10.64,10.52,10.55,10.74,10.55,11.01z M14.96,17.9\n\tc0,0.14,0.05,0.27,0.15,0.37s0.23,0.15,0.37,0.15c0.14,0,0.27-0.05,0.37-0.15c0.1-0.1,0.15-0.23,0.15-0.37v-3.79h2.86\n\tc0.14,0,0.27-0.05,0.37-0.16s0.15-0.23,0.15-0.38c0-0.15-0.05-0.27-0.15-0.38c-0.1-0.1-0.23-0.15-0.38-0.15h-2.86v-2.73h3.82\n\tc0.14,0,0.26-0.05,0.36-0.15s0.14-0.23,0.14-0.38s-0.05-0.27-0.14-0.38s-0.21-0.15-0.36-0.15h-4.77c-0.07,0-0.1,0.04-0.1,0.11V17.9z\n\t\" />",
};
#[cfg(feature = "WiFire")]
const WI_FIRE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.38,21.83c0-0.3,0.1-0.55,0.29-0.76c0.19-0.21,0.43-0.31,0.7-0.31v-0.02l13.16,0.04c0.06-0.01,0.1-0.02,0.1-0.02\n\tc0.27,0.01,0.51,0.12,0.7,0.33c0.19,0.21,0.29,0.47,0.28,0.77c0,0.3-0.1,0.55-0.29,0.76c-0.19,0.21-0.43,0.31-0.7,0.31v0.01\n\tL8.59,22.9c-0.1,0.01-0.17,0.02-0.22,0.02c-0.28-0.01-0.51-0.11-0.7-0.32C7.47,22.39,7.37,22.13,7.38,21.83z M8.2,16.37\n\tc-0.01-0.43,0.04-0.93,0.16-1.52c0.06-0.3,0.2-0.76,0.44-1.37c0.02-0.05,0.07-0.14,0.13-0.28c0.01,0.02,0.03,0.03,0.04,0.05\n\ts0.02,0.02,0.02,0.03c0.11,0.44,0.27,0.84,0.49,1.2c0.21,0.32,0.48,0.56,0.82,0.69c0.26,0.11,0.63,0.17,1.1,0.18\n\tc0.02,0,0.05,0,0.08,0c0.03,0,0.06,0,0.08,0c-0.33-0.33-0.59-0.67-0.79-1c-0.3-0.52-0.49-1.12-0.57-1.81\n\tc-0.06-0.54-0.03-1.19,0.09-1.96c0.02-0.15,0.12-0.49,0.29-1.01c0.15-0.47,0.36-0.9,0.64-1.28C11.54,7.8,12,7.3,12.61,6.78\n\tc0.37-0.31,0.89-0.67,1.56-1.07c0.07-0.04,0.18-0.11,0.35-0.19c0,0.02,0,0.04,0,0.05s0,0.03,0,0.04v0.02\n\tc-0.24,0.57-0.41,1.15-0.49,1.73c-0.06,0.53,0.02,1.02,0.24,1.48c0.17,0.36,0.48,0.75,0.92,1.15c0.09,0.09,0.29,0.29,0.6,0.58\n\tc0.3,0.29,0.54,0.52,0.7,0.68l0.25,0.25c0.26-0.38,0.41-0.83,0.44-1.35c0.04-0.55,0-1.15-0.14-1.8c0-0.01,0-0.04,0.01-0.11\n\tc0.02,0.02,0.13,0.1,0.3,0.24c0.56,0.5,0.98,0.95,1.28,1.34c0.48,0.62,0.83,1.21,1.06,1.74c0.19,0.46,0.31,0.92,0.38,1.4\n\tc0.06,0.42,0.08,0.77,0.07,1.05c-0.01,0.78-0.1,1.43-0.25,1.96c-0.07,0.21-0.13,0.38-0.19,0.52c0.25-0.07,0.47-0.16,0.65-0.26\n\tc0.25-0.16,0.45-0.37,0.6-0.66c0.16-0.29,0.29-0.62,0.38-0.98c0-0.01,0.01-0.03,0.03-0.05c0.01,0.02,0.02,0.05,0.05,0.09\n\tc0.02,0.04,0.04,0.07,0.05,0.1c0.13,0.31,0.22,0.63,0.27,0.97c0.08,0.38,0.1,0.75,0.08,1.13c-0.02,0.29-0.07,0.56-0.16,0.81\n\tc-0.08,0.24-0.16,0.43-0.22,0.58c-0.19,0.38-0.39,0.71-0.62,0.98c-0.06,0.07-0.11,0.13-0.14,0.16H9.67\n\tc-0.01-0.01-0.03-0.03-0.07-0.06s-0.06-0.05-0.08-0.07C9.26,18.98,8.98,18.6,8.7,18.1c-0.08-0.15-0.18-0.38-0.29-0.69\n\tC8.29,17.1,8.22,16.75,8.2,16.37z\" />",
};
#[cfg(feature = "WiFlood")]
const WI_FLOOD: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M6.72,20.76c0-0.05,0.01-0.12,0.02-0.21v-4.76c0.28,0.49,0.66,0.89,1.15,1.19c0.49,0.3,1.03,0.45,1.61,0.45\n\tc0.59,0,1.13-0.15,1.62-0.45c0.49-0.3,0.87-0.69,1.15-1.19c0.27,0.49,0.66,0.89,1.15,1.19c0.49,0.3,1.03,0.45,1.62,0.45\n\tc0.58,0,1.12-0.15,1.61-0.45c0.49-0.3,0.87-0.69,1.15-1.19c0.28,0.49,0.67,0.89,1.15,1.19c0.49,0.3,1.02,0.45,1.6,0.45\n\tc0.58,0,1.12-0.15,1.61-0.45c0.49-0.3,0.88-0.69,1.15-1.19v4.81c0,0.02,0,0.05,0,0.08c0,0.03,0,0.06,0,0.08c0,0.01,0,0.04,0,0.07\n\ts0,0.06,0,0.08c-0.03,0.23-0.13,0.43-0.3,0.6c-0.17,0.17-0.37,0.27-0.6,0.3c-0.02,0-0.05,0-0.08,0.01\n\tc-0.03,0.01-0.06,0.01-0.08,0.01c-0.01,0-0.04,0-0.07-0.01c-0.03-0.01-0.06-0.01-0.08-0.01H7.94c-0.02,0-0.04,0-0.08,0.01\n\tc-0.03,0.01-0.06,0.01-0.07,0.01c-0.02,0-0.05,0-0.08-0.01c-0.03-0.01-0.06-0.01-0.07-0.01c-0.22-0.03-0.42-0.12-0.58-0.28\n\tc-0.16-0.16-0.27-0.34-0.32-0.56C6.73,20.88,6.72,20.81,6.72,20.76z M12.23,9.67c0-0.16,0.06-0.3,0.17-0.42l2.21-2.22l0.03-0.02\n\tc0.01,0,0.01,0,0.01-0.01c0.01,0,0.01,0,0.01-0.01c0.01,0,0.01,0,0.01-0.01h0.01c0.01,0,0.01,0,0.01-0.01s0-0.01,0.01-0.02h0.02\n\tl0.01-0.01h0.01l0.01-0.01h0.01l0.01-0.01h0.01c0.01-0.01,0.01-0.01,0.02-0.01h0.01c0-0.01,0.01-0.01,0.02-0.01\n\tc0.01-0.01,0.01-0.01,0.02-0.01l0.04-0.02h0.01c0.01,0,0.01,0,0.01-0.01h0.07l0.01-0.01h0.12c0.01,0,0.01,0,0.02,0.01h0.06\n\tc0,0.01,0,0.01,0.01,0.01h0.02c0.01,0.01,0.02,0.02,0.03,0.02l0.02,0.01h0.02l0.01,0.01h0.01l0.01,0.01c0.01,0,0.01,0,0.01,0.01\n\th0.02c0.01,0,0.01,0,0.01,0c0,0,0,0,0.01,0c0.01,0.01,0.01,0.01,0.02,0.01c0,0.01,0,0.02,0.01,0.02l0.01,0.01h0.02l0.01,0.01\n\tL15.41,7l0.01,0.01l0.02,0.02l2.22,2.22c0.12,0.12,0.18,0.26,0.18,0.42c0,0.16-0.06,0.3-0.18,0.41c-0.11,0.12-0.25,0.18-0.41,0.18\n\tc-0.16,0-0.3-0.06-0.41-0.18l-1.23-1.22v6.9c0,0.16-0.06,0.29-0.17,0.4c-0.11,0.11-0.25,0.17-0.41,0.17c-0.16,0-0.3-0.06-0.42-0.17\n\tc-0.12-0.11-0.17-0.25-0.17-0.4v-6.9l-1.22,1.22c-0.12,0.12-0.26,0.18-0.42,0.18c-0.16,0-0.3-0.06-0.41-0.18\n\tC12.29,9.97,12.23,9.83,12.23,9.67z\" />",
};
#[cfg(feature = "WiFog")]
const WI_FOG: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.62,21.05c0-0.24,0.08-0.45,0.25-0.61c0.17-0.16,0.38-0.24,0.63-0.24h18.67c0.25,0,0.45,0.08,0.61,0.24\n\tc0.16,0.16,0.24,0.36,0.24,0.61c0,0.23-0.08,0.43-0.25,0.58c-0.17,0.16-0.37,0.23-0.6,0.23H3.5c-0.25,0-0.46-0.08-0.63-0.23\n\tC2.7,21.47,2.62,21.28,2.62,21.05z M5.24,17.91c0-0.24,0.09-0.44,0.26-0.6c0.15-0.15,0.35-0.23,0.59-0.23h18.67\n\tc0.23,0,0.42,0.08,0.58,0.24c0.16,0.16,0.23,0.35,0.23,0.59c0,0.24-0.08,0.44-0.23,0.6c-0.16,0.17-0.35,0.25-0.58,0.25H6.09\n\tc-0.24,0-0.44-0.08-0.6-0.25C5.32,18.34,5.24,18.14,5.24,17.91z M5.37,15.52c0,0.09,0.05,0.13,0.15,0.13h1.43\n\tc0.06,0,0.13-0.05,0.2-0.16c0.24-0.52,0.59-0.94,1.06-1.27c0.47-0.33,0.99-0.52,1.55-0.56l0.55-0.07c0.11,0,0.17-0.06,0.17-0.18\n\tl0.07-0.5c0.11-1.08,0.56-1.98,1.37-2.7c0.81-0.72,1.76-1.08,2.85-1.08c1.08,0,2.02,0.36,2.83,1.07c0.8,0.71,1.26,1.61,1.37,2.68\n\tl0.08,0.57c0,0.11,0.07,0.17,0.2,0.17h1.59c0.64,0,1.23,0.17,1.76,0.52s0.92,0.8,1.18,1.37c0.07,0.11,0.14,0.16,0.21,0.16h1.43\n\tc0.12,0,0.17-0.07,0.14-0.23c-0.29-1.02-0.88-1.86-1.74-2.51c-0.87-0.65-1.86-0.97-2.97-0.97h-0.32c-0.33-1.33-1.03-2.42-2.1-3.27\n\ts-2.28-1.27-3.65-1.27c-1.4,0-2.64,0.44-3.73,1.32s-1.78,2-2.09,3.36c-0.85,0.2-1.6,0.6-2.24,1.21c-0.64,0.61-1.09,1.33-1.34,2.18\n\tv-0.04C5.37,15.45,5.37,15.48,5.37,15.52z M6.98,24.11c0-0.24,0.09-0.43,0.26-0.59c0.15-0.15,0.35-0.23,0.6-0.23h18.68\n\tc0.24,0,0.44,0.08,0.6,0.23c0.17,0.16,0.25,0.35,0.25,0.58c0,0.24-0.08,0.44-0.25,0.61c-0.17,0.17-0.37,0.25-0.6,0.25H7.84\n\tc-0.23,0-0.43-0.09-0.6-0.26C7.07,24.55,6.98,24.34,6.98,24.11z\" />",
};
#[cfg(feature = "WiGaleWarning")]
const WI_GALE_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M10.67,24.6V7.45h1.03V24.6H10.67z M12.4,22.44v-7.41l8.65,3.69L12.4,22.44z M12.4,14.86V7.45l8.65,3.69L12.4,14.86z\" />",
};
#[cfg(feature = "WiHail")]
const WI_HAIL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,16.9c0,1.33,0.46,2.47,1.39,3.43c0.93,0.96,2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.34\n\tc0-0.11-0.06-0.17-0.17-0.17c-0.86-0.04-1.58-0.38-2.18-1.02c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.54,0.84-2.16\n\tc0.56-0.61,1.26-0.97,2.09-1.07l0.53-0.03c0.13,0,0.2-0.06,0.2-0.19l0.06-0.53c0.11-1.08,0.56-1.99,1.37-2.71\n\tc0.81-0.73,1.76-1.09,2.85-1.09c1.09,0,2.04,0.36,2.85,1.09c0.81,0.73,1.27,1.63,1.39,2.71l0.08,0.58c0,0.11,0.06,0.17,0.18,0.17\n\th1.61c0.89,0,1.66,0.32,2.31,0.96c0.65,0.64,0.98,1.39,0.98,2.27c0,0.87-0.3,1.62-0.9,2.26c-0.6,0.64-1.33,0.98-2.18,1.02\n\tc-0.13,0-0.2,0.06-0.2,0.17v1.34c0,0.11,0.07,0.17,0.2,0.17c0.87-0.02,1.67-0.26,2.4-0.71c0.73-0.45,1.31-1.05,1.73-1.8\n\tc0.42-0.75,0.63-1.57,0.63-2.44c0-0.89-0.22-1.72-0.67-2.47c-0.44-0.75-1.05-1.35-1.81-1.78S21.29,12,20.4,12h-0.32\n\tc-0.32-1.34-1.03-2.43-2.1-3.28s-2.3-1.28-3.68-1.28c-1.41,0-2.66,0.44-3.75,1.31c-1.09,0.87-1.79,1.99-2.1,3.35\n\tc-1.11,0.26-2.02,0.83-2.73,1.73S4.64,15.75,4.64,16.9z M10.09,24.1c0.09,0.21,0.25,0.37,0.46,0.46c0.2,0.1,0.41,0.11,0.62,0.02\n\tc0.22-0.09,0.36-0.24,0.45-0.45c0.1-0.22,0.11-0.43,0.02-0.64c-0.08-0.21-0.24-0.35-0.45-0.44c-0.2-0.11-0.4-0.12-0.61-0.03\n\tc-0.21,0.09-0.36,0.24-0.46,0.47C10.01,23.66,10.01,23.86,10.09,24.1z M10.72,21.28c0,0.16,0.05,0.31,0.15,0.45\n\tc0.1,0.15,0.26,0.25,0.46,0.32c0.19,0.11,0.4,0.12,0.62,0.01c0.22-0.1,0.37-0.3,0.44-0.6l0.9-3.38c0.06-0.25,0.04-0.47-0.08-0.67\n\tc-0.12-0.2-0.29-0.32-0.53-0.36c-0.08-0.02-0.16-0.03-0.24-0.03c-0.16,0-0.32,0.05-0.47,0.15c-0.15,0.1-0.26,0.25-0.32,0.44\n\tl-0.88,3.39C10.73,21.16,10.72,21.25,10.72,21.28z M12.58,26.87c0,0.12,0.02,0.22,0.06,0.29c0.09,0.22,0.24,0.37,0.45,0.45\n\tc0.09,0.05,0.2,0.08,0.33,0.08c0.06,0,0.16-0.02,0.3-0.06c0.22-0.08,0.38-0.23,0.47-0.45c0.1-0.22,0.1-0.44,0-0.66\n\tc-0.1-0.22-0.25-0.37-0.45-0.46c-0.2-0.09-0.4-0.09-0.62,0c-0.19,0.08-0.32,0.2-0.41,0.36C12.62,26.58,12.58,26.73,12.58,26.87z\n\t M13.31,24.26c0,0.37,0.21,0.61,0.63,0.73c0.11,0.03,0.19,0.04,0.24,0.04c0.15,0,0.28-0.03,0.38-0.08c0.21-0.08,0.35-0.27,0.42-0.57\n\tl1.67-6.29c0.06-0.24,0.04-0.45-0.06-0.65c-0.1-0.19-0.27-0.32-0.49-0.38c-0.08-0.02-0.17-0.03-0.27-0.03\n\tc-0.16,0-0.32,0.05-0.48,0.15c-0.16,0.1-0.26,0.25-0.3,0.44l-1.71,6.34C13.32,24.1,13.31,24.2,13.31,24.26z M16.74,23.8\n\tc0,0.12,0.02,0.23,0.08,0.32c0.08,0.19,0.23,0.34,0.44,0.44c0.11,0.04,0.23,0.07,0.35,0.07c0.06,0,0.16-0.02,0.3-0.06\n\tc0.21-0.08,0.37-0.23,0.46-0.44c0.07-0.22,0.07-0.43-0.01-0.63c-0.08-0.2-0.22-0.35-0.42-0.45c-0.23-0.11-0.44-0.12-0.65-0.03\n\tc-0.21,0.09-0.36,0.24-0.46,0.47C16.77,23.59,16.74,23.69,16.74,23.8z M17.47,21.23c0,0.14,0.05,0.29,0.16,0.45\n\tc0.11,0.16,0.26,0.27,0.45,0.33c0.16,0.03,0.25,0.05,0.27,0.05c0.09,0,0.22-0.03,0.37-0.1c0.2-0.09,0.33-0.27,0.4-0.52l0.9-3.34\n\tc0.02-0.17,0.03-0.26,0.03-0.26c0-0.16-0.05-0.31-0.15-0.46c-0.1-0.15-0.25-0.25-0.45-0.31c-0.09-0.02-0.18-0.03-0.26-0.03\n\tc-0.16,0-0.32,0.05-0.47,0.15s-0.25,0.25-0.31,0.45l-0.9,3.36L17.47,21.23z\" />",
};
#[cfg(feature = "WiHorizon")]
const WI_HORIZON: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.93,20.97c0-0.26,0.09-0.47,0.28-0.62c0.14-0.16,0.35-0.23,0.63-0.23h18.34c0.25,0,0.46,0.08,0.64,0.24\n\tc0.18,0.16,0.26,0.37,0.26,0.61c0,0.24-0.09,0.45-0.27,0.62s-0.39,0.27-0.63,0.27H5.84c-0.25,0-0.46-0.09-0.64-0.27\n\tC5.02,21.42,4.93,21.21,4.93,20.97z M6.9,12.68c0-0.26,0.08-0.47,0.23-0.63c0.17-0.18,0.38-0.26,0.65-0.26\n\tc0.23,0,0.43,0.09,0.6,0.26l1.5,1.5c0.18,0.18,0.27,0.39,0.27,0.63c0,0.23-0.09,0.44-0.27,0.62c-0.15,0.18-0.35,0.27-0.6,0.27\n\ts-0.47-0.09-0.64-0.27l-1.5-1.5C6.98,13.15,6.9,12.95,6.9,12.68z M9.83,18.27c-0.04,0.16,0.01,0.23,0.15,0.23h1.49\n\tc0.07,0,0.14-0.06,0.22-0.17c0.3-0.64,0.74-1.14,1.33-1.52s1.24-0.56,1.96-0.56c0.73,0,1.39,0.19,1.99,0.56s1.05,0.88,1.35,1.52\n\tc0.08,0.11,0.16,0.17,0.23,0.17h1.48c0.13,0,0.18-0.08,0.15-0.23c-0.34-1.13-0.99-2.05-1.95-2.76c-0.96-0.71-2.04-1.06-3.25-1.06\n\tc-1.2,0-2.28,0.35-3.23,1.06C10.82,16.22,10.17,17.14,9.83,18.27z M14.14,11.81V9.68c0-0.25,0.08-0.46,0.24-0.64\n\tc0.16-0.18,0.37-0.26,0.61-0.26c0.25,0,0.46,0.09,0.63,0.26c0.17,0.17,0.25,0.39,0.25,0.64v2.14c0,0.26-0.08,0.47-0.25,0.64\n\tc-0.17,0.17-0.38,0.25-0.63,0.25c-0.24,0-0.45-0.09-0.61-0.26S14.14,12.06,14.14,11.81z M19.86,14.18c0-0.24,0.08-0.45,0.25-0.63\n\tl1.54-1.5c0.16-0.18,0.36-0.26,0.62-0.26c0.24,0,0.44,0.08,0.6,0.25s0.23,0.38,0.23,0.64c0,0.26-0.08,0.47-0.23,0.62l-1.48,1.5\n\tc-0.17,0.17-0.36,0.26-0.56,0.28c-0.23,0.02-0.44-0.06-0.65-0.24S19.86,14.43,19.86,14.18z\" />",
};
#[cfg(feature = "WiHorizonAlt")]
const WI_HORIZON_ALT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.11,15.9c0-0.26,0.09-0.47,0.27-0.62c0.14-0.15,0.35-0.23,0.62-0.23h2.08c0.24,0,0.45,0.08,0.6,0.24\n\tc0.16,0.16,0.24,0.36,0.24,0.6c0,0.24-0.08,0.44-0.24,0.61c-0.16,0.17-0.36,0.25-0.6,0.25H5c-0.24,0-0.45-0.08-0.63-0.25\n\tC4.19,16.34,4.11,16.14,4.11,15.9z M5.03,20.98c0-0.23,0.09-0.43,0.26-0.61c0.16-0.16,0.37-0.23,0.61-0.23h18.21\n\tc0.24,0,0.45,0.08,0.62,0.24c0.17,0.16,0.25,0.36,0.25,0.6c0,0.24-0.09,0.44-0.26,0.61s-0.38,0.26-0.61,0.26H5.91\n\tc-0.24,0-0.44-0.09-0.61-0.26S5.03,21.22,5.03,20.98z M7.08,8.81c0-0.26,0.08-0.45,0.23-0.59c0.17-0.18,0.38-0.27,0.62-0.27\n\tc0.24,0,0.44,0.09,0.61,0.27l1.44,1.46c0.18,0.16,0.26,0.36,0.26,0.6c0,0.25-0.08,0.45-0.24,0.61s-0.36,0.24-0.6,0.24\n\tc-0.22,0-0.42-0.08-0.6-0.24L7.31,9.42C7.16,9.28,7.08,9.08,7.08,8.81z M9.68,15.9c0,0.87,0.18,1.65,0.53,2.33\n\tc0.03,0.09,0.11,0.14,0.24,0.14h1.67c0.07,0,0.12-0.02,0.14-0.06c0.02-0.04-0.01-0.1-0.07-0.16c-0.53-0.65-0.8-1.4-0.8-2.25\n\tc0-0.99,0.36-1.84,1.07-2.54c0.71-0.7,1.56-1.05,2.55-1.05c0.99,0,1.84,0.35,2.55,1.05s1.05,1.55,1.05,2.54\n\tc0,0.86-0.27,1.61-0.8,2.25c-0.04,0.06-0.06,0.1-0.06,0.12c-0.01,0.03,0,0.06,0.03,0.07c0.02,0.02,0.06,0.03,0.1,0.03h1.7\n\tc0.09,0,0.16-0.05,0.21-0.14c0.38-0.71,0.56-1.48,0.56-2.33c0-0.96-0.24-1.85-0.72-2.67c-0.48-0.82-1.13-1.47-1.95-1.95\n\ts-1.71-0.72-2.67-0.72s-1.85,0.24-2.67,0.72c-0.82,0.48-1.47,1.13-1.95,1.95C9.91,14.05,9.68,14.94,9.68,15.9z M14.15,7.97V5.88\n\tc0-0.24,0.08-0.44,0.25-0.62C14.57,5.08,14.77,5,15.01,5c0.24,0,0.44,0.09,0.62,0.26c0.17,0.17,0.26,0.38,0.26,0.62v2.09\n\tc0,0.24-0.09,0.44-0.26,0.62c-0.18,0.18-0.38,0.26-0.62,0.26c-0.24,0-0.44-0.09-0.61-0.26C14.23,8.41,14.15,8.21,14.15,7.97z\n\t M19.77,10.28c0-0.24,0.08-0.44,0.24-0.6l1.44-1.46c0.17-0.18,0.38-0.27,0.62-0.27c0.25,0,0.46,0.08,0.62,0.25\n\tc0.17,0.17,0.25,0.37,0.25,0.61c0,0.26-0.08,0.46-0.23,0.61l-1.51,1.47c-0.16,0.15-0.36,0.22-0.59,0.22\n\tc-0.25,0.01-0.45-0.07-0.61-0.22C19.85,10.74,19.77,10.53,19.77,10.28z M22.1,15.9c0-0.27,0.08-0.47,0.24-0.62\n\tc0.14-0.15,0.34-0.23,0.59-0.23h2.09c0.24,0,0.45,0.08,0.62,0.24c0.17,0.16,0.26,0.36,0.26,0.6c0,0.24-0.09,0.44-0.26,0.61\n\tc-0.18,0.17-0.38,0.25-0.62,0.25h-2.09c-0.23,0-0.43-0.08-0.59-0.25C22.18,16.34,22.1,16.14,22.1,15.9z\" />",
};
#[cfg(feature = "WiHot")]
const WI_HOT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.14,14.76c0-0.24,0.09-0.44,0.27-0.61c0.17-0.17,0.38-0.25,0.62-0.25h2.06c0.24,0,0.44,0.08,0.6,0.25\n\tc0.17,0.17,0.25,0.37,0.25,0.61c0,0.25-0.08,0.46-0.25,0.63c-0.17,0.17-0.37,0.25-0.6,0.25H5.03c-0.25,0-0.46-0.08-0.63-0.25\n\tC4.22,15.22,4.14,15.01,4.14,14.76z M7.06,7.74c0-0.23,0.08-0.44,0.24-0.62C7.5,6.96,7.71,6.87,7.96,6.87\n\tc0.21,0,0.41,0.08,0.59,0.25l1.45,1.48c0.17,0.16,0.25,0.36,0.25,0.6c0,0.24-0.08,0.44-0.25,0.6c-0.17,0.17-0.36,0.25-0.6,0.25\n\tc-0.26,0-0.46-0.08-0.61-0.25l-1.5-1.44C7.15,8.19,7.06,7.98,7.06,7.74z M9.67,14.76c0-0.71,0.14-1.39,0.43-2.04\n\tc0.28-0.65,0.67-1.22,1.14-1.69c0.48-0.47,1.05-0.85,1.7-1.13S14.28,9.47,15,9.47c0.96,0,1.84,0.24,2.66,0.72\n\tc0.82,0.48,1.47,1.12,1.94,1.94c0.47,0.81,0.71,1.69,0.71,2.63c0,0.15-0.01,0.29-0.03,0.42c-0.28-0.17-0.6-0.25-0.97-0.25\n\tc-0.24,0-0.48,0.05-0.72,0.15c0.01-0.07,0.01-0.18,0.01-0.32c0-0.98-0.35-1.81-1.06-2.5c-0.71-0.69-1.56-1.04-2.55-1.04\n\tc-0.99,0-1.83,0.35-2.54,1.04s-1.05,1.52-1.05,2.5v0.18c-0.29,0.02-0.57,0.1-0.84,0.25c-0.01,0.01-0.04,0.03-0.1,0.07\n\ts-0.12,0.09-0.19,0.14s-0.14,0.11-0.23,0.19s-0.18,0.16-0.26,0.24C9.7,15.51,9.67,15.15,9.67,14.76z M9.7,18.41v-0.15\n\tc-0.01-0.09,0-0.2,0.02-0.33c0.05-0.36,0.19-0.7,0.42-1.02c0.13-0.16,0.22-0.27,0.27-0.31c0.01-0.02,0.02-0.04,0.04-0.06l0.04-0.04\n\tc0.03-0.01,0.07-0.05,0.12-0.1c0.02-0.02,0.05-0.05,0.08-0.08s0.06-0.04,0.07-0.06c0.05-0.05,0.1-0.08,0.14-0.1l0.17-0.11\n\tc0.14-0.09,0.31-0.14,0.5-0.14h0.03c0.1,0,0.19,0.01,0.26,0.03c0.03,0.01,0.07,0.03,0.13,0.07v0.01c0.14,0.06,0.26,0.16,0.34,0.29\n\tc0.08,0.13,0.13,0.27,0.13,0.42c0,0.17-0.05,0.31-0.14,0.42c-0.06,0.09-0.14,0.17-0.22,0.23c-0.02,0.01-0.04,0.02-0.06,0.03\n\tc-0.02,0.01-0.04,0.02-0.04,0.02l-0.06,0.04c-0.04,0.03-0.07,0.06-0.1,0.08s-0.06,0.06-0.11,0.11c-0.04,0.05-0.08,0.09-0.11,0.14\n\tc-0.03,0.04-0.06,0.1-0.09,0.15c-0.03,0.06-0.05,0.12-0.05,0.17v0.15c0.04,0.15,0.08,0.27,0.11,0.36c0.07,0.14,0.18,0.28,0.34,0.44\n\tc0.01,0.02,0.09,0.1,0.24,0.25c0.86,0.78,1.27,1.62,1.21,2.5c-0.02,0.3-0.09,0.59-0.21,0.87c-0.12,0.28-0.26,0.51-0.43,0.7\n\tc-0.16,0.19-0.29,0.33-0.39,0.43c-0.1,0.09-0.18,0.16-0.25,0.21c-0.01,0.01-0.03,0.02-0.06,0.04c-0.03,0.02-0.06,0.04-0.07,0.04\n\tc-0.08,0.04-0.15,0.06-0.22,0.07c-0.09,0.01-0.15,0.02-0.2,0.02c-0.3,0-0.54-0.1-0.71-0.3c-0.14-0.17-0.2-0.37-0.18-0.59\n\tc0.02-0.22,0.13-0.4,0.33-0.53L11,22.75c0.01-0.01,0.03-0.03,0.05-0.04c0.02-0.02,0.04-0.04,0.07-0.06\n\tc0.03-0.02,0.06-0.05,0.08-0.08c0.03-0.03,0.06-0.07,0.08-0.1c0.03-0.04,0.06-0.08,0.08-0.12c0.03-0.04,0.06-0.09,0.08-0.14\n\tc0.03-0.05,0.05-0.1,0.07-0.15c0.02-0.05,0.03-0.1,0.05-0.16c0.01-0.06,0.02-0.12,0.02-0.17c0.02-0.2-0.03-0.4-0.15-0.6\n\tc-0.05-0.11-0.12-0.22-0.22-0.33c-0.07-0.08-0.12-0.13-0.15-0.16c-0.09-0.11-0.14-0.17-0.15-0.18c-0.02-0.01-0.04-0.03-0.07-0.06\n\ts-0.05-0.04-0.06-0.05c-0.15-0.14-0.26-0.26-0.34-0.36c-0.12-0.16-0.21-0.26-0.24-0.32c-0.19-0.26-0.32-0.52-0.39-0.78\n\tc-0.04-0.14-0.07-0.25-0.08-0.32c0-0.02-0.01-0.05-0.02-0.08S9.7,18.44,9.7,18.41z M13.58,18.08c0-0.06,0-0.1,0.01-0.14\n\tc0.02-0.22,0.09-0.43,0.2-0.64c0.11-0.21,0.22-0.39,0.35-0.53c0.13-0.14,0.25-0.27,0.38-0.38s0.23-0.19,0.31-0.25l0.12-0.07\n\tc0.15-0.09,0.32-0.14,0.5-0.14c0.11,0,0.21,0.01,0.3,0.03c0.01,0,0.02,0.01,0.05,0.02c0.03,0.02,0.05,0.03,0.06,0.04\n\tc0.01,0,0.02,0,0.03,0.01c0.01,0,0.03,0.02,0.07,0.05c0.2,0.12,0.32,0.3,0.38,0.54c0,0.02,0,0.04,0,0.07c0,0.02,0,0.04,0,0.05\n\tc0,0.03-0.01,0.07-0.02,0.12s-0.02,0.08-0.02,0.09c-0.07,0.23-0.21,0.39-0.42,0.5c-0.33,0.22-0.51,0.45-0.53,0.69\n\tc-0.01,0.08-0.01,0.15,0,0.22c0.02,0.12,0.08,0.25,0.17,0.39c0.11,0.16,0.19,0.27,0.24,0.32c0.16,0.16,0.25,0.25,0.28,0.27\n\tc0.12,0.11,0.28,0.28,0.47,0.51c0.54,0.65,0.79,1.32,0.74,2c-0.02,0.3-0.09,0.59-0.21,0.87c-0.12,0.28-0.26,0.51-0.43,0.7\n\tc-0.16,0.18-0.3,0.33-0.4,0.43c-0.11,0.1-0.19,0.17-0.25,0.21l-0.12,0.08c-0.11,0.04-0.17,0.06-0.21,0.07\n\tc-0.11,0.01-0.18,0.02-0.2,0.02h-0.03c-0.08,0-0.14-0.01-0.19-0.02c-0.02,0-0.05,0-0.08-0.01c-0.03-0.01-0.06-0.01-0.07-0.01\n\tc-0.01,0-0.02,0-0.03-0.01c-0.01-0.01-0.02-0.01-0.04-0.02c-0.01-0.01-0.02-0.01-0.03-0.01c-0.15-0.11-0.24-0.17-0.26-0.21\n\tc-0.16-0.18-0.22-0.38-0.19-0.6c0.03-0.22,0.14-0.39,0.34-0.53l0.03-0.04c0.02-0.02,0.05-0.05,0.09-0.08l0.12-0.12l0.13-0.16\n\tl0.12-0.19l0.09-0.22l0.04-0.24c0.01-0.4-0.22-0.82-0.69-1.27c-0.19-0.17-0.33-0.31-0.44-0.43C13.79,19.37,13.54,18.72,13.58,18.08z\n\t M14.12,6.92V4.85c0-0.24,0.09-0.45,0.26-0.62c0.17-0.17,0.38-0.25,0.61-0.25c0.24,0,0.45,0.08,0.62,0.25\n\tc0.17,0.17,0.25,0.38,0.25,0.62v2.07c0,0.24-0.08,0.43-0.25,0.59S15.24,7.74,15,7.74c-0.24,0-0.44-0.08-0.61-0.23\n\tC14.21,7.35,14.12,7.16,14.12,6.92z M17.48,17.93c0.02-0.22,0.09-0.43,0.2-0.64c0.11-0.21,0.22-0.39,0.35-0.53\n\tc0.13-0.14,0.25-0.27,0.38-0.38c0.12-0.11,0.22-0.19,0.3-0.25l0.13-0.07c0.02-0.02,0.06-0.04,0.1-0.08\n\tc0.11-0.04,0.24-0.07,0.38-0.07c0.34,0,0.59,0.13,0.77,0.38c0.05,0.07,0.08,0.14,0.1,0.23c0.01,0.02,0.02,0.05,0.02,0.08v0.11\n\tc0,0.31-0.15,0.55-0.45,0.7c-0.32,0.21-0.49,0.44-0.52,0.69c-0.04,0.34,0.19,0.74,0.68,1.2c0.88,0.77,1.28,1.61,1.23,2.5\n\tc-0.02,0.3-0.09,0.59-0.21,0.87c-0.12,0.28-0.27,0.51-0.43,0.7c-0.17,0.19-0.3,0.33-0.39,0.43s-0.18,0.16-0.25,0.21\n\tc-0.16,0.1-0.3,0.15-0.41,0.16c-0.03,0.01-0.08,0.01-0.15,0.01c-0.3,0-0.53-0.1-0.69-0.3c-0.15-0.17-0.21-0.37-0.19-0.59\n\ts0.13-0.4,0.32-0.53c0.03-0.01,0.09-0.06,0.18-0.14s0.18-0.21,0.29-0.38c0.1-0.18,0.16-0.35,0.17-0.53c0.02-0.4-0.22-0.82-0.7-1.27\n\tc-0.41-0.36-0.73-0.75-0.94-1.16C17.51,18.86,17.42,18.4,17.48,17.93z M19.77,9.21c0-0.25,0.08-0.45,0.23-0.6l1.46-1.48\n\tc0.18-0.17,0.38-0.25,0.61-0.25c0.24,0,0.44,0.09,0.61,0.26s0.26,0.38,0.26,0.61c0,0.25-0.09,0.46-0.26,0.62l-1.48,1.44\n\tc-0.18,0.17-0.38,0.25-0.61,0.25c-0.23,0-0.43-0.08-0.58-0.25C19.85,9.65,19.77,9.44,19.77,9.21z M22.07,14.76\n\tc0-0.22,0.09-0.42,0.26-0.61c0.16-0.17,0.35-0.25,0.58-0.25h2.06c0.24,0,0.45,0.09,0.62,0.26s0.27,0.37,0.27,0.6\n\tc0,0.24-0.09,0.45-0.26,0.62c-0.18,0.17-0.38,0.26-0.63,0.26h-2.06c-0.24,0-0.45-0.08-0.6-0.25C22.15,15.22,22.07,15.01,22.07,14.76\n\tz\" />",
};
#[cfg(feature = "WiHumidity")]
const WI_HUMIDITY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.56,17.19c0-0.88,0.24-1.89,0.72-3.03s1.1-2.25,1.86-3.31c1.56-2.06,2.92-3.62,4.06-4.67l0.75-0.72\n\tc0.25,0.26,0.53,0.5,0.83,0.72c0.41,0.42,1.04,1.11,1.88,2.09s1.57,1.85,2.17,2.65c0.71,1.01,1.32,2.1,1.81,3.25\n\ts0.74,2.16,0.74,3.03c0,1-0.19,1.95-0.58,2.86c-0.39,0.91-0.91,1.7-1.57,2.36c-0.66,0.66-1.45,1.19-2.37,1.58\n\tc-0.92,0.39-1.89,0.59-2.91,0.59c-1,0-1.95-0.19-2.86-0.57c-0.91-0.38-1.7-0.89-2.36-1.55c-0.66-0.65-1.19-1.44-1.58-2.35\n\tS7.56,18.23,7.56,17.19z M9.82,14.26c0,0.83,0.17,1.49,0.52,1.99c0.35,0.49,0.88,0.74,1.59,0.74c0.72,0,1.25-0.25,1.61-0.74\n\tc0.35-0.49,0.53-1.15,0.54-1.99c-0.01-0.84-0.19-1.5-0.54-2c-0.35-0.49-0.89-0.74-1.61-0.74c-0.71,0-1.24,0.25-1.59,0.74\n\tC9.99,12.76,9.82,13.42,9.82,14.26z M11.39,14.26c0-0.15,0-0.27,0-0.35s0.01-0.19,0.02-0.33c0.01-0.14,0.02-0.25,0.05-0.32\n\ts0.05-0.16,0.09-0.24c0.04-0.08,0.09-0.15,0.15-0.18c0.07-0.04,0.14-0.06,0.23-0.06c0.14,0,0.25,0.04,0.33,0.12s0.14,0.21,0.17,0.38\n\tc0.03,0.18,0.05,0.32,0.06,0.45s0.01,0.3,0.01,0.52c0,0.23,0,0.4-0.01,0.52c-0.01,0.12-0.03,0.27-0.06,0.45\n\tc-0.03,0.17-0.09,0.3-0.17,0.38s-0.19,0.12-0.33,0.12c-0.09,0-0.16-0.02-0.23-0.06c-0.07-0.04-0.12-0.1-0.15-0.18\n\tc-0.04-0.08-0.07-0.17-0.09-0.24c-0.02-0.08-0.04-0.19-0.05-0.32c-0.01-0.14-0.02-0.25-0.02-0.32S11.39,14.41,11.39,14.26z\n\t M11.98,22.01h1.32l4.99-10.74h-1.35L11.98,22.01z M16.28,19.02c0.01,0.84,0.2,1.5,0.55,2c0.35,0.49,0.89,0.74,1.6,0.74\n\tc0.72,0,1.25-0.25,1.6-0.74c0.35-0.49,0.52-1.16,0.53-2c-0.01-0.84-0.18-1.5-0.53-1.99c-0.35-0.49-0.88-0.74-1.6-0.74\n\tc-0.71,0-1.25,0.25-1.6,0.74C16.47,17.52,16.29,18.18,16.28,19.02z M17.85,19.02c0-0.23,0-0.4,0.01-0.52\n\tc0.01-0.12,0.03-0.27,0.06-0.45s0.09-0.3,0.17-0.38s0.19-0.12,0.33-0.12c0.09,0,0.17,0.02,0.24,0.06c0.07,0.04,0.12,0.1,0.16,0.19\n\tc0.04,0.09,0.07,0.17,0.1,0.24s0.04,0.18,0.05,0.32l0.01,0.32l0,0.34c0,0.16,0,0.28,0,0.35l-0.01,0.32l-0.05,0.32l-0.1,0.24\n\tl-0.16,0.19l-0.24,0.06c-0.14,0-0.25-0.04-0.33-0.12s-0.14-0.21-0.17-0.38c-0.03-0.18-0.05-0.33-0.06-0.45S17.85,19.25,17.85,19.02z\n\t\" />",
};
#[cfg(feature = "WiHurricane")]
const WI_HURRICANE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M11.08,14.53v-0.02c-0.01-0.08,0-0.2,0-0.37c0.01-0.16,0.04-0.43,0.1-0.81c0.06-0.38,0.14-0.76,0.25-1.15\n\ts0.28-0.84,0.51-1.35c0.23-0.51,0.5-0.99,0.82-1.44C13.08,8.94,13.5,8.47,14.02,8c0.52-0.47,1.1-0.89,1.73-1.24\n\tc0.16-0.09,0.32-0.11,0.49-0.06s0.3,0.15,0.38,0.31c0.09,0.16,0.11,0.32,0.06,0.5c-0.05,0.17-0.15,0.31-0.3,0.39\n\tc-1.31,0.72-2.32,1.73-3.03,3.05c0.54-0.25,1.08-0.38,1.63-0.38c1.07,0,2,0.38,2.77,1.15c0.77,0.77,1.15,1.69,1.15,2.76\n\tc0,0.08,0,0.16,0,0.24c0,0.08-0.02,0.25-0.04,0.52c-0.02,0.27-0.06,0.52-0.11,0.77c-0.05,0.25-0.13,0.56-0.23,0.93\n\tc-0.11,0.37-0.23,0.73-0.38,1.06c-0.15,0.33-0.34,0.7-0.58,1.1s-0.51,0.77-0.81,1.12c-0.3,0.35-0.66,0.7-1.08,1.05\n\tc-0.43,0.35-0.89,0.67-1.39,0.95c-0.09,0.06-0.2,0.08-0.31,0.08c-0.26,0-0.45-0.12-0.58-0.35c-0.09-0.16-0.11-0.32-0.06-0.49\n\tc0.05-0.17,0.15-0.3,0.31-0.38c1.34-0.75,2.36-1.78,3.06-3.08c-0.54,0.26-1.11,0.38-1.71,0.38c-0.69,0-1.34-0.17-1.94-0.52\n\tc-0.6-0.34-1.07-0.81-1.43-1.41C11.27,15.87,11.09,15.23,11.08,14.53z M12.78,14.48c0,0.61,0.22,1.13,0.65,1.57\n\tc0.43,0.43,0.95,0.65,1.56,0.65c0.57,0,1.06-0.19,1.49-0.57c0.42-0.38,0.66-0.85,0.73-1.41l0.01-0.23c0-0.02,0-0.04,0.01-0.05\n\tc-0.01-0.6-0.23-1.11-0.66-1.52c-0.43-0.42-0.96-0.62-1.57-0.62c-0.56,0-1.04,0.18-1.46,0.54s-0.66,0.82-0.73,1.36l-0.02,0.25V14.48\n\tz\" />",
};
#[cfg(feature = "WiHurricaneWarning")]
const WI_HURRICANE_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.8,24.6V7.45h1.13V24.6H7.8z M9.73,21.52v-6.6h8.55v6.6H9.73z M9.73,14.05v-6.6h8.55v6.6H9.73z M12.09,19.52h3.81v-2.51\n\th-3.81V19.52z M12.09,12.05h3.81v-2.5h-3.81V12.05z\" />",
};
#[cfg(feature = "WiLightning")]
const WI_LIGHTNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.96,24.51h0.39l6.88-10.18c0.09-0.18,0.04-0.27-0.15-0.27h-2.84l2.99-5.45c0.09-0.18,0.02-0.27-0.2-0.27h-3.81\n\tc-0.11,0-0.2,0.06-0.29,0.18l-2.78,7.4c-0.02,0.18,0.04,0.27,0.19,0.27h2.75L7.96,24.51z M16.46,18.18h0.27l5.22-7.67\n\tc0.05-0.08,0.06-0.15,0.04-0.2s-0.08-0.07-0.17-0.07h-2.1l2.18-4.03c0.12-0.2,0.06-0.3-0.18-0.3h-2.74c-0.13,0-0.23,0.06-0.3,0.19\n\tl-2.08,5.48c-0.03,0.09-0.03,0.16,0.01,0.21c0.04,0.05,0.1,0.07,0.19,0.07h2.04L16.46,18.18z\" />",
};
#[cfg(feature = "WiLunarEclipse")]
const WI_LUNAR_ECLIPSE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.8,14.62c0-0.93,0.23-1.8,0.7-2.6s1.1-1.44,1.9-1.91s1.67-0.7,2.6-0.7c0.94,0,1.81,0.23,2.61,0.7\n\tc0.8,0.47,1.43,1.1,1.9,1.9c0.47,0.8,0.7,1.67,0.7,2.61s-0.23,1.81-0.7,2.61c-0.47,0.8-1.1,1.43-1.9,1.9\n\tc-0.8,0.47-1.67,0.7-2.61,0.7s-1.8-0.23-2.6-0.7c-0.8-0.47-1.43-1.1-1.9-1.9C10.03,16.43,9.8,15.56,9.8,14.62z M14.25,11.22\n\tc0.87,0.11,1.6,0.49,2.19,1.15c0.59,0.66,0.89,1.44,0.89,2.33c0,0.83-0.26,1.56-0.78,2.2s-1.18,1.04-1.98,1.21\n\tc0.2,0.02,0.34,0.04,0.43,0.04c0.98,0,1.81-0.35,2.5-1.04c0.69-0.69,1.04-1.52,1.04-2.5c0-0.96-0.35-1.78-1.04-2.47\n\tc-0.69-0.68-1.53-1.02-2.5-1.02C14.74,11.14,14.49,11.17,14.25,11.22z\" />",
};
#[cfg(feature = "WiMeteor")]
const WI_METEOR: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.09,19.39c0-0.13,0-0.23,0.01-0.29v-0.08c0-0.02,0-0.04,0-0.06c0-0.02,0-0.03,0-0.05s0-0.03,0-0.05c0-0.02,0-0.03,0-0.04\n\tv-0.02c0-0.03,0.01-0.07,0.02-0.12c0.01-0.05,0.02-0.08,0.02-0.09v-0.03c0-0.01,0-0.03,0-0.05c0-0.02,0-0.03,0-0.04l0.08-0.37\n\tc0-0.01,0-0.01,0.01-0.02v-0.02l0.04-0.14c0.01-0.01,0.01-0.01,0.01-0.02c0.01-0.01,0.01-0.02,0.01-0.03v-0.03\n\tc0.04-0.12,0.07-0.22,0.1-0.28c0-0.01,0.01-0.02,0.02-0.03c0.01-0.01,0.02-0.06,0.05-0.15c0.17-0.38,0.38-0.74,0.63-1.08l0.06-0.07\n\tc0.01-0.01,0.02-0.02,0.03-0.04c0.01-0.02,0.02-0.03,0.03-0.04c0.01-0.01,0.03-0.03,0.07-0.06c0.01-0.02,0.02-0.04,0.04-0.06\n\tc0.02-0.02,0.03-0.04,0.04-0.06c0.04-0.02,0.06-0.05,0.07-0.07c0.01-0.01,0.03-0.02,0.07-0.06l0.07-0.07l7.6-8.33l-0.38,2.2\n\tl6.82-7.29l-4.18,8.14l4.18-3.16l-3.79,7.6l2.71-1.87l-4.68,8.33c0,0.01-0.01,0.02-0.02,0.04s-0.02,0.04-0.03,0.05\n\tc-0.01,0.01-0.01,0.02-0.02,0.04c-0.01,0.02-0.01,0.03-0.02,0.05c-0.01,0.01-0.01,0.02-0.02,0.05c-0.01,0.02-0.02,0.04-0.02,0.05\n\tc-0.43,0.84-1.05,1.51-1.86,2.02c-0.81,0.51-1.7,0.76-2.67,0.76c-0.92,0-1.77-0.23-2.55-0.68c-0.78-0.46-1.4-1.07-1.86-1.86\n\tS7.09,20.31,7.09,19.39z M8.29,19.39c0,1.08,0.38,1.99,1.14,2.75c0.76,0.76,1.68,1.14,2.75,1.14c0.82,0,1.56-0.24,2.22-0.71\n\tc0.66-0.47,1.13-1.09,1.41-1.84c0.17-0.43,0.25-0.87,0.25-1.34c0-1.07-0.38-1.99-1.13-2.75c-0.76-0.76-1.67-1.13-2.75-1.13\n\tc-1,0-1.87,0.33-2.6,1c-0.41,0.36-0.72,0.78-0.95,1.28C8.4,18.3,8.29,18.83,8.29,19.39z\" />",
};
#[cfg(feature = "WiMoonAltFirstQuarter")]
const WI_MOON_ALT_FIRST_QUARTER: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M14.8,24.51h0.19\n\tc1.37,0,2.67-0.27,3.91-0.8s2.31-1.25,3.21-2.15s1.61-1.97,2.15-3.21s0.8-2.54,0.8-3.91c0-1.36-0.27-2.66-0.8-3.9\n\ts-1.25-2.31-2.15-3.21s-1.97-1.61-3.21-2.15s-2.54-0.8-3.91-0.8H14.8V24.51z\" />",
};
#[cfg(feature = "WiMoonAltFull")]
const WI_MOON_ALT_FULL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.53,0.3-3,0.89-4.39s1.4-2.59,2.4-3.6s2.2-1.81,3.6-2.4s2.85-0.89,4.37-0.89c1.53,0,3,0.3,4.39,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.39,0.89c-1.52,0-2.98-0.3-4.37-0.89s-2.59-1.4-3.6-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.36,0.27,2.66,0.8,3.9s1.25,2.32,2.15,3.22s1.97,1.61,3.22,2.15s2.55,0.8,3.9,0.8c1.37,0,2.67-0.27,3.91-0.8\n\ts2.31-1.25,3.22-2.15s1.62-1.97,2.15-3.22s0.8-2.55,0.8-3.9c0-1.82-0.45-3.5-1.35-5.05s-2.13-2.77-3.68-3.68s-3.23-1.35-5.05-1.35\n\tc-1.36,0-2.66,0.27-3.9,0.8S8.79,6.41,7.89,7.32s-1.61,1.98-2.15,3.22S4.94,13.08,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltNew")]
const WI_MOON_ALT_NEW: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.1,4.1s3.61,1.51,5.65,1.51s3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1\n\ts1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09s-3.61-1.51-5.65-1.51S11.08,3.7,9.35,4.7s-3.1,2.37-4.1,4.09\n\tS3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonAltThirdQuarter")]
const WI_MOON_ALT_THIRD_QUARTER: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8h0.21V4.39h-0.21c-1.36,0-2.66,0.27-3.9,0.8\n\tS8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningCrescent1")]
const WI_MOON_ALT_WANING_CRESCENT1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8h0.07c-0.59-2.67-0.88-6.02-0.88-10.06\n\tc0-3.39,0.3-6.74,0.91-10.05h-0.1c-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningCrescent2")]
const WI_MOON_ALT_WANING_CRESCENT2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.36,0.26,2.65,0.79,3.89s1.24,2.3,2.12,3.2s1.95,1.62,3.17,2.15s2.52,0.81,3.87,0.82c-1.16-2.47-1.74-5.83-1.74-10.06\n\tc0-3.61,0.6-6.96,1.8-10.05c-1.36,0-2.65,0.27-3.89,0.81s-2.3,1.25-3.19,2.15s-1.61,1.97-2.14,3.2S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningCrescent3")]
const WI_MOON_ALT_WANING_CRESCENT3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.34,0.26,2.62,0.77,3.85s1.21,2.29,2.08,3.19s1.92,1.62,3.13,2.16s2.48,0.83,3.81,0.87c-1.71-2.32-2.56-5.68-2.56-10.06\n\tc0-1.87,0.23-3.67,0.69-5.41s1.11-3.29,1.95-4.64c-1.8,0.03-3.45,0.5-4.96,1.41s-2.7,2.13-3.58,3.65S4.94,12.65,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningCrescent4")]
const WI_MOON_ALT_WANING_CRESCENT4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.33,0.25,2.6,0.75,3.81s1.18,2.26,2.03,3.15s1.87,1.61,3.05,2.17s2.43,0.87,3.74,0.94c-1.24-1.19-2.11-2.63-2.61-4.31\n\ts-0.75-3.6-0.75-5.76c0-1.93,0.31-3.78,0.92-5.54s1.47-3.26,2.56-4.5c-1.77,0.07-3.39,0.56-4.88,1.47S7.09,8,6.23,9.51\n\tS4.94,12.68,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningCrescent5")]
const WI_MOON_ALT_WANING_CRESCENT5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.27,0.23,2.49,0.7,3.66s1.09,2.2,1.89,3.08s1.75,1.61,2.85,2.19s2.28,0.94,3.52,1.08c-1.75-1.04-2.98-2.39-3.7-4.07\n\ts-1.08-3.66-1.08-5.93c0-2.07,0.44-4,1.32-5.78s2.1-3.2,3.66-4.24c-1.26,0.11-2.46,0.45-3.59,1.02s-2.1,1.3-2.92,2.19\n\ts-1.46,1.92-1.93,3.11S4.94,13.15,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningCrescent6")]
const WI_MOON_ALT_WANING_CRESCENT6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,2.48,0.8,4.66,2.41,6.53s3.62,3.01,6.03,3.41c-1.01-0.5-1.86-1.1-2.56-1.82s-1.25-1.5-1.63-2.37s-0.66-1.77-0.83-2.7\n\ts-0.26-1.95-0.26-3.06c0-2.11,0.5-4.06,1.49-5.84s2.37-3.16,4.12-4.12c-1.63,0.21-3.11,0.77-4.45,1.7S6.87,8.3,6.1,9.76\n\tS4.94,12.77,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningGibbous1")]
const WI_MOON_ALT_WANING_GIBBOUS1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.43c0-2.04,0.51-3.92,1.52-5.65S7.64,5.7,9.37,4.69s3.61-1.5,5.65-1.5s3.92,0.5,5.65,1.5s3.09,2.36,4.09,4.09\n\ts1.5,3.61,1.5,5.65s-0.5,3.93-1.5,5.65s-2.36,3.1-4.09,4.11s-3.61,1.52-5.65,1.52s-3.93-0.51-5.65-1.52s-3.1-2.38-4.11-4.11\n\tS3.74,16.47,3.74,14.43z M4.94,14.43c0,1.36,0.27,2.66,0.81,3.9S7,20.65,7.9,21.55s1.97,1.62,3.22,2.15s2.55,0.81,3.9,0.81\n\tc0.86,0,1.62-0.09,2.29-0.28c0.83-0.44,1.55-0.96,2.17-1.57s1.1-1.22,1.46-1.85s0.64-1.33,0.86-2.09s0.36-1.48,0.43-2.14\n\ts0.1-1.37,0.1-2.15c0-0.93-0.1-1.84-0.3-2.74S21.52,9.9,21.1,9.02s-0.99-1.72-1.72-2.5s-1.57-1.45-2.54-1.99\n\tc-0.4-0.09-1.01-0.13-1.82-0.13c-1.36,0-2.66,0.26-3.9,0.79S8.8,6.43,7.9,7.32s-1.62,1.97-2.15,3.2S4.94,13.06,4.94,14.43z\" />",
};
#[cfg(feature = "WiMoonAltWaningGibbous2")]
const WI_MOON_ALT_WANING_GIBBOUS2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8c0.36,0,0.76-0.02,1.2-0.07\n\tc0.93-0.57,1.71-1.24,2.35-2.03s1.12-1.64,1.43-2.56s0.53-1.8,0.65-2.65s0.18-1.77,0.18-2.75c0-1.25-0.15-2.46-0.46-3.64\n\ts-0.84-2.34-1.59-3.49s-1.69-2.11-2.81-2.89c-0.41-0.02-0.73-0.03-0.95-0.03c-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34\n\ts-1.61,1.97-2.15,3.21S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningGibbous3")]
const WI_MOON_ALT_WANING_GIBBOUS3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8c0.33,0,0.58,0,0.73-0.01\n\tc0.78-0.61,1.44-1.31,1.96-2.11s0.92-1.66,1.18-2.57s0.44-1.79,0.54-2.63s0.15-1.75,0.15-2.74c0-1.91-0.32-3.76-0.97-5.54\n\ts-1.65-3.28-3.02-4.49c-0.13-0.01-0.32-0.01-0.59-0.01c-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21\n\tS4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningGibbous4")]
const WI_MOON_ALT_WANING_GIBBOUS4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8h0.38c0.6-0.64,1.1-1.37,1.5-2.19\n\ts0.71-1.67,0.9-2.58s0.33-1.77,0.41-2.59s0.12-1.73,0.12-2.7c0-1.88-0.24-3.7-0.73-5.46s-1.25-3.28-2.3-4.59h-0.28\n\tc-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningGibbous5")]
const WI_MOON_ALT_WANING_GIBBOUS5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8c0.26,0,0.46,0,0.59-0.01\n\tc0.77-1.33,1.3-2.83,1.57-4.5s0.42-3.52,0.42-5.55c0-4.01-0.68-7.36-2.04-10.03c-0.11-0.01-0.29-0.01-0.54-0.01\n\tc-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaningGibbous6")]
const WI_MOON_ALT_WANING_GIBBOUS6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8h0.38c0.67-2.49,1.01-5.84,1.01-10.06\n\tc0-3.82-0.34-7.17-1.03-10.05h-0.37c-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21S4.94,13.09,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaxingCrescent1")]
const WI_MOON_ALT_WAXING_CRESCENT1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M16.38,4.48\n\tc1.64,0.9,2.99,2.2,4.04,3.9s1.57,3.72,1.57,6.06c0,4.9-1.72,8.2-5.16,9.89c2.35-0.44,4.31-1.59,5.87-3.44s2.34-4,2.34-6.45\n\tc0-1.66-0.38-3.21-1.14-4.66s-1.8-2.63-3.13-3.57S18,4.71,16.38,4.48z\" />",
};
#[cfg(feature = "WiMoonAltWaxingCrescent2")]
const WI_MOON_ALT_WAXING_CRESCENT2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M15.9,4.42\n\tc1.48,0.99,2.7,2.34,3.65,4.05s1.42,3.7,1.42,5.97c0,4.8-1.6,8.13-4.79,9.99c1.65-0.2,3.15-0.76,4.5-1.68s2.42-2.12,3.2-3.58\n\ts1.17-3.03,1.17-4.72c0-1.72-0.41-3.32-1.22-4.8s-1.91-2.69-3.31-3.61S17.59,4.57,15.9,4.42z\" />",
};
#[cfg(feature = "WiMoonAltWaxingCrescent3")]
const WI_MOON_ALT_WAXING_CRESCENT3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M15.58,4.41\n\tc1.28,1.1,2.32,2.51,3.13,4.22s1.22,3.65,1.22,5.82c0,4.64-1.39,7.99-4.16,10.05c1.28-0.1,2.49-0.43,3.63-1s2.13-1.29,2.96-2.18\n\ts1.49-1.93,1.97-3.13s0.73-2.44,0.73-3.74c0-1.75-0.42-3.38-1.26-4.89s-1.99-2.72-3.44-3.64S17.31,4.5,15.58,4.41z\" />",
};
#[cfg(feature = "WiMoonAltWaxingCrescent4")]
const WI_MOON_ALT_WAXING_CRESCENT4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.75,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4S13.48,3.19,15,3.19s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.75,15.97,3.75,14.44z M15.35,4.39\n\tc1.05,1.27,1.91,2.75,2.57,4.44s0.99,3.56,0.99,5.61c0,4.39-1.14,7.75-3.43,10.06c1.31-0.06,2.55-0.37,3.74-0.92s2.2-1.28,3.05-2.18\n\ts1.53-1.95,2.04-3.16s0.75-2.48,0.75-3.81c0-1.78-0.43-3.43-1.3-4.94s-2.04-2.73-3.53-3.65S17.12,4.45,15.35,4.39z\" />",
};
#[cfg(feature = "WiMoonAltWaxingCrescent5")]
const WI_MOON_ALT_WAXING_CRESCENT5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M15.15,4.39\n\tc1.83,3.22,2.74,6.57,2.74,10.05c0,4.16-0.88,7.51-2.65,10.06c1.34-0.03,2.61-0.32,3.82-0.86s2.25-1.27,3.13-2.16\n\ts1.57-1.95,2.09-3.18s0.78-2.51,0.78-3.86c0-1.8-0.44-3.46-1.33-5s-2.08-2.75-3.6-3.65S16.95,4.42,15.15,4.39z\" />",
};
#[cfg(feature = "WiMoonAltWaxingCrescent6")]
const WI_MOON_ALT_WAXING_CRESCENT6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M15.01,4.39\n\tc1.23,3.54,1.85,6.89,1.85,10.05c0,3.93-0.59,7.28-1.77,10.06c1.35-0.01,2.64-0.28,3.87-0.81s2.3-1.25,3.19-2.15s1.6-1.97,2.12-3.21\n\ts0.79-2.54,0.79-3.9s-0.27-2.66-0.8-3.9s-1.25-2.31-2.15-3.21s-1.97-1.61-3.21-2.15S16.36,4.39,15.01,4.39z\" />",
};
#[cfg(feature = "WiMoonAltWaxingGibbous1")]
const WI_MOON_ALT_WAXING_GIBBOUS1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M13.38,14.44\n\tc0,3.81,0.41,7.16,1.23,10.06h0.38c1.37,0,2.67-0.27,3.91-0.8s2.31-1.25,3.21-2.15s1.61-1.97,2.15-3.21s0.8-2.54,0.8-3.91\n\tc0-1.36-0.27-2.66-0.8-3.9s-1.25-2.31-2.15-3.21s-1.97-1.61-3.21-2.15s-2.54-0.8-3.91-0.8h-0.34C13.81,7.99,13.38,11.34,13.38,14.44\n\tz\" />",
};
#[cfg(feature = "WiMoonAltWaxingGibbous2")]
const WI_MOON_ALT_WAXING_GIBBOUS2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M11.96,14.44\n\tc0,4.15,0.81,7.5,2.42,10.05c0.15,0.01,0.35,0.01,0.62,0.01c1.37,0,2.67-0.27,3.91-0.8s2.31-1.25,3.21-2.15s1.61-1.97,2.15-3.21\n\ts0.8-2.54,0.8-3.91c0-1.36-0.27-2.66-0.8-3.9s-1.25-2.31-2.15-3.21s-1.97-1.61-3.21-2.15s-2.54-0.8-3.91-0.8\n\tc-0.23,0-0.42,0-0.54,0.01C12.79,7.55,11.96,10.9,11.96,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaxingGibbous3")]
const WI_MOON_ALT_WAXING_GIBBOUS3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-2.03,0.5-3.91,1.51-5.64s2.37-3.1,4.1-4.1s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89s2.59,1.4,3.6,2.4\n\ts1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4s-2.85,0.89-4.37,0.89\n\ts-2.98-0.3-4.37-0.89s-2.59-1.4-3.59-2.4s-1.8-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M10.54,14.44c0,4.44,1.17,7.78,3.5,10.02\n\tc0.07,0,0.17,0,0.3,0.01s0.25,0.02,0.35,0.02s0.2,0.01,0.3,0.01c1.36,0,2.66-0.27,3.9-0.8s2.32-1.25,3.22-2.15s1.61-1.97,2.15-3.21\n\ts0.8-2.54,0.8-3.91c0-1.36-0.27-2.66-0.8-3.9s-1.25-2.31-2.15-3.21s-1.97-1.61-3.22-2.15s-2.55-0.8-3.9-0.8\n\tc-0.36,0-0.63,0.01-0.81,0.03c-1.08,1.22-1.96,2.69-2.64,4.42S10.54,12.43,10.54,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaxingGibbous4")]
const WI_MOON_ALT_WAXING_GIBBOUS4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.53,0.3-3,0.89-4.39s1.4-2.59,2.4-3.6s2.2-1.81,3.6-2.4s2.85-0.89,4.37-0.89c1.53,0,3,0.3,4.39,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.39,0.89c-1.52,0-2.98-0.3-4.37-0.89s-2.59-1.4-3.6-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M9.13,14.43\n\tc0,4.68,1.48,8,4.45,9.98c0.44,0.07,0.91,0.1,1.42,0.1c1.37,0,2.67-0.27,3.91-0.8s2.31-1.25,3.22-2.15s1.62-1.97,2.15-3.22\n\ts0.8-2.55,0.8-3.9c0-1.82-0.45-3.5-1.35-5.05s-2.13-2.77-3.68-3.68s-3.23-1.35-5.05-1.35c-0.45,0-0.84,0.02-1.19,0.06\n\tc-1.4,1.06-2.53,2.46-3.39,4.2S9.13,12.29,9.13,14.43z\" />",
};
#[cfg(feature = "WiMoonAltWaxingGibbous5")]
const WI_MOON_ALT_WAXING_GIBBOUS5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.53,0.3-3,0.89-4.39s1.4-2.59,2.4-3.6s2.2-1.81,3.6-2.4s2.85-0.89,4.37-0.89c1.53,0,3,0.3,4.39,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.39,0.89c-1.52,0-2.98-0.3-4.37-0.89s-2.59-1.4-3.6-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M7.71,14.44\n\tc0,4.76,1.66,8.02,4.97,9.79c0.73,0.19,1.51,0.28,2.33,0.28c1.37,0,2.67-0.27,3.91-0.8s2.31-1.25,3.22-2.15s1.62-1.97,2.15-3.22\n\ts0.8-2.55,0.8-3.9c0-1.82-0.45-3.5-1.35-5.05s-2.13-2.77-3.68-3.68s-3.23-1.35-5.05-1.35c-0.68,0-1.3,0.05-1.85,0.16\n\tc-1.63,0.94-2.95,2.27-3.95,3.99S7.71,12.22,7.71,14.44z\" />",
};
#[cfg(feature = "WiMoonAltWaxingGibbous6")]
const WI_MOON_ALT_WAXING_GIBBOUS6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.53,0.3-3,0.89-4.39s1.4-2.59,2.4-3.6s2.2-1.81,3.6-2.4s2.85-0.89,4.37-0.89c1.53,0,3,0.3,4.39,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.39,0.89c-1.52,0-2.98-0.3-4.37-0.89s-2.59-1.4-3.6-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M6.42,14.44\n\tc0,0.78,0.05,1.5,0.15,2.15c0.1,0.65,0.29,1.35,0.57,2.09s0.66,1.41,1.13,2.03s1.09,1.24,1.88,1.86s1.7,1.2,2.77,1.71\n\tc0.67,0.15,1.37,0.22,2.09,0.22c1.37,0,2.67-0.27,3.91-0.8s2.31-1.25,3.22-2.15s1.62-1.97,2.15-3.22s0.8-2.55,0.8-3.9\n\tc0-1.82-0.45-3.5-1.35-5.05s-2.13-2.77-3.68-3.68s-3.23-1.35-5.05-1.35c-0.85,0-1.62,0.09-2.31,0.26C11.81,5.05,11.03,5.51,10.35,6\n\tS9.1,7.01,8.66,7.54S7.84,8.62,7.53,9.16s-0.54,1.12-0.69,1.74s-0.26,1.2-0.32,1.75S6.42,13.8,6.42,14.44z\" />",
};
#[cfg(feature = "WiMoonFirstQuarter")]
const WI_MOON_FIRST_QUARTER: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09\n\ts-3.61-1.51-5.65-1.51V25.71z\" />",
};
#[cfg(feature = "WiMoonFull")]
const WI_MOON_FULL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.1,4.1s3.61,1.51,5.65,1.51s3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1\n\ts1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09s-3.61-1.51-5.65-1.51S11.08,3.7,9.35,4.7s-3.1,2.37-4.1,4.09\n\tS3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonNew")]
const WI_MOON_NEW: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0-1.53,0.3-3,0.89-4.39s1.4-2.59,2.4-3.6s2.2-1.81,3.6-2.4s2.85-0.89,4.37-0.89c1.53,0,3,0.3,4.39,0.89\n\ts2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4\n\ts-2.85,0.89-4.39,0.89c-1.52,0-2.98-0.3-4.37-0.89s-2.59-1.4-3.6-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44\n\tc0,1.36,0.27,2.66,0.8,3.9s1.25,2.32,2.15,3.22s1.97,1.61,3.22,2.15s2.55,0.8,3.9,0.8c1.37,0,2.67-0.27,3.91-0.8\n\ts2.31-1.25,3.22-2.15s1.62-1.97,2.15-3.22s0.8-2.55,0.8-3.9c0-1.82-0.45-3.5-1.35-5.05s-2.13-2.77-3.68-3.68s-3.23-1.35-5.05-1.35\n\tc-1.36,0-2.66,0.27-3.9,0.8S8.79,6.41,7.89,7.32s-1.61,1.98-2.15,3.22S4.94,13.08,4.94,14.44z\" />",
};
#[cfg(feature = "WiMoonThirdQuarter")]
const WI_MOON_THIRD_QUARTER: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.09,4.1s3.61,1.51,5.65,1.51V3.19c-2.04,0-3.92,0.5-5.65,1.51\n\tS6.26,7.07,5.25,8.8S3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningCrescent1")]
const WI_MOON_WANING_CRESCENT1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.09,4.1s3.61,1.51,5.65,1.51c-1-3.14-1.49-6.9-1.49-11.26\n\tc0-3.43,0.5-7.18,1.49-11.25c-2.04,0-3.92,0.5-5.65,1.51S6.26,7.07,5.25,8.8S3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningCrescent2")]
const WI_MOON_WANING_CRESCENT2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.1,4.1s3.61,1.51,5.65,1.51c-2.01-2.74-3.02-6.5-3.02-11.26\n\tc0-3.98,1.01-7.73,3.02-11.25c-2.04,0-3.93,0.5-5.65,1.51s-3.1,2.37-4.1,4.09S3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningCrescent3")]
const WI_MOON_WANING_CRESCENT3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.09,4.1s3.61,1.51,5.65,1.51c-2.99-2.33-4.48-6.09-4.48-11.26\n\tc0-2.32,0.42-4.46,1.25-6.4s1.91-3.56,3.23-4.85c-2.04,0-3.92,0.5-5.65,1.51S6.26,7.07,5.25,8.8S3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningCrescent4")]
const WI_MOON_WANING_CRESCENT4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.1,4.1s3.61,1.51,5.65,1.51c-2.07-1.01-3.59-2.45-4.56-4.33\n\tS9,17.19,9,14.44c0-2.53,0.56-4.78,1.69-6.75s2.57-3.47,4.31-4.5c-2.04,0-3.93,0.5-5.65,1.51s-3.1,2.37-4.1,4.09\n\tS3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningCrescent5")]
const WI_MOON_WANING_CRESCENT5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.09,4.1s3.61,1.51,5.65,1.51c-2.59-0.79-4.48-2.13-5.69-4.02\n\ts-1.81-4.3-1.81-7.24c0-1.39,0.2-2.7,0.6-3.95c0.4-1.25,0.94-2.34,1.63-3.27s1.48-1.75,2.37-2.44s1.86-1.22,2.89-1.59\n\tc-2.04,0-3.92,0.5-5.65,1.51S6.26,7.07,5.25,8.8S3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningCrescent6")]
const WI_MOON_WANING_CRESCENT6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.09,4.1s3.61,1.51,5.65,1.51c-1.46-0.56-2.72-1.18-3.79-1.88\n\ts-1.93-1.39-2.57-2.1s-1.15-1.49-1.53-2.34s-0.64-1.66-0.77-2.42s-0.2-1.6-0.2-2.52c0-0.65,0.03-1.26,0.08-1.81s0.16-1.14,0.32-1.77\n\ts0.38-1.21,0.64-1.75s0.63-1.09,1.08-1.66s0.98-1.1,1.59-1.57s1.34-0.95,2.21-1.42s1.85-0.89,2.93-1.27c-2.04,0-3.92,0.5-5.65,1.51\n\tS6.26,7.07,5.25,8.8S3.74,12.4,3.74,14.44z\" />",
};
#[cfg(feature = "WiMoonWaningGibbous1")]
const WI_MOON_WANING_GIBBOUS1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.49c0,1.22,0.19,2.4,0.56,3.54s0.91,2.17,1.6,3.09s1.5,1.72,2.42,2.42s1.95,1.23,3.09,1.6s2.32,0.56,3.54,0.56\n\tc5.03-1.4,7.54-5.14,7.54-11.22c0-1.18-0.14-2.3-0.42-3.37s-0.65-2.01-1.13-2.83s-1.04-1.57-1.68-2.24s-1.34-1.24-2.06-1.68\n\ts-1.47-0.81-2.26-1.07c-1.52,0-2.98,0.3-4.37,0.89S8.02,5.57,7.02,6.57s-1.8,2.19-2.39,3.57S3.74,12.97,3.74,14.49z\" />",
};
#[cfg(feature = "WiMoonWaningGibbous2")]
const WI_MOON_WANING_GIBBOUS2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.49c0,1.22,0.19,2.4,0.56,3.54s0.91,2.17,1.6,3.09s1.5,1.72,2.42,2.42s1.95,1.23,3.09,1.6s2.32,0.56,3.54,0.56\n\tc4.33-1.73,6.49-5.47,6.49-11.22c0-1.39-0.18-2.7-0.54-3.93s-0.85-2.31-1.47-3.23s-1.31-1.71-2.06-2.39s-1.56-1.23-2.42-1.66\n\tc-2.03,0-3.91,0.5-5.63,1.5S6.25,7.14,5.24,8.86S3.74,12.46,3.74,14.49z\" />",
};
#[cfg(feature = "WiMoonWaningGibbous3")]
const WI_MOON_WANING_GIBBOUS3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.49c0,1.22,0.19,2.4,0.56,3.54s0.91,2.17,1.6,3.09s1.5,1.72,2.42,2.42s1.95,1.23,3.09,1.6s2.32,0.56,3.54,0.56\n\tc3.61-2.07,5.42-5.81,5.42-11.22c0-1.31-0.15-2.56-0.45-3.74s-0.71-2.24-1.23-3.17s-1.1-1.75-1.72-2.46s-1.3-1.33-2.02-1.86\n\tc-1.52,0-2.98,0.3-4.37,0.89s-2.58,1.39-3.58,2.4s-1.8,2.2-2.39,3.59S3.74,12.96,3.74,14.49z\" />",
};
#[cfg(feature = "WiMoonWaningGibbous4")]
const WI_MOON_WANING_GIBBOUS4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0,1.52,0.3,2.98,0.89,4.37s1.39,2.58,2.4,3.59s2.2,1.8,3.59,2.4s2.84,0.89,4.37,0.89\n\tc2.89-2.39,4.34-6.14,4.34-11.24c0-2.34-0.41-4.47-1.22-6.36s-1.85-3.52-3.11-4.87c-2.03,0-3.91,0.5-5.64,1.51S6.25,7.12,5.24,8.84\n\tS3.74,12.44,3.74,14.47z\" />",
};
#[cfg(feature = "WiMoonWaningGibbous5")]
const WI_MOON_WANING_GIBBOUS5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0,2.03,0.5,3.91,1.51,5.63s2.37,3.09,4.09,4.09s3.6,1.51,5.63,1.51c2.17-2.75,3.25-6.5,3.25-11.24\n\tc0-3.96-1.08-7.71-3.25-11.24c-2.03,0-3.91,0.5-5.63,1.5S6.26,7.1,5.25,8.83S3.74,12.44,3.74,14.47z\" />",
};
#[cfg(feature = "WiMoonWaningGibbous6")]
const WI_MOON_WANING_GIBBOUS6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.46c0,2.04,0.5,3.92,1.51,5.65s2.37,3.09,4.09,4.09s3.61,1.51,5.65,1.51c1.44-3.08,2.15-6.83,2.15-11.25\n\tc0-3.46-0.72-7.2-2.15-11.24c-1.52,0-2.98,0.3-4.37,0.89S8.03,5.5,7.03,6.5s-1.8,2.2-2.4,3.59S3.74,12.93,3.74,14.46z\" />",
};
#[cfg(feature = "WiMoonWaxingCrescent1")]
const WI_MOON_WAXING_CRESCENT1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09\n\ts-3.61-1.51-5.65-1.51c1.32,0.52,2.48,1.2,3.47,2.06s1.78,1.79,2.35,2.82s0.99,2.07,1.27,3.13s0.41,2.14,0.41,3.24\n\tc0,0.64-0.02,1.26-0.07,1.84c-0.05,0.58-0.15,1.2-0.29,1.87s-0.33,1.28-0.56,1.86s-0.54,1.15-0.92,1.74s-0.83,1.11-1.35,1.58\n\ts-1.14,0.92-1.87,1.33S15.9,25.42,15.01,25.71z\" />",
};
#[cfg(feature = "WiMoonWaxingCrescent2")]
const WI_MOON_WAXING_CRESCENT2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09\n\ts-3.61-1.51-5.65-1.51c1.1,0.59,2.07,1.32,2.89,2.19s1.47,1.82,1.95,2.83s0.83,2.03,1.05,3.07s0.34,2.09,0.34,3.16\n\tc0,0.91-0.04,1.76-0.13,2.54s-0.27,1.63-0.53,2.53s-0.62,1.71-1.06,2.43s-1.04,1.42-1.82,2.09S16.03,25.26,15.01,25.71z\" />",
};
#[cfg(feature = "WiMoonWaxingCrescent3")]
const WI_MOON_WAXING_CRESCENT3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09\n\ts-3.61-1.51-5.65-1.51c1.71,1.26,2.97,2.9,3.78,4.91S20,12.24,20,14.44c0,0.9-0.03,1.73-0.1,2.5s-0.21,1.59-0.43,2.47\n\ts-0.51,1.68-0.86,2.4s-0.83,1.42-1.45,2.12S15.83,25.21,15.01,25.71z\" />",
};
#[cfg(feature = "WiMoonWaxingCrescent4")]
const WI_MOON_WAXING_CRESCENT4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09\n\ts-3.61-1.51-5.65-1.51c1.29,1.39,2.24,3.07,2.84,5.05s0.91,4.05,0.91,6.2c0,0.88-0.03,1.69-0.08,2.44s-0.16,1.55-0.32,2.41\n\ts-0.38,1.65-0.64,2.37s-0.63,1.43-1.09,2.15S15.62,25.15,15.01,25.71z\" />",
};
#[cfg(feature = "WiMoonWaxingCrescent5")]
const WI_MOON_WAXING_CRESCENT5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M14.99,25.71c2.04,0,3.93-0.5,5.65-1.51s3.1-2.37,4.1-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.1-4.09\n\ts-3.61-1.51-5.65-1.51c1.67,2.9,2.5,6.65,2.5,11.25c0,2.33-0.17,4.43-0.52,6.3S15.97,24.26,14.99,25.71z\" />",
};
#[cfg(feature = "WiMoonWaxingCrescent6")]
const WI_MOON_WAXING_CRESCENT6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M14.99,25.71c2.04,0,3.93-0.5,5.65-1.51s3.1-2.37,4.1-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.1-4.09\n\ts-3.61-1.51-5.65-1.51c1.67,2.9,2.5,6.65,2.5,11.25c0,2.33-0.17,4.43-0.52,6.3S15.97,24.26,14.99,25.71z\" />",
};
#[cfg(feature = "WiMoonWaxingGibbous1")]
const WI_MOON_WAXING_GIBBOUS1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M13.93,14.44c-0.02,4.53,0.33,8.29,1.04,11.27c2.04,0.01,3.93-0.49,5.65-1.49s3.1-2.36,4.11-4.08s1.52-3.61,1.53-5.65\n\tc0.01-2.04-0.49-3.93-1.49-5.65c-1-1.73-2.36-3.1-4.08-4.11s-3.6-1.52-5.64-1.53C14.32,6.91,13.94,10.66,13.93,14.44z\" />",
};
#[cfg(feature = "WiMoonWaxingGibbous2")]
const WI_MOON_WAXING_GIBBOUS2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M12.85,14.44c0,4.77,0.71,8.52,2.14,11.26c2.04,0,3.93-0.5,5.65-1.51s3.1-2.37,4.1-4.1s1.51-3.61,1.51-5.65\n\ts-0.5-3.92-1.51-5.65s-2.37-3.09-4.1-4.09s-3.61-1.51-5.65-1.51C13.57,6.61,12.85,10.36,12.85,14.44z\" />",
};
#[cfg(feature = "WiMoonWaxingGibbous3")]
const WI_MOON_WAXING_GIBBOUS3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M11.8,14.43c0,2.39,0.24,4.52,0.71,6.39s1.31,3.5,2.51,4.89c1.52,0,2.98-0.3,4.37-0.89s2.59-1.4,3.6-2.4s1.81-2.2,2.4-3.6\n\ts0.89-2.85,0.89-4.39s-0.3-2.99-0.89-4.38s-1.4-2.58-2.4-3.59s-2.2-1.81-3.6-2.4s-2.85-0.89-4.37-0.89\n\tc-1.02,1.46-1.81,3.16-2.37,5.13S11.8,12.3,11.8,14.43z\" />",
};
#[cfg(feature = "WiMoonWaxingGibbous4")]
const WI_MOON_WAXING_GIBBOUS4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M10.73,14.43c0,1.19,0.07,2.29,0.2,3.3s0.35,2,0.67,2.99s0.76,1.9,1.33,2.75s1.27,1.59,2.09,2.25c1.53,0,2.99-0.3,4.38-0.89\n\ts2.58-1.4,3.59-2.4s1.81-2.2,2.4-3.6s0.89-2.85,0.89-4.39c0-2.04-0.5-3.93-1.51-5.65s-2.37-3.1-4.1-4.1s-3.61-1.51-5.65-1.51\n\tc-1.35,1.3-2.4,2.94-3.16,4.93S10.73,12.19,10.73,14.43z\" />",
};
#[cfg(feature = "WiMoonWaxingGibbous5")]
const WI_MOON_WAXING_GIBBOUS5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.65,14.43c0,1.24,0.08,2.38,0.25,3.41s0.44,2.05,0.83,3.04s0.95,1.89,1.67,2.71s1.6,1.52,2.62,2.12\n\tc1.52,0,2.98-0.3,4.37-0.89s2.59-1.4,3.6-2.4s1.81-2.2,2.4-3.6s0.89-2.85,0.89-4.39s-0.3-2.99-0.89-4.38s-1.4-2.58-2.4-3.59\n\ts-2.2-1.81-3.6-2.4s-2.85-0.89-4.37-0.89c-1.67,1.14-2.98,2.72-3.94,4.74S9.65,12.09,9.65,14.43z\" />",
};
#[cfg(feature = "WiMoonWaxingGibbous6")]
const WI_MOON_WAXING_GIBBOUS6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M8.58,14.43c0,1.03,0.06,1.97,0.18,2.83s0.32,1.73,0.62,2.59s0.69,1.65,1.16,2.34s1.1,1.35,1.85,1.96s1.63,1.12,2.63,1.55\n\tc1.53,0,2.99-0.3,4.38-0.89s2.58-1.4,3.59-2.4s1.81-2.2,2.4-3.6s0.89-2.85,0.89-4.39c0-2.04-0.5-3.93-1.51-5.65s-2.37-3.1-4.1-4.1\n\ts-3.61-1.51-5.65-1.51c-1.99,1-3.56,2.51-4.72,4.55S8.58,11.99,8.58,14.43z\" />",
};
#[cfg(feature = "WiMoonrise")]
const WI_MOONRISE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.8,14.86c0-0.98,0.19-1.92,0.58-2.82c0.38-0.9,0.9-1.67,1.55-2.32c0.65-0.65,1.43-1.17,2.32-1.56s1.84-0.58,2.83-0.58\n\th1.17c0.16,0.04,0.24,0.14,0.24,0.28l0.04,0.9c0.04,1.3,0.51,2.41,1.41,3.33s2,1.41,3.28,1.46l0.85,0.07c0.16,0,0.23,0.08,0.23,0.23\n\tv1.01c0.01,1.03-0.19,2-0.58,2.92h-2.05c0.51-0.74,0.83-1.59,0.97-2.53c-1.67-0.35-3.02-1.07-4.03-2.16s-1.59-2.37-1.75-3.83\n\tc-0.97,0.05-1.85,0.35-2.66,0.9c-0.8,0.55-1.43,1.24-1.87,2.08c-0.44,0.84-0.66,1.72-0.66,2.63c0,1.07,0.28,2.04,0.83,2.92H8.4\n\tC8,16.85,7.8,15.88,7.8,14.86z M8.09,20.87c0-0.29,0.09-0.52,0.28-0.68c0.18-0.18,0.41-0.26,0.69-0.26h2.63L14.8,17\n\tc0.1-0.08,0.22-0.08,0.35,0l3.16,2.92h2.77c0.27,0,0.5,0.09,0.69,0.28c0.19,0.18,0.29,0.41,0.29,0.67c0,0.27-0.1,0.5-0.29,0.69\n\tc-0.19,0.19-0.42,0.29-0.69,0.29h-3.38c-0.1,0-0.2-0.02-0.29-0.07l-2.41-2.27l-2.39,2.27c-0.08,0.05-0.17,0.07-0.28,0.07H9.06\n\tc-0.27,0-0.5-0.1-0.69-0.29S8.09,21.14,8.09,20.87z\" />",
};
#[cfg(feature = "WiMoonset")]
const WI_MOONSET: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.74,14.86c0-0.98,0.19-1.92,0.58-2.82c0.38-0.9,0.9-1.67,1.55-2.32c0.65-0.65,1.43-1.17,2.32-1.56\n\tc0.9-0.39,1.84-0.58,2.83-0.58h1.17c0.16,0.04,0.24,0.14,0.24,0.28l0.05,0.9c0.02,0.64,0.15,1.25,0.4,1.83s0.58,1.08,1,1.5\n\tc0.42,0.43,0.91,0.77,1.48,1.03c0.57,0.26,1.17,0.4,1.8,0.43l0.85,0.07c0.16,0,0.24,0.08,0.24,0.23v1.01\n\tc0.01,1.01-0.19,1.98-0.59,2.92h-2.05c0.51-0.74,0.83-1.59,0.97-2.53c-1.68-0.35-3.02-1.07-4.03-2.16s-1.59-2.37-1.75-3.83\n\tc-0.97,0.05-1.85,0.35-2.66,0.9c-0.8,0.55-1.42,1.24-1.87,2.08c-0.44,0.84-0.66,1.72-0.66,2.63c0,1.07,0.28,2.04,0.83,2.92H8.34\n\tC7.94,16.85,7.74,15.88,7.74,14.86z M7.99,20.89c0-0.26,0.1-0.49,0.3-0.69c0.18-0.18,0.41-0.27,0.68-0.27h3.22\n\tc0.11,0,0.2,0.02,0.28,0.08l2.35,2.22L17.21,20c0.07-0.05,0.17-0.08,0.29-0.08h3.3c0.27,0,0.5,0.09,0.69,0.28\n\tc0.19,0.19,0.29,0.42,0.29,0.68c0,0.27-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.69,0.29h-2.68l-3.13,2.84\n\tc-0.12,0.09-0.24,0.09-0.34,0l-3.08-2.84h-2.6c-0.27,0-0.5-0.1-0.69-0.29C8.09,21.39,7.99,21.16,7.99,20.89z\" />",
};
#[cfg(feature = "WiNa")]
const WI_NA: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M6.87,18.23h1.94v-3.64h0.02l2.05,3.64h1.99v-6.66h-1.94v3.55h-0.02l-1.94-3.55h-2.1V18.23z M13.39,18.38h1.43l2.61-6.97\n\th-1.42L13.39,18.38z M16.26,18.23h2.07l0.29-0.95h2.12l0.28,0.95h2.13l-2.43-6.66h-2.01L16.26,18.23z M19.07,15.84l0.64-2.04h0.03\n\tl0.6,2.04H19.07z\" />",
};
#[cfg(feature = "WiNightAltCloudy")]
const WI_NIGHT_ALT_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.14,16.9c0-1.16,0.35-2.18,1.06-3.08s1.62-1.47,2.74-1.72c0.23-1.03,0.7-1.93,1.4-2.7c0.7-0.77,1.55-1.32,2.53-1.65\n\tc0.62-0.21,1.26-0.32,1.93-0.32c0.81,0,1.6,0.16,2.35,0.48c0.28-0.47,0.61-0.88,0.99-1.22c0.38-0.34,0.77-0.61,1.17-0.79\n\tc0.4-0.18,0.8-0.32,1.18-0.41s0.76-0.13,1.12-0.13c0.38,0,0.79,0.05,1.23,0.16l0.82,0.25c0.14,0.06,0.18,0.13,0.14,0.22l-0.14,0.6\n\tc-0.07,0.31-0.1,0.6-0.1,0.86c0,0.31,0.05,0.63,0.15,0.95c0.1,0.32,0.24,0.63,0.44,0.94c0.19,0.31,0.46,0.58,0.8,0.83\n\tc0.34,0.25,0.72,0.44,1.15,0.57l0.62,0.22c0.1,0.03,0.15,0.08,0.15,0.16c0,0.02-0.01,0.04-0.02,0.07l-0.18,0.67\n\tc-0.27,1.08-0.78,1.93-1.5,2.57c0.4,0.7,0.62,1.45,0.65,2.24c0.01,0.05,0.01,0.12,0.01,0.23c0,0.89-0.22,1.72-0.67,2.48\n\tc-0.44,0.76-1.05,1.36-1.8,1.8c-0.76,0.44-1.59,0.67-2.48,0.67H9.07c-0.89,0-1.72-0.22-2.48-0.67s-1.35-1.05-1.79-1.8\n\tS4.14,17.8,4.14,16.9z M5.85,16.9c0,0.89,0.32,1.66,0.96,2.31c0.64,0.65,1.39,0.98,2.26,0.98h10.81c0.89,0,1.65-0.32,2.28-0.97\n\ts0.95-1.42,0.95-2.32c0-0.88-0.32-1.63-0.96-2.26c-0.64-0.63-1.4-0.95-2.28-0.95h-1.78l-0.1-0.75c-0.1-1.01-0.52-1.88-1.26-2.59\n\ts-1.62-1.11-2.63-1.2c-0.03,0-0.08,0-0.15-0.01c-0.07-0.01-0.11-0.01-0.15-0.01c-0.51,0-1.02,0.1-1.54,0.29V9.4\n\tc-0.73,0.28-1.35,0.74-1.84,1.37c-0.5,0.63-0.8,1.35-0.9,2.17l-0.07,0.72l-0.68,0.03c-0.84,0.1-1.54,0.45-2.1,1.06\n\tS5.85,16.07,5.85,16.9z M17.6,8.79c1.06,0.91,1.72,1.97,1.97,3.18h0.32c1.24,0,2.3,0.39,3.17,1.18c0.33-0.31,0.58-0.67,0.76-1.07\n\tc-0.91-0.43-1.63-1.09-2.16-1.97c-0.52-0.88-0.79-1.81-0.79-2.78V7.09c-0.05-0.01-0.13-0.01-0.24-0.01\n\tc-0.58-0.01-1.15,0.13-1.7,0.44C18.38,7.82,17.93,8.24,17.6,8.79z\" />",
};
#[cfg(feature = "WiNightAltCloudyGusts")]
const WI_NIGHT_ALT_CLOUDY_GUSTS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.98,21.02c0,0.23,0.09,0.43,0.27,0.6c0.17,0.17,0.37,0.25,0.61,0.25H9.6c0.26,0,0.49,0.1,0.69,0.29\n\tc0.2,0.19,0.3,0.42,0.3,0.68c0,0.27-0.1,0.5-0.3,0.69s-0.43,0.29-0.69,0.29c-0.26,0-0.48-0.1-0.68-0.3\n\tc-0.18-0.16-0.38-0.24-0.61-0.24c-0.24,0-0.44,0.08-0.6,0.24c-0.16,0.16-0.24,0.36-0.24,0.6c0,0.22,0.08,0.42,0.24,0.6\n\tc0.52,0.53,1.16,0.79,1.89,0.79s1.37-0.26,1.89-0.78c0.52-0.52,0.78-1.15,0.78-1.89c0-0.74-0.26-1.37-0.78-1.89\n\ts-1.15-0.79-1.89-0.79H3.86c-0.24,0-0.44,0.09-0.62,0.26C3.07,20.59,2.98,20.79,2.98,21.02z M2.98,18c0,0.22,0.09,0.41,0.27,0.58\n\tc0.17,0.16,0.38,0.24,0.61,0.24h10.85c0.74,0,1.37-0.26,1.89-0.78c0.52-0.52,0.78-1.15,0.78-1.88c0-0.74-0.26-1.36-0.78-1.88\n\tc-0.52-0.52-1.15-0.77-1.89-0.77c-0.76,0-1.39,0.25-1.89,0.76c-0.15,0.16-0.23,0.36-0.23,0.61c0,0.25,0.08,0.45,0.23,0.6\n\tc0.15,0.15,0.35,0.23,0.59,0.23s0.44-0.07,0.62-0.23c0.19-0.19,0.42-0.28,0.68-0.28c0.26,0,0.48,0.09,0.67,0.28\n\tc0.19,0.19,0.29,0.42,0.29,0.68c0,0.26-0.1,0.49-0.29,0.68c-0.19,0.19-0.42,0.29-0.67,0.29H3.86c-0.24,0-0.44,0.09-0.62,0.26\n\tC3.07,17.57,2.98,17.77,2.98,18z M5.5,15.65c0,0.09,0.05,0.13,0.16,0.13H7.1c0.08,0,0.15-0.05,0.22-0.15\n\tc0.22-0.54,0.57-0.99,1.05-1.35c0.47-0.35,1-0.55,1.58-0.6l0.52-0.07c0.12,0,0.19-0.06,0.19-0.17l0.08-0.52\n\tc0.11-1.08,0.57-1.99,1.38-2.71c0.81-0.73,1.77-1.09,2.86-1.09s2.05,0.36,2.85,1.09c0.81,0.72,1.27,1.63,1.38,2.72l0.07,0.58\n\tc0,0.12,0.06,0.18,0.19,0.18h1.62c0.91,0,1.68,0.32,2.32,0.95c0.64,0.63,0.96,1.39,0.96,2.28c0,0.89-0.32,1.65-0.96,2.29\n\tc-0.64,0.64-1.41,0.96-2.31,0.96h-6.91c-0.11,0-0.17,0.06-0.17,0.18v1.37c0,0.11,0.06,0.17,0.17,0.17h6.91\n\tc0.89,0,1.72-0.22,2.48-0.67s1.36-1.05,1.8-1.81s0.67-1.59,0.67-2.48c0-0.88-0.23-1.71-0.68-2.48c0.73-0.71,1.23-1.57,1.51-2.58\n\tL27,11.18c0.02-0.02,0.03-0.04,0.03-0.07c0-0.04-0.05-0.1-0.14-0.16l-0.6-0.21c-0.84-0.26-1.48-0.71-1.92-1.36\n\tc-0.44-0.65-0.66-1.32-0.66-2.02c0-0.24,0.03-0.51,0.09-0.79l0.13-0.62c0.02-0.1-0.02-0.18-0.13-0.22l-0.8-0.24\n\tc-0.44-0.11-0.85-0.16-1.25-0.16c-0.37,0-0.74,0.04-1.12,0.13c-0.38,0.09-0.77,0.22-1.18,0.41c-0.4,0.18-0.8,0.45-1.18,0.8\n\tc-0.38,0.35-0.72,0.75-1,1.22c-0.71-0.3-1.48-0.45-2.32-0.45c-1.41,0-2.66,0.44-3.75,1.31s-1.77,1.99-2.07,3.35\n\tc-0.86,0.2-1.61,0.61-2.27,1.23c-0.66,0.62-1.11,1.35-1.36,2.2v0.03C5.51,15.58,5.5,15.61,5.5,15.65z M18.73,8.76\n\tc0.31-0.55,0.74-0.97,1.29-1.26c0.55-0.29,1.12-0.44,1.71-0.44c0.14,0,0.24,0,0.31,0.01c-0.01,0.09-0.02,0.21-0.02,0.36\n\tc0,0.94,0.26,1.85,0.79,2.71c0.52,0.86,1.25,1.51,2.17,1.94c-0.16,0.38-0.41,0.72-0.75,1.03c-0.89-0.76-1.94-1.14-3.16-1.14h-0.33\n\tC20.48,10.71,19.81,9.64,18.73,8.76z\" />",
};
#[cfg(feature = "WiNightAltCloudyHigh")]
const WI_NIGHT_ALT_CLOUDY_HIGH: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.57,13.43c0-1.15,0.36-2.18,1.08-3.08s1.63-1.48,2.73-1.74C7.7,7.24,8.4,6.12,9.5,5.24s2.35-1.31,3.76-1.31\n\tc1.38,0,2.61,0.43,3.69,1.28s1.78,1.95,2.1,3.29h0.33c0.9,0,1.73,0.22,2.49,0.65c0.76,0.43,1.37,1.03,1.81,1.79\n\tc0.44,0.76,0.67,1.58,0.67,2.48c0,1.15-0.35,2.18-1.06,3.08c0.64,0.55,1.4,0.84,2.26,0.87l0.66,0.06c0.12,0,0.18,0.06,0.18,0.19\n\tv0.77c0.01,1.01-0.24,1.95-0.73,2.8c-0.49,0.86-1.17,1.53-2.02,2.03c-0.85,0.5-1.78,0.75-2.79,0.75c-0.77,0-1.5-0.15-2.19-0.44\n\tc-0.69-0.29-1.28-0.69-1.78-1.19c-0.49-0.5-0.89-1.09-1.18-1.78c-0.29-0.69-0.44-1.41-0.44-2.17H8.37c-1.34-0.06-2.47-0.57-3.4-1.53\n\tS3.57,14.76,3.57,13.43z M5.28,13.43c0,0.87,0.3,1.62,0.9,2.26s1.33,0.98,2.19,1.02h11.19c0.86-0.04,1.59-0.38,2.19-1.02\n\tc0.6-0.64,0.9-1.39,0.9-2.26c0-0.88-0.32-1.63-0.97-2.28c-0.65-0.64-1.42-0.97-2.31-0.97h-1.62c-0.11,0-0.17-0.06-0.17-0.17\n\tl-0.07-0.58c-0.11-1.08-0.58-1.99-1.4-2.71s-1.77-1.09-2.86-1.09c-1.1,0-2.05,0.36-2.86,1.09S9.13,8.35,9.03,9.43l-0.07,0.58\n\tc0,0.11-0.07,0.17-0.2,0.17H8.23c-0.84,0.1-1.54,0.46-2.1,1.07S5.28,12.59,5.28,13.43z M16.71,18.39c0,0.79,0.2,1.52,0.6,2.17\n\tc0.4,0.65,0.91,1.15,1.54,1.5c0.63,0.35,1.29,0.52,1.99,0.52c0.62,0,1.23-0.15,1.82-0.45c0.6-0.3,1.12-0.75,1.58-1.36\n\ts0.75-1.31,0.86-2.1c-1.08-0.22-1.98-0.65-2.72-1.3c-0.84,0.65-1.78,0.99-2.82,1.01H16.71z\" />",
};
#[cfg(feature = "WiNightAltCloudyWindy")]
const WI_NIGHT_ALT_CLOUDY_WINDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.35,21.05c0,0.24,0.08,0.43,0.25,0.59c0.17,0.16,0.38,0.23,0.63,0.23h9.4c0.24,0,0.43-0.08,0.59-0.23\n\tc0.16-0.16,0.23-0.35,0.23-0.59c0-0.25-0.08-0.45-0.23-0.61c-0.16-0.16-0.35-0.24-0.59-0.24h-9.4c-0.25,0-0.46,0.08-0.63,0.24\n\tS2.35,20.8,2.35,21.05z M4.98,18c0,0.24,0.09,0.44,0.26,0.6c0.16,0.17,0.36,0.25,0.6,0.25h9.42c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.6c0-0.23-0.08-0.43-0.23-0.59s-0.35-0.24-0.59-0.24H5.85c-0.24,0-0.44,0.08-0.61,0.24\n\tC5.07,17.57,4.98,17.77,4.98,18z M6.02,15.66c0,0.09,0.06,0.14,0.18,0.14h1.43c0.09,0,0.16-0.05,0.22-0.14\n\tc0.23-0.54,0.57-0.99,1.04-1.35c0.47-0.36,0.99-0.56,1.58-0.6l0.55-0.07c0.12,0,0.18-0.06,0.18-0.17l0.07-0.52\n\tc0.11-1.09,0.57-2,1.38-2.72c0.82-0.73,1.77-1.09,2.87-1.09c1.09,0,2.04,0.36,2.84,1.08c0.8,0.72,1.27,1.62,1.41,2.71l0.08,0.58\n\tc0,0.11,0.06,0.17,0.18,0.17h1.61c0.91,0,1.68,0.32,2.32,0.96c0.64,0.64,0.96,1.41,0.96,2.31c0,0.88-0.32,1.65-0.97,2.29\n\tc-0.65,0.65-1.41,0.97-2.3,0.97h-6.91c-0.11,0-0.17,0.06-0.17,0.17v1.34c0,0.11,0.06,0.17,0.17,0.17h6.91c0.9,0,1.73-0.22,2.49-0.66\n\tc0.76-0.44,1.37-1.04,1.81-1.8c0.44-0.76,0.67-1.59,0.67-2.49s-0.22-1.72-0.65-2.47c0.72-0.64,1.22-1.5,1.51-2.58l0.18-0.68\n\tc0.01-0.01,0.01-0.03,0.01-0.07c0-0.08-0.05-0.13-0.15-0.16l-0.62-0.22c-0.44-0.13-0.83-0.32-1.16-0.57\n\tc-0.34-0.25-0.6-0.53-0.8-0.84c-0.2-0.31-0.34-0.62-0.44-0.94c-0.1-0.32-0.15-0.63-0.15-0.95c0-0.24,0.04-0.53,0.11-0.87l0.13-0.61\n\tc0.04-0.09,0-0.17-0.13-0.23L23.62,5.5c-0.44-0.11-0.85-0.16-1.25-0.16c-0.38,0-0.75,0.04-1.13,0.13s-0.77,0.22-1.18,0.41\n\tc-0.41,0.18-0.8,0.45-1.18,0.8c-0.38,0.35-0.71,0.75-0.99,1.22c-0.77-0.32-1.57-0.48-2.37-0.48c-1.41,0-2.66,0.44-3.75,1.32\n\ts-1.78,2-2.08,3.38c-0.87,0.2-1.63,0.61-2.28,1.22c-0.65,0.62-1.11,1.35-1.36,2.21v0.03C6.03,15.59,6.02,15.62,6.02,15.66z\n\t M6.75,24.15c0,0.24,0.08,0.44,0.25,0.6C7.16,24.92,7.36,25,7.6,25h9.43c0.24,0,0.44-0.08,0.61-0.25c0.17-0.17,0.25-0.37,0.25-0.6\n\tc0-0.23-0.08-0.43-0.25-0.59c-0.17-0.16-0.37-0.24-0.61-0.24H7.6c-0.24,0-0.44,0.08-0.6,0.24C6.83,23.72,6.75,23.92,6.75,24.15z\n\t M19.33,8.78c0.34-0.55,0.79-0.98,1.35-1.28c0.55-0.3,1.12-0.45,1.7-0.44c0.11,0,0.2,0,0.25,0.01v0.24c0,0.97,0.26,1.9,0.79,2.79\n\tc0.53,0.88,1.25,1.55,2.17,1.98c-0.17,0.4-0.43,0.76-0.76,1.07c-0.88-0.79-1.95-1.18-3.2-1.18h-0.32\n\tC21.06,10.77,20.4,9.71,19.33,8.78z\" />",
};
#[cfg(feature = "WiNightAltHail")]
const WI_NIGHT_ALT_HAIL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.1,16.91c0,1.33,0.46,2.48,1.39,3.43s2.06,1.47,3.4,1.53c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.86-0.04-1.58-0.38-2.18-1.02s-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.84-2.17c0.56-0.61,1.26-0.97,2.1-1.07l0.53-0.03\n\tc0.13,0,0.2-0.06,0.2-0.18l0.07-0.54c0.11-1.08,0.56-1.99,1.37-2.72c0.81-0.73,1.76-1.1,2.85-1.1c1.08,0,2.03,0.37,2.85,1.1\n\tc0.82,0.73,1.28,1.64,1.4,2.72l0.08,0.58c0,0.11,0.06,0.17,0.17,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96c0.65,0.64,0.98,1.4,0.98,2.27\n\tc0,0.87-0.3,1.62-0.9,2.26c-0.6,0.64-1.33,0.98-2.18,1.02c-0.13,0-0.2,0.06-0.2,0.17v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc1.33-0.04,2.46-0.55,3.38-1.51c0.93-0.96,1.39-2.11,1.39-3.45c0-0.86-0.22-1.66-0.65-2.41c0.79-0.74,1.3-1.62,1.55-2.62l0.13-0.68\n\tc0.02-0.01,0.03-0.03,0.03-0.07c0-0.07-0.05-0.13-0.16-0.16l-0.56-0.17c-0.57-0.17-1.05-0.45-1.46-0.85\n\tc-0.4-0.4-0.69-0.81-0.86-1.25c-0.17-0.43-0.25-0.87-0.25-1.32c-0.01-0.24,0.02-0.51,0.08-0.79l0.14-0.58\n\tc0.03-0.09-0.02-0.16-0.14-0.22l-0.8-0.25c-0.42-0.12-0.86-0.19-1.31-0.19c-0.35,0-0.71,0.04-1.08,0.13s-0.76,0.22-1.17,0.4\n\tc-0.41,0.18-0.8,0.45-1.19,0.8c-0.38,0.35-0.72,0.75-1,1.22c-0.75-0.32-1.54-0.49-2.37-0.49c-1.41,0-2.67,0.44-3.76,1.31\n\ts-1.79,1.99-2.1,3.36c-1.11,0.26-2.02,0.83-2.74,1.73S4.1,15.76,4.1,16.91z M9.58,23.94c0.09,0.21,0.24,0.36,0.46,0.45\n\tc0.19,0.1,0.4,0.11,0.62,0.02c0.22-0.08,0.37-0.23,0.45-0.45c0.1-0.22,0.11-0.43,0.02-0.65c-0.08-0.21-0.23-0.36-0.45-0.44\n\tc-0.2-0.1-0.41-0.11-0.62-0.02c-0.21,0.09-0.37,0.24-0.47,0.46C9.5,23.48,9.49,23.69,9.58,23.94z M10.2,21.11\n\tc0,0.15,0.05,0.3,0.16,0.45s0.26,0.26,0.46,0.32c0.26,0.1,0.48,0.1,0.67,0c0.19-0.1,0.32-0.29,0.4-0.57l0.88-3.21\n\tc0.07-0.25,0.04-0.47-0.08-0.67c-0.12-0.2-0.3-0.32-0.54-0.37c-0.22-0.07-0.43-0.05-0.63,0.07c-0.2,0.11-0.33,0.28-0.4,0.51\n\tl-0.88,3.22c0,0.02-0.01,0.06-0.02,0.12C10.21,21.03,10.2,21.08,10.2,21.11z M12.07,26.71c0,0.12,0.02,0.22,0.06,0.29\n\tc0.09,0.22,0.24,0.37,0.45,0.45c0.09,0.05,0.2,0.07,0.33,0.07c0.06,0,0.16-0.02,0.3-0.06c0.23-0.08,0.39-0.23,0.48-0.45\n\tc0.1-0.22,0.1-0.44,0-0.66c-0.1-0.22-0.25-0.37-0.45-0.46c-0.2-0.09-0.4-0.09-0.61,0c-0.19,0.08-0.33,0.2-0.42,0.36\n\tC12.11,26.42,12.07,26.57,12.07,26.71z M12.81,24.06c0,0.38,0.21,0.64,0.64,0.78c0.09,0.03,0.17,0.05,0.23,0.05\n\tc0.11,0,0.23-0.03,0.35-0.08c0.23-0.08,0.39-0.27,0.47-0.57l1.65-6.12c0.06-0.24,0.04-0.45-0.07-0.65c-0.11-0.19-0.28-0.32-0.5-0.39\n\tc-0.23-0.07-0.45-0.05-0.65,0.07c-0.2,0.11-0.34,0.28-0.4,0.51l-1.68,6.17C12.82,23.92,12.81,24,12.81,24.06z M16.25,23.64\n\tc0,0.13,0.02,0.23,0.07,0.31c0.08,0.2,0.23,0.35,0.44,0.44c0.12,0.05,0.23,0.08,0.35,0.08c0.06,0,0.16-0.02,0.3-0.06\n\tc0.22-0.09,0.37-0.23,0.45-0.44c0.08-0.22,0.08-0.43,0-0.63c-0.08-0.2-0.22-0.35-0.42-0.45c-0.22-0.1-0.44-0.11-0.65-0.02\n\tc-0.22,0.08-0.37,0.24-0.47,0.46C16.27,23.41,16.25,23.51,16.25,23.64z M16.97,21.08c0,0.16,0.05,0.32,0.15,0.46\n\tc0.1,0.14,0.25,0.25,0.45,0.31c0.17,0.02,0.26,0.03,0.27,0.03c0.41,0,0.66-0.2,0.77-0.61l0.87-3.17c0.06-0.24,0.04-0.45-0.07-0.65\n\tc-0.11-0.19-0.28-0.32-0.5-0.39c-0.23-0.07-0.45-0.05-0.64,0.07c-0.2,0.11-0.33,0.28-0.4,0.51L17,20.81\n\tC16.98,20.9,16.97,20.99,16.97,21.08z M17.62,8.83c0.31-0.57,0.75-1.01,1.3-1.31c0.55-0.3,1.14-0.45,1.76-0.44\n\tc0.11,0,0.2,0.01,0.25,0.02v0.31c0,0.98,0.26,1.89,0.78,2.75c0.52,0.86,1.25,1.51,2.17,1.95c-0.19,0.44-0.44,0.79-0.75,1.07\n\tC22.25,12.39,21.17,12,19.88,12h-0.32C19.3,10.75,18.66,9.69,17.62,8.83z\" />",
};
#[cfg(feature = "WiNightAltLightning")]
const WI_NIGHT_ALT_LIGHTNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.12,16.91c0,1.33,0.46,2.48,1.39,3.43s2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.37c0-0.12-0.06-0.18-0.17-0.18\n\tc-0.87-0.07-1.6-0.41-2.19-1.04c-0.59-0.62-0.89-1.36-0.89-2.21c0-0.84,0.28-1.57,0.85-2.19c0.57-0.62,1.26-0.97,2.1-1.04l0.53-0.07\n\tc0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.85-1.09c1.09,0,2.04,0.36,2.85,1.09\n\tc0.81,0.72,1.27,1.63,1.39,2.72l0.07,0.58c0,0.12,0.06,0.18,0.18,0.18h1.61c0.91,0,1.68,0.32,2.32,0.95\n\tc0.64,0.63,0.96,1.39,0.96,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.32,0.97-2.19,1.04c-0.13,0-0.2,0.06-0.2,0.18v1.37\n\tc0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.38-1.51s1.38-2.11,1.38-3.45c0-0.89-0.23-1.72-0.68-2.48\n\tc0.8-0.72,1.32-1.58,1.55-2.58l0.15-0.72c0.01-0.01,0.01-0.03,0.01-0.07c0-0.07-0.05-0.13-0.16-0.16l-0.58-0.17\n\tc-0.57-0.16-1.05-0.44-1.45-0.82c-0.4-0.39-0.68-0.8-0.85-1.23c-0.17-0.43-0.25-0.87-0.25-1.32c0-0.24,0.03-0.51,0.09-0.79\n\tl0.14-0.62c0.03-0.09-0.02-0.17-0.14-0.22l-0.79-0.24c-0.44-0.11-0.85-0.16-1.25-0.16c-0.36,0-0.73,0.04-1.12,0.13\n\tc-0.38,0.09-0.78,0.22-1.19,0.41c-0.41,0.18-0.81,0.45-1.2,0.8c-0.39,0.35-0.72,0.75-1,1.22c-0.71-0.3-1.48-0.45-2.33-0.45\n\tc-1.41,0-2.66,0.44-3.75,1.31c-1.09,0.87-1.79,1.99-2.1,3.35c-1.1,0.26-2.01,0.84-2.73,1.74C4.48,14.74,4.12,15.76,4.12,16.91z\n\t M11.79,21.56c-0.05,0.14,0,0.22,0.14,0.22h2.16l-1.31,4.14h0.3l4.17-5.59c0.04-0.04,0.05-0.09,0.03-0.14\n\tc-0.02-0.05-0.06-0.07-0.13-0.07h-2.2l2.31-4.21c0.07-0.14,0.02-0.22-0.14-0.22h-2.94c-0.08,0-0.15,0.05-0.22,0.14L11.79,21.56z\n\t M17.6,8.81c0.33-0.57,0.77-1,1.33-1.3c0.55-0.3,1.13-0.45,1.72-0.45c0.13,0,0.22,0.01,0.27,0.02v0.31c0,0.96,0.26,1.87,0.78,2.73\n\tc0.52,0.86,1.24,1.51,2.17,1.96c-0.16,0.37-0.41,0.73-0.75,1.07c-0.92-0.79-1.99-1.18-3.22-1.18h-0.32\n\tC19.29,10.71,18.63,9.66,17.6,8.81z\" />",
};
#[cfg(feature = "WiNightAltPartlyCloudy")]
const WI_NIGHT_ALT_PARTLY_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M6.77,19.65c0-0.79,0.23-1.48,0.68-2.09c0.45-0.61,1.06-1.03,1.81-1.27c0.32-1.09,0.98-1.92,1.99-2.49v-0.35\n\tc0-1.46,0.46-2.74,1.38-3.85s2.09-1.8,3.5-2.06c0.36-0.06,0.72-0.09,1.08-0.09h0.03c0.21,0,0.44,0.02,0.7,0.05\n\tc0.26,0.02,0.5,0.06,0.73,0.11l0.91,0.28c0.13,0.07,0.18,0.16,0.16,0.26l-0.13,0.7C19.54,9.18,19.5,9.5,19.5,9.82\n\tc0,0.35,0.05,0.71,0.16,1.07c0.11,0.37,0.27,0.72,0.5,1.08s0.52,0.68,0.91,0.97c0.38,0.29,0.83,0.51,1.33,0.66l0.71,0.21\n\tc0.11,0.03,0.17,0.08,0.17,0.18c0,0.04,0,0.06-0.01,0.07l-0.18,0.68c-0.06,0.24-0.13,0.49-0.22,0.73c-0.15,0.44-0.38,0.89-0.7,1.37\n\tc0-0.01,0-0.01-0.01-0.01c-0.44,0.63-0.98,1.16-1.64,1.58c0.14,0.34,0.21,0.75,0.21,1.24c0,0.99-0.35,1.83-1.04,2.53\n\tc-0.69,0.7-1.52,1.05-2.49,1.05h-6.85c-0.97,0-1.81-0.35-2.52-1.06C7.13,21.46,6.77,20.62,6.77,19.65z M8.75,19.65\n\tc0,0.45,0.15,0.83,0.46,1.15s0.69,0.47,1.14,0.47h6.85c0.43,0,0.8-0.16,1.12-0.48c0.32-0.32,0.47-0.7,0.47-1.14\n\tc0-0.43-0.16-0.8-0.47-1.12s-0.69-0.47-1.12-0.47H15.9c-0.11,0-0.19-0.07-0.25-0.23l-0.08-0.64c-0.07-0.58-0.32-1.06-0.75-1.44\n\ts-0.93-0.58-1.51-0.58c-0.57,0-1.06,0.19-1.48,0.58c-0.42,0.39-0.66,0.87-0.73,1.44l-0.1,0.55c0,0.15-0.06,0.22-0.19,0.22\n\tl-0.63,0.08c-0.41,0.04-0.75,0.21-1.02,0.51C8.89,18.87,8.75,19.23,8.75,19.65z M13.18,13.25h0.12c0.93,0,1.75,0.26,2.49,0.78\n\tc0.73,0.52,1.25,1.22,1.54,2.1c0.77,0,1.45,0.24,2.03,0.72c0.69-0.43,1.2-1.02,1.53-1.75c-1.04-0.52-1.85-1.27-2.43-2.25\n\ts-0.88-2.01-0.88-3.11V9.39c-0.03,0-0.07,0-0.12,0c-0.05,0-0.09,0-0.12,0c-0.61,0-1.2,0.13-1.77,0.39\n\tc-0.57,0.26-1.05,0.64-1.44,1.12l-0.03-0.02C13.55,11.56,13.25,12.35,13.18,13.25z\" />",
};
#[cfg(feature = "WiNightAltRain")]
const WI_NIGHT_ALT_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.07,16.9c0,1.33,0.47,2.48,1.4,3.44s2.07,1.47,3.4,1.53c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.86-0.05-1.59-0.39-2.19-1.03c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.85-2.17c0.57-0.62,1.27-0.97,2.1-1.07l0.53-0.04\n\tc0.13,0,0.2-0.06,0.2-0.17l0.07-0.54c0.11-1.08,0.57-1.99,1.38-2.72c0.81-0.73,1.77-1.1,2.86-1.1c1.09,0,2.04,0.37,2.86,1.1\n\tc0.82,0.73,1.28,1.64,1.4,2.72l0.08,0.57c0,0.12,0.06,0.18,0.17,0.18h1.62c0.89,0,1.67,0.32,2.32,0.96c0.65,0.64,0.98,1.4,0.98,2.28\n\tc0,0.87-0.3,1.62-0.9,2.26c-0.6,0.64-1.33,0.98-2.19,1.03c-0.14,0-0.21,0.06-0.21,0.17v1.34c0,0.11,0.07,0.17,0.21,0.17\n\tc1.33-0.04,2.46-0.55,3.38-1.51c0.93-0.97,1.39-2.12,1.39-3.45c0-0.88-0.23-1.7-0.68-2.46c0.81-0.73,1.33-1.6,1.58-2.62l0.14-0.72\n\tc0.01-0.01,0.02-0.03,0.02-0.07c0-0.07-0.05-0.13-0.16-0.16l-0.56-0.18c-0.57-0.16-1.06-0.44-1.46-0.83\n\tc-0.41-0.39-0.7-0.8-0.87-1.23c-0.17-0.43-0.26-0.86-0.26-1.28c-0.02-0.22,0.01-0.5,0.08-0.82l0.14-0.61c0.04-0.1,0-0.18-0.14-0.24\n\tl-0.79-0.24c-0.45-0.1-0.87-0.15-1.27-0.15c-0.38,0-0.76,0.04-1.14,0.13c-0.39,0.09-0.79,0.22-1.2,0.41\n\tc-0.41,0.18-0.81,0.45-1.2,0.8c-0.39,0.35-0.72,0.75-1,1.22c-0.82-0.3-1.6-0.45-2.33-0.45c-1.41,0-2.67,0.44-3.76,1.32\n\ts-1.8,2-2.11,3.37c-1.11,0.26-2.02,0.83-2.74,1.73C4.43,14.72,4.07,15.75,4.07,16.9z M9.63,23.74c0,0.17,0.05,0.33,0.16,0.49\n\tc0.11,0.16,0.27,0.27,0.49,0.33c0.23,0.07,0.45,0.05,0.64-0.04c0.2-0.1,0.33-0.28,0.4-0.56l1.43-5.87c0.06-0.25,0.03-0.48-0.08-0.67\n\tc-0.12-0.2-0.29-0.32-0.52-0.37c-0.22-0.07-0.43-0.05-0.63,0.07c-0.2,0.11-0.34,0.28-0.41,0.51l-1.44,5.9\n\tc0,0.01-0.01,0.04-0.02,0.09C9.64,23.67,9.63,23.71,9.63,23.74z M12.24,26.81c0,0.16,0.05,0.31,0.15,0.46\n\tc0.1,0.15,0.25,0.25,0.45,0.31c0.11,0.03,0.19,0.04,0.24,0.04c0.44,0,0.71-0.2,0.82-0.59l2.25-8.93c0.06-0.24,0.04-0.46-0.07-0.65\n\tc-0.11-0.19-0.28-0.32-0.5-0.39c-0.23-0.07-0.45-0.05-0.66,0.07s-0.34,0.28-0.39,0.5l-2.26,8.92c0,0.01,0,0.05-0.01,0.12\n\tC12.24,26.73,12.24,26.78,12.24,26.81z M16.4,23.82c0,0.36,0.21,0.6,0.63,0.74c0.14,0.04,0.24,0.06,0.3,0.06\n\tc0.11,0,0.23-0.02,0.35-0.07c0.21-0.08,0.34-0.28,0.39-0.58l1.43-5.87c0.06-0.24,0.04-0.45-0.08-0.65\n\tc-0.11-0.19-0.28-0.32-0.51-0.39c-0.23-0.07-0.45-0.05-0.66,0.07c-0.21,0.11-0.33,0.28-0.38,0.51l-1.43,5.9\n\tC16.42,23.7,16.4,23.8,16.4,23.82z M17.58,8.77c0.32-0.58,0.75-1.02,1.31-1.33c0.55-0.3,1.14-0.45,1.76-0.44\n\tc0.12,0,0.21,0,0.27,0.01v0.3c-0.01,0.97,0.24,1.88,0.77,2.75c0.52,0.86,1.26,1.52,2.21,1.97c-0.22,0.46-0.49,0.81-0.79,1.07\n\tc-0.92-0.76-1.99-1.13-3.23-1.13h-0.31C19.3,10.7,18.64,9.64,17.58,8.77z\" />",
};
#[cfg(feature = "WiNightAltRainMix")]
const WI_NIGHT_ALT_RAIN_MIX: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.07,16.93c0,1.33,0.47,2.47,1.4,3.43s2.07,1.47,3.4,1.51c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.85-0.04-1.58-0.39-2.18-1.03s-0.91-1.39-0.91-2.23c0-0.85,0.28-1.59,0.85-2.21c0.57-0.62,1.27-0.97,2.1-1.04l0.53-0.07\n\tc0.13,0,0.2-0.06,0.2-0.18l0.07-0.51c0.11-1.1,0.56-2.02,1.37-2.75c0.81-0.73,1.76-1.1,2.86-1.1c1.09,0,2.04,0.37,2.86,1.1\n\tc0.82,0.73,1.29,1.64,1.4,2.72l0.07,0.57c0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.68,0.32,2.33,0.97\n\tc0.65,0.64,0.97,1.41,0.97,2.31c0,0.55-0.14,1.07-0.41,1.56s-0.65,0.89-1.13,1.2c-0.48,0.31-1,0.48-1.56,0.51\n\tc-0.13,0-0.2,0.06-0.2,0.17v1.34c0,0.11,0.07,0.17,0.2,0.17c0.88-0.02,1.69-0.26,2.42-0.71c0.73-0.45,1.31-1.05,1.73-1.8\n\tc0.42-0.75,0.63-1.56,0.63-2.43c0-0.88-0.23-1.72-0.68-2.51c0.83-0.74,1.36-1.62,1.58-2.62l0.14-0.68c0.02-0.02,0.03-0.04,0.03-0.07\n\tc0-0.06-0.06-0.11-0.17-0.16l-0.55-0.18c-0.57-0.17-1.07-0.45-1.47-0.85c-0.41-0.4-0.7-0.81-0.87-1.25\n\tc-0.17-0.43-0.26-0.86-0.26-1.29c-0.02-0.21,0.01-0.49,0.09-0.82l0.13-0.58c0.04-0.1,0-0.18-0.13-0.23l-0.8-0.24\n\tc-0.41-0.11-0.84-0.17-1.29-0.17c-0.36,0-0.74,0.04-1.12,0.13c-0.38,0.09-0.78,0.22-1.19,0.41s-0.81,0.46-1.2,0.81\n\tc-0.39,0.35-0.72,0.76-1,1.23c-0.81-0.31-1.6-0.46-2.35-0.46c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.37\n\tc-1.12,0.29-2.04,0.88-2.75,1.77C4.42,14.74,4.07,15.77,4.07,16.93z M9.48,23.98c0,0.17,0.05,0.34,0.16,0.51\n\tc0.11,0.17,0.27,0.28,0.47,0.35c0.23,0.07,0.44,0.06,0.64-0.04s0.32-0.28,0.39-0.56l0.14-0.61c0.05-0.23,0.02-0.44-0.09-0.63\n\tc-0.11-0.2-0.28-0.33-0.52-0.4c-0.23-0.07-0.44-0.04-0.64,0.08s-0.34,0.3-0.4,0.53L9.5,23.79C9.48,23.83,9.48,23.89,9.48,23.98z\n\t M10.24,21.08c0,0.21,0.08,0.4,0.25,0.57c0.16,0.17,0.34,0.25,0.56,0.25c0.24,0,0.44-0.08,0.6-0.24c0.16-0.16,0.24-0.35,0.24-0.59\n\tc0-0.23-0.08-0.43-0.24-0.59c-0.16-0.16-0.36-0.24-0.6-0.24c-0.23,0-0.43,0.08-0.58,0.23S10.24,20.85,10.24,21.08z M10.85,18.81\n\tc-0.01,0.16,0.03,0.31,0.14,0.45c0.1,0.15,0.26,0.25,0.48,0.32c0.21,0.06,0.41,0.04,0.62-0.07c0.21-0.11,0.34-0.28,0.41-0.51\n\tl0.28-0.9c0.07-0.24,0.05-0.46-0.07-0.65c-0.12-0.19-0.3-0.32-0.54-0.39c-0.22-0.07-0.43-0.05-0.63,0.07\n\tc-0.2,0.11-0.34,0.28-0.41,0.5l-0.24,0.92c0,0.02-0.01,0.06-0.02,0.12C10.85,18.72,10.85,18.77,10.85,18.81z M12.01,27.1\n\tc0,0.18,0.05,0.34,0.15,0.5c0.1,0.16,0.26,0.27,0.48,0.33c0.08,0.02,0.17,0.03,0.25,0.03c0.43,0,0.69-0.2,0.79-0.61l0.14-0.59\n\tc0.06-0.26,0.03-0.48-0.08-0.68s-0.29-0.32-0.52-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.14,0.59\n\tC12.02,26.97,12.01,27.04,12.01,27.1z M12.79,24.2c0,0.22,0.08,0.41,0.25,0.58c0.16,0.16,0.35,0.24,0.57,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58s-0.35-0.23-0.59-0.23s-0.43,0.08-0.59,0.23\n\tC12.87,23.77,12.79,23.97,12.79,24.2z M13.42,21.93c-0.01,0.15,0.03,0.31,0.13,0.47s0.25,0.26,0.45,0.3\n\tc0.23,0.06,0.44,0.04,0.64-0.06c0.19-0.1,0.33-0.29,0.41-0.56l0.27-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4\n\tc-0.22-0.07-0.43-0.04-0.64,0.08s-0.34,0.3-0.41,0.53l-0.22,0.9C13.43,21.74,13.42,21.83,13.42,21.93z M16.17,24.08\n\tc0,0.16,0.05,0.32,0.15,0.48s0.26,0.27,0.46,0.33c0.03,0,0.08,0.01,0.14,0.02c0.06,0.01,0.1,0.02,0.14,0.02\n\tc0.41,0,0.66-0.22,0.76-0.66l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.11-0.21-0.28-0.34-0.51-0.41\n\tc-0.25-0.06-0.48-0.04-0.68,0.08s-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.02-0.01,0.07-0.02,0.12C16.18,24,16.17,24.04,16.17,24.08z\n\t M16.91,21.12c0,0.22,0.08,0.42,0.25,0.57c0.15,0.16,0.34,0.24,0.57,0.24c0.24,0,0.43-0.08,0.59-0.23s0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59s-0.35-0.23-0.59-0.23c-0.24,0-0.43,0.08-0.59,0.23S16.91,20.88,16.91,21.12z M17.52,18.81\n\tc0,0.17,0.05,0.33,0.16,0.48s0.27,0.26,0.49,0.32c0.02,0,0.06,0.01,0.12,0.02c0.06,0.01,0.11,0.02,0.14,0.02\n\tc0.1,0,0.22-0.03,0.36-0.09c0.21-0.11,0.35-0.29,0.41-0.52l0.24-0.9c0.06-0.23,0.04-0.44-0.08-0.63c-0.11-0.2-0.28-0.33-0.51-0.4\n\tc-0.23-0.07-0.44-0.05-0.64,0.06s-0.32,0.27-0.39,0.51l-0.28,0.91c0,0.02-0.01,0.06-0.02,0.12C17.53,18.74,17.52,18.78,17.52,18.81z\n\t M17.59,8.76c0.32-0.58,0.76-1.02,1.31-1.34c0.56-0.32,1.13-0.47,1.73-0.46c0.09,0,0.19,0.01,0.3,0.03V7.3\n\tc-0.01,0.98,0.25,1.9,0.77,2.76c0.53,0.86,1.27,1.5,2.22,1.94c-0.19,0.41-0.46,0.78-0.8,1.11c-0.92-0.76-2-1.14-3.23-1.14h-0.31\n\tC19.27,10.66,18.61,9.59,17.59,8.76z\" />",
};
#[cfg(feature = "WiNightAltRainWind")]
const WI_NIGHT_ALT_RAIN_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.06,16.93c0,1.12,0.33,2.12,1,3c0.67,0.88,1.52,1.47,2.57,1.77c0.09,0.02,0.17-0.01,0.24-0.08L9,20.22\n\tc-0.88,0-1.63-0.32-2.27-0.97c-0.64-0.65-0.96-1.42-0.96-2.32c0-0.84,0.28-1.56,0.84-2.17s1.27-0.95,2.11-1.03l0.5-0.07\n\tc0.12,0,0.19-0.06,0.19-0.19l0.08-0.53c0.12-1.09,0.59-2,1.41-2.73c0.81-0.73,1.77-1.1,2.86-1.1c1.09,0,2.04,0.37,2.86,1.1\n\ts1.29,1.64,1.41,2.72l0.07,0.58c0,0.11,0.06,0.17,0.18,0.17h1.62c0.88,0,1.64,0.32,2.28,0.96s0.96,1.4,0.96,2.28\n\tc0,0.85-0.28,1.59-0.84,2.22s-1.25,0.98-2.07,1.05c-0.45,0.06-0.74,0.15-0.86,0.28l-2.33,2.91c-0.16,0.17-0.22,0.38-0.19,0.63\n\tc0.02,0.24,0.13,0.43,0.31,0.59c0.18,0.16,0.37,0.23,0.57,0.23c0.23,0,0.44-0.12,0.64-0.38l2.04-2.59c0.62-0.06,1.2-0.24,1.76-0.52\n\tc0.55-0.28,1.03-0.65,1.42-1.08c0.39-0.44,0.71-0.95,0.94-1.53c0.23-0.58,0.35-1.18,0.35-1.81c0-0.87-0.23-1.68-0.68-2.44\n\tc0.81-0.74,1.34-1.61,1.58-2.62v-0.09l0.2-0.77l-0.76-0.26c-0.57-0.17-1.06-0.45-1.47-0.83s-0.69-0.8-0.86-1.23\n\tc-0.17-0.43-0.26-0.87-0.26-1.31c0-0.26,0.03-0.52,0.08-0.8l0.19-0.78l-0.83-0.23c-0.01,0-0.02,0-0.03-0.01s-0.02-0.02-0.04-0.02\n\ts-0.03-0.01-0.04-0.02C21.91,5.5,21.9,5.49,21.9,5.49c-0.44-0.11-0.85-0.16-1.25-0.16c-0.38,0.01-0.76,0.05-1.15,0.14\n\ts-0.78,0.22-1.2,0.41c-0.42,0.19-0.82,0.46-1.2,0.81s-0.72,0.76-1,1.24c-0.75-0.33-1.53-0.49-2.34-0.49c-1.41,0-2.67,0.44-3.76,1.31\n\ts-1.8,1.99-2.11,3.36c-1.13,0.27-2.05,0.86-2.76,1.75S4.06,15.77,4.06,16.93z M7.77,24.92c0,0.13,0.02,0.23,0.07,0.31\n\tc0.09,0.22,0.23,0.37,0.43,0.46c0.22,0.1,0.44,0.11,0.67,0.03c0.23-0.08,0.38-0.23,0.46-0.44c0.1-0.22,0.1-0.44,0.01-0.67\n\tc-0.09-0.23-0.24-0.38-0.45-0.45c-0.22-0.1-0.44-0.11-0.66-0.02c-0.22,0.08-0.37,0.24-0.45,0.45C7.79,24.67,7.77,24.79,7.77,24.92z\n\t M9.61,22.47v0.11c0.02,0.23,0.13,0.41,0.33,0.55c0.13,0.15,0.31,0.22,0.54,0.22c0.23-0.01,0.45-0.11,0.66-0.32l2.33-2.92\n\tc0.14-0.17,0.19-0.38,0.17-0.62c-0.03-0.24-0.12-0.43-0.3-0.58c-0.18-0.14-0.38-0.2-0.63-0.18c-0.24,0.02-0.43,0.14-0.57,0.34\n\tl-2.32,2.86C9.68,22.09,9.61,22.27,9.61,22.47z M10.19,27.68c0.09,0.21,0.24,0.36,0.46,0.45c0.11,0.05,0.22,0.08,0.33,0.08\n\tc0.06,0,0.16-0.02,0.3-0.06c0.21-0.09,0.36-0.23,0.44-0.44c0.08-0.22,0.08-0.43,0.01-0.65c-0.07-0.21-0.22-0.37-0.44-0.48\n\tc-0.22-0.08-0.43-0.08-0.63,0s-0.35,0.23-0.45,0.44C10.1,27.22,10.09,27.43,10.19,27.68z M11.78,25.02v0.08\n\tc0.02,0.22,0.13,0.42,0.32,0.58c0.19,0.16,0.38,0.24,0.56,0.24c0.22,0,0.42-0.11,0.6-0.34l4.31-5.36c0.14-0.17,0.21-0.38,0.19-0.62\n\tc-0.02-0.24-0.12-0.44-0.29-0.58c-0.2-0.14-0.42-0.2-0.66-0.18c-0.24,0.02-0.43,0.12-0.57,0.3l-4.27,5.36\n\tC11.84,24.65,11.78,24.83,11.78,25.02z M15.29,26.13c0,0.11,0.02,0.22,0.07,0.33c0.08,0.23,0.24,0.38,0.47,0.47\n\tc0.23,0.09,0.43,0.09,0.61,0.02c0.22-0.09,0.37-0.24,0.46-0.46c0.1-0.22,0.11-0.43,0.03-0.64c-0.08-0.21-0.23-0.36-0.45-0.46\n\tc-0.22-0.08-0.44-0.08-0.65,0c-0.22,0.08-0.37,0.22-0.47,0.42C15.31,25.92,15.29,26.03,15.29,26.13z M17.57,8.81\n\tc0.31-0.57,0.75-1.01,1.3-1.32c0.55-0.3,1.14-0.45,1.76-0.44c0.12,0,0.21,0,0.26,0.01v0.3c0,0.97,0.27,1.89,0.8,2.75\n\tc0.53,0.87,1.26,1.52,2.19,1.96c-0.25,0.47-0.51,0.84-0.79,1.12c-0.89-0.79-1.96-1.18-3.22-1.18h-0.32\n\tC19.26,10.74,18.6,9.67,17.57,8.81z\" />",
};
#[cfg(feature = "WiNightAltShowers")]
const WI_NIGHT_ALT_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.07,16.91c0,1.33,0.46,2.48,1.39,3.43s2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.37c0-0.12-0.06-0.18-0.17-0.18\n\tc-0.87-0.07-1.6-0.41-2.19-1.04c-0.59-0.62-0.89-1.36-0.89-2.21c0-0.83,0.28-1.54,0.84-2.16s1.26-0.97,2.1-1.07l0.53-0.07\n\tc0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.85-1.09c1.09,0,2.04,0.36,2.85,1.09\n\tc0.81,0.72,1.28,1.63,1.39,2.72l0.08,0.58c0,0.12,0.06,0.18,0.18,0.18h1.61c0.9,0,1.67,0.32,2.32,0.95\n\tc0.64,0.63,0.97,1.39,0.97,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.33,0.97-2.19,1.04c-0.13,0-0.2,0.06-0.2,0.18v1.37\n\tc0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.38-1.51c0.92-0.96,1.38-2.11,1.38-3.45c0-0.87-0.22-1.68-0.65-2.43\n\tc0.81-0.73,1.34-1.6,1.58-2.62v-0.13l0.19-0.79l-0.76-0.21c-0.81-0.24-1.44-0.7-1.89-1.35c-0.45-0.66-0.67-1.34-0.67-2.03\n\tc0-0.26,0.03-0.52,0.08-0.78l0.2-0.8l-0.85-0.25L21.9,5.49c-0.47-0.09-0.88-0.14-1.25-0.14c-0.38,0-0.76,0.04-1.14,0.13\n\tc-0.39,0.09-0.79,0.22-1.2,0.41c-0.42,0.19-0.82,0.45-1.2,0.8c-0.38,0.35-0.72,0.76-1,1.23c-0.74-0.33-1.53-0.49-2.36-0.49\n\tc-1.41,0-2.66,0.44-3.75,1.31s-1.77,1.99-2.07,3.36c-1.12,0.26-2.05,0.83-2.77,1.72C4.43,14.73,4.07,15.76,4.07,16.91z M9.47,23.68\n\tc0,0.15,0.05,0.3,0.15,0.45c0.1,0.15,0.25,0.26,0.45,0.33c0.22,0.07,0.43,0.06,0.64-0.05s0.34-0.28,0.41-0.51l0.28-1.06\n\tc0.07-0.21,0.05-0.41-0.07-0.62c-0.12-0.21-0.29-0.34-0.51-0.41c-0.23-0.06-0.45-0.03-0.65,0.08s-0.34,0.3-0.42,0.53l-0.23,0.99\n\tC9.49,23.57,9.47,23.66,9.47,23.68z M10.77,18.95c0,0.11,0.03,0.23,0.1,0.36c0.07,0.17,0.25,0.3,0.53,0.38\n\tc0.24,0.06,0.46,0.04,0.66-0.06c0.19-0.1,0.33-0.28,0.4-0.52l0.28-1.03c0.07-0.23,0.05-0.45-0.07-0.64\n\tc-0.12-0.2-0.29-0.33-0.51-0.39c-0.24-0.06-0.47-0.04-0.67,0.07c-0.2,0.11-0.33,0.28-0.4,0.52l-0.27,1.01\n\tC10.79,18.78,10.77,18.88,10.77,18.95z M12.02,26.8c0,0.17,0.05,0.33,0.15,0.49c0.1,0.16,0.25,0.27,0.45,0.33\n\tc0.11,0.03,0.18,0.05,0.23,0.05c0.09,0,0.21-0.03,0.38-0.1c0.2-0.08,0.34-0.27,0.43-0.55l0.3-1.05c0.07-0.21,0.05-0.42-0.07-0.63\n\tc-0.12-0.21-0.29-0.34-0.51-0.41c-0.24-0.06-0.47-0.04-0.67,0.08c-0.2,0.12-0.34,0.29-0.41,0.53l-0.25,1.01\n\tC12.03,26.63,12.02,26.72,12.02,26.8z M13.35,22.03c0,0.15,0.05,0.3,0.15,0.45s0.25,0.26,0.46,0.33c0.22,0.07,0.44,0.05,0.64-0.06\n\tc0.2-0.11,0.33-0.28,0.4-0.52l0.27-1.04c0.07-0.21,0.05-0.42-0.06-0.62c-0.11-0.2-0.27-0.34-0.49-0.41\n\tc-0.24-0.06-0.47-0.03-0.68,0.08s-0.35,0.3-0.42,0.53l-0.24,1L13.35,22.03z M16.16,23.79c0,0.38,0.21,0.62,0.64,0.75\n\tc0.09,0.02,0.17,0.03,0.24,0.03c0.15,0,0.27-0.02,0.37-0.07c0.21-0.08,0.36-0.27,0.44-0.57l0.27-1.02c0.06-0.25,0.04-0.47-0.08-0.67\n\ts-0.29-0.32-0.53-0.37c-0.23-0.07-0.45-0.05-0.64,0.07s-0.33,0.29-0.4,0.51l-0.27,1.04c0,0.02-0.01,0.07-0.02,0.15\n\tC16.16,23.71,16.16,23.76,16.16,23.79z M17.55,18.98c0,0.16,0.05,0.31,0.15,0.46c0.1,0.15,0.26,0.26,0.46,0.32\n\tc0.14,0.03,0.22,0.05,0.23,0.05c0.09,0,0.21-0.03,0.38-0.1c0.21-0.08,0.35-0.27,0.44-0.55l0.28-1.04c0.06-0.22,0.03-0.43-0.08-0.63\n\ts-0.3-0.33-0.53-0.4c-0.22-0.07-0.43-0.05-0.63,0.07s-0.33,0.29-0.4,0.52l-0.26,1.06C17.56,18.83,17.55,18.92,17.55,18.98z\n\t M17.58,8.81c0.32-0.56,0.76-1,1.33-1.31c0.57-0.31,1.17-0.47,1.81-0.47h0.21c-0.01,0.09-0.01,0.21-0.01,0.38\n\tc0,0.95,0.26,1.85,0.78,2.71c0.52,0.86,1.25,1.51,2.17,1.96c-0.22,0.43-0.48,0.8-0.78,1.1c-0.93-0.81-2.02-1.21-3.25-1.21h-0.32\n\tC19.27,10.78,18.63,9.73,17.58,8.81z\" />",
};
#[cfg(feature = "WiNightAltSleet")]
const WI_NIGHT_ALT_SLEET: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.09,17.13v-0.37c0.04-1.12,0.42-2.11,1.13-2.97c0.71-0.86,1.61-1.42,2.68-1.67c0.31-1.36,1.02-2.48,2.11-3.36\n\ts2.35-1.31,3.76-1.31c0.74,0,1.53,0.15,2.38,0.46c0.28-0.46,0.61-0.86,0.99-1.2c0.38-0.34,0.77-0.6,1.18-0.79\n\tc0.41-0.18,0.8-0.32,1.19-0.4c0.38-0.08,0.76-0.12,1.13-0.12c0.39,0,0.8,0.04,1.24,0.13l0.8,0.25c0.12,0.06,0.17,0.13,0.14,0.23\n\tL22.7,6.63c-0.07,0.29-0.1,0.57-0.1,0.84c0,0.31,0.05,0.62,0.15,0.95c0.1,0.32,0.24,0.64,0.44,0.95s0.46,0.59,0.8,0.85\n\tc0.34,0.26,0.72,0.45,1.16,0.58l0.62,0.17c0.1,0.03,0.15,0.08,0.15,0.16c0,0.04,0,0.06-0.01,0.07l-0.19,0.68\n\tc-0.24,1.01-0.75,1.89-1.51,2.62c0.44,0.76,0.66,1.58,0.66,2.45c0,1.34-0.46,2.49-1.39,3.45c-0.93,0.97-2.06,1.47-3.39,1.51\n\tc-0.12,0-0.19-0.06-0.19-0.19v-1.36c0-0.12,0.06-0.18,0.19-0.18c0.87-0.06,1.6-0.41,2.2-1.03c0.6-0.62,0.9-1.36,0.9-2.21\n\tc0-0.89-0.32-1.66-0.97-2.29s-1.42-0.95-2.33-0.95h-1.62c-0.11,0-0.17-0.06-0.17-0.18l-0.07-0.58c-0.11-1.07-0.58-1.98-1.4-2.71\n\tc-0.82-0.73-1.77-1.1-2.86-1.1c-1.09,0-2.05,0.37-2.85,1.1c-0.81,0.73-1.27,1.64-1.37,2.72l-0.07,0.52c0,0.11-0.07,0.17-0.2,0.17\n\tl-0.53,0.07c-0.84,0.1-1.54,0.46-2.1,1.07c-0.57,0.62-0.85,1.34-0.85,2.17v0.02l0.01,0.01h0.02C5.83,17.5,5.95,18,6.19,18.46\n\tc0.24,0.46,0.56,0.84,0.97,1.14h0.01l0.01,0.01l0.01,0.01c0.22,0.16,0.42,0.27,0.62,0.35c0.34,0.15,0.69,0.23,1.07,0.24\n\tc0.11,0,0.17,0.06,0.17,0.17v1.34c0,0.12-0.06,0.18-0.17,0.18c-0.93-0.04-1.78-0.32-2.55-0.82H6.32c-0.66-0.43-1.19-0.99-1.59-1.69\n\tC4.33,18.69,4.12,17.94,4.09,17.13z M9.59,24.1c0-0.03,0.01-0.07,0.02-0.13s0.02-0.09,0.02-0.12l0.09-0.59\n\tc0.07-0.24,0.2-0.41,0.41-0.53s0.43-0.14,0.68-0.08c0.23,0.07,0.39,0.21,0.51,0.41c0.11,0.21,0.13,0.42,0.07,0.63l-0.14,0.6\n\tc-0.1,0.43-0.35,0.65-0.76,0.65c-0.03,0-0.08,0-0.15-0.01c-0.07-0.01-0.11-0.01-0.13-0.01c-0.21-0.06-0.36-0.17-0.46-0.33\n\tC9.64,24.43,9.59,24.27,9.59,24.1z M10.33,21.16c0-0.24,0.08-0.43,0.23-0.59s0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23\n\ts0.23,0.35,0.23,0.59c0,0.23-0.08,0.43-0.23,0.58s-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.57-0.25\n\tC10.42,21.57,10.33,21.38,10.33,21.16z M11.97,27.16c0-0.04,0.01-0.11,0.04-0.22l0.13-0.59c0.07-0.23,0.21-0.4,0.41-0.51\n\tc0.21-0.12,0.42-0.14,0.63-0.07c0.23,0.04,0.41,0.17,0.53,0.37c0.12,0.2,0.15,0.43,0.08,0.68l-0.13,0.59\n\tc-0.1,0.41-0.37,0.61-0.8,0.61c-0.07,0-0.16-0.01-0.24-0.03c-0.22-0.06-0.38-0.17-0.49-0.33C12.03,27.49,11.97,27.33,11.97,27.16z\n\t M12.76,24.25c0-0.23,0.08-0.42,0.23-0.58c0.16-0.16,0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23c0.16,0.16,0.23,0.35,0.23,0.58\n\tc0,0.24-0.08,0.43-0.23,0.59c-0.16,0.16-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.58-0.24C12.84,24.67,12.76,24.48,12.76,24.25z\n\t M13.38,21.98c0-0.09,0.01-0.18,0.03-0.27l0.23-0.9c0.07-0.23,0.21-0.41,0.41-0.53c0.21-0.12,0.42-0.15,0.64-0.08\n\tc0.24,0.07,0.41,0.2,0.53,0.4c0.12,0.2,0.14,0.41,0.07,0.63l-0.26,0.9c-0.08,0.28-0.22,0.46-0.41,0.56\n\tc-0.19,0.1-0.41,0.12-0.64,0.06c-0.2-0.04-0.35-0.14-0.45-0.3C13.41,22.29,13.37,22.13,13.38,21.98z M16.14,24.12\n\tc0-0.03,0-0.08,0.01-0.13s0.01-0.09,0.01-0.11l0.09-0.59c0.07-0.24,0.2-0.41,0.41-0.53s0.43-0.14,0.68-0.08\n\tc0.23,0.07,0.4,0.21,0.51,0.41c0.12,0.21,0.14,0.42,0.07,0.63l-0.14,0.6c-0.1,0.44-0.35,0.66-0.76,0.66c-0.03,0-0.08-0.01-0.14-0.02\n\tc-0.06-0.01-0.11-0.02-0.14-0.02c-0.2-0.06-0.35-0.17-0.45-0.33S16.14,24.29,16.14,24.12z M16.88,21.18c0-0.24,0.08-0.43,0.23-0.59\n\ts0.35-0.23,0.59-0.23c0.24,0,0.43,0.08,0.59,0.23c0.16,0.16,0.23,0.35,0.23,0.59c0,0.23-0.08,0.42-0.23,0.58\n\tc-0.16,0.16-0.35,0.23-0.59,0.23c-0.24,0-0.43-0.08-0.58-0.24C16.97,21.6,16.88,21.41,16.88,21.18z M17.59,8.82\n\tc1.04,0.85,1.7,1.9,1.98,3.16h0.33c1.23,0,2.3,0.39,3.22,1.18c0.34-0.31,0.59-0.65,0.76-1.04c-0.62-0.3-1.15-0.7-1.61-1.21\n\tc-0.45-0.51-0.79-1.06-1.02-1.66c-0.23-0.6-0.34-1.22-0.34-1.86V7.09h-0.22c-0.62,0-1.21,0.15-1.77,0.45\n\tC18.36,7.84,17.92,8.27,17.59,8.82z\" />",
};
#[cfg(feature = "WiNightAltSleetStorm")]
const WI_NIGHT_ALT_SLEET_STORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.09,16.89c0,1.11,0.33,2.1,0.99,2.97c0.66,0.87,1.52,1.47,2.58,1.79l-0.65,1.7c-0.04,0.14,0,0.21,0.14,0.21h2.12\n\tl-1.29,4.18h0.28l4.23-5.62c0.04-0.04,0.04-0.09,0.02-0.14c-0.03-0.05-0.07-0.07-0.14-0.07h-2.18l2.47-4.64\n\tc0.07-0.14,0.03-0.22-0.13-0.22H9.57c-0.09,0-0.16,0.05-0.22,0.15l-1.07,2.88c-0.71-0.18-1.3-0.57-1.78-1.17s-0.71-1.27-0.71-2.01\n\tc0-0.83,0.28-1.55,0.85-2.17c0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.07c0.13,0,0.2-0.06,0.2-0.18l0.07-0.51\n\tc0.11-1.08,0.56-1.99,1.37-2.72c0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.28,1.64,1.4,2.71l0.07,0.57\n\tc0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.68,0.32,2.33,0.95s0.97,1.4,0.97,2.28c0,0.85-0.3,1.59-0.9,2.21\n\tc-0.6,0.62-1.33,0.97-2.2,1.03c-0.12,0-0.19,0.06-0.19,0.19v1.36c0,0.12,0.06,0.18,0.19,0.18c1.33-0.04,2.46-0.55,3.39-1.51\n\tc0.93-0.97,1.39-2.12,1.39-3.45c0-0.87-0.22-1.68-0.66-2.45c0.76-0.74,1.27-1.61,1.51-2.62l0.19-0.68c0.01-0.01,0.01-0.03,0.01-0.07\n\tc0-0.08-0.05-0.13-0.15-0.16l-0.62-0.17c-0.57-0.17-1.06-0.45-1.46-0.84c-0.4-0.39-0.68-0.8-0.85-1.22s-0.25-0.84-0.24-1.26\n\tc0-0.28,0.03-0.56,0.1-0.85l0.11-0.61c0.02-0.1-0.02-0.18-0.14-0.23l-0.8-0.24c-0.47-0.09-0.88-0.14-1.24-0.14\n\tc-0.37-0.01-0.75,0.03-1.13,0.12c-0.38,0.08-0.78,0.22-1.19,0.4c-0.41,0.18-0.8,0.45-1.18,0.79c-0.38,0.34-0.71,0.74-0.99,1.2\n\tC15.3,7.55,14.51,7.4,13.77,7.4c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.11,0.26-2.02,0.84-2.74,1.74\n\tC4.45,14.71,4.09,15.74,4.09,16.89z M11.97,27.1c0,0.17,0.05,0.33,0.16,0.49c0.11,0.16,0.27,0.27,0.49,0.33\n\tc0.09,0.02,0.17,0.03,0.24,0.03c0.43,0,0.7-0.2,0.8-0.61l0.13-0.59c0.06-0.26,0.03-0.48-0.08-0.68c-0.12-0.2-0.29-0.32-0.53-0.37\n\tc-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.13,0.59C11.98,26.99,11.97,27.07,11.97,27.1z M12.76,24.2\n\tc0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59\n\tc0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23s-0.43,0.08-0.59,0.23C12.84,23.77,12.76,23.97,12.76,24.2z\n\t M13.38,21.93c-0.01,0.15,0.03,0.31,0.14,0.47c0.1,0.16,0.25,0.26,0.45,0.3c0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56\n\tl0.26-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4c-0.22-0.07-0.43-0.04-0.64,0.08s-0.34,0.3-0.41,0.53\n\tl-0.23,0.9C13.39,21.74,13.38,21.83,13.38,21.93z M16.14,24.08c0,0.17,0.05,0.33,0.15,0.48c0.1,0.15,0.25,0.26,0.46,0.32\n\tc0.03,0,0.08,0.01,0.14,0.02c0.06,0.01,0.11,0.02,0.14,0.02c0.41,0,0.66-0.22,0.76-0.66l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63\n\tc-0.12-0.21-0.29-0.34-0.51-0.41c-0.25-0.06-0.48-0.04-0.68,0.08s-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.01,0,0.05-0.01,0.11\n\tC16.15,24,16.14,24.04,16.14,24.08z M16.88,21.12c0,0.23,0.08,0.42,0.24,0.57c0.15,0.16,0.34,0.24,0.58,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23\n\tc-0.24,0-0.43,0.08-0.59,0.23S16.88,20.88,16.88,21.12z M17.59,8.77c0.33-0.56,0.78-0.99,1.34-1.29s1.15-0.45,1.76-0.45h0.22v0.32\n\tc0,0.64,0.11,1.26,0.34,1.86c0.23,0.6,0.56,1.15,1.02,1.66c0.45,0.51,0.99,0.91,1.61,1.21c-0.17,0.38-0.42,0.72-0.76,1.03\n\tc-0.91-0.78-1.98-1.17-3.22-1.17h-0.33C19.28,10.68,18.62,9.62,17.59,8.77z\" />",
};
#[cfg(feature = "WiNightAltSnow")]
const WI_NIGHT_ALT_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.07,16.93c0,1.33,0.47,2.47,1.4,3.43s2.07,1.47,3.4,1.51c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.85-0.04-1.58-0.39-2.18-1.03c-0.61-0.64-0.91-1.39-0.91-2.24c0-0.85,0.28-1.58,0.85-2.2c0.57-0.62,1.27-0.96,2.1-1.03l0.53-0.07\n\tc0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.09,0.56-2.01,1.37-2.75s1.76-1.11,2.86-1.11c1.09,0,2.04,0.37,2.86,1.1\n\tc0.82,0.73,1.28,1.64,1.4,2.72l0.08,0.57c0,0.12,0.06,0.18,0.17,0.18h1.62c0.91,0,1.68,0.32,2.33,0.97\n\tc0.65,0.64,0.97,1.41,0.97,2.31c0,0.85-0.3,1.6-0.91,2.24c-0.61,0.64-1.33,0.98-2.18,1.03c-0.14,0-0.21,0.06-0.21,0.17v1.34\n\tc0,0.11,0.07,0.17,0.21,0.17c0.88-0.02,1.68-0.26,2.41-0.71c0.73-0.45,1.31-1.05,1.73-1.8s0.63-1.56,0.63-2.43\n\tc0-0.91-0.22-1.74-0.65-2.48c0.74-0.66,1.24-1.52,1.52-2.58l0.17-0.72c0.01-0.01,0.02-0.04,0.02-0.08c0-0.07-0.05-0.13-0.16-0.16\n\tl-0.61-0.17c-0.44-0.13-0.83-0.32-1.17-0.57s-0.61-0.53-0.81-0.84c-0.2-0.31-0.34-0.62-0.44-0.95c-0.1-0.32-0.15-0.64-0.15-0.95\n\tc0-0.27,0.03-0.56,0.1-0.86l0.11-0.62c0.02-0.09-0.02-0.17-0.14-0.22l-0.8-0.24c-0.44-0.11-0.85-0.16-1.25-0.16\n\tc-0.37,0-0.74,0.04-1.12,0.13c-0.38,0.09-0.77,0.22-1.18,0.41c-0.41,0.19-0.8,0.45-1.18,0.8c-0.38,0.35-0.71,0.75-0.99,1.22\n\tc-0.81-0.33-1.6-0.5-2.38-0.5c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.37c-1.12,0.28-2.04,0.87-2.75,1.76\n\tC4.43,14.74,4.07,15.77,4.07,16.93z M10.46,21.02c0,0.24,0.08,0.44,0.24,0.6c0.16,0.17,0.35,0.25,0.59,0.25\n\tc0.24,0,0.44-0.08,0.6-0.25s0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58s-0.36-0.24-0.6-0.24c-0.23,0-0.43,0.08-0.59,0.24\n\tC10.54,20.6,10.46,20.79,10.46,21.02z M10.46,24.66c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.44-0.08,0.6-0.23c0.16-0.16,0.24-0.35,0.24-0.59c0-0.24-0.08-0.43-0.24-0.59c-0.16-0.16-0.36-0.23-0.6-0.23\n\tc-0.24,0-0.43,0.08-0.59,0.23C10.54,24.22,10.46,24.42,10.46,24.66z M13.66,22.96c0,0.24,0.08,0.44,0.24,0.59\n\tc0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.61-0.24s0.25-0.36,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61s-0.37-0.26-0.61-0.26\n\tc-0.22,0-0.41,0.09-0.58,0.26S13.66,22.72,13.66,22.96z M13.66,19.32c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.45-0.08,0.61-0.23s0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25c-0.22,0-0.42,0.08-0.58,0.25\n\tS13.66,19.09,13.66,19.32z M13.66,26.63c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.61-0.24\n\tc0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26\n\tC13.75,26.19,13.66,26.4,13.66,26.63z M16.9,21.02c0,0.24,0.08,0.44,0.25,0.6s0.36,0.25,0.6,0.25s0.43-0.08,0.59-0.25\n\ts0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58s-0.35-0.24-0.59-0.24s-0.43,0.08-0.6,0.24S16.9,20.79,16.9,21.02z M16.9,24.66\n\tc0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.6,0.24s0.43-0.08,0.59-0.24c0.16-0.16,0.23-0.35,0.23-0.59\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23s-0.44,0.08-0.6,0.23C16.98,24.22,16.9,24.42,16.9,24.66z M17.58,8.77\n\tc0.31-0.54,0.75-0.96,1.3-1.26S20,7.06,20.59,7.05c0.15,0,0.26,0.01,0.33,0.02v0.31c0,0.97,0.26,1.88,0.78,2.74s1.25,1.51,2.17,1.96\n\tc-0.16,0.36-0.41,0.72-0.76,1.07c-0.89-0.79-1.96-1.18-3.23-1.18h-0.31C19.3,10.74,18.64,9.68,17.58,8.77z\" />",
};
#[cfg(feature = "WiNightAltSnowThunderstorm")]
const WI_NIGHT_ALT_SNOW_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.09,16.89c0,1.11,0.33,2.1,0.99,2.97c0.66,0.87,1.52,1.47,2.58,1.79l-0.65,1.7c-0.04,0.14,0,0.21,0.14,0.21h2.12\n\tl-1.29,4.18h0.28l4.23-5.62c0.04-0.04,0.04-0.09,0.02-0.14c-0.03-0.05-0.07-0.07-0.14-0.07h-2.18l2.47-4.64\n\tc0.07-0.14,0.03-0.22-0.13-0.22H9.57c-0.09,0-0.16,0.05-0.22,0.15l-1.07,2.88c-0.71-0.18-1.3-0.57-1.78-1.17s-0.71-1.27-0.71-2.01\n\tc0-0.83,0.28-1.55,0.85-2.17c0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.07c0.13,0,0.2-0.06,0.2-0.18l0.07-0.51\n\tc0.11-1.08,0.56-1.99,1.37-2.72c0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.28,1.64,1.4,2.71l0.07,0.57\n\tc0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.68,0.32,2.33,0.95s0.97,1.4,0.97,2.28c0,0.85-0.3,1.59-0.9,2.21\n\tc-0.6,0.62-1.33,0.97-2.2,1.03c-0.12,0-0.19,0.06-0.19,0.19v1.36c0,0.12,0.06,0.18,0.19,0.18c1.33-0.04,2.46-0.55,3.39-1.51\n\tc0.93-0.97,1.39-2.12,1.39-3.45c0-0.87-0.22-1.68-0.66-2.45c0.76-0.74,1.27-1.61,1.51-2.62l0.19-0.68c0.01-0.01,0.01-0.03,0.01-0.07\n\tc0-0.08-0.05-0.13-0.15-0.16l-0.62-0.17c-0.57-0.17-1.06-0.45-1.46-0.84c-0.4-0.39-0.68-0.8-0.85-1.22s-0.25-0.84-0.24-1.26\n\tc0-0.28,0.03-0.56,0.1-0.85l0.11-0.61c0.02-0.1-0.02-0.18-0.14-0.23l-0.8-0.24c-0.47-0.09-0.88-0.14-1.24-0.14\n\tc-0.37-0.01-0.75,0.03-1.13,0.12c-0.38,0.08-0.78,0.22-1.19,0.4c-0.41,0.18-0.8,0.45-1.18,0.79c-0.38,0.34-0.71,0.74-0.99,1.2\n\tC15.3,7.55,14.51,7.4,13.77,7.4c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.11,0.26-2.02,0.84-2.74,1.74\n\tC4.45,14.71,4.09,15.74,4.09,16.89z M13.68,22.96c0,0.24,0.08,0.44,0.24,0.59c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.44-0.08,0.61-0.24s0.25-0.36,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61s-0.37-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26\n\tS13.68,22.72,13.68,22.96z M13.68,19.32c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.45-0.08,0.61-0.23\n\ts0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25S13.68,19.09,13.68,19.32z\n\t M13.68,26.63c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.61-0.24c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26C13.76,26.19,13.68,26.4,13.68,26.63z\n\t M16.91,21.02c0,0.24,0.08,0.44,0.25,0.6s0.36,0.25,0.6,0.25c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.6\n\tc0-0.22-0.08-0.42-0.24-0.58c-0.16-0.16-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.6,0.24S16.91,20.79,16.91,21.02z M16.91,24.66\n\tc0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.24c0.16-0.16,0.23-0.35,0.23-0.59\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.23C16.99,24.22,16.91,24.42,16.91,24.66z\n\t M17.59,8.77c0.33-0.56,0.78-0.99,1.34-1.29s1.15-0.45,1.76-0.45h0.22v0.32c0,0.64,0.11,1.26,0.34,1.86\n\tc0.23,0.6,0.56,1.15,1.02,1.66c0.45,0.51,0.99,0.91,1.61,1.21c-0.17,0.38-0.42,0.72-0.76,1.03c-0.91-0.78-1.98-1.17-3.22-1.17h-0.33\n\tC19.28,10.68,18.62,9.62,17.59,8.77z\" />",
};
#[cfg(feature = "WiNightAltSnowWind")]
const WI_NIGHT_ALT_SNOW_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.07,16.93c0,1.33,0.47,2.47,1.4,3.43s2.07,1.47,3.4,1.51c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.85-0.04-1.58-0.39-2.18-1.03c-0.61-0.64-0.91-1.39-0.91-2.24c0-0.85,0.28-1.58,0.85-2.2c0.57-0.62,1.27-0.96,2.1-1.03l0.53-0.07\n\tc0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.09,0.56-2.01,1.37-2.75s1.76-1.11,2.86-1.11c1.09,0,2.04,0.37,2.86,1.1\n\tc0.82,0.73,1.28,1.64,1.4,2.72l0.08,0.57c0,0.12,0.06,0.18,0.17,0.18h1.62c0.91,0,1.68,0.32,2.33,0.97\n\tc0.65,0.64,0.97,1.41,0.97,2.31c0,0.85-0.3,1.6-0.91,2.24c-0.61,0.64-1.33,0.98-2.18,1.03c-0.14,0-0.21,0.06-0.21,0.17v1.34\n\tc0,0.11,0.07,0.17,0.21,0.17c0.88-0.02,1.68-0.26,2.41-0.71c0.73-0.45,1.31-1.05,1.73-1.8s0.63-1.56,0.63-2.43\n\tc0-0.91-0.22-1.74-0.65-2.48c0.74-0.66,1.24-1.52,1.52-2.58l0.17-0.72c0.01-0.01,0.02-0.04,0.02-0.08c0-0.07-0.05-0.13-0.16-0.16\n\tl-0.61-0.17c-0.44-0.13-0.83-0.32-1.17-0.57s-0.61-0.53-0.81-0.84c-0.2-0.31-0.34-0.62-0.44-0.95c-0.1-0.32-0.15-0.64-0.15-0.95\n\tc0-0.27,0.03-0.56,0.1-0.86l0.11-0.62c0.02-0.09-0.02-0.17-0.14-0.22l-0.8-0.24c-0.44-0.11-0.85-0.16-1.25-0.16\n\tc-0.37,0-0.74,0.04-1.12,0.13c-0.38,0.09-0.77,0.22-1.18,0.41c-0.41,0.19-0.8,0.45-1.18,0.8c-0.38,0.35-0.71,0.75-0.99,1.22\n\tc-0.81-0.33-1.6-0.5-2.38-0.5c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.37c-1.12,0.28-2.04,0.87-2.75,1.76\n\tC4.43,14.74,4.07,15.77,4.07,16.93z M9.6,24.66c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.44-0.08,0.6-0.23c0.16-0.16,0.24-0.35,0.24-0.59c0-0.24-0.08-0.43-0.24-0.59c-0.16-0.16-0.36-0.23-0.6-0.23\n\tS10,23.91,9.84,24.07C9.68,24.22,9.6,24.42,9.6,24.66z M10.03,21.02c0,0.24,0.08,0.44,0.24,0.6s0.35,0.25,0.59,0.25\n\tc0.24,0,0.44-0.08,0.6-0.25c0.16-0.17,0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58c-0.16-0.16-0.36-0.24-0.6-0.24\n\tc-0.23,0-0.43,0.08-0.59,0.24S10.03,20.79,10.03,21.02z M12.38,26.63c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25\n\tc0.24,0,0.44-0.08,0.61-0.24c0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.26-0.61-0.26\n\tc-0.22,0-0.41,0.09-0.58,0.26C12.46,26.19,12.38,26.4,12.38,26.63z M13.02,22.96c0,0.24,0.08,0.43,0.25,0.59\n\tc0.16,0.16,0.35,0.24,0.57,0.24c0.24,0,0.44-0.08,0.61-0.24c0.17-0.16,0.26-0.36,0.26-0.59c0-0.24-0.09-0.44-0.26-0.61\n\tc-0.17-0.17-0.38-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26S13.02,22.72,13.02,22.96z M13.66,19.32c0,0.24,0.08,0.43,0.24,0.58\n\tc0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.45-0.08,0.61-0.23s0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25\n\tc-0.22,0-0.42,0.08-0.58,0.25S13.66,19.09,13.66,19.32z M16.05,24.66c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.6,0.24\n\tc0.23,0,0.43-0.08,0.59-0.24c0.16-0.16,0.24-0.35,0.24-0.59c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23\n\tc-0.24,0-0.44,0.08-0.6,0.23C16.13,24.22,16.05,24.42,16.05,24.66z M16.47,21.02c0,0.23,0.08,0.43,0.25,0.6\n\tc0.17,0.17,0.37,0.25,0.6,0.25s0.43-0.08,0.59-0.25s0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58s-0.35-0.24-0.59-0.24\n\ts-0.43,0.08-0.6,0.25C16.55,20.61,16.47,20.8,16.47,21.02z M17.58,8.77c0.31-0.54,0.75-0.96,1.3-1.26S20,7.06,20.59,7.05\n\tc0.15,0,0.26,0.01,0.33,0.02v0.31c0,0.97,0.26,1.88,0.78,2.74s1.25,1.51,2.17,1.96c-0.16,0.36-0.41,0.72-0.76,1.07\n\tc-0.89-0.79-1.96-1.18-3.23-1.18h-0.31C19.3,10.74,18.64,9.68,17.58,8.77z\" />",
};
#[cfg(feature = "WiNightAltSprinkle")]
const WI_NIGHT_ALT_SPRINKLE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.11,16.89c0,1.33,0.46,2.48,1.39,3.43c0.93,0.96,2.06,1.47,3.4,1.54c0.12,0,0.18-0.06,0.18-0.18v-1.33\n\tc0-0.12-0.06-0.18-0.18-0.18c-0.86-0.04-1.58-0.38-2.18-1.02c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.84-2.17\n\tc0.56-0.61,1.26-0.97,2.1-1.07l0.52-0.04c0.13,0,0.2-0.06,0.2-0.18l0.07-0.54c0.11-1.08,0.56-1.99,1.37-2.71\n\tc0.81-0.73,1.76-1.09,2.85-1.09c1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.28,1.63,1.4,2.71l0.07,0.57c0,0.12,0.06,0.19,0.18,0.19h1.62\n\tc0.89,0,1.65,0.32,2.3,0.96s0.97,1.4,0.97,2.27c0,0.87-0.3,1.62-0.9,2.26c-0.6,0.64-1.33,0.98-2.18,1.02\n\tc-0.12,0-0.19,0.06-0.19,0.18v1.33c0,0.12,0.06,0.18,0.19,0.18c0.88-0.03,1.68-0.27,2.41-0.72s1.31-1.05,1.73-1.8\n\ts0.63-1.57,0.63-2.44c0-0.87-0.23-1.68-0.68-2.45c0.78-0.74,1.29-1.6,1.54-2.58l0.14-0.73c0.01-0.01,0.02-0.03,0.02-0.07\n\tc0-0.07-0.05-0.13-0.16-0.16l-0.57-0.17c-0.57-0.16-1.06-0.44-1.46-0.82c-0.41-0.38-0.7-0.79-0.87-1.21\n\tc-0.17-0.43-0.26-0.85-0.26-1.28c0-0.29,0.04-0.57,0.11-0.85l0.13-0.61c0.02-0.1-0.02-0.18-0.13-0.23l-0.8-0.24\n\tc-0.45-0.1-0.87-0.15-1.27-0.15c-0.36,0-0.73,0.04-1.12,0.13c-0.38,0.09-0.78,0.22-1.19,0.4s-0.81,0.44-1.2,0.79s-0.72,0.74-1,1.2\n\tc-0.81-0.31-1.59-0.46-2.33-0.46c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.11,0.26-2.02,0.84-2.73,1.74\n\tC4.47,14.71,4.11,15.74,4.11,16.89z M10.05,17.77c0,0.38,0.14,0.71,0.42,0.98c0.28,0.27,0.62,0.4,1.02,0.4c0.4,0,0.73-0.13,1-0.4\n\tc0.27-0.27,0.4-0.59,0.4-0.98c0-0.26-0.12-0.6-0.35-1.02c-0.23-0.42-0.45-0.75-0.65-0.98c-0.11-0.12-0.24-0.26-0.41-0.43l-0.35,0.41\n\tc-0.27,0.29-0.52,0.64-0.75,1.04S10.05,17.51,10.05,17.77z M13.04,21.76c0,0.66,0.23,1.21,0.68,1.66c0.46,0.45,1.01,0.67,1.65,0.67\n\tc0.66,0,1.21-0.23,1.66-0.68c0.45-0.46,0.68-1.01,0.68-1.65c0-0.55-0.27-1.22-0.8-2c-0.44-0.58-0.87-1.08-1.28-1.49\n\tc-0.08-0.06-0.17-0.13-0.26-0.23l-0.23,0.23c-0.39,0.36-0.82,0.86-1.28,1.48c-0.24,0.33-0.43,0.68-0.59,1.04\n\tC13.11,21.16,13.04,21.48,13.04,21.76z M14.51,15.09c0,0.26,0.1,0.47,0.29,0.66s0.42,0.27,0.7,0.27c0.26,0,0.47-0.09,0.66-0.27\n\tc0.18-0.18,0.27-0.4,0.27-0.66c0-0.43-0.31-0.97-0.93-1.62l-0.25,0.27c-0.18,0.2-0.35,0.43-0.5,0.7\n\tC14.58,14.71,14.51,14.93,14.51,15.09z M17.56,8.77c0.35-0.57,0.8-1,1.34-1.29c0.54-0.29,1.12-0.44,1.72-0.44\n\tc0.12,0,0.21,0.01,0.27,0.02v0.3c0,0.96,0.26,1.87,0.79,2.74s1.25,1.52,2.18,1.97c-0.16,0.38-0.41,0.72-0.75,1.03\n\tc-0.93-0.76-1.99-1.14-3.21-1.14h-0.33C19.27,10.65,18.6,9.59,17.56,8.77z\" />",
};
#[cfg(feature = "WiNightAltStormShowers")]
const WI_NIGHT_ALT_STORM_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.09,16.89c0,1.11,0.33,2.1,0.99,2.97c0.66,0.87,1.52,1.47,2.58,1.79l-0.65,1.7c-0.04,0.14,0,0.21,0.14,0.21h2.12\n\tl-1.29,4.18h0.28l4.23-5.62c0.04-0.04,0.04-0.09,0.02-0.14c-0.03-0.05-0.07-0.07-0.14-0.07h-2.18l2.47-4.64\n\tc0.07-0.14,0.03-0.22-0.13-0.22H9.57c-0.09,0-0.16,0.05-0.22,0.15l-1.07,2.88c-0.71-0.18-1.3-0.57-1.78-1.17s-0.71-1.27-0.71-2.01\n\tc0-0.83,0.28-1.55,0.85-2.17c0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.07c0.13,0,0.2-0.06,0.2-0.18l0.07-0.51\n\tc0.11-1.08,0.56-1.99,1.37-2.72c0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.28,1.64,1.4,2.71l0.07,0.57\n\tc0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.68,0.32,2.33,0.95s0.97,1.4,0.97,2.28c0,0.85-0.3,1.59-0.9,2.21\n\tc-0.6,0.62-1.33,0.97-2.2,1.03c-0.12,0-0.19,0.06-0.19,0.19v1.36c0,0.12,0.06,0.18,0.19,0.18c1.33-0.04,2.46-0.55,3.39-1.51\n\tc0.93-0.97,1.39-2.12,1.39-3.45c0-0.87-0.22-1.68-0.66-2.45c0.76-0.74,1.27-1.61,1.51-2.62l0.19-0.68c0.01-0.01,0.01-0.03,0.01-0.07\n\tc0-0.08-0.05-0.13-0.15-0.16l-0.62-0.17c-0.57-0.17-1.06-0.45-1.46-0.84c-0.4-0.39-0.68-0.8-0.85-1.22s-0.25-0.84-0.24-1.26\n\tc0-0.28,0.03-0.56,0.1-0.85l0.11-0.61c0.02-0.1-0.02-0.18-0.14-0.23l-0.8-0.24c-0.47-0.09-0.88-0.14-1.24-0.14\n\tc-0.37-0.01-0.75,0.03-1.13,0.12c-0.38,0.08-0.78,0.22-1.19,0.4c-0.41,0.18-0.8,0.45-1.18,0.79c-0.38,0.34-0.71,0.74-0.99,1.2\n\tC15.3,7.55,14.51,7.4,13.77,7.4c-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.11,0.26-2.02,0.84-2.74,1.74\n\tC4.45,14.71,4.09,15.74,4.09,16.89z M12.26,26.76c0,0.16,0.05,0.31,0.15,0.47c0.1,0.16,0.25,0.27,0.45,0.33\n\tc0.16,0.03,0.25,0.05,0.27,0.05c0.09,0,0.22-0.03,0.37-0.1c0.21-0.1,0.35-0.27,0.42-0.53l0.28-1.05c0.06-0.22,0.04-0.43-0.08-0.63\n\ts-0.29-0.34-0.53-0.41c-0.22-0.06-0.43-0.03-0.63,0.08c-0.2,0.12-0.34,0.3-0.41,0.53l-0.27,1L12.26,26.76z M13.6,22\n\tc0,0.43,0.2,0.68,0.61,0.75c0.14,0.03,0.23,0.05,0.27,0.05c0.38,0,0.63-0.21,0.77-0.63l0.3-1.02c0.06-0.22,0.03-0.43-0.08-0.63\n\ts-0.3-0.34-0.53-0.41c-0.22-0.07-0.44-0.05-0.64,0.07c-0.2,0.12-0.34,0.29-0.41,0.53l-0.25,1.01C13.61,21.81,13.6,21.9,13.6,22z\n\t M16.41,23.67c0.01,0.17,0.07,0.33,0.18,0.48s0.27,0.27,0.48,0.34c0.16,0.04,0.27,0.06,0.33,0.06c0.34,0,0.58-0.23,0.71-0.68\n\tl0.24-1.02c0.07-0.23,0.05-0.45-0.06-0.66c-0.11-0.21-0.28-0.34-0.5-0.41c-0.25-0.06-0.48-0.03-0.68,0.08\n\tc-0.2,0.12-0.33,0.3-0.37,0.53l-0.29,1.03c0,0.02-0.01,0.06-0.02,0.12C16.41,23.61,16.41,23.65,16.41,23.67z M17.59,8.77\n\tc0.33-0.56,0.78-0.99,1.34-1.29s1.15-0.45,1.76-0.45h0.22v0.32c0,0.64,0.11,1.26,0.34,1.86c0.23,0.6,0.56,1.15,1.02,1.66\n\tc0.45,0.51,0.99,0.91,1.61,1.21c-0.17,0.38-0.42,0.72-0.76,1.03c-0.91-0.78-1.98-1.17-3.22-1.17h-0.33\n\tC19.28,10.68,18.62,9.62,17.59,8.77z M17.78,18.87c0,0.43,0.22,0.71,0.65,0.82c0.14,0.02,0.24,0.04,0.3,0.04\n\tc0.36,0,0.61-0.22,0.74-0.65l0.28-1.04c0.01-0.05,0.01-0.12,0.01-0.22c0.01-0.17-0.03-0.33-0.14-0.49\n\tc-0.11-0.16-0.27-0.27-0.49-0.33c-0.01,0-0.05,0-0.1-0.01c-0.05-0.01-0.1-0.01-0.13-0.01c-0.16,0-0.32,0.05-0.48,0.15\n\ts-0.27,0.26-0.33,0.47l-0.29,1.02c0,0.01,0,0.04-0.01,0.1C17.79,18.79,17.78,18.84,17.78,18.87z\" />",
};
#[cfg(feature = "WiNightAltThunderstorm")]
const WI_NIGHT_ALT_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.08,16.88c0,1.11,0.33,2.1,0.99,2.98s1.52,1.47,2.58,1.79l-0.66,1.69c-0.03,0.14,0.02,0.21,0.15,0.21h2.12l-0.97,3.51\n\th0.29l3.91-4.94c0.04-0.05,0.04-0.1,0.01-0.15c-0.03-0.05-0.08-0.07-0.15-0.07h-2.18l2.48-4.63c0.07-0.14,0.03-0.22-0.13-0.22H9.56\n\tc-0.09,0-0.16,0.05-0.23,0.14l-1.07,2.88c-0.72-0.18-1.31-0.57-1.78-1.17c-0.47-0.6-0.7-1.27-0.7-2.01c0-0.83,0.28-1.55,0.85-2.17\n\tc0.57-0.62,1.27-0.97,2.1-1.07l0.52-0.08c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.85-1.1\n\tc1.08,0,2.03,0.37,2.85,1.1s1.29,1.64,1.41,2.71l0.07,0.59c0,0.11,0.06,0.17,0.18,0.17h1.62c0.91,0,1.68,0.32,2.33,0.95\n\ts0.97,1.4,0.97,2.29c0,0.85-0.3,1.59-0.9,2.21c-0.6,0.62-1.33,0.97-2.2,1.04c-0.12,0-0.19,0.06-0.19,0.17v1.38\n\tc0,0.12,0.06,0.18,0.19,0.18c0.88-0.03,1.68-0.27,2.41-0.72c0.73-0.45,1.31-1.05,1.73-1.8c0.42-0.75,0.63-1.57,0.63-2.45\n\tc0-0.87-0.22-1.68-0.66-2.45c0.79-0.76,1.31-1.63,1.56-2.62l0.14-0.72c0.01-0.01,0.02-0.04,0.02-0.07c0-0.07-0.05-0.12-0.16-0.15\n\tl-0.56-0.18c-0.57-0.16-1.06-0.44-1.46-0.82c-0.41-0.38-0.7-0.8-0.87-1.23c-0.17-0.44-0.26-0.88-0.26-1.32\n\tc0-0.26,0.03-0.53,0.08-0.8l0.14-0.61c0.04-0.1,0-0.18-0.14-0.23c-0.21-0.09-0.51-0.17-0.9-0.26c-0.39-0.09-0.77-0.13-1.15-0.13\n\tc-0.36,0-0.73,0.04-1.12,0.13c-0.38,0.09-0.78,0.22-1.19,0.41c-0.41,0.18-0.81,0.45-1.2,0.8c-0.39,0.35-0.72,0.75-1,1.22\n\tc-0.82-0.3-1.62-0.45-2.38-0.45c-1.41,0-2.67,0.44-3.76,1.31s-1.8,1.99-2.11,3.36c-1.11,0.26-2.02,0.84-2.74,1.74\n\tC4.44,14.69,4.08,15.72,4.08,16.88z M12.18,26.7c0,0.16,0.05,0.32,0.15,0.46c0.1,0.15,0.25,0.25,0.45,0.3\n\tc0.11,0.02,0.21,0.03,0.3,0.03c0.41,0,0.66-0.21,0.76-0.63l2.32-8.79c0.06-0.24,0.04-0.45-0.07-0.65c-0.11-0.2-0.28-0.33-0.5-0.39\n\tc-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.34,0.27-0.4,0.49l-2.32,8.84C12.19,26.52,12.18,26.61,12.18,26.7z M16.35,23.68\n\tc0,0.16,0.05,0.32,0.15,0.46c0.1,0.14,0.25,0.25,0.46,0.31c0.03,0,0.08,0,0.15,0.01c0.07,0.01,0.13,0.01,0.16,0.01\n\tc0.38,0,0.62-0.21,0.72-0.63l1.5-5.77c0.06-0.24,0.04-0.46-0.08-0.66c-0.11-0.19-0.28-0.32-0.51-0.38\n\tc-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.33,0.27-0.39,0.5l-1.5,5.82C16.36,23.51,16.35,23.6,16.35,23.68z M17.59,8.75\n\tc0.33-0.57,0.77-1,1.33-1.3c0.55-0.3,1.14-0.45,1.76-0.45c0.12,0,0.22,0,0.27,0.01v0.32c0,0.96,0.26,1.87,0.78,2.73\n\ts1.25,1.51,2.17,1.97c-0.18,0.42-0.44,0.77-0.79,1.07c-0.92-0.79-1.99-1.18-3.22-1.18h-0.32C19.29,10.66,18.63,9.61,17.59,8.75z\" />",
};
#[cfg(feature = "WiNightClear")]
const WI_NIGHT_CLEAR: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.91,14.48c0-0.96,0.19-1.87,0.56-2.75s0.88-1.63,1.51-2.26c0.63-0.63,1.39-1.14,2.27-1.52c0.88-0.38,1.8-0.57,2.75-0.57\n\th1.14c0.16,0.04,0.23,0.14,0.23,0.28l0.05,0.88c0.04,1.27,0.49,2.35,1.37,3.24c0.88,0.89,1.94,1.37,3.19,1.42l0.82,0.07\n\tc0.16,0,0.24,0.08,0.24,0.23v0.98c0.01,1.28-0.3,2.47-0.93,3.56c-0.63,1.09-1.48,1.95-2.57,2.59c-1.08,0.63-2.27,0.95-3.55,0.95\n\tc-0.97,0-1.9-0.19-2.78-0.56s-1.63-0.88-2.26-1.51c-0.63-0.63-1.13-1.39-1.5-2.26C8.1,16.37,7.91,15.45,7.91,14.48z M9.74,14.48\n\tc0,0.76,0.15,1.48,0.45,2.16c0.3,0.67,0.7,1.24,1.19,1.7c0.49,0.46,1.05,0.82,1.69,1.08c0.63,0.27,1.28,0.4,1.94,0.4\n\tc0.58,0,1.17-0.11,1.76-0.34c0.59-0.23,1.14-0.55,1.65-0.96c0.51-0.41,0.94-0.93,1.31-1.57c0.37-0.64,0.6-1.33,0.71-2.09\n\tc-1.63-0.34-2.94-1.04-3.92-2.1s-1.55-2.3-1.7-3.74C13.86,9.08,13,9.37,12.21,9.9c-0.78,0.53-1.39,1.2-1.82,2.02\n\tC9.96,12.74,9.74,13.59,9.74,14.48z\" />",
};
#[cfg(feature = "WiNightCloudy")]
const WI_NIGHT_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.3,16.89c0,0.89,0.22,1.72,0.66,2.48s1.03,1.36,1.79,1.8s1.58,0.67,2.48,0.67h10.81c0.89,0,1.72-0.22,2.48-0.67\n\ts1.36-1.05,1.8-1.8c0.44-0.76,0.67-1.59,0.67-2.48c0-0.64-0.14-1.3-0.42-2c0.76-0.93,1.13-2.03,1.13-3.3c0-0.95-0.23-1.83-0.69-2.63\n\tc-0.46-0.8-1.1-1.44-1.9-1.91c-0.8-0.47-1.68-0.7-2.63-0.7c-1.49,0-2.78,0.58-3.87,1.74c-0.76-0.43-1.66-0.65-2.69-0.65\n\tc-1.41,0-2.65,0.43-3.73,1.3s-1.77,1.98-2.08,3.35c-1.12,0.25-2.03,0.82-2.74,1.72C4.66,14.71,4.3,15.74,4.3,16.89z M6.01,16.89\n\tc0-0.83,0.28-1.55,0.83-2.16c0.56-0.61,1.26-0.96,2.1-1.06l0.68-0.03l0.07-0.72c0.14-1.08,0.61-1.99,1.41-2.71\n\tc0.8-0.73,1.74-1.09,2.81-1.09c1.09,0,2.05,0.37,2.86,1.1s1.27,1.63,1.38,2.71l0.1,0.75h1.78c0.88,0,1.64,0.32,2.28,0.95\n\ts0.96,1.39,0.96,2.26c0,0.9-0.32,1.67-0.95,2.32s-1.4,0.97-2.28,0.97H9.23c-0.87,0-1.62-0.32-2.26-0.97\n\tC6.33,18.55,6.01,17.78,6.01,16.89z M18.04,9.06c0.69-0.66,1.5-0.99,2.44-0.99c0.99,0,1.83,0.34,2.52,1.03\n\tc0.69,0.68,1.04,1.52,1.04,2.51c0,0.62-0.17,1.23-0.52,1.84C22.56,12.48,21.4,12,20.03,12h-0.31C19.45,10.89,18.89,9.91,18.04,9.06z\n\t\" />",
};
#[cfg(feature = "WiNightCloudyGusts")]
const WI_NIGHT_CLOUDY_GUSTS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.06,20.98c0,0.24,0.09,0.44,0.27,0.6c0.18,0.18,0.38,0.27,0.61,0.27h5.88c0.26,0,0.49,0.09,0.69,0.28\n\tc0.2,0.19,0.3,0.42,0.3,0.68c0,0.26-0.1,0.48-0.3,0.68s-0.43,0.3-0.68,0.3c-0.25,0-0.48-0.1-0.68-0.3\n\tc-0.19-0.17-0.39-0.26-0.62-0.26c-0.23,0-0.43,0.08-0.59,0.25c-0.16,0.17-0.24,0.37-0.24,0.61s0.08,0.44,0.24,0.6\n\tc0.52,0.52,1.15,0.78,1.88,0.78c0.74,0,1.38-0.26,1.89-0.77c0.52-0.52,0.78-1.14,0.78-1.88c0-0.74-0.26-1.38-0.78-1.9\n\tc-0.52-0.52-1.15-0.79-1.89-0.79H3.94c-0.24,0-0.44,0.08-0.62,0.25C3.14,20.55,3.06,20.75,3.06,20.98z M3.06,17.97\n\tc0,0.23,0.09,0.42,0.27,0.58c0.15,0.16,0.35,0.24,0.61,0.24h10.99c0.74,0,1.37-0.26,1.89-0.78c0.52-0.52,0.78-1.15,0.78-1.88\n\ts-0.26-1.36-0.78-1.88c-0.52-0.52-1.15-0.77-1.89-0.77c-0.76,0-1.39,0.25-1.89,0.75c-0.15,0.17-0.23,0.38-0.23,0.63\n\tc0,0.24,0.08,0.43,0.23,0.59s0.35,0.23,0.6,0.23c0.25,0,0.45-0.07,0.61-0.23c0.19-0.19,0.42-0.28,0.68-0.28\n\tc0.26,0,0.48,0.09,0.68,0.28c0.19,0.19,0.29,0.42,0.29,0.68s-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.68,0.29H3.94\n\tc-0.24,0-0.44,0.08-0.62,0.25C3.14,17.54,3.06,17.74,3.06,17.97z M5.71,15.63c0,0.08,0.06,0.12,0.17,0.12h1.43\n\tc0.08,0,0.15-0.05,0.22-0.14c0.23-0.54,0.57-0.99,1.05-1.35c0.47-0.36,1-0.56,1.59-0.6l0.52-0.07c0.12,0,0.19-0.06,0.19-0.19\n\tl0.07-0.5c0.11-1.08,0.57-1.99,1.38-2.71c0.81-0.73,1.77-1.09,2.86-1.09s2.04,0.36,2.85,1.08c0.81,0.72,1.27,1.63,1.39,2.72\n\tl0.07,0.57c0,0.12,0.06,0.18,0.18,0.18h1.63c0.9,0,1.67,0.32,2.31,0.95c0.64,0.63,0.96,1.39,0.96,2.28c0,0.89-0.32,1.66-0.96,2.29\n\tc-0.64,0.63-1.41,0.95-2.31,0.95h-6.91c-0.11,0-0.17,0.06-0.17,0.18v1.37c0,0.12,0.06,0.18,0.17,0.18h6.91\n\tc0.89,0,1.72-0.22,2.48-0.67c0.76-0.44,1.36-1.05,1.8-1.81c0.44-0.76,0.66-1.59,0.66-2.49c0-0.74-0.14-1.42-0.42-2.02\n\tc0.76-1,1.14-2.11,1.14-3.33c0-0.71-0.14-1.39-0.42-2.04s-0.65-1.2-1.12-1.67c-0.47-0.47-1.03-0.84-1.67-1.11S22.42,6.3,21.71,6.3\n\tc-1.54,0-2.84,0.58-3.88,1.73c-0.78-0.41-1.67-0.61-2.65-0.61c-1.41,0-2.66,0.44-3.75,1.31s-1.77,1.99-2.07,3.35\n\tc-0.85,0.2-1.6,0.61-2.26,1.23s-1.11,1.35-1.37,2.18v0.04C5.72,15.58,5.71,15.62,5.71,15.63z M19.24,9\n\tc0.72-0.68,1.54-1.02,2.48-1.02c0.98,0,1.81,0.35,2.51,1.05s1.05,1.53,1.05,2.5c0,0.61-0.17,1.22-0.51,1.85\n\tc-0.96-0.96-2.11-1.43-3.47-1.43h-0.33C20.73,10.88,20.16,9.89,19.24,9z\" />",
};
#[cfg(feature = "WiNightCloudyHigh")]
const WI_NIGHT_CLOUDY_HIGH: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.58,13.45c0-1.15,0.36-2.18,1.08-3.07C5.38,9.48,6.29,8.9,7.4,8.64c0.31-1.37,1.02-2.49,2.11-3.37s2.35-1.32,3.76-1.32\n\tc1.38,0,2.61,0.43,3.69,1.28s1.78,1.95,2.1,3.29h0.33c0.9,0,1.73,0.22,2.49,0.65s1.37,1.03,1.81,1.79c0.44,0.76,0.67,1.58,0.67,2.48\n\tc0,0.2-0.01,0.4-0.03,0.61c0.65,0.51,1.16,1.15,1.54,1.91s0.56,1.57,0.56,2.43c0,0.77-0.15,1.5-0.45,2.19\n\tc-0.3,0.69-0.7,1.28-1.2,1.78c-0.5,0.49-1.1,0.89-1.79,1.18c-0.69,0.29-1.41,0.44-2.17,0.44c-0.75,0-1.47-0.15-2.16-0.44\n\tc-0.69-0.29-1.28-0.69-1.78-1.19c-0.5-0.5-0.89-1.09-1.19-1.78s-0.45-1.41-0.45-2.16H8.38c-1.34-0.06-2.47-0.57-3.4-1.53\n\tC4.05,15.94,3.58,14.79,3.58,13.45z M5.29,13.45c0,0.87,0.3,1.62,0.9,2.26c0.6,0.64,1.33,0.98,2.19,1.03h11.19\n\tc0.86-0.04,1.59-0.39,2.19-1.03c0.61-0.64,0.91-1.4,0.91-2.26c0-0.88-0.33-1.63-0.98-2.27s-1.42-0.96-2.32-0.96h-1.62\n\tc-0.11,0-0.17-0.06-0.17-0.17l-0.07-0.58c-0.11-1.08-0.58-1.99-1.4-2.72s-1.77-1.1-2.86-1.1c-1.09,0-2.05,0.37-2.85,1.1\n\tS9.14,8.39,9.04,9.47l-0.08,0.58c0,0.11-0.07,0.17-0.2,0.17H8.24c-0.84,0.1-1.54,0.46-2.1,1.07C5.57,11.9,5.29,12.62,5.29,13.45z\n\t M16.55,18.56c0.06,1.12,0.52,2.07,1.37,2.83c0.85,0.76,1.82,1.14,2.91,1.14c0.6,0,1.17-0.12,1.7-0.35s0.98-0.55,1.34-0.93\n\tc0.36-0.39,0.65-0.83,0.85-1.33c0.21-0.5,0.31-1,0.31-1.52c0-0.49-0.1-0.98-0.3-1.47s-0.48-0.94-0.85-1.35\n\tc-0.39,0.82-0.97,1.5-1.74,2.02c-0.77,0.52-1.63,0.79-2.57,0.83h-3.03C16.54,18.44,16.54,18.47,16.55,18.56z\" />",
};
#[cfg(feature = "WiNightCloudyWindy")]
const WI_NIGHT_CLOUDY_WINDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.43,21c0,0.25,0.09,0.45,0.27,0.6c0.17,0.17,0.37,0.26,0.61,0.26h9.54c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.61s-0.08-0.44-0.24-0.61c-0.16-0.17-0.35-0.25-0.59-0.25H3.31c-0.24,0-0.44,0.09-0.62,0.26\n\tC2.52,20.57,2.43,20.77,2.43,21z M5.07,17.97c0,0.23,0.09,0.42,0.27,0.58c0.16,0.16,0.36,0.24,0.6,0.24h9.55\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.24-0.08-0.44-0.24-0.6c-0.16-0.17-0.35-0.25-0.59-0.25H5.94\n\tc-0.24,0-0.44,0.08-0.61,0.25C5.15,17.54,5.07,17.74,5.07,17.97z M6.21,15.64c0,0.07,0.07,0.11,0.2,0.11h1.42\n\tc0.09,0,0.16-0.05,0.23-0.14c0.22-0.54,0.57-0.99,1.05-1.35c0.47-0.36,1-0.56,1.58-0.6l0.54-0.07c0.12,0,0.18-0.06,0.18-0.18\n\tl0.07-0.51c0.11-1.08,0.57-1.99,1.38-2.72c0.81-0.73,1.77-1.1,2.87-1.1s2.06,0.36,2.87,1.09c0.81,0.72,1.28,1.63,1.39,2.73\n\tl0.08,0.57c0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.69,0.32,2.33,0.95c0.64,0.63,0.96,1.39,0.96,2.29c0,0.89-0.32,1.65-0.96,2.29\n\tc-0.64,0.64-1.42,0.96-2.33,0.96h-6.91c-0.11,0-0.17,0.06-0.17,0.17v1.38c0,0.12,0.06,0.18,0.17,0.18h6.91\n\tc0.91,0,1.74-0.22,2.51-0.67c0.77-0.44,1.37-1.05,1.82-1.81c0.45-0.76,0.67-1.59,0.67-2.49c0-0.71-0.15-1.37-0.44-2.01\n\tc0.77-1.01,1.15-2.1,1.15-3.29c0-0.95-0.24-1.83-0.71-2.64s-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.7-2.64-0.7\n\tc-1.52,0-2.81,0.56-3.84,1.67C17.6,7.6,16.7,7.4,15.74,7.4c-0.93,0-1.81,0.2-2.63,0.59s-1.51,0.95-2.07,1.66\n\tc-0.56,0.71-0.94,1.52-1.13,2.43c-0.88,0.2-1.64,0.6-2.29,1.2c-0.65,0.6-1.11,1.33-1.36,2.17L6.21,15.64z M6.83,24.09\n\tc0,0.23,0.09,0.43,0.26,0.58c0.16,0.16,0.36,0.24,0.6,0.24h9.56c0.24,0,0.44-0.08,0.6-0.23s0.25-0.35,0.25-0.59\n\ts-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.25-0.6-0.25H7.69c-0.23,0-0.43,0.09-0.6,0.26C6.92,23.66,6.83,23.86,6.83,24.09z\n\t M19.83,9.02c0.67-0.65,1.5-0.98,2.47-0.98c0.99,0,1.83,0.35,2.52,1.04c0.69,0.69,1.04,1.53,1.04,2.52c0,0.63-0.16,1.22-0.49,1.77\n\tc-0.98-0.96-2.15-1.43-3.52-1.43h-0.32C21.3,10.84,20.73,9.87,19.83,9.02z\" />",
};
#[cfg(feature = "WiNightFog")]
const WI_NIGHT_FOG: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.66,20.92c0,0.23,0.08,0.42,0.25,0.57c0.17,0.16,0.38,0.23,0.62,0.23h18.61c0.24,0,0.44-0.08,0.6-0.23\n\tc0.17-0.16,0.25-0.35,0.25-0.57c0-0.24-0.08-0.45-0.24-0.61c-0.16-0.17-0.37-0.25-0.61-0.25H3.53c-0.24,0-0.44,0.08-0.61,0.25\n\tC2.75,20.48,2.66,20.69,2.66,20.92z M5.27,17.81c0,0.24,0.09,0.43,0.26,0.59c0.14,0.18,0.33,0.27,0.59,0.27h18.61\n\tc0.23,0,0.42-0.08,0.58-0.25s0.23-0.37,0.23-0.61c0-0.23-0.08-0.43-0.23-0.58C25.16,17.08,24.96,17,24.73,17H6.12\n\tc-0.24,0-0.44,0.08-0.6,0.23C5.35,17.39,5.27,17.58,5.27,17.81z M5.42,15.39v-0.05c-0.04,0.15,0,0.22,0.12,0.22h1.44\n\tc0.06,0,0.12-0.05,0.19-0.15c0.24-0.52,0.59-0.94,1.06-1.27c0.47-0.33,0.99-0.52,1.55-0.56l0.53-0.08c0.12,0,0.19-0.06,0.19-0.18\n\tl0.06-0.5c0.11-1.08,0.56-1.97,1.36-2.7c0.8-0.72,1.75-1.08,2.84-1.08c1.07,0,2.02,0.36,2.82,1.07s1.27,1.6,1.38,2.67l0.07,0.57\n\tc0,0.12,0.07,0.18,0.21,0.18h1.58c0.64,0,1.23,0.17,1.75,0.52c0.52,0.34,0.92,0.8,1.17,1.36c0.07,0.1,0.14,0.15,0.22,0.15h1.42\n\tc0.12,0,0.17-0.07,0.15-0.22c-0.22-0.56-0.37-0.91-0.46-1.06c0.72-0.65,1.23-1.51,1.5-2.57l0.17-0.66c0.03-0.06,0.02-0.12-0.01-0.16\n\tc-0.03-0.04-0.07-0.07-0.12-0.07l-0.62-0.22c-0.89-0.26-1.57-0.78-2.04-1.58c-0.47-0.8-0.59-1.65-0.37-2.56l0.13-0.58\n\tc0.05-0.09,0.01-0.17-0.13-0.23l-0.84-0.23c-1.09-0.27-2.17-0.18-3.22,0.26c-1.05,0.44-1.87,1.15-2.47,2.12\n\tc-0.79-0.31-1.56-0.46-2.29-0.46c-1.39,0-2.62,0.44-3.71,1.31s-1.78,1.99-2.1,3.35c-0.84,0.2-1.58,0.6-2.22,1.21\n\tS5.67,14.55,5.42,15.39z M7,23.97c0,0.24,0.09,0.43,0.26,0.59c0.17,0.18,0.37,0.27,0.59,0.27H26.5c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.61c0-0.23-0.08-0.42-0.24-0.58s-0.36-0.23-0.59-0.23H7.86c-0.24,0-0.44,0.08-0.6,0.23\n\tC7.09,23.55,7,23.74,7,23.97z M18.51,8.7c0.35-0.57,0.82-1.02,1.41-1.33c0.59-0.31,1.21-0.44,1.87-0.38\n\tc-0.07,1.04,0.17,2.02,0.7,2.93c0.54,0.91,1.28,1.58,2.22,2.02c-0.15,0.35-0.4,0.71-0.75,1.07c-0.92-0.76-1.97-1.13-3.14-1.13H20.5\n\tC20.18,10.57,19.52,9.51,18.51,8.7z\" />",
};
#[cfg(feature = "WiNightHail")]
const WI_NIGHT_HAIL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.25,16.89c0,1.33,0.46,2.48,1.39,3.43c0.93,0.96,2.06,1.47,3.4,1.54c0.12,0,0.18-0.06,0.18-0.18v-1.33\n\tc0-0.12-0.06-0.18-0.18-0.18c-0.86-0.04-1.58-0.38-2.18-1.02c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.84-2.17\n\tc0.56-0.61,1.26-0.97,2.1-1.07l0.52-0.07c0.13,0,0.2-0.06,0.2-0.18l0.08-0.51c0.11-1.08,0.56-1.99,1.37-2.71\n\tc0.81-0.73,1.76-1.09,2.85-1.09c1.09,0,2.04,0.37,2.86,1.1s1.28,1.63,1.4,2.71l0.07,0.58c0,0.12,0.06,0.18,0.18,0.18h1.62\n\tc0.91,0,1.68,0.32,2.32,0.95c0.64,0.63,0.96,1.39,0.96,2.29c0,0.87-0.3,1.62-0.9,2.26c-0.6,0.64-1.33,0.98-2.18,1.02\n\tc-0.13,0-0.19,0.06-0.19,0.18v1.33c0,0.12,0.06,0.18,0.19,0.18c0.88-0.03,1.68-0.27,2.41-0.72c0.73-0.45,1.31-1.05,1.73-1.8\n\ts0.63-1.57,0.63-2.44c0-0.74-0.14-1.41-0.43-2.01c0.79-0.96,1.18-2.06,1.18-3.32c0-0.94-0.24-1.82-0.71-2.62\n\tc-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.68-0.71-2.62-0.71c-1.56,0-2.85,0.58-3.88,1.73c-0.82-0.44-1.72-0.66-2.71-0.66\n\tc-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.1,0.26-2.01,0.84-2.73,1.74C4.61,14.71,4.25,15.74,4.25,16.89z M9.62,23.87\n\tc0.09,0.22,0.24,0.37,0.46,0.46c0.2,0.1,0.41,0.11,0.62,0.02c0.22-0.08,0.36-0.23,0.45-0.45c0.09-0.22,0.09-0.44,0.01-0.65\n\tc-0.08-0.22-0.23-0.37-0.44-0.47c-0.2-0.08-0.4-0.08-0.61,0.01c-0.21,0.09-0.36,0.23-0.46,0.43C9.54,23.39,9.53,23.61,9.62,23.87z\n\t M10.25,21.04c0,0.16,0.05,0.31,0.15,0.46c0.1,0.15,0.26,0.25,0.46,0.31c0.22,0.07,0.44,0.05,0.65-0.06s0.35-0.29,0.43-0.55\n\tl0.98-3.11c0.07-0.24,0.05-0.47-0.08-0.67c-0.12-0.2-0.31-0.33-0.55-0.38c-0.22-0.07-0.43-0.05-0.62,0.06\n\tc-0.2,0.11-0.33,0.28-0.4,0.5l-1,3.18L10.25,21.04z M12.11,26.64c0,0.07,0.02,0.17,0.06,0.29c0.09,0.22,0.25,0.38,0.46,0.45\n\tc0.08,0.05,0.19,0.08,0.33,0.08c0.06,0,0.16-0.02,0.3-0.06c0.22-0.08,0.38-0.23,0.47-0.45c0.1-0.22,0.1-0.44,0-0.66\n\tc-0.1-0.22-0.25-0.37-0.45-0.46s-0.41-0.09-0.62-0.01c-0.19,0.08-0.33,0.2-0.42,0.36C12.15,26.34,12.11,26.49,12.11,26.64z\n\t M12.85,23.97c0,0.18,0.05,0.33,0.15,0.48c0.1,0.14,0.26,0.24,0.48,0.28c0.02,0,0.06,0.01,0.11,0.02s0.1,0.02,0.13,0.02\n\tc0.43-0.03,0.7-0.24,0.81-0.62l1.76-6.07c0.07-0.24,0.05-0.46-0.06-0.65c-0.11-0.19-0.28-0.32-0.5-0.39\n\tc-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.33,0.28-0.4,0.5l-1.8,6.07c0,0.02,0,0.06-0.01,0.1c-0.01,0.04-0.01,0.08-0.01,0.11\n\tC12.85,23.92,12.85,23.95,12.85,23.97z M16.29,23.57c0,0.1,0.02,0.21,0.05,0.32c0.08,0.21,0.23,0.36,0.46,0.44\n\tc0.09,0.04,0.21,0.07,0.36,0.07c0.12,0,0.22-0.02,0.29-0.06c0.23-0.09,0.38-0.23,0.46-0.43c0.08-0.22,0.08-0.43,0-0.65\n\tc-0.08-0.21-0.22-0.37-0.42-0.48c-0.22-0.08-0.44-0.08-0.65,0.01c-0.22,0.09-0.37,0.23-0.47,0.43\n\tC16.32,23.33,16.29,23.44,16.29,23.57z M17.01,21.03c0,0.36,0.21,0.61,0.62,0.75c0.14,0.04,0.24,0.06,0.3,0.06\n\tc0.12,0,0.23-0.03,0.34-0.08c0.17-0.09,0.31-0.27,0.4-0.55l0.98-3.11c0.08-0.23,0.05-0.45-0.06-0.64c-0.12-0.2-0.29-0.33-0.51-0.4\n\tc-0.23-0.07-0.44-0.05-0.64,0.06c-0.19,0.11-0.33,0.28-0.4,0.5l-0.98,3.13C17.02,20.91,17.01,21.01,17.01,21.03z M18.04,9.02\n\tc0.69-0.66,1.51-0.99,2.48-0.99c0.97,0,1.81,0.35,2.5,1.04c0.69,0.69,1.04,1.53,1.04,2.5c0,0.62-0.17,1.23-0.52,1.84\n\tc-0.98-0.98-2.14-1.47-3.49-1.47h-0.33C19.41,10.78,18.85,9.81,18.04,9.02z\" />",
};
#[cfg(feature = "WiNightLightning")]
const WI_NIGHT_LIGHTNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.29,16.93c0,0.66,0.12,1.28,0.38,1.88s0.59,1.11,1.02,1.55s0.94,0.79,1.52,1.05s1.21,0.42,1.87,0.45\n\tc0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17c-0.87-0.06-1.6-0.41-2.19-1.03c-0.59-0.62-0.89-1.37-0.89-2.22\n\tc0-0.84,0.28-1.57,0.85-2.18c0.57-0.62,1.26-0.97,2.1-1.04l0.52-0.06c0.12,0,0.19-0.06,0.19-0.18l0.08-0.52\n\tc0.07-0.71,0.3-1.36,0.69-1.94s0.9-1.04,1.52-1.36s1.29-0.49,2.02-0.49c1.09,0,2.04,0.36,2.85,1.08c0.81,0.72,1.27,1.62,1.39,2.69\n\tl0.07,0.58c0,0.11,0.06,0.17,0.19,0.17h1.6c0.9,0,1.67,0.32,2.32,0.96c0.64,0.64,0.97,1.4,0.97,2.29c0,0.86-0.3,1.6-0.89,2.22\n\tc-0.59,0.62-1.33,0.97-2.19,1.03c-0.13,0-0.2,0.06-0.2,0.17v1.34c0,0.11,0.07,0.17,0.2,0.17c1.34-0.06,2.47-0.57,3.38-1.51\n\tC24.54,19.4,25,18.26,25,16.93c0-0.64-0.16-1.32-0.47-2.06c0.79-0.99,1.19-2.08,1.19-3.27c0-0.95-0.24-1.83-0.71-2.63\n\tc-0.47-0.81-1.11-1.44-1.91-1.91s-1.68-0.7-2.62-0.7c-1.59,0-2.88,0.58-3.87,1.73c-0.81-0.43-1.7-0.64-2.66-0.64\n\tc-1.41,0-2.66,0.44-3.75,1.32s-1.79,2-2.1,3.37c-1.12,0.26-2.03,0.83-2.74,1.72C4.64,14.75,4.29,15.77,4.29,16.93z M11.94,21.55\n\tc-0.02,0.14,0.02,0.21,0.14,0.21h2.17l-1.32,4.17h0.29l4.18-5.58c0.04-0.04,0.05-0.09,0.02-0.14s-0.07-0.07-0.14-0.07H15.1l2.3-4.24\n\tc0.07-0.14,0.03-0.22-0.14-0.22h-2.94c-0.08,0-0.14,0.05-0.21,0.14L11.94,21.55z M18,9.05c0.67-0.66,1.49-0.99,2.47-0.99\n\tc0.98,0,1.81,0.34,2.5,1.03C23.66,9.77,24,10.61,24,11.6c0,0.59-0.17,1.19-0.52,1.8c-0.97-0.93-2.12-1.4-3.45-1.4h-0.31\n\tC19.44,10.81,18.86,9.83,18,9.05z\" />",
};
#[cfg(feature = "WiNightPartlyCloudy")]
const WI_NIGHT_PARTLY_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M6.77,19.61c0,0.97,0.35,1.81,1.06,2.52c0.71,0.71,1.55,1.06,2.52,1.06h6.85c0.97,0,1.8-0.35,2.49-1.05\n\tc0.69-0.7,1.04-1.54,1.04-2.53c0-0.48-0.07-0.89-0.21-1.23c0.83-0.53,1.49-1.24,1.97-2.12c0.48-0.88,0.73-1.83,0.73-2.84\n\tc0-0.81-0.16-1.59-0.47-2.33c-0.32-0.74-0.74-1.38-1.28-1.91S20.3,8.22,19.56,7.9c-0.74-0.32-1.51-0.48-2.32-0.48\n\tc-1.09,0-2.1,0.27-3.02,0.81s-1.65,1.27-2.18,2.18c-0.53,0.92-0.79,1.92-0.79,3.01v0.34c-1.01,0.57-1.67,1.41-1.99,2.49\n\tc-0.76,0.24-1.36,0.66-1.81,1.27C7,18.13,6.77,18.83,6.77,19.61z M8.75,19.61c0-0.42,0.13-0.78,0.4-1.08\n\tc0.27-0.3,0.61-0.47,1.02-0.51l0.63-0.08c0.12,0,0.19-0.08,0.19-0.23l0.1-0.56c0.07-0.58,0.31-1.06,0.73-1.44\n\tc0.42-0.39,0.91-0.58,1.48-0.58c0.58,0,1.09,0.19,1.51,0.58c0.43,0.39,0.68,0.87,0.75,1.44l0.08,0.65c0.06,0.15,0.15,0.23,0.25,0.23\n\th1.31c0.43,0,0.8,0.16,1.12,0.47c0.32,0.31,0.47,0.68,0.47,1.12c0,0.45-0.16,0.83-0.47,1.15s-0.69,0.48-1.12,0.48h-6.85\n\tc-0.45,0-0.83-0.16-1.14-0.48S8.75,20.06,8.75,19.61z M13.18,13.22c0.07-1.09,0.49-2.01,1.27-2.76c0.78-0.74,1.71-1.12,2.8-1.12\n\tc1.11,0,2.06,0.4,2.84,1.19c0.78,0.79,1.17,1.76,1.17,2.89c0,0.7-0.17,1.35-0.51,1.95c-0.34,0.6-0.8,1.08-1.38,1.45\n\tc-0.59-0.49-1.27-0.73-2.03-0.73c-0.29-0.88-0.81-1.57-1.54-2.09c-0.73-0.52-1.56-0.78-2.49-0.78H13.18z\" />",
};
#[cfg(feature = "WiNightRain")]
const WI_NIGHT_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.26,16.93c0,0.66,0.12,1.28,0.38,1.88s0.59,1.11,1.02,1.55c0.43,0.43,0.94,0.79,1.53,1.05s1.21,0.42,1.87,0.45\n\tc0.11,0,0.17-0.06,0.17-0.17v-1.34c0-0.11-0.06-0.17-0.17-0.17c-0.87-0.06-1.6-0.41-2.19-1.03c-0.59-0.62-0.89-1.37-0.89-2.22\n\tc0-0.84,0.28-1.57,0.85-2.19c0.57-0.62,1.26-0.97,2.1-1.04l0.53-0.06c0.13,0,0.2-0.06,0.2-0.19l0.06-0.51\n\tc0.11-1.09,0.56-1.99,1.37-2.71s1.76-1.08,2.86-1.08c1.09,0,2.05,0.36,2.85,1.07c0.81,0.72,1.27,1.61,1.38,2.69l0.07,0.58\n\tc0,0.12,0.06,0.18,0.19,0.18h1.6c0.9,0,1.67,0.32,2.32,0.96s0.97,1.4,0.97,2.29c0,0.86-0.3,1.6-0.89,2.22\n\tc-0.59,0.62-1.33,0.97-2.19,1.03c-0.13,0-0.2,0.06-0.2,0.17v1.34c0,0.11,0.07,0.17,0.2,0.17c1.34-0.06,2.47-0.57,3.39-1.51\n\tc0.92-0.95,1.38-2.09,1.38-3.42c0-0.72-0.14-1.38-0.42-1.99c0.78-0.94,1.17-2.06,1.17-3.36c0-0.94-0.23-1.81-0.7-2.62\n\tc-0.47-0.81-1.11-1.45-1.91-1.92s-1.68-0.71-2.62-0.71c-1.56,0-2.85,0.58-3.88,1.73c-0.88-0.43-1.78-0.65-2.7-0.65\n\tc-1.41,0-2.66,0.44-3.75,1.32s-1.79,2-2.1,3.38c-1.1,0.26-2.01,0.83-2.73,1.73C4.62,14.76,4.26,15.78,4.26,16.93z M9.75,23.61\n\tc0,0.4,0.22,0.66,0.65,0.78c0.21,0.07,0.42,0.05,0.63-0.06c0.21-0.11,0.35-0.28,0.41-0.5l1.5-5.73c0.06-0.22,0.03-0.43-0.09-0.63\n\tc-0.12-0.2-0.3-0.33-0.54-0.4c-0.22-0.07-0.43-0.05-0.63,0.07s-0.33,0.29-0.39,0.52l-1.5,5.7C9.76,23.47,9.75,23.55,9.75,23.61z\n\t M12.34,26.66c0,0.12,0.03,0.24,0.08,0.37c0.1,0.21,0.27,0.35,0.51,0.43c0.02,0,0.06,0,0.1,0.01s0.08,0.01,0.11,0.01s0.06,0,0.09,0\n\tc0.43,0,0.68-0.22,0.76-0.66l2.3-8.74c0.07-0.22,0.05-0.43-0.06-0.63c-0.11-0.2-0.28-0.33-0.5-0.4c-0.24-0.07-0.47-0.05-0.68,0.07\n\ts-0.33,0.29-0.38,0.52l-2.31,8.75C12.35,26.49,12.34,26.58,12.34,26.66z M16.5,23.6c0,0.16,0.05,0.31,0.16,0.47\n\tc0.11,0.16,0.26,0.26,0.45,0.32c0.06,0.02,0.14,0.03,0.23,0.03c0.17,0,0.33-0.05,0.49-0.14c0.16-0.09,0.26-0.24,0.32-0.45l1.5-5.73\n\tc0.08-0.21,0.05-0.41-0.07-0.62c-0.12-0.21-0.29-0.34-0.52-0.41c-0.24-0.07-0.46-0.05-0.66,0.07c-0.2,0.12-0.32,0.29-0.36,0.52\n\tl-1.5,5.7C16.51,23.47,16.5,23.55,16.5,23.6z M18.03,9.08c0.67-0.67,1.49-1,2.48-1c0.98,0,1.81,0.34,2.49,1.02\n\tc0.69,0.68,1.03,1.51,1.03,2.48c0,0.63-0.17,1.24-0.52,1.85C22.56,12.48,21.4,12,20.02,12h-0.31C19.43,10.83,18.87,9.86,18.03,9.08z\n\t\" />",
};
#[cfg(feature = "WiNightRainMix")]
const WI_NIGHT_RAIN_MIX: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.19,16.91c0,0.87,0.21,1.68,0.64,2.43c0.42,0.75,1.01,1.35,1.74,1.8C7.3,21.6,8.11,21.84,9,21.86\n\tc0.12,0,0.19-0.06,0.19-0.17v-1.34c0-0.12-0.06-0.18-0.19-0.18c-0.86-0.04-1.59-0.39-2.19-1.03s-0.91-1.39-0.91-2.24\n\tc0-0.85,0.28-1.59,0.85-2.21c0.57-0.62,1.27-0.97,2.11-1.04l0.52-0.07c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52\n\tc0.11-1.1,0.57-2.02,1.38-2.76s1.77-1.11,2.87-1.11c1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.28,1.65,1.4,2.73l0.08,0.58\n\tc0,0.11,0.06,0.17,0.18,0.17h1.62c0.9,0,1.67,0.32,2.32,0.97c0.65,0.65,0.97,1.42,0.97,2.32c0,0.85-0.3,1.6-0.91,2.24\n\tc-0.61,0.64-1.33,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17c0.88-0.02,1.69-0.26,2.42-0.72\n\tc0.73-0.45,1.31-1.05,1.73-1.8s0.63-1.56,0.63-2.43c0-0.73-0.14-1.4-0.42-2.02c0.81-0.99,1.21-2.12,1.21-3.37\n\tc0-0.96-0.24-1.84-0.71-2.65s-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71c-0.74,0-1.46,0.15-2.15,0.46\n\tC17.71,7.03,17.12,7.45,16.62,8c-0.88-0.43-1.78-0.65-2.71-0.65c-1.42,0-2.68,0.44-3.78,1.32s-1.81,2-2.12,3.37\n\tc-1.12,0.29-2.04,0.88-2.76,1.78C4.54,14.72,4.19,15.75,4.19,16.91z M9.52,23.98c0,0.17,0.05,0.34,0.16,0.51\n\tc0.11,0.17,0.27,0.28,0.47,0.35c0.23,0.07,0.44,0.06,0.64-0.04c0.19-0.09,0.32-0.28,0.39-0.56l0.14-0.61\n\tc0.05-0.23,0.02-0.44-0.09-0.63c-0.11-0.2-0.28-0.33-0.52-0.4c-0.22-0.07-0.44-0.04-0.64,0.08s-0.34,0.3-0.4,0.53l-0.14,0.59\n\tC9.53,23.83,9.52,23.89,9.52,23.98z M10.28,21.08c0,0.21,0.08,0.4,0.25,0.57c0.16,0.17,0.34,0.25,0.56,0.25\n\tc0.24,0,0.44-0.08,0.6-0.24c0.16-0.16,0.24-0.35,0.24-0.59c0-0.23-0.08-0.43-0.24-0.59c-0.16-0.16-0.36-0.24-0.6-0.24\n\tc-0.23,0-0.42,0.08-0.58,0.23C10.36,20.65,10.28,20.85,10.28,21.08z M10.89,18.81c-0.01,0.16,0.03,0.31,0.14,0.45\n\tc0.1,0.15,0.26,0.25,0.48,0.32c0.21,0.06,0.41,0.04,0.62-0.07c0.21-0.11,0.34-0.28,0.41-0.51l0.28-0.9\n\tc0.07-0.24,0.05-0.46-0.07-0.65c-0.12-0.19-0.3-0.32-0.54-0.39c-0.22-0.07-0.43-0.05-0.63,0.07c-0.2,0.11-0.34,0.28-0.41,0.5\n\tl-0.24,0.92c0,0.02-0.01,0.06-0.02,0.12C10.9,18.72,10.89,18.77,10.89,18.81z M12.05,27.1c0,0.18,0.05,0.34,0.15,0.5\n\tc0.1,0.16,0.26,0.27,0.48,0.33c0.08,0.02,0.17,0.03,0.25,0.03c0.43,0,0.69-0.2,0.79-0.61l0.14-0.59c0.06-0.26,0.03-0.48-0.08-0.68\n\ts-0.29-0.32-0.52-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.14,0.59\n\tC12.06,26.97,12.05,27.04,12.05,27.1z M12.83,24.2c0,0.22,0.08,0.41,0.25,0.58c0.16,0.16,0.35,0.24,0.57,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23\n\ts-0.43,0.08-0.59,0.23C12.91,23.77,12.83,23.97,12.83,24.2z M13.46,21.93c-0.01,0.15,0.03,0.31,0.13,0.47s0.25,0.26,0.45,0.3\n\tc0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56l0.27-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4\n\tc-0.22-0.07-0.43-0.04-0.64,0.08c-0.21,0.12-0.34,0.3-0.41,0.53l-0.23,0.9C13.47,21.74,13.46,21.83,13.46,21.93z M16.21,24.08\n\tc0,0.16,0.05,0.32,0.15,0.48s0.26,0.27,0.46,0.33c0.03,0,0.08,0.01,0.14,0.02s0.1,0.02,0.14,0.02c0.41,0,0.66-0.22,0.76-0.66\n\tl0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.11-0.21-0.28-0.34-0.51-0.41c-0.25-0.06-0.48-0.04-0.68,0.08\n\tc-0.2,0.12-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.02-0.01,0.07-0.02,0.12S16.21,24.04,16.21,24.08z M16.95,21.12\n\tc0,0.22,0.08,0.42,0.25,0.57c0.15,0.16,0.34,0.24,0.57,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.43,0.08-0.59,0.23S16.95,20.88,16.95,21.12z M17.56,18.81\n\tc0,0.17,0.05,0.33,0.16,0.48s0.27,0.26,0.49,0.32c0.02,0,0.06,0.01,0.12,0.02s0.11,0.02,0.14,0.02c0.1,0,0.22-0.03,0.36-0.09\n\tc0.21-0.11,0.35-0.29,0.41-0.52l0.24-0.9c0.06-0.23,0.04-0.44-0.07-0.63c-0.11-0.2-0.28-0.33-0.51-0.4\n\tc-0.23-0.07-0.44-0.05-0.64,0.06c-0.19,0.11-0.32,0.27-0.39,0.51l-0.28,0.91c0,0.02-0.01,0.06-0.02,0.12\n\tC17.57,18.74,17.56,18.78,17.56,18.81z M18.03,9.01c0.69-0.69,1.53-1.04,2.51-1.04c0.98,0,1.82,0.35,2.51,1.05\n\tc0.69,0.7,1.04,1.54,1.04,2.52c0,0.67-0.17,1.28-0.51,1.85c-0.96-0.96-2.14-1.44-3.54-1.44h-0.32C19.44,10.77,18.88,9.79,18.03,9.01\n\tz\" />",
};
#[cfg(feature = "WiNightRainWind")]
const WI_NIGHT_RAIN_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.11,17.02c0,1.13,0.33,2.13,1,3.01c0.67,0.88,1.54,1.48,2.62,1.8c0.1,0.01,0.18-0.01,0.25-0.08l1.13-1.46\n\tc-0.89,0-1.66-0.32-2.31-0.96s-0.97-1.41-0.97-2.31c0-0.86,0.29-1.61,0.86-2.24s1.29-0.98,2.14-1.05l0.52-0.07\n\tc0.11,0,0.16-0.05,0.16-0.14l0.07-0.56c0.12-1.1,0.59-2.02,1.41-2.76c0.82-0.74,1.78-1.11,2.88-1.11c1.11,0,2.08,0.37,2.91,1.1\n\tc0.83,0.73,1.3,1.64,1.4,2.74l0.07,0.59c0.02,0.11,0.09,0.17,0.21,0.17h1.63c0.9,0,1.67,0.33,2.32,0.98\n\tc0.65,0.66,0.98,1.44,0.98,2.35c0,0.84-0.28,1.58-0.85,2.21c-0.57,0.63-1.27,0.98-2.1,1.06c-0.48,0-0.78,0.09-0.91,0.28l-2.18,2.4\n\tc-0.16,0.18-0.22,0.39-0.19,0.62c0.03,0.23,0.13,0.45,0.31,0.63c0.13,0.17,0.33,0.25,0.59,0.23s0.46-0.15,0.6-0.38L20.64,22\n\tc0.82-0.08,1.58-0.35,2.28-0.82c0.69-0.47,1.24-1.07,1.65-1.8s0.6-1.52,0.6-2.36c0-0.63-0.14-1.32-0.43-2.08\n\tc0.77-0.98,1.15-2.08,1.15-3.32c0-0.98-0.24-1.87-0.71-2.69c-0.48-0.82-1.12-1.46-1.94-1.93S21.53,6.3,20.56,6.3\n\tc-1.57,0-2.87,0.57-3.9,1.71c-0.87-0.43-1.79-0.65-2.77-0.65c-1.43,0-2.7,0.44-3.79,1.33s-1.8,2.03-2.11,3.43\n\tc-1.14,0.26-2.07,0.84-2.79,1.75S4.11,15.83,4.11,17.02z M7.91,24.52c0,0.14,0.02,0.25,0.05,0.32c0.08,0.21,0.23,0.36,0.44,0.44\n\tc0.23,0.1,0.45,0.11,0.68,0.02c0.23-0.08,0.38-0.24,0.45-0.45c0.1-0.22,0.11-0.44,0.02-0.67c-0.09-0.23-0.24-0.38-0.46-0.46\n\tc-0.23-0.08-0.44-0.08-0.66,0c-0.21,0.08-0.37,0.23-0.47,0.45C7.93,24.25,7.91,24.37,7.91,24.52z M9.75,22.08\n\tc0,0.23,0.11,0.45,0.32,0.67c0.43,0.36,0.84,0.31,1.26-0.15l2.19-2.44c0.15-0.17,0.21-0.38,0.18-0.61\n\tc-0.03-0.23-0.13-0.42-0.31-0.57c-0.18-0.14-0.39-0.19-0.63-0.16c-0.24,0.03-0.43,0.13-0.59,0.29l-2.2,2.38\n\tC9.82,21.7,9.75,21.9,9.75,22.08z M10.39,27.01c0,0.12,0.03,0.23,0.08,0.32c0.09,0.23,0.22,0.38,0.41,0.46\n\tc0.12,0.05,0.24,0.07,0.37,0.07c0.07,0,0.18-0.02,0.32-0.06c0.21-0.09,0.36-0.24,0.44-0.45c0.1-0.2,0.11-0.41,0.02-0.64\n\tc-0.08-0.23-0.23-0.38-0.45-0.46c-0.22-0.11-0.44-0.12-0.66-0.03c-0.21,0.09-0.38,0.25-0.49,0.48C10.41,26.8,10.39,26.9,10.39,27.01\n\tz M12.06,24.62v0.13c0.02,0.24,0.12,0.44,0.32,0.6c0.14,0.18,0.34,0.26,0.6,0.24c0.25-0.02,0.45-0.15,0.6-0.38l4.22-4.91\n\tc0.16-0.18,0.22-0.39,0.2-0.64c-0.02-0.24-0.14-0.43-0.35-0.57c-0.17-0.14-0.38-0.21-0.6-0.19c-0.23,0.02-0.42,0.12-0.58,0.3\n\tl-4.22,4.92C12.12,24.25,12.06,24.41,12.06,24.62z M15.69,25.45c-0.07,0.23-0.07,0.43,0,0.62c0.09,0.22,0.24,0.38,0.45,0.49\n\tc0.11,0.05,0.23,0.07,0.36,0.07c0.06,0,0.16-0.02,0.3-0.06c0.23-0.09,0.38-0.24,0.46-0.46c0.1-0.23,0.11-0.44,0.03-0.66\n\tc-0.08-0.21-0.23-0.36-0.44-0.44c-0.23-0.11-0.45-0.12-0.66-0.03C15.97,25.07,15.8,25.23,15.69,25.45z M18.09,9.03\n\tc0.68-0.68,1.51-1.02,2.48-1.02c1.01,0,1.86,0.35,2.56,1.05s1.05,1.56,1.05,2.56c0,0.62-0.17,1.23-0.52,1.82\n\tc-0.97-0.98-2.16-1.46-3.55-1.46H19.8C19.55,10.84,18.98,9.86,18.09,9.03z\" />",
};
#[cfg(feature = "WiNightShowers")]
const WI_NIGHT_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.25,16.92c0,0.66,0.12,1.28,0.38,1.88c0.25,0.6,0.59,1.11,1.02,1.55c0.43,0.44,0.94,0.79,1.53,1.06s1.22,0.42,1.88,0.45\n\tc0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17c-0.85-0.04-1.58-0.39-2.18-1.03s-0.9-1.39-0.9-2.24\n\ts0.28-1.58,0.84-2.2s1.26-0.96,2.1-1.03l0.53-0.07c0.1,0,0.15-0.05,0.15-0.15l0.08-0.53c0.11-1.09,0.58-2,1.4-2.73\n\tc0.82-0.73,1.78-1.09,2.88-1.09c1.09,0,2.04,0.36,2.85,1.08c0.82,0.72,1.28,1.62,1.4,2.7l0.07,0.57c0,0.12,0.06,0.19,0.18,0.19h1.62\n\tc0.89,0,1.65,0.32,2.29,0.96c0.64,0.64,0.96,1.41,0.96,2.31c0,0.85-0.3,1.6-0.89,2.24s-1.32,0.98-2.17,1.03\n\tc-0.13,0-0.19,0.06-0.19,0.17v1.34c0,0.11,0.06,0.17,0.19,0.17c1.34-0.06,2.47-0.57,3.39-1.52c0.93-0.95,1.39-2.09,1.39-3.42\n\tc0-0.64-0.14-1.31-0.41-2c0.76-0.99,1.13-2.09,1.13-3.29c0-0.94-0.24-1.82-0.71-2.63s-1.11-1.45-1.92-1.92\n\tc-0.81-0.47-1.68-0.71-2.62-0.71c-1.53,0-2.82,0.56-3.86,1.67c-0.87-0.43-1.77-0.65-2.7-0.65c-1.42,0-2.68,0.44-3.77,1.32\n\ts-1.79,2.01-2.1,3.38c-1.13,0.26-2.05,0.84-2.76,1.72S4.25,15.74,4.25,16.92z M9.64,23.67c0,0.17,0.05,0.33,0.15,0.48\n\tc0.1,0.15,0.26,0.27,0.48,0.34c0.11,0.06,0.24,0.08,0.37,0.07c0.13-0.01,0.27-0.08,0.4-0.2c0.13-0.12,0.23-0.28,0.28-0.48l0.28-1.01\n\tc0.06-0.25,0.04-0.48-0.08-0.67c-0.12-0.2-0.29-0.32-0.53-0.37c-0.21-0.07-0.42-0.05-0.63,0.07s-0.34,0.28-0.41,0.5l-0.28,1.04\n\tC9.65,23.53,9.64,23.61,9.64,23.67z M10.95,18.9c0.01,0.19,0.06,0.37,0.17,0.52s0.27,0.25,0.48,0.28c0.18,0.03,0.27,0.05,0.3,0.05\n\tc0.38,0,0.63-0.22,0.76-0.66l0.28-1c0.06-0.23,0.04-0.45-0.08-0.66c-0.12-0.21-0.29-0.35-0.53-0.42c-0.22-0.06-0.44-0.04-0.64,0.08\n\ts-0.33,0.29-0.4,0.52l-0.3,1.05C10.97,18.78,10.95,18.86,10.95,18.9z M12.19,26.8c0,0.18,0.05,0.34,0.15,0.5\n\tc0.1,0.16,0.26,0.27,0.48,0.33c0.14,0.03,0.23,0.05,0.28,0.05c0.09,0,0.21-0.03,0.38-0.1c0.17-0.08,0.3-0.27,0.38-0.55l0.3-1.01\n\tc0.06-0.25,0.03-0.48-0.08-0.68c-0.12-0.2-0.29-0.33-0.52-0.37c-0.22-0.07-0.43-0.05-0.64,0.07c-0.21,0.12-0.35,0.29-0.42,0.51\n\tl-0.28,1.04C12.2,26.67,12.19,26.74,12.19,26.8z M13.52,22c0,0.17,0.05,0.33,0.16,0.49c0.11,0.16,0.27,0.27,0.49,0.33\n\tc0.22,0.07,0.44,0.05,0.63-0.05c0.2-0.1,0.33-0.29,0.41-0.56l0.28-1.01c0.07-0.25,0.05-0.47-0.07-0.67\n\tc-0.12-0.19-0.29-0.31-0.53-0.36c-0.22-0.08-0.43-0.05-0.64,0.06s-0.34,0.29-0.41,0.51l-0.28,1.04C13.54,21.88,13.52,21.95,13.52,22\n\tz M16.37,23.75c-0.01,0.16,0.03,0.31,0.14,0.46s0.26,0.26,0.46,0.33l0.25,0.03c0.11,0.01,0.24-0.02,0.38-0.07\n\tc0.21-0.08,0.35-0.26,0.42-0.54l0.28-1.05c0.07-0.23,0.05-0.45-0.07-0.64c-0.12-0.2-0.29-0.33-0.51-0.4\n\tc-0.25-0.06-0.47-0.03-0.67,0.08s-0.32,0.3-0.36,0.53l-0.29,1C16.38,23.65,16.37,23.74,16.37,23.75z M17.72,18.95\n\tc0,0.17,0.05,0.34,0.16,0.5c0.11,0.16,0.27,0.26,0.48,0.3l0.25,0.03c0.43,0,0.7-0.21,0.81-0.62l0.28-1.03\n\tc0.06-0.25,0.03-0.48-0.08-0.68s-0.3-0.32-0.53-0.37c-0.23-0.07-0.45-0.05-0.64,0.07c-0.2,0.12-0.33,0.29-0.39,0.53l-0.3,1.02\n\tC17.74,18.82,17.72,18.9,17.72,18.95z M18.06,9.05c0.67-0.64,1.48-0.97,2.45-0.97c0.98,0,1.81,0.35,2.49,1.05\n\tc0.69,0.7,1.03,1.53,1.03,2.51c0,0.64-0.16,1.23-0.48,1.77c-0.96-0.96-2.12-1.44-3.49-1.44h-0.32C19.5,10.87,18.94,9.9,18.06,9.05z\" />",
};
#[cfg(feature = "WiNightSleet")]
const WI_NIGHT_SLEET: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.23,16.99v-0.02c0-1.16,0.36-2.19,1.08-3.09s1.64-1.49,2.74-1.74c0.31-1.37,1.01-2.49,2.1-3.37s2.35-1.32,3.77-1.32\n\tc0.99,0,1.9,0.22,2.72,0.66c0.5-0.53,1.09-0.95,1.76-1.25c0.67-0.3,1.37-0.45,2.09-0.45c0.95,0,1.83,0.24,2.64,0.71\n\tc0.81,0.47,1.45,1.11,1.92,1.92s0.71,1.69,0.71,2.64c0,1.23-0.38,2.33-1.14,3.29c0.29,0.61,0.43,1.28,0.43,2.02\n\tc0,0.88-0.21,1.7-0.64,2.45c-0.42,0.75-1,1.36-1.74,1.81c-0.73,0.45-1.54,0.69-2.42,0.72c-0.13,0-0.2-0.06-0.2-0.17v-1.34\n\tc0-0.12,0.07-0.18,0.2-0.18c0.86-0.04,1.58-0.39,2.18-1.03s0.9-1.4,0.9-2.26c0-0.89-0.32-1.65-0.97-2.29\n\tc-0.64-0.64-1.41-0.96-2.31-0.96h-1.61c-0.12,0-0.18-0.06-0.18-0.18l-0.08-0.59c-0.11-1.08-0.58-1.99-1.4-2.72\n\tc-0.82-0.73-1.78-1.1-2.86-1.1c-1.1,0-2.05,0.37-2.86,1.1c-0.81,0.73-1.27,1.64-1.37,2.72l-0.08,0.59\n\tc-0.03,0.09-0.11,0.14-0.22,0.14l-0.51,0.04c-0.84,0.1-1.55,0.45-2.11,1.06s-0.84,1.34-0.84,2.18v0.04\n\tc0.01,0.01,0.02,0.02,0.03,0.02c0.01,0.85,0.31,1.59,0.9,2.22c0.28,0.29,0.59,0.52,0.92,0.67v0.02c0.38,0.19,0.79,0.31,1.24,0.34\n\tc0.11,0,0.17,0.06,0.17,0.17v1.34c0,0.11-0.06,0.17-0.17,0.17c-0.49-0.02-0.97-0.12-1.43-0.29C6.83,21.41,6.17,21,5.62,20.42\n\ts-0.95-1.24-1.18-2C4.3,17.95,4.23,17.48,4.23,16.99z M9.73,24.16c0-0.03,0.01-0.07,0.02-0.13c0.01-0.06,0.02-0.1,0.02-0.12\n\tl0.09-0.58c0.07-0.24,0.21-0.42,0.41-0.53c0.21-0.12,0.43-0.15,0.68-0.08c0.23,0.07,0.39,0.21,0.51,0.41\n\tc0.11,0.21,0.13,0.42,0.07,0.63l-0.14,0.6c-0.1,0.44-0.35,0.66-0.76,0.66c-0.03,0-0.08,0-0.15-0.01c-0.07-0.01-0.11-0.01-0.13-0.01\n\tc-0.21-0.06-0.36-0.17-0.46-0.33C9.78,24.49,9.73,24.33,9.73,24.16z M10.47,21.21c0-0.24,0.08-0.43,0.23-0.59s0.35-0.23,0.59-0.23\n\ts0.43,0.08,0.59,0.23s0.23,0.35,0.23,0.59c0,0.23-0.08,0.43-0.23,0.58s-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.57-0.24\n\tC10.56,21.62,10.47,21.43,10.47,21.21z M12.11,27.19c0-0.04,0.01-0.11,0.04-0.23l0.13-0.59c0.07-0.23,0.21-0.39,0.41-0.51\n\tc0.21-0.11,0.42-0.13,0.63-0.07c0.23,0.04,0.41,0.17,0.53,0.37c0.12,0.2,0.15,0.43,0.08,0.68l-0.13,0.58\n\tc-0.11,0.41-0.37,0.62-0.8,0.62c-0.05,0-0.13-0.01-0.24-0.04c-0.22-0.06-0.38-0.17-0.49-0.33S12.11,27.36,12.11,27.19z M12.9,24.28\n\tc0-0.23,0.08-0.42,0.23-0.58c0.16-0.15,0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23c0.16,0.15,0.23,0.35,0.23,0.58\n\tc0,0.24-0.08,0.43-0.23,0.59c-0.16,0.16-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.58-0.24C12.98,24.7,12.9,24.51,12.9,24.28z\n\t M13.52,22.01c0-0.09,0.01-0.18,0.03-0.26l0.23-0.9c0.07-0.24,0.21-0.42,0.41-0.53s0.42-0.15,0.64-0.08c0.24,0.07,0.41,0.2,0.53,0.4\n\ts0.14,0.41,0.07,0.63l-0.26,0.9c-0.08,0.27-0.22,0.46-0.41,0.56c-0.19,0.1-0.41,0.12-0.64,0.06c-0.2-0.04-0.35-0.14-0.45-0.3\n\tC13.55,22.32,13.51,22.16,13.52,22.01z M16.28,24.16c0-0.03,0-0.08,0.01-0.14c0.01-0.06,0.01-0.1,0.01-0.11l0.09-0.58\n\tc0.07-0.24,0.21-0.42,0.41-0.53c0.21-0.12,0.43-0.15,0.67-0.08c0.23,0.07,0.4,0.21,0.51,0.41c0.12,0.21,0.14,0.42,0.07,0.63\n\tl-0.14,0.6c-0.1,0.44-0.35,0.66-0.76,0.66c-0.03,0-0.08,0-0.15-0.01c-0.07-0.01-0.11-0.01-0.13-0.01c-0.2-0.06-0.35-0.17-0.45-0.33\n\tS16.28,24.33,16.28,24.16z M17.02,21.21c0-0.24,0.08-0.43,0.23-0.59s0.35-0.23,0.59-0.23c0.24,0,0.43,0.08,0.59,0.23\n\tc0.16,0.16,0.23,0.35,0.23,0.59c0,0.23-0.08,0.43-0.23,0.58c-0.16,0.16-0.35,0.23-0.59,0.23c-0.24,0-0.43-0.08-0.58-0.24\n\tC17.11,21.63,17.02,21.44,17.02,21.21z M18.06,9.1c0.84,0.76,1.4,1.74,1.7,2.93h0.31c1.38,0,2.55,0.48,3.52,1.44\n\tc0.31-0.55,0.47-1.15,0.47-1.81c0-0.98-0.35-1.81-1.04-2.5c-0.69-0.68-1.53-1.03-2.51-1.03C19.54,8.14,18.73,8.46,18.06,9.1z\" />",
};
#[cfg(feature = "WiNightSleetStorm")]
const WI_NIGHT_SLEET_STORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.19,16.88c0,1.12,0.33,2.12,1,3s1.53,1.47,2.58,1.76l-0.66,1.7c-0.05,0.14,0,0.22,0.14,0.22h2.13l-1.43,4.21h0.29\n\tl4.36-5.66c0.04-0.04,0.04-0.09,0.02-0.14c-0.02-0.05-0.07-0.07-0.14-0.07h-2.19l2.49-4.65c0.07-0.14,0.03-0.22-0.14-0.22H9.68\n\tc-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.88C7.66,19.88,7.07,19.5,6.6,18.9c-0.47-0.59-0.7-1.26-0.7-2.02c0-0.84,0.28-1.57,0.84-2.18\n\tC7.3,14.09,8,13.73,8.85,13.63l0.51-0.03c0.12,0,0.19-0.05,0.22-0.14l0.07-0.59c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1\n\tc1.09,0,2.04,0.37,2.86,1.1c0.82,0.73,1.29,1.64,1.4,2.72l0.08,0.59c0,0.11,0.06,0.17,0.18,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96\n\ts0.97,1.4,0.97,2.29c0,0.87-0.3,1.62-0.9,2.26s-1.32,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc0.88-0.02,1.69-0.26,2.42-0.72c0.73-0.45,1.31-1.06,1.74-1.81s0.64-1.57,0.64-2.45c0-0.73-0.14-1.4-0.43-2.02\n\tc0.76-0.96,1.14-2.06,1.14-3.29c0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71\n\tc-0.72,0-1.42,0.15-2.1,0.45c-0.68,0.3-1.26,0.72-1.76,1.25c-0.81-0.43-1.71-0.65-2.72-0.65c-1.42,0-2.68,0.44-3.77,1.32\n\ts-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74C4.55,14.7,4.19,15.73,4.19,16.88z M12.07,27.1c0,0.17,0.05,0.33,0.16,0.49\n\tc0.11,0.16,0.27,0.27,0.49,0.33c0.09,0.02,0.17,0.03,0.24,0.03c0.43,0,0.7-0.2,0.8-0.61l0.13-0.59c0.06-0.26,0.03-0.48-0.08-0.68\n\tc-0.12-0.2-0.29-0.32-0.53-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.13,0.59\n\tC12.08,26.99,12.07,27.07,12.07,27.1z M12.86,24.2c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23\n\tc-0.24,0-0.43,0.08-0.59,0.23S12.86,23.97,12.86,24.2z M13.48,21.93c-0.01,0.15,0.03,0.31,0.14,0.47c0.1,0.16,0.25,0.26,0.45,0.3\n\tc0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56l0.26-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4\n\tc-0.22-0.07-0.43-0.04-0.64,0.08s-0.34,0.3-0.41,0.53l-0.22,0.9C13.49,21.74,13.48,21.83,13.48,21.93z M16.24,24.08\n\tc0,0.17,0.05,0.33,0.15,0.48c0.1,0.15,0.25,0.26,0.46,0.32c0.03,0,0.08,0.01,0.14,0.02c0.06,0.01,0.11,0.02,0.14,0.02\n\tc0.41,0,0.66-0.22,0.76-0.66l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.12-0.21-0.29-0.34-0.51-0.41\n\tc-0.25-0.06-0.48-0.04-0.68,0.08s-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.01,0,0.05-0.01,0.11C16.25,24,16.24,24.04,16.24,24.08z\n\t M16.98,21.12c0,0.23,0.08,0.42,0.24,0.57c0.15,0.16,0.34,0.24,0.58,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.43,0.08-0.59,0.23S16.98,20.88,16.98,21.12z M18.02,9.02\n\tc0.67-0.64,1.48-0.97,2.45-0.97c0.98,0,1.82,0.34,2.51,1.03c0.69,0.68,1.04,1.52,1.04,2.5c0,0.66-0.16,1.26-0.47,1.81\n\tc-0.96-0.96-2.13-1.44-3.52-1.44h-0.31C19.42,10.76,18.85,9.78,18.02,9.02z\" />",
};
#[cfg(feature = "WiNightSnow")]
const WI_NIGHT_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.23,16.89c0,1.33,0.47,2.48,1.4,3.44s2.07,1.47,3.4,1.53c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.86-0.05-1.59-0.39-2.19-1.03c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.85-2.17c0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.04\n\tc0.13,0,0.2-0.06,0.2-0.18l0.07-0.54c0.11-1.08,0.56-1.99,1.37-2.72c0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.86,1.1\n\tc0.82,0.73,1.28,1.64,1.4,2.72l0.08,0.57c0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.68,0.32,2.33,0.95c0.64,0.63,0.97,1.4,0.97,2.28\n\tc0,0.86-0.3,1.61-0.91,2.25c-0.61,0.64-1.34,0.99-2.19,1.04c-0.12,0-0.19,0.06-0.19,0.17v1.34c0,0.11,0.06,0.17,0.19,0.17\n\tc1.34-0.04,2.47-0.55,3.4-1.51c0.93-0.97,1.39-2.12,1.39-3.45c0-0.71-0.14-1.38-0.43-2.01c0.79-0.96,1.18-2.07,1.18-3.32\n\tc0-0.95-0.24-1.83-0.71-2.64s-1.11-1.45-1.92-1.92s-1.68-0.7-2.62-0.7c-1.55,0-2.85,0.58-3.89,1.73c-0.81-0.43-1.71-0.65-2.71-0.65\n\tc-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.11,0.26-2.02,0.84-2.74,1.74C4.59,14.71,4.23,15.74,4.23,16.89z M10.62,21.01\n\tc0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.23,0,0.43-0.08,0.59-0.23s0.24-0.35,0.24-0.59\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.43,0.08-0.59,0.23C10.7,20.57,10.62,20.77,10.62,21.01z\n\t M10.62,24.64c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59\n\tc0-0.24-0.08-0.44-0.24-0.6s-0.35-0.25-0.59-0.25c-0.23,0-0.43,0.08-0.59,0.25S10.62,24.4,10.62,24.64z M13.81,22.9\n\tc0,0.23,0.08,0.44,0.25,0.61s0.36,0.27,0.58,0.27c0.23,0,0.43-0.09,0.6-0.26c0.17-0.17,0.25-0.38,0.25-0.62\n\tc0-0.22-0.08-0.41-0.25-0.58c-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25S13.81,22.68,13.81,22.9z M13.81,19.31\n\tc0,0.23,0.08,0.42,0.24,0.58s0.36,0.24,0.59,0.24c0.24,0,0.44-0.08,0.6-0.24s0.25-0.35,0.25-0.59s-0.08-0.43-0.25-0.6\n\ts-0.37-0.25-0.6-0.25c-0.23,0-0.42,0.08-0.59,0.25S13.81,19.08,13.81,19.31z M13.81,26.58c0,0.23,0.08,0.43,0.25,0.6\n\tc0.17,0.17,0.36,0.25,0.59,0.25c0.23,0,0.43-0.08,0.6-0.25c0.17-0.17,0.25-0.37,0.25-0.6c0-0.22-0.08-0.41-0.25-0.58\n\tc-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25C13.9,26.17,13.81,26.36,13.81,26.58z M17.05,21.01\n\tc0,0.21,0.08,0.4,0.24,0.57c0.17,0.17,0.37,0.25,0.6,0.25c0.23,0,0.43-0.08,0.59-0.23c0.16-0.16,0.24-0.35,0.24-0.59\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.23C17.13,20.57,17.05,20.77,17.05,21.01z\n\t M17.05,24.64c0,0.22,0.08,0.42,0.24,0.58s0.36,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59\n\tc0-0.24-0.08-0.44-0.24-0.6c-0.16-0.17-0.35-0.25-0.59-0.25c-0.23,0-0.43,0.08-0.6,0.25S17.05,24.4,17.05,24.64z M18.03,9.02\n\tc0.68-0.68,1.5-1.01,2.48-1.01c0.98,0,1.81,0.35,2.5,1.04s1.03,1.53,1.03,2.52c0,0.59-0.17,1.2-0.51,1.84\n\tc-0.96-0.96-2.13-1.44-3.5-1.44h-0.31C19.44,10.8,18.87,9.82,18.03,9.02z\" />",
};
#[cfg(feature = "WiNightSnowThunderstorm")]
const WI_NIGHT_SNOW_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.23,16.88c0,1.12,0.33,2.12,1,3s1.53,1.47,2.58,1.76l-0.66,1.7c-0.05,0.14,0,0.22,0.14,0.22h2.13L8,27.77h0.29l4.36-5.66\n\tc0.04-0.04,0.04-0.09,0.02-0.14c-0.02-0.05-0.07-0.07-0.14-0.07h-2.19l2.49-4.65c0.07-0.14,0.03-0.22-0.14-0.22H9.72\n\tc-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.88C7.7,19.88,7.11,19.5,6.64,18.9c-0.47-0.59-0.7-1.26-0.7-2.02c0-0.84,0.28-1.57,0.84-2.18\n\tc0.56-0.61,1.27-0.97,2.11-1.07l0.51-0.03c0.12,0,0.19-0.05,0.22-0.14l0.08-0.59c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1\n\tc1.09,0,2.04,0.37,2.86,1.1s1.29,1.64,1.4,2.72l0.08,0.59c0,0.11,0.06,0.17,0.18,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96\n\ts0.97,1.4,0.97,2.29c0,0.87-0.3,1.62-0.9,2.26s-1.32,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17\n\tc0.88-0.02,1.69-0.26,2.42-0.72c0.73-0.45,1.31-1.06,1.74-1.81s0.64-1.57,0.64-2.45c0-0.73-0.14-1.4-0.43-2.02\n\tc0.76-0.96,1.14-2.06,1.14-3.29c0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71\n\tc-0.72,0-1.42,0.15-2.1,0.45c-0.68,0.3-1.26,0.72-1.76,1.25c-0.81-0.43-1.71-0.65-2.72-0.65c-1.42,0-2.68,0.44-3.77,1.32\n\ts-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74C4.59,14.7,4.23,15.73,4.23,16.88z M13.82,22.96c0,0.24,0.08,0.44,0.24,0.59\n\tc0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.61-0.24s0.25-0.36,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61s-0.37-0.26-0.61-0.26\n\tc-0.22,0-0.41,0.09-0.58,0.26S13.82,22.72,13.82,22.96z M13.82,19.32c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24\n\tc0.24,0,0.45-0.08,0.61-0.23s0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25\n\tS13.82,19.09,13.82,19.32z M13.82,26.63c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.61-0.24\n\tc0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26\n\tC13.9,26.19,13.82,26.4,13.82,26.63z M17.05,21.02c0,0.24,0.08,0.44,0.25,0.6s0.36,0.25,0.6,0.25c0.23,0,0.43-0.08,0.59-0.25\n\tc0.16-0.17,0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58c-0.16-0.16-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.6,0.24\n\tS17.05,20.79,17.05,21.02z M17.05,24.66c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.24\n\tc0.16-0.16,0.23-0.35,0.23-0.59c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.23\n\tC17.13,24.22,17.05,24.42,17.05,24.66z M18.06,9.02c0.67-0.64,1.48-0.97,2.45-0.97c0.98,0,1.82,0.34,2.51,1.03\n\tc0.69,0.68,1.04,1.52,1.04,2.5c0,0.66-0.16,1.26-0.47,1.81c-0.96-0.96-2.13-1.44-3.52-1.44h-0.31C19.46,10.76,18.89,9.78,18.06,9.02\n\tz\" />",
};
#[cfg(feature = "WiNightSnowWind")]
const WI_NIGHT_SNOW_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.23,16.89c0,1.33,0.47,2.48,1.4,3.44s2.07,1.47,3.4,1.53c0.12,0,0.18-0.06,0.18-0.17v-1.34c0-0.11-0.06-0.17-0.18-0.17\n\tc-0.86-0.05-1.59-0.39-2.19-1.03c-0.6-0.64-0.9-1.39-0.9-2.26c0-0.83,0.28-1.55,0.85-2.17c0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.04\n\tc0.13,0,0.2-0.06,0.2-0.18l0.07-0.54c0.11-1.08,0.56-1.99,1.37-2.72c0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.86,1.1\n\tc0.82,0.73,1.28,1.64,1.4,2.72l0.08,0.57c0,0.12,0.06,0.19,0.17,0.19h1.62c0.91,0,1.68,0.32,2.33,0.95c0.64,0.63,0.97,1.4,0.97,2.28\n\tc0,0.86-0.3,1.61-0.91,2.25c-0.61,0.64-1.34,0.99-2.19,1.04c-0.12,0-0.19,0.06-0.19,0.17v1.34c0,0.11,0.06,0.17,0.19,0.17\n\tc1.34-0.04,2.47-0.55,3.4-1.51c0.93-0.97,1.39-2.12,1.39-3.45c0-0.71-0.14-1.38-0.43-2.01c0.79-0.96,1.18-2.07,1.18-3.32\n\tc0-0.95-0.24-1.83-0.71-2.64s-1.11-1.45-1.92-1.92s-1.68-0.7-2.62-0.7c-1.55,0-2.85,0.58-3.89,1.73c-0.81-0.43-1.71-0.65-2.71-0.65\n\tc-1.41,0-2.67,0.44-3.76,1.32s-1.8,2-2.11,3.36c-1.11,0.26-2.02,0.84-2.74,1.74C4.59,14.71,4.23,15.74,4.23,16.89z M9.72,24.61\n\tc0,0.21,0.08,0.4,0.24,0.57c0.18,0.16,0.37,0.24,0.58,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.24-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.23,0-0.43,0.08-0.59,0.23C9.8,24.17,9.72,24.37,9.72,24.61z\n\t M10.58,20.98c0,0.24,0.08,0.44,0.24,0.61c0.16,0.17,0.35,0.25,0.59,0.25c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61\n\tc0-0.23-0.08-0.42-0.24-0.58c-0.16-0.16-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.59,0.24C10.66,20.56,10.58,20.76,10.58,20.98z\n\t M12.47,26.56c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.6-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\ts-0.08-0.44-0.25-0.6c-0.17-0.17-0.37-0.25-0.6-0.25c-0.22,0-0.41,0.08-0.58,0.25C12.56,26.13,12.47,26.33,12.47,26.56z M13.33,22.9\n\tc0,0.22,0.08,0.42,0.25,0.6c0.16,0.16,0.35,0.24,0.57,0.24c0.24,0,0.44-0.08,0.61-0.24s0.25-0.36,0.25-0.6\n\tc0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25S13.33,22.67,13.33,22.9z M13.76,19.3\n\tc0,0.23,0.08,0.42,0.24,0.58s0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.6-0.24c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.08-0.43-0.25-0.59s-0.37-0.24-0.6-0.24c-0.22,0-0.42,0.08-0.58,0.24S13.76,19.07,13.76,19.3z M16.13,24.61\n\tc0,0.21,0.08,0.4,0.23,0.57c0.17,0.16,0.38,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.24C16.21,24.18,16.13,24.38,16.13,24.61z\n\t M16.98,20.98c0,0.24,0.08,0.44,0.24,0.61c0.16,0.17,0.36,0.25,0.59,0.25c0.23,0,0.43-0.08,0.59-0.25c0.16-0.17,0.24-0.37,0.24-0.61\n\tc0-0.23-0.08-0.42-0.24-0.58c-0.16-0.16-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.59,0.24S16.98,20.76,16.98,20.98z M18.03,9.02\n\tc0.68-0.68,1.5-1.01,2.48-1.01c0.98,0,1.81,0.35,2.5,1.04s1.03,1.53,1.03,2.52c0,0.59-0.17,1.2-0.51,1.84\n\tc-0.96-0.96-2.13-1.44-3.5-1.44h-0.31C19.44,10.8,18.87,9.82,18.03,9.02z\" />",
};
#[cfg(feature = "WiNightSprinkle")]
const WI_NIGHT_SPRINKLE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.22,16.89c0,1.33,0.46,2.48,1.39,3.44s2.06,1.47,3.41,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.37c0-0.13-0.06-0.19-0.17-0.19\n\tc-0.88-0.06-1.61-0.41-2.21-1.03c-0.6-0.62-0.9-1.36-0.9-2.21c0-0.84,0.28-1.58,0.85-2.2c0.57-0.62,1.27-0.97,2.11-1.04l0.52-0.07\n\tc0.12,0,0.19-0.06,0.19-0.19l0.07-0.5c0.11-1.08,0.57-1.99,1.38-2.72s1.77-1.1,2.87-1.1c1.09,0,2.05,0.36,2.86,1.09\n\tc0.81,0.73,1.28,1.64,1.4,2.72l0.06,0.57c0,0.12,0.06,0.18,0.19,0.18h1.6c0.91,0,1.68,0.32,2.32,0.95c0.64,0.63,0.97,1.4,0.97,2.28\n\tc0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.33,0.97-2.19,1.03c-0.14,0-0.21,0.06-0.21,0.19v1.37c0,0.11,0.07,0.17,0.21,0.17\n\tc1.33-0.04,2.46-0.55,3.39-1.51c0.93-0.97,1.39-2.12,1.39-3.45c0-0.74-0.14-1.41-0.43-2.01c0.79-0.96,1.18-2.07,1.18-3.36\n\tc0-0.94-0.24-1.82-0.71-2.63s-1.11-1.45-1.92-1.92c-0.81-0.47-1.68-0.71-2.62-0.71c-1.52,0-2.83,0.58-3.93,1.75\n\tC15.74,7.61,14.85,7.4,13.9,7.4c-1.41,0-2.67,0.44-3.76,1.31s-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74\n\tC4.58,14.71,4.22,15.74,4.22,16.89z M10.15,17.77c0,0.38,0.14,0.7,0.43,0.98c0.28,0.27,0.62,0.41,1.02,0.41s0.73-0.13,1-0.4\n\tc0.27-0.27,0.41-0.6,0.41-0.98c0-0.26-0.12-0.6-0.35-1.02c-0.23-0.42-0.45-0.76-0.66-1c-0.02-0.02-0.08-0.09-0.18-0.2\n\tc-0.1-0.11-0.17-0.19-0.21-0.24l-0.36,0.4c-0.28,0.3-0.53,0.65-0.75,1.05C10.27,17.17,10.15,17.51,10.15,17.77z M13.14,21.76\n\tc0,0.63,0.23,1.18,0.69,1.64c0.46,0.46,1.01,0.69,1.65,0.69c0.64,0,1.2-0.23,1.66-0.69c0.46-0.46,0.69-1.01,0.69-1.64\n\tc0-0.27-0.08-0.59-0.23-0.97c-0.16-0.38-0.34-0.72-0.56-1.04c-0.46-0.59-0.89-1.09-1.29-1.49c-0.06-0.04-0.14-0.13-0.26-0.24\n\tL14.9,18.6c-0.44,0.42-0.85,0.95-1.21,1.56C13.32,20.78,13.14,21.31,13.14,21.76z M14.62,15.06c0,0.27,0.09,0.49,0.28,0.67\n\ts0.43,0.27,0.71,0.27c0.26,0,0.48-0.09,0.66-0.27s0.27-0.4,0.27-0.67c0-0.41-0.31-0.94-0.93-1.61l-0.25,0.26\n\tc-0.19,0.2-0.36,0.43-0.51,0.7C14.69,14.67,14.62,14.89,14.62,15.06z M18.01,9.02c0.67-0.66,1.5-0.99,2.48-0.99\n\tc0.98,0,1.81,0.34,2.49,1.02s1.03,1.51,1.03,2.48c0,0.63-0.17,1.25-0.51,1.85c-1-0.96-2.17-1.44-3.51-1.44H19.7\n\tC19.42,10.76,18.85,9.79,18.01,9.02z\" />",
};
#[cfg(feature = "WiNightStormShowers")]
const WI_NIGHT_STORM_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.25,16.86c0,1.1,0.33,2.09,1,2.98c0.67,0.88,1.52,1.48,2.57,1.8l-0.65,1.66c-0.04,0.14,0,0.21,0.14,0.21h2.12l-1.14,4.09\n\th0.29l4.08-5.49c0.04-0.04,0.04-0.09,0.01-0.14c-0.03-0.05-0.08-0.07-0.15-0.07h-2.17l2.47-4.67c0.07-0.14,0.03-0.22-0.13-0.22H9.73\n\tc-0.09,0-0.16,0.05-0.19,0.14l-1.11,2.93c-0.71-0.18-1.3-0.57-1.78-1.17c-0.47-0.6-0.71-1.27-0.71-2.02c0-0.84,0.28-1.57,0.85-2.19\n\ts1.27-0.97,2.1-1.05l0.52-0.07c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.09,0.56-2,1.37-2.72c0.81-0.73,1.76-1.09,2.86-1.09\n\tc1.09,0,2.05,0.36,2.86,1.09c0.81,0.73,1.28,1.64,1.4,2.72l0.07,0.58c0,0.11,0.06,0.17,0.18,0.17h1.62c0.91,0,1.68,0.32,2.33,0.95\n\tc0.64,0.63,0.97,1.4,0.97,2.28c0,0.86-0.3,1.6-0.9,2.23c-0.6,0.63-1.33,0.97-2.2,1.04c-0.12,0-0.19,0.06-0.19,0.18v1.38\n\tc0,0.11,0.06,0.17,0.19,0.17c1.33-0.04,2.46-0.55,3.39-1.52c0.93-0.97,1.39-2.12,1.39-3.47c0-0.73-0.14-1.39-0.41-2\n\tc0.76-1,1.14-2.1,1.14-3.29c0-0.71-0.14-1.39-0.42-2.04c-0.28-0.65-0.66-1.2-1.12-1.67s-1.03-0.84-1.68-1.12\n\tc-0.65-0.28-1.33-0.42-2.03-0.42c-0.74,0-1.44,0.15-2.12,0.45c-0.67,0.3-1.26,0.72-1.74,1.26c-0.82-0.44-1.72-0.66-2.7-0.66\n\tc-1.42,0-2.68,0.44-3.77,1.31s-1.8,2-2.11,3.37c-1.11,0.26-2.02,0.83-2.74,1.73S4.25,15.71,4.25,16.86z M12.42,26.73\n\tc0,0.18,0.05,0.35,0.16,0.51c0.11,0.17,0.26,0.27,0.46,0.3c0.02,0,0.05,0,0.08,0.01s0.07,0.01,0.09,0.01s0.05,0,0.08,0\n\tc0.43-0.03,0.69-0.23,0.8-0.61l0.29-1.06c0.06-0.22,0.03-0.43-0.09-0.63c-0.12-0.2-0.3-0.34-0.53-0.41\n\tc-0.22-0.06-0.43-0.03-0.63,0.08c-0.2,0.12-0.34,0.3-0.41,0.53l-0.25,1C12.44,26.64,12.42,26.72,12.42,26.73z M13.76,21.96\n\tc0,0.15,0.05,0.3,0.15,0.45c0.1,0.15,0.26,0.26,0.46,0.34c0.22,0.08,0.43,0.06,0.63-0.05c0.2-0.11,0.33-0.29,0.4-0.53l0.3-1.04\n\tc0.06-0.25,0.04-0.48-0.08-0.68s-0.29-0.32-0.53-0.37c-0.22-0.07-0.44-0.05-0.64,0.07s-0.34,0.29-0.42,0.53l-0.25,1.02\n\tc0,0.02,0,0.05-0.01,0.08s-0.01,0.07-0.01,0.09C13.76,21.9,13.76,21.93,13.76,21.96z M16.59,23.58c0,0.19,0.05,0.36,0.16,0.52\n\tc0.11,0.16,0.26,0.27,0.47,0.32c0.16,0.03,0.25,0.05,0.27,0.05c0.39,0,0.65-0.2,0.77-0.6l0.24-1.06c0.07-0.22,0.05-0.43-0.06-0.63\n\tc-0.11-0.2-0.27-0.34-0.5-0.41c-0.25-0.06-0.48-0.03-0.68,0.09c-0.21,0.12-0.33,0.3-0.38,0.53l-0.28,0.99\n\tC16.6,23.43,16.59,23.5,16.59,23.58z M17.95,18.83c0,0.16,0.05,0.32,0.16,0.47c0.11,0.15,0.27,0.27,0.49,0.34\n\tc0.17,0.02,0.26,0.03,0.26,0.03c0.12,0,0.24-0.03,0.38-0.08c0.19-0.1,0.33-0.27,0.39-0.52l0.29-1.04c0.06-0.22,0.03-0.43-0.09-0.63\n\tc-0.12-0.2-0.3-0.34-0.53-0.41c-0.23-0.07-0.44-0.05-0.64,0.07s-0.33,0.29-0.4,0.53l-0.28,1.02C17.96,18.7,17.95,18.77,17.95,18.83z\n\t M18.06,8.98c0.66-0.64,1.48-0.96,2.45-0.96c0.98,0,1.82,0.35,2.5,1.04c0.69,0.69,1.03,1.53,1.03,2.51c0,0.63-0.16,1.22-0.49,1.78\n\tc-0.99-0.96-2.15-1.44-3.49-1.44h-0.32C19.44,10.75,18.88,9.77,18.06,8.98z\" />",
};
#[cfg(feature = "WiNightThunderstorm")]
const WI_NIGHT_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.28,16.89c0,1.11,0.33,2.11,0.99,2.98s1.52,1.46,2.56,1.75l-0.64,1.68c-0.05,0.14,0,0.22,0.14,0.22h2.12l-1.04,4.19h0.28\n\tl3.97-5.62c0.04-0.04,0.04-0.09,0.01-0.14c-0.03-0.05-0.08-0.07-0.15-0.07h-2.17l2.47-4.61c0.07-0.14,0.02-0.22-0.14-0.22H9.74\n\tc-0.09,0-0.16,0.05-0.23,0.14l-1.07,2.87c-0.71-0.17-1.3-0.56-1.77-1.14s-0.7-1.26-0.7-2.02c0-0.83,0.28-1.55,0.84-2.16\n\ts1.26-0.96,2.1-1.06l0.53-0.04c0.12,0,0.18-0.06,0.18-0.18l0.07-0.53c0.07-0.71,0.3-1.35,0.69-1.94c0.39-0.58,0.9-1.04,1.52-1.37\n\ts1.29-0.5,2.01-0.5c1.08,0,2.03,0.37,2.84,1.1c0.81,0.73,1.27,1.63,1.39,2.71l0.08,0.56c0,0.12,0.06,0.19,0.17,0.19h1.62\n\tc0.89,0,1.65,0.32,2.3,0.96s0.97,1.39,0.97,2.27c0,0.86-0.3,1.61-0.9,2.25s-1.33,0.97-2.18,1.02c-0.13,0-0.2,0.06-0.2,0.18v1.34\n\tc0,0.11,0.07,0.17,0.2,0.17c0.87-0.02,1.67-0.26,2.4-0.72c0.73-0.45,1.31-1.05,1.72-1.8s0.63-1.56,0.63-2.43\n\tc0-0.73-0.14-1.4-0.42-2.01c0.78-0.93,1.17-2.03,1.17-3.31c0-0.71-0.14-1.38-0.42-2.02c-0.28-0.64-0.65-1.2-1.12-1.67\n\tc-0.47-0.47-1.02-0.84-1.67-1.12c-0.64-0.28-1.32-0.42-2.02-0.42c-1.54,0-2.83,0.58-3.86,1.73c-0.81-0.43-1.71-0.65-2.7-0.65\n\tc-1.41,0-2.66,0.44-3.75,1.31s-1.79,1.99-2.1,3.35c-1.1,0.26-2.01,0.83-2.73,1.73S4.28,15.74,4.28,16.89z M12.21,26.77\n\tc0,0.16,0.05,0.32,0.15,0.46s0.25,0.25,0.45,0.31l0.25,0.03c0.42,0,0.68-0.2,0.8-0.6l2.43-8.89c0.06-0.23,0.04-0.45-0.07-0.64\n\tc-0.11-0.2-0.27-0.33-0.49-0.4c-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.34,0.28-0.4,0.5l-2.45,8.9\n\tC12.22,26.67,12.21,26.76,12.21,26.77z M16.35,23.74c0,0.4,0.21,0.67,0.62,0.8c0.17,0.02,0.26,0.03,0.26,0.03\n\tc0.11,0,0.23-0.02,0.35-0.08c0.2-0.09,0.34-0.27,0.42-0.55l1.64-5.85c0.06-0.23,0.04-0.45-0.08-0.64c-0.11-0.2-0.28-0.33-0.51-0.4\n\tc-0.23-0.07-0.45-0.05-0.65,0.06c-0.2,0.11-0.33,0.28-0.39,0.5l-1.62,5.89C16.37,23.64,16.35,23.72,16.35,23.74z M18.02,9.04\n\tc0.68-0.64,1.5-0.96,2.48-0.96c0.97,0,1.8,0.34,2.48,1.02c0.69,0.68,1.03,1.51,1.03,2.48c0,0.63-0.17,1.25-0.51,1.85\n\tc-0.96-0.96-2.12-1.44-3.48-1.44h-0.32C19.42,10.84,18.86,9.86,18.02,9.04z\" />",
};
#[cfg(feature = "WiRain")]
const WI_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,16.91c0-1.15,0.36-2.17,1.08-3.07c0.72-0.9,1.63-1.47,2.73-1.73c0.31-1.36,1.02-2.48,2.11-3.36s2.34-1.31,3.75-1.31\n\tc1.38,0,2.6,0.43,3.68,1.28c1.08,0.85,1.78,1.95,2.1,3.29h0.32c0.89,0,1.72,0.22,2.48,0.65s1.37,1.03,1.81,1.78\n\tc0.44,0.75,0.67,1.58,0.67,2.47c0,0.88-0.21,1.69-0.63,2.44c-0.42,0.75-1,1.35-1.73,1.8c-0.73,0.45-1.53,0.69-2.4,0.71\n\tc-0.13,0-0.2-0.06-0.2-0.17v-1.33c0-0.12,0.07-0.18,0.2-0.18c0.85-0.04,1.58-0.38,2.18-1.02s0.9-1.39,0.9-2.26s-0.33-1.62-0.98-2.26\n\ts-1.42-0.96-2.31-0.96h-1.61c-0.12,0-0.18-0.06-0.18-0.17l-0.08-0.58c-0.11-1.08-0.58-1.99-1.39-2.71\n\tc-0.82-0.73-1.76-1.09-2.85-1.09c-1.09,0-2.05,0.36-2.85,1.09c-0.81,0.73-1.26,1.63-1.36,2.71l-0.07,0.53c0,0.12-0.07,0.19-0.2,0.19\n\tl-0.53,0.03c-0.83,0.1-1.53,0.46-2.1,1.07s-0.85,1.33-0.85,2.16c0,0.87,0.3,1.62,0.9,2.26s1.33,0.98,2.18,1.02\n\tc0.11,0,0.17,0.06,0.17,0.18v1.33c0,0.11-0.06,0.17-0.17,0.17c-1.34-0.06-2.47-0.57-3.4-1.53S4.64,18.24,4.64,16.91z M9.99,23.6\n\tc0-0.04,0.01-0.11,0.04-0.2l1.63-5.77c0.06-0.19,0.17-0.34,0.32-0.44c0.15-0.1,0.31-0.15,0.46-0.15c0.07,0,0.15,0.01,0.24,0.03\n\tc0.24,0.04,0.42,0.17,0.54,0.37c0.12,0.2,0.15,0.42,0.08,0.67l-1.63,5.73c-0.12,0.43-0.4,0.64-0.82,0.64\n\tc-0.04,0-0.07-0.01-0.11-0.02c-0.06-0.02-0.09-0.03-0.1-0.03c-0.22-0.06-0.38-0.17-0.49-0.33C10.04,23.93,9.99,23.77,9.99,23.6z\n\t M12.61,26.41l2.44-8.77c0.04-0.19,0.14-0.34,0.3-0.44c0.16-0.1,0.32-0.15,0.49-0.15c0.09,0,0.18,0.01,0.27,0.03\n\tc0.22,0.06,0.38,0.19,0.49,0.39c0.11,0.2,0.13,0.41,0.07,0.64l-2.43,8.78c-0.04,0.17-0.13,0.31-0.29,0.43\n\tc-0.16,0.12-0.32,0.18-0.51,0.18c-0.09,0-0.18-0.02-0.25-0.05c-0.2-0.05-0.37-0.18-0.52-0.39C12.56,26.88,12.54,26.67,12.61,26.41z\n\t M16.74,23.62c0-0.04,0.01-0.11,0.04-0.23l1.63-5.77c0.06-0.19,0.16-0.34,0.3-0.44c0.15-0.1,0.3-0.15,0.46-0.15\n\tc0.08,0,0.17,0.01,0.26,0.03c0.21,0.06,0.36,0.16,0.46,0.31c0.1,0.15,0.15,0.31,0.15,0.47c0,0.03-0.01,0.08-0.02,0.14\n\ts-0.02,0.1-0.02,0.12l-1.63,5.73c-0.04,0.19-0.13,0.35-0.28,0.46s-0.32,0.17-0.51,0.17l-0.24-0.05c-0.2-0.06-0.35-0.16-0.46-0.32\n\tC16.79,23.94,16.74,23.78,16.74,23.62z\" />",
};
#[cfg(feature = "WiRainMix")]
const WI_RAIN_MIX: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.65,16.96c0,1.32,0.47,2.46,1.4,3.41c0.93,0.96,2.06,1.46,3.38,1.5c0.12,0,0.18-0.06,0.18-0.17v-1.33\n\tc0-0.12-0.06-0.18-0.18-0.18c-0.84-0.04-1.57-0.38-2.17-1.02s-0.91-1.37-0.91-2.22c0-0.84,0.28-1.57,0.85-2.19\n\tc0.57-0.62,1.26-0.97,2.1-1.04l0.53-0.07c0.12,0,0.19-0.06,0.19-0.18l0.07-0.5c0.1-1.09,0.55-2.01,1.36-2.75s1.76-1.11,2.86-1.11\n\tc1.08,0,2.03,0.37,2.84,1.1c0.81,0.73,1.28,1.63,1.4,2.71l0.07,0.58c0,0.12,0.06,0.18,0.19,0.18h1.6c0.9,0,1.67,0.32,2.32,0.97\n\tc0.64,0.64,0.97,1.41,0.97,2.3c0,0.84-0.3,1.58-0.9,2.22c-0.6,0.63-1.33,0.97-2.18,1.02c-0.13,0-0.2,0.06-0.2,0.18v1.33\n\tc0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.54,3.38-1.5s1.38-2.09,1.38-3.42c0-0.89-0.22-1.72-0.67-2.48\n\tc-0.44-0.76-1.05-1.36-1.81-1.8c-0.76-0.44-1.59-0.66-2.48-0.66h-0.31c-0.33-1.34-1.03-2.43-2.11-3.29\n\tc-1.07-0.85-2.3-1.28-3.68-1.28c-1.41,0-2.66,0.44-3.75,1.31s-1.79,1.99-2.1,3.35c-1.13,0.29-2.04,0.88-2.75,1.77\n\tS4.65,15.8,4.65,16.96z M10.05,23.98c0,0.17,0.05,0.34,0.16,0.51c0.11,0.17,0.27,0.28,0.47,0.35c0.23,0.07,0.44,0.06,0.64-0.04\n\tc0.19-0.09,0.33-0.28,0.39-0.56l0.14-0.61c0.05-0.23,0.02-0.44-0.09-0.63s-0.28-0.33-0.52-0.4c-0.22-0.07-0.44-0.04-0.64,0.08\n\ts-0.34,0.3-0.4,0.53l-0.14,0.59C10.06,23.83,10.05,23.89,10.05,23.98z M10.81,21.08c0,0.21,0.08,0.4,0.25,0.57\n\tc0.16,0.17,0.34,0.25,0.56,0.25c0.24,0,0.44-0.08,0.6-0.24c0.16-0.16,0.24-0.35,0.24-0.59c0-0.23-0.08-0.43-0.24-0.59\n\tc-0.16-0.16-0.36-0.24-0.6-0.24c-0.23,0-0.42,0.08-0.58,0.23C10.89,20.65,10.81,20.85,10.81,21.08z M11.42,18.81\n\tc-0.01,0.16,0.03,0.31,0.14,0.45c0.1,0.15,0.26,0.25,0.48,0.32c0.21,0.06,0.41,0.04,0.62-0.07S13,19.23,13.07,19l0.28-0.9\n\tc0.07-0.24,0.05-0.46-0.07-0.65c-0.12-0.19-0.3-0.32-0.54-0.39c-0.22-0.07-0.43-0.05-0.63,0.07c-0.2,0.11-0.34,0.28-0.41,0.5\n\tl-0.24,0.92c0,0.02-0.01,0.06-0.02,0.12C11.43,18.72,11.42,18.77,11.42,18.81z M12.59,27.1c0,0.18,0.05,0.34,0.15,0.5\n\tc0.1,0.16,0.26,0.27,0.48,0.33c0.08,0.02,0.17,0.03,0.25,0.03c0.43,0,0.69-0.2,0.79-0.61l0.14-0.59c0.06-0.26,0.03-0.48-0.08-0.68\n\tc-0.12-0.2-0.29-0.32-0.52-0.37c-0.21-0.07-0.42-0.05-0.63,0.07c-0.21,0.12-0.34,0.29-0.41,0.51l-0.14,0.59\n\tC12.6,26.97,12.59,27.04,12.59,27.1z M13.36,24.2c0,0.22,0.08,0.41,0.25,0.58c0.16,0.16,0.35,0.24,0.57,0.24\n\tc0.24,0,0.43-0.08,0.59-0.23c0.16-0.16,0.23-0.35,0.23-0.59c0-0.23-0.08-0.42-0.23-0.58c-0.16-0.16-0.35-0.23-0.59-0.23\n\tc-0.24,0-0.43,0.08-0.59,0.23S13.36,23.97,13.36,24.2z M13.99,21.93c-0.01,0.15,0.03,0.31,0.13,0.47c0.1,0.16,0.25,0.26,0.45,0.3\n\tc0.23,0.06,0.44,0.04,0.64-0.06s0.33-0.29,0.41-0.56l0.27-0.9c0.07-0.22,0.05-0.43-0.07-0.63c-0.12-0.2-0.29-0.33-0.53-0.4\n\tc-0.22-0.07-0.43-0.04-0.64,0.08c-0.21,0.12-0.34,0.3-0.41,0.53l-0.23,0.9C14,21.74,13.99,21.83,13.99,21.93z M16.75,24.08\n\tc0,0.16,0.05,0.32,0.15,0.48c0.1,0.16,0.26,0.27,0.46,0.33c0.03,0,0.08,0.01,0.14,0.02c0.06,0.01,0.1,0.02,0.14,0.02\n\tc0.41,0,0.66-0.22,0.76-0.66l0.14-0.6c0.07-0.21,0.05-0.42-0.07-0.63c-0.11-0.21-0.28-0.34-0.51-0.41\n\tc-0.25-0.06-0.48-0.04-0.68,0.08s-0.34,0.29-0.41,0.53l-0.09,0.59c0,0.02-0.01,0.07-0.02,0.12S16.75,24.04,16.75,24.08z\n\t M17.49,21.12c0,0.22,0.08,0.42,0.25,0.57c0.15,0.16,0.34,0.24,0.57,0.24c0.24,0,0.43-0.08,0.59-0.23s0.23-0.35,0.23-0.58\n\tc0-0.24-0.08-0.43-0.23-0.59s-0.35-0.23-0.59-0.23c-0.24,0-0.43,0.08-0.59,0.23C17.57,20.69,17.49,20.88,17.49,21.12z M18.1,18.81\n\tc0,0.17,0.05,0.33,0.16,0.48c0.11,0.15,0.27,0.26,0.49,0.32c0.02,0,0.06,0.01,0.12,0.02s0.11,0.02,0.14,0.02\n\tc0.1,0,0.22-0.03,0.36-0.09c0.21-0.11,0.35-0.29,0.41-0.52l0.24-0.9c0.06-0.23,0.04-0.44-0.08-0.63c-0.11-0.2-0.28-0.33-0.51-0.4\n\tc-0.23-0.07-0.44-0.05-0.64,0.06c-0.19,0.11-0.33,0.27-0.39,0.51l-0.28,0.91c0,0.02-0.01,0.06-0.02,0.12\n\tC18.1,18.74,18.1,18.78,18.1,18.81z\" />",
};
#[cfg(feature = "WiRainWind")]
const WI_RAIN_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.63,16.93c0,1.12,0.33,2.11,0.98,2.99c0.65,0.87,1.5,1.47,2.55,1.79c0.09,0.02,0.17-0.01,0.24-0.08l1.16-1.43\n\tc-0.89,0-1.65-0.32-2.28-0.96c-0.63-0.64-0.95-1.41-0.95-2.31c0-0.84,0.28-1.58,0.84-2.2s1.26-0.97,2.1-1.04l0.53-0.07\n\tc0.11,0,0.16-0.04,0.16-0.13l0.08-0.55c0.12-1.1,0.59-2.01,1.39-2.73s1.75-1.08,2.85-1.08c1.1,0,2.06,0.36,2.87,1.09\n\tc0.82,0.73,1.27,1.64,1.37,2.72l0.07,0.58c0.02,0.11,0.1,0.17,0.22,0.17h1.62c0.9,0,1.67,0.32,2.3,0.95s0.95,1.39,0.95,2.29\n\tc0,0.83-0.28,1.56-0.84,2.18s-1.25,0.98-2.07,1.08c-0.12,0-0.28,0.02-0.49,0.06c-0.19,0.02-0.33,0.09-0.41,0.23l-2.36,2.79\n\tc-0.14,0.18-0.2,0.39-0.16,0.63c0.03,0.24,0.14,0.43,0.31,0.57c0.11,0.12,0.29,0.19,0.56,0.19c0.26,0,0.47-0.12,0.61-0.35l2.12-2.44\n\tc1.24-0.13,2.29-0.66,3.15-1.61c0.86-0.95,1.28-2.06,1.28-3.33c0-0.67-0.13-1.32-0.39-1.93c-0.26-0.61-0.61-1.14-1.05-1.58\n\tc-0.44-0.44-0.97-0.79-1.58-1.05c-0.61-0.26-1.25-0.39-1.93-0.39h-0.32c-0.33-1.32-1.04-2.41-2.12-3.26s-2.32-1.27-3.72-1.27\n\tc-0.93,0-1.81,0.2-2.63,0.6c-0.82,0.4-1.51,0.95-2.08,1.66s-0.94,1.52-1.13,2.42c-1.12,0.25-2.04,0.82-2.75,1.72\n\tC4.98,14.74,4.63,15.77,4.63,16.93z M8.01,24.95c0,0.06,0.02,0.16,0.06,0.3c0.09,0.21,0.23,0.36,0.44,0.44\n\tc0.22,0.1,0.44,0.11,0.67,0.02c0.23-0.09,0.38-0.24,0.46-0.45c0.1-0.22,0.11-0.43,0.02-0.65c-0.09-0.21-0.24-0.36-0.46-0.43\n\tc-0.22-0.11-0.44-0.12-0.65-0.03c-0.21,0.09-0.36,0.24-0.46,0.47C8.04,24.72,8.01,24.83,8.01,24.95z M9.86,22.51v0.1\n\tc0.02,0.23,0.12,0.41,0.3,0.56c0.23,0.13,0.43,0.19,0.62,0.19c0.22,0,0.43-0.11,0.61-0.33l2.32-2.77c0.14-0.17,0.21-0.39,0.2-0.66\n\tc-0.02-0.21-0.12-0.39-0.28-0.53c-0.16-0.14-0.33-0.22-0.52-0.22c-0.06,0-0.1,0-0.14,0.01c-0.23,0.04-0.42,0.15-0.56,0.33\n\tl-2.36,2.77C9.92,22.12,9.86,22.3,9.86,22.51z M10.63,27.23c0,0.12,0.03,0.23,0.08,0.32c0.08,0.21,0.23,0.37,0.44,0.47\n\tc0.11,0.05,0.22,0.07,0.33,0.07c0.12,0,0.23-0.02,0.31-0.07c0.23-0.09,0.39-0.23,0.47-0.41c0.1-0.22,0.11-0.44,0.02-0.67\n\tc-0.08-0.23-0.23-0.38-0.45-0.46c-0.22-0.1-0.44-0.11-0.67-0.02c-0.23,0.09-0.38,0.24-0.45,0.45C10.65,27,10.63,27.11,10.63,27.23z\n\t M12.3,24.88v0.11c0.02,0.22,0.13,0.4,0.31,0.55c0.18,0.15,0.37,0.22,0.55,0.22c0.23,0,0.43-0.11,0.63-0.33l4.35-5.24\n\tc0.11-0.12,0.17-0.3,0.17-0.52v-0.12c-0.02-0.23-0.12-0.4-0.27-0.53s-0.33-0.2-0.52-0.2h-0.13c-0.23,0.01-0.42,0.12-0.55,0.31\n\tl-4.35,5.2C12.35,24.51,12.3,24.69,12.3,24.88z M15.81,26.03c0,0.09,0.02,0.19,0.06,0.3c0.09,0.22,0.24,0.38,0.46,0.47\n\tc0.14,0.04,0.24,0.06,0.31,0.06c0.14,0,0.26-0.03,0.34-0.08c0.22-0.09,0.38-0.23,0.46-0.42c0.1-0.17,0.11-0.39,0.02-0.67\n\tc-0.08-0.21-0.23-0.35-0.44-0.44l-0.36-0.09c-0.09,0.02-0.19,0.04-0.32,0.07c-0.22,0.08-0.37,0.23-0.45,0.44\n\tC15.84,25.8,15.81,25.92,15.81,26.03z\" />",
};
#[cfg(feature = "WiRaindrop")]
const WI_RAINDROP: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.81,15.25c0,0.92,0.23,1.78,0.7,2.57s1.1,1.43,1.9,1.9c0.8,0.47,1.66,0.71,2.59,0.71c0.93,0,1.8-0.24,2.61-0.71\n\tc0.81-0.47,1.45-1.11,1.92-1.9c0.47-0.8,0.71-1.65,0.71-2.57c0-0.6-0.17-1.31-0.52-2.14c-0.35-0.83-0.77-1.6-1.26-2.3\n\tc-0.44-0.57-0.96-1.2-1.56-1.88c-0.6-0.68-1.65-1.73-1.89-1.97l-1.28,1.29c-0.62,0.6-1.22,1.29-1.79,2.08\n\tc-0.57,0.79-1.07,1.64-1.49,2.55C10.01,13.79,9.81,14.58,9.81,15.25z\" />",
};
#[cfg(feature = "WiRaindrops")]
const WI_RAINDROPS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M11.01,12.23c0-0.26,0.13-0.59,0.38-1.01c0.25-0.42,0.5-0.77,0.73-1.04c0.06-0.07,0.14-0.17,0.23-0.28s0.15-0.17,0.16-0.18\n\tl0.37,0.43c0.28,0.31,0.53,0.66,0.76,1.07c0.23,0.41,0.35,0.74,0.35,1.01c0,0.41-0.14,0.77-0.43,1.06\n\tc-0.28,0.29-0.63,0.44-1.05,0.44c-0.41,0-0.77-0.15-1.06-0.44C11.16,12.99,11.01,12.64,11.01,12.23z M14.13,16.38\n\tc0-0.29,0.08-0.62,0.24-1.01c0.16-0.38,0.36-0.74,0.6-1.06c0.46-0.61,0.89-1.12,1.31-1.53c0.04-0.03,0.13-0.11,0.26-0.24l0.25,0.24\n\tc0.39,0.37,0.83,0.88,1.32,1.52c0.26,0.34,0.46,0.7,0.62,1.08s0.24,0.71,0.24,1c0,0.69-0.23,1.26-0.7,1.73\n\tc-0.47,0.47-1.05,0.7-1.73,0.7c-0.68,0-1.25-0.24-1.72-0.71S14.13,17.05,14.13,16.38z M15.65,9.48c0-0.43,0.33-1,1-1.7l0.25,0.28\n\tc0.19,0.22,0.36,0.46,0.51,0.74c0.15,0.27,0.23,0.5,0.23,0.68c0,0.28-0.1,0.5-0.29,0.69c-0.19,0.18-0.42,0.28-0.7,0.28\n\tc-0.29,0-0.53-0.09-0.72-0.28C15.75,9.98,15.65,9.75,15.65,9.48z\" />",
};
#[cfg(feature = "WiRefresh")]
const WI_REFRESH: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.77,15.53c0,0.94,0.24,1.82,0.71,2.62c0.47,0.8,1.11,1.44,1.91,1.9c0.8,0.47,1.67,0.7,2.61,0.7\n\tc0.96,0,1.83-0.23,2.63-0.69c0.8-0.46,1.43-1.09,1.89-1.89c0.46-0.8,0.69-1.68,0.69-2.63c0-0.24-0.08-0.44-0.24-0.61\n\tc-0.16-0.17-0.35-0.25-0.59-0.25c-0.23,0-0.43,0.08-0.6,0.25c-0.17,0.17-0.25,0.37-0.25,0.61c0,0.98-0.35,1.82-1.04,2.51\n\tc-0.69,0.69-1.53,1.04-2.51,1.04c-0.97,0-1.79-0.35-2.47-1.04c-0.68-0.69-1.02-1.53-1.02-2.51c0-0.85,0.26-1.62,0.79-2.31\n\ts1.14-1.06,1.84-1.1l-0.38,0.37c-0.16,0.18-0.24,0.37-0.24,0.58c0,0.22,0.08,0.42,0.24,0.6c0.36,0.35,0.77,0.35,1.21,0l1.84-1.82\n\tc0.16-0.12,0.24-0.33,0.24-0.62c0-0.26-0.08-0.45-0.24-0.57L14.97,8.8c-0.18-0.16-0.37-0.24-0.57-0.24c-0.25,0-0.46,0.08-0.63,0.25\n\tc-0.17,0.17-0.25,0.37-0.25,0.6c0,0.24,0.08,0.45,0.24,0.61l0.38,0.36c-1.25,0.22-2.29,0.82-3.12,1.8S9.77,14.27,9.77,15.53z\" />",
};
#[cfg(feature = "WiRefreshAlt")]
const WI_REFRESH_ALT: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M11.78,14.91c0,0.79,0.19,1.51,0.57,2.17c0.38,0.66,0.9,1.19,1.57,1.57c0.67,0.38,1.39,0.58,2.18,0.58\n\tc0.19,0,0.35-0.07,0.48-0.22c0.13-0.14,0.2-0.31,0.2-0.51c0-0.19-0.07-0.35-0.2-0.48s-0.29-0.19-0.49-0.19\n\tc-0.81,0-1.5-0.28-2.07-0.85c-0.57-0.57-0.85-1.26-0.85-2.07c0-0.78,0.27-1.45,0.8-2.02s1.16-0.86,1.88-0.86l-0.33,0.32\n\tc-0.15,0.15-0.22,0.31-0.21,0.49c0,0.18,0.07,0.34,0.2,0.48c0.13,0.14,0.29,0.21,0.49,0.21c0.2,0,0.37-0.07,0.51-0.21l1.51-1.5\n\tc0.13-0.11,0.2-0.27,0.2-0.51c0-0.22-0.07-0.38-0.2-0.47l-1.51-1.53c-0.13-0.14-0.29-0.21-0.49-0.21s-0.36,0.07-0.5,0.21\n\ts-0.21,0.3-0.21,0.5c0,0.21,0.07,0.38,0.22,0.51l0.3,0.28c-1.15,0.08-2.11,0.53-2.89,1.35C12.17,12.77,11.78,13.76,11.78,14.91z\" />",
};
#[cfg(feature = "WiSandstorm")]
const WI_SANDSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.1,16.97c0,0.24,0.09,0.45,0.28,0.62c0.16,0.19,0.37,0.28,0.63,0.28H18.7c0.29,0,0.53,0.1,0.73,0.3\n\tc0.2,0.2,0.3,0.45,0.3,0.74c0,0.29-0.1,0.53-0.3,0.72c-0.2,0.19-0.44,0.29-0.74,0.29c-0.29,0-0.54-0.1-0.73-0.29\n\tc-0.16-0.18-0.36-0.26-0.6-0.26c-0.25,0-0.46,0.09-0.64,0.26s-0.27,0.38-0.27,0.61c0,0.25,0.09,0.46,0.28,0.63\n\tc0.56,0.55,1.22,0.83,1.96,0.83c0.78,0,1.45-0.27,2.01-0.81c0.56-0.54,0.83-1.19,0.83-1.97s-0.28-1.44-0.84-2\n\tc-0.56-0.56-1.23-0.84-2-0.84H4.01c-0.25,0-0.46,0.09-0.64,0.26C3.19,16.51,3.1,16.72,3.1,16.97z M3.1,13.69\n\tc0,0.23,0.09,0.43,0.28,0.61c0.17,0.18,0.38,0.26,0.63,0.26h20.04c0.78,0,1.45-0.27,2.01-0.82c0.56-0.54,0.84-1.2,0.84-1.97\n\tc0-0.77-0.28-1.44-0.84-1.99s-1.23-0.83-2.01-0.83c-0.77,0-1.42,0.27-1.95,0.8c-0.18,0.16-0.27,0.38-0.27,0.67\n\tc0,0.26,0.09,0.47,0.26,0.63c0.17,0.16,0.38,0.24,0.63,0.24c0.24,0,0.45-0.08,0.63-0.24c0.19-0.21,0.42-0.31,0.7-0.31\n\tc0.29,0,0.53,0.1,0.73,0.3c0.2,0.2,0.3,0.44,0.3,0.73c0,0.29-0.1,0.53-0.3,0.72c-0.2,0.19-0.44,0.29-0.73,0.29H4.01\n\tc-0.25,0-0.46,0.09-0.64,0.26C3.19,13.23,3.1,13.44,3.1,13.69z M4.67,20.61c0,0.24,0.08,0.43,0.24,0.58\n\tc0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.45-0.08,0.62-0.23s0.25-0.35,0.25-0.59c0-0.23-0.09-0.43-0.26-0.6\n\tc-0.17-0.17-0.38-0.25-0.61-0.25c-0.22,0-0.42,0.08-0.58,0.25C4.75,20.18,4.67,20.38,4.67,20.61z M5.62,10.68\n\tc0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.45-0.08,0.61-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.08-0.43-0.25-0.6C6.89,9.92,6.68,9.83,6.45,9.83c-0.22,0-0.42,0.08-0.58,0.25C5.7,10.25,5.62,10.45,5.62,10.68z\n\t M8.65,8.37c0,0.24,0.08,0.43,0.24,0.58C9.05,9.12,9.25,9.2,9.47,9.2c0.24,0,0.45-0.08,0.62-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.09-0.43-0.26-0.6C9.92,7.61,9.71,7.53,9.47,7.53c-0.22,0-0.42,0.08-0.58,0.25C8.73,7.94,8.65,8.14,8.65,8.37z M8.74,19.97\n\tc0,0.23,0.08,0.43,0.25,0.58c0.16,0.16,0.35,0.24,0.57,0.24c0.24,0,0.45-0.08,0.62-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.09-0.43-0.26-0.6c-0.17-0.17-0.38-0.25-0.61-0.25c-0.22,0-0.42,0.08-0.58,0.25C8.82,19.54,8.74,19.74,8.74,19.97z\n\t M12.92,10.14c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.45-0.08,0.62-0.23s0.25-0.35,0.25-0.59\n\tc0-0.23-0.09-0.43-0.26-0.6c-0.17-0.17-0.38-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25C13,9.71,12.92,9.91,12.92,10.14z\n\t M13.24,21.13c0,0.23,0.08,0.42,0.25,0.58c0.16,0.16,0.35,0.24,0.57,0.24c0.24,0,0.45-0.08,0.62-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.09-0.43-0.26-0.6c-0.17-0.17-0.38-0.25-0.61-0.25c-0.22,0-0.42,0.08-0.58,0.25S13.24,20.9,13.24,21.13z M17.72,9.98\n\tc0,0.23,0.08,0.42,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.45-0.08,0.62-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.09-0.43-0.26-0.6c-0.17-0.17-0.38-0.25-0.61-0.25c-0.22,0-0.41,0.08-0.58,0.25C17.81,9.55,17.72,9.75,17.72,9.98z\n\t M22.81,17.04c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.45-0.08,0.61-0.23c0.17-0.16,0.25-0.35,0.25-0.59\n\tc0-0.23-0.08-0.43-0.25-0.6c-0.17-0.17-0.37-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25C22.9,16.61,22.81,16.81,22.81,17.04z\" />",
};
#[cfg(feature = "WiShowers")]
const WI_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.6,16.93c0-1.16,0.36-2.18,1.09-3.08c0.72-0.9,1.65-1.48,2.78-1.73c0.29-1.38,0.98-2.5,2.07-3.39S12.88,7.4,14.3,7.4\n\tc1.39,0,2.63,0.43,3.72,1.28c1.08,0.85,1.79,1.95,2.12,3.3h0.34c0.9,0,1.73,0.22,2.48,0.66c0.76,0.44,1.35,1.04,1.79,1.8\n\tc0.43,0.76,0.65,1.59,0.65,2.49c0,1.34-0.46,2.48-1.37,3.44c-0.92,0.96-2.04,1.46-3.37,1.5c-0.12,0-0.18-0.06-0.18-0.17v-1.34\n\tc0-0.11,0.06-0.17,0.18-0.17c0.84-0.07,1.57-0.42,2.17-1.05s0.9-1.37,0.9-2.22c0-0.89-0.32-1.66-0.96-2.31\n\tc-0.64-0.64-1.4-0.97-2.29-0.97h-1.63c-0.12,0-0.19-0.06-0.22-0.18l-0.07-0.57c-0.07-0.71-0.3-1.36-0.7-1.94s-0.91-1.03-1.53-1.36\n\tc-0.62-0.33-1.3-0.49-2.02-0.49c-1.1,0-2.05,0.36-2.86,1.09c-0.81,0.73-1.27,1.64-1.37,2.72l-0.07,0.54c0,0.09-0.05,0.14-0.16,0.14\n\tL9.31,13.7c-0.84,0.07-1.55,0.41-2.11,1.03c-0.57,0.62-0.85,1.35-0.85,2.2c0,0.87,0.3,1.62,0.89,2.25c0.59,0.63,1.31,0.97,2.17,1.02\n\tc0.12,0,0.18,0.06,0.18,0.17v1.34c0,0.11-0.06,0.17-0.18,0.17c-0.66-0.03-1.28-0.18-1.88-0.45S6.42,20.8,6,20.36\n\tc-0.43-0.44-0.77-0.95-1.02-1.55S4.6,17.59,4.6,16.93z M10.02,23.7c0-0.03,0.01-0.08,0.02-0.13s0.02-0.09,0.02-0.11l0.27-1.03\n\tc0.07-0.22,0.2-0.4,0.4-0.51c0.2-0.12,0.41-0.14,0.64-0.07c0.23,0.07,0.4,0.2,0.52,0.4c0.12,0.2,0.14,0.41,0.07,0.64l-0.24,1.01\n\tc-0.13,0.44-0.38,0.66-0.76,0.66c-0.03,0-0.05,0-0.09,0c-0.03,0-0.07-0.01-0.11-0.01c-0.04-0.01-0.07-0.01-0.1-0.01\n\tc-0.21-0.06-0.37-0.18-0.48-0.34S10.02,23.86,10.02,23.7z M11.34,18.88c0-0.02,0-0.06,0.01-0.11c0.01-0.05,0.01-0.08,0.01-0.09\n\tl0.3-1.05c0.06-0.19,0.17-0.34,0.32-0.45c0.15-0.1,0.31-0.15,0.47-0.15c0.02,0,0.05,0,0.08,0c0.03,0,0.06,0.01,0.09,0.01\n\tc0.03,0.01,0.06,0.01,0.08,0.01c0.23,0.07,0.4,0.2,0.51,0.4c0.12,0.2,0.14,0.41,0.07,0.64l-0.24,1c-0.07,0.28-0.2,0.47-0.4,0.59\n\ts-0.42,0.12-0.65,0.02c-0.22-0.06-0.38-0.17-0.49-0.34S11.34,19.04,11.34,18.88z M12.57,26.83c0-0.03,0.01-0.07,0.02-0.13\n\ts0.02-0.09,0.02-0.12l0.29-0.99c0.06-0.24,0.2-0.42,0.4-0.54c0.2-0.12,0.42-0.15,0.65-0.08c0.23,0.07,0.39,0.2,0.51,0.41\n\ts0.13,0.42,0.07,0.65l-0.25,1.04c-0.11,0.41-0.37,0.61-0.8,0.61c-0.05,0-0.13-0.01-0.24-0.04c-0.22-0.04-0.38-0.14-0.49-0.3\n\tC12.63,27.18,12.57,27.01,12.57,26.83z M13.91,22.06c0-0.06,0.01-0.14,0.04-0.25l0.27-1.03c0.07-0.23,0.2-0.4,0.41-0.51\n\tc0.2-0.12,0.42-0.14,0.65-0.07c0.23,0.06,0.39,0.19,0.51,0.39c0.11,0.2,0.13,0.41,0.06,0.65l-0.24,0.99\n\tc-0.13,0.45-0.37,0.68-0.72,0.68c-0.04,0-0.15-0.02-0.31-0.06c-0.22-0.04-0.38-0.14-0.49-0.3C13.97,22.4,13.91,22.23,13.91,22.06z\n\t M16.73,23.74c0-0.07,0.01-0.15,0.03-0.24l0.28-0.99c0.07-0.24,0.2-0.42,0.41-0.54s0.41-0.15,0.63-0.09\n\tc0.23,0.07,0.41,0.2,0.53,0.41c0.12,0.2,0.15,0.41,0.09,0.63l-0.29,1.06c-0.1,0.41-0.36,0.61-0.79,0.61c-0.09,0-0.18-0.01-0.26-0.03\n\tc-0.2-0.04-0.35-0.14-0.46-0.3C16.8,24.08,16.74,23.91,16.73,23.74z M18.11,18.98c0-0.03,0.02-0.12,0.05-0.26l0.3-1.03\n\tc0.04-0.21,0.13-0.37,0.29-0.47c0.16-0.1,0.32-0.15,0.49-0.14c0.04-0.01,0.13,0,0.24,0.03c0.22,0.05,0.39,0.18,0.52,0.38\n\tc0.12,0.17,0.14,0.38,0.07,0.65l-0.24,1.03c-0.13,0.43-0.38,0.65-0.76,0.65c-0.06,0-0.17-0.02-0.34-0.06\n\tc-0.21-0.06-0.36-0.17-0.46-0.31C18.16,19.29,18.11,19.14,18.11,18.98z\" />",
};
#[cfg(feature = "WiSleet")]
const WI_SLEET: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.09,16.95c0-1.16,0.36-2.19,1.08-3.09s1.64-1.49,2.74-1.74c0.31-1.37,1.01-2.49,2.1-3.37s2.35-1.32,3.77-1.32\n\tc0.81,0,1.55,0.13,2.2,0.4c0,0.01,0,0.01,0.01,0.02c0.84,0.32,1.58,0.84,2.21,1.55h0.03c0.68,0.73,1.13,1.6,1.37,2.62h0.31\n\tc1.08,0,2.02,0.29,2.83,0.86v-0.01c0.53,0.36,0.98,0.8,1.34,1.33c0.36,0.53,0.6,1.11,0.73,1.74c0.04,0.21,0.06,0.38,0.08,0.52v0.06\n\tc0,0.01,0,0.06,0.01,0.17s0.01,0.19,0.01,0.24v0.03c0,0.88-0.21,1.7-0.64,2.45c-0.42,0.75-1,1.36-1.74,1.81\n\tc-0.73,0.45-1.54,0.69-2.42,0.72c-0.13,0-0.2-0.06-0.2-0.17v-1.34c0-0.13,0.07-0.19,0.2-0.19c0.86-0.04,1.58-0.38,2.18-1.02\n\tc0.6-0.64,0.9-1.39,0.9-2.26c0-0.89-0.32-1.65-0.97-2.29c-0.64-0.64-1.41-0.96-2.31-0.96h-1.61c-0.12,0-0.18-0.06-0.18-0.17\n\tl-0.08-0.59c-0.1-1-0.52-1.86-1.27-2.59c-0.01-0.01-0.01-0.02-0.02-0.03s-0.02-0.02-0.02-0.03c-0.01-0.01-0.02-0.02-0.04-0.02\n\tc0-0.02-0.01-0.03-0.02-0.03c-0.73-0.66-1.58-1.04-2.56-1.12c-0.07-0.01-0.18-0.01-0.34-0.01c-1.1,0-2.05,0.37-2.86,1.1\n\ts-1.27,1.64-1.37,2.72l-0.08,0.59c-0.03,0.09-0.11,0.14-0.22,0.14L8.75,13.7c-0.84,0.1-1.55,0.46-2.11,1.07s-0.84,1.34-0.84,2.18\n\tv0.04h0.03c0.01,0.48,0.11,0.93,0.3,1.35c0.2,0.43,0.46,0.79,0.8,1.09c0.21,0.18,0.45,0.34,0.74,0.48v0.01\n\tc0.4,0.19,0.8,0.3,1.21,0.32c0.11,0,0.17,0.06,0.17,0.18v1.34c0,0.11-0.06,0.17-0.17,0.17c-0.52-0.03-1.01-0.13-1.48-0.3v0.01\n\tc-0.83-0.29-1.54-0.77-2.11-1.43s-0.95-1.44-1.11-2.31v-0.03c-0.01-0.01-0.01-0.02-0.01-0.04C4.11,17.57,4.09,17.27,4.09,16.95z\n\t M9.59,24.12c0-0.03,0.01-0.07,0.02-0.13c0.01-0.05,0.02-0.09,0.02-0.12l0.09-0.59c0.07-0.24,0.2-0.41,0.41-0.53\n\ts0.43-0.14,0.68-0.08c0.23,0.07,0.39,0.21,0.51,0.41c0.11,0.21,0.13,0.42,0.07,0.63l-0.14,0.6c-0.1,0.44-0.35,0.66-0.76,0.66\n\tc-0.03,0-0.08-0.01-0.14-0.02c-0.06-0.01-0.1-0.02-0.14-0.02c-0.21-0.06-0.36-0.17-0.46-0.33C9.64,24.45,9.59,24.29,9.59,24.12z\n\t M10.33,21.18c0-0.24,0.08-0.43,0.23-0.59s0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23s0.23,0.35,0.23,0.59c0,0.23-0.08,0.42-0.23,0.58\n\ts-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.57-0.24C10.42,21.59,10.33,21.4,10.33,21.18z M11.97,27.17c0-0.04,0.01-0.11,0.04-0.23\n\tl0.13-0.59c0.07-0.23,0.21-0.4,0.41-0.51c0.21-0.12,0.42-0.14,0.63-0.07c0.23,0.04,0.41,0.17,0.53,0.37\n\tc0.12,0.2,0.15,0.43,0.08,0.68l-0.13,0.59c-0.1,0.41-0.37,0.61-0.8,0.61c-0.07,0-0.16-0.01-0.24-0.03\n\tc-0.22-0.06-0.38-0.17-0.49-0.33C12.03,27.5,11.97,27.34,11.97,27.17z M12.76,24.26c0-0.23,0.08-0.43,0.23-0.58\n\tc0.16-0.16,0.35-0.23,0.59-0.23s0.43,0.08,0.59,0.23c0.16,0.16,0.23,0.35,0.23,0.58c0,0.24-0.08,0.43-0.23,0.59\n\tc-0.16,0.16-0.35,0.23-0.59,0.23c-0.23,0-0.42-0.08-0.58-0.24C12.84,24.68,12.76,24.49,12.76,24.26z M13.38,21.99\n\tc0-0.1,0.01-0.19,0.03-0.27l0.23-0.9c0.07-0.23,0.21-0.41,0.41-0.53c0.21-0.12,0.42-0.15,0.64-0.08c0.24,0.07,0.41,0.2,0.53,0.4\n\ts0.14,0.41,0.07,0.63l-0.26,0.9c-0.08,0.28-0.22,0.46-0.41,0.56c-0.19,0.1-0.41,0.12-0.64,0.06c-0.2-0.04-0.35-0.14-0.45-0.3\n\tC13.41,22.3,13.37,22.14,13.38,21.99z M16.14,24.13c0-0.03,0-0.08,0.01-0.13s0.01-0.09,0.01-0.11l0.09-0.59\n\tc0.07-0.24,0.2-0.41,0.41-0.53s0.43-0.14,0.68-0.08c0.23,0.07,0.4,0.21,0.51,0.41c0.12,0.21,0.14,0.42,0.07,0.63l-0.14,0.6\n\tc-0.1,0.44-0.35,0.66-0.76,0.66c-0.03,0-0.08-0.01-0.14-0.02c-0.06-0.01-0.11-0.02-0.14-0.02c-0.2-0.06-0.35-0.17-0.45-0.33\n\tC16.2,24.46,16.14,24.3,16.14,24.13z M16.88,21.19c0-0.24,0.08-0.43,0.23-0.59c0.16-0.16,0.35-0.23,0.59-0.23\n\tc0.24,0,0.43,0.08,0.59,0.23c0.16,0.16,0.23,0.35,0.23,0.59c0,0.23-0.08,0.43-0.23,0.58C18.14,21.92,17.95,22,17.71,22\n\tc-0.24,0-0.43-0.08-0.58-0.24C16.97,21.61,16.88,21.42,16.88,21.19z\" />",
};
#[cfg(feature = "WiSmallCraftAdvisory")]
const WI_SMALL_CRAFT_ADVISORY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.81,24.6V7.45h1.03V24.6H9.81z M11.54,14.86V7.45l8.65,3.69L11.54,14.86z\" />",
};
#[cfg(feature = "WiSmog")]
const WI_SMOG: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M6.35,12.46c0-0.94,0.3-1.77,0.9-2.5s1.37-1.21,2.31-1.43c0.23-1.11,0.79-2.03,1.7-2.75c0.91-0.72,1.95-1.08,3.12-1.08\n\tc1.12,0,2.13,0.35,3,1.04c0.88,0.69,1.45,1.59,1.72,2.7h0.28c0.76,0,1.46,0.16,2.12,0.49s1.18,0.77,1.57,1.34\n\tc0.39,0.57,0.59,1.18,0.59,1.84c0,1.12-0.43,2.08-1.29,2.86c0,0.35-0.11,0.75-0.32,1.2c-0.22,0.45-0.5,0.86-0.87,1.23\n\tc-0.36,0.37-0.73,0.59-1.1,0.68c-0.12,0.62-0.41,1.14-0.86,1.57s-0.99,0.71-1.63,0.85c0.3,0.3,0.45,0.65,0.45,1.06\n\tc0,0.49-0.17,0.91-0.52,1.26c-0.35,0.35-0.77,0.52-1.27,0.52c-0.49,0-0.91-0.17-1.26-0.52c-0.35-0.35-0.53-0.77-0.53-1.26\n\tc0-0.06,0.01-0.14,0.04-0.26s0.04-0.21,0.04-0.27h-0.08c-0.59,0-1.09-0.21-1.51-0.63c-0.42-0.42-0.63-0.93-0.63-1.51\n\tc0-0.23,0.12-0.58,0.37-1.06c-0.49-0.26-0.88-0.67-1.17-1.26h-1.25c-1.09-0.09-2.02-0.53-2.78-1.3C6.73,14.49,6.35,13.55,6.35,12.46\n\tz\" />",
};
#[cfg(feature = "WiSmoke")]
const WI_SMOKE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M6.34,12.48c0-0.94,0.3-1.78,0.89-2.52s1.34-1.21,2.25-1.41C9.73,7.43,10.3,6.5,11.2,5.78s1.92-1.08,3.08-1.08\n\tc1.12,0,2.13,0.35,3.02,1.05c0.89,0.7,1.46,1.6,1.73,2.69h0.27c1.12,0,2.08,0.39,2.88,1.18c0.79,0.78,1.19,1.74,1.19,2.85\n\tc0,0.6-0.12,1.17-0.37,1.7c-0.25,0.53-0.59,0.99-1.03,1.37v0.03c0,0.59-0.19,1.12-0.56,1.59c-0.37,0.47-0.84,0.76-1.4,0.89\n\tc-0.14,0.62-0.45,1.15-0.91,1.58c-0.46,0.43-1.01,0.7-1.63,0.8c0.29,0.34,0.43,0.72,0.43,1.13c0,0.48-0.17,0.89-0.51,1.24\n\tc-0.34,0.34-0.75,0.52-1.23,0.52c-0.48,0-0.89-0.17-1.23-0.52c-0.34-0.34-0.51-0.76-0.51-1.24c0-0.19,0.03-0.38,0.1-0.57h-0.1\n\tc-0.58,0-1.08-0.21-1.5-0.63c-0.42-0.42-0.63-0.92-0.63-1.5c0-0.4,0.1-0.76,0.3-1.07c-0.52-0.29-0.89-0.7-1.12-1.25h-1.28v-0.01\n\tc-1.07-0.07-1.98-0.49-2.73-1.27S6.34,13.56,6.34,12.48z M7.74,12.23c0,0.8,0.28,1.48,0.84,2.04s1.24,0.84,2.03,0.84\n\tc0.49,0,0.95-0.11,1.37-0.34c0.12,0.74,0.47,1.36,1.04,1.86s1.25,0.74,2.02,0.74c0.87,0,1.61-0.31,2.22-0.92\n\tc0.41,0.48,0.92,0.71,1.54,0.71c0.57,0,1.05-0.2,1.46-0.6c0.4-0.4,0.6-0.89,0.6-1.46c0.4-0.27,0.72-0.61,0.95-1.04\n\tc0.23-0.42,0.35-0.88,0.35-1.37c0-0.79-0.28-1.47-0.85-2.02c-0.57-0.55-1.25-0.83-2.05-0.83c-0.56,0-1.07,0.15-1.53,0.44\n\tc0.06-0.24,0.08-0.51,0.08-0.79c0-0.96-0.34-1.78-1.03-2.46c-0.69-0.68-1.52-1.01-2.49-1.01c-0.94,0-1.75,0.33-2.43,0.97\n\ts-1.04,1.44-1.07,2.37c-0.02,0-0.05,0-0.08,0c-0.04,0-0.07,0-0.09,0c-0.79,0-1.46,0.28-2.03,0.84S7.74,11.45,7.74,12.23z\" />",
};
#[cfg(feature = "WiSnow")]
const WI_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,16.95c0-1.16,0.35-2.18,1.06-3.08s1.62-1.48,2.74-1.76c0.31-1.36,1.01-2.48,2.1-3.36s2.34-1.31,3.75-1.31\n\tc1.38,0,2.6,0.43,3.68,1.28c1.08,0.85,1.78,1.95,2.1,3.29h0.32c0.89,0,1.72,0.22,2.48,0.66c0.76,0.44,1.37,1.04,1.81,1.8\n\tc0.44,0.76,0.67,1.59,0.67,2.48c0,1.32-0.46,2.47-1.39,3.42c-0.92,0.96-2.05,1.46-3.38,1.5c-0.13,0-0.2-0.06-0.2-0.17v-1.33\n\tc0-0.12,0.07-0.18,0.2-0.18c0.85-0.04,1.58-0.38,2.18-1.02s0.9-1.38,0.9-2.23c0-0.89-0.32-1.65-0.97-2.3s-1.42-0.97-2.32-0.97h-1.61\n\tc-0.12,0-0.18-0.06-0.18-0.17l-0.08-0.58c-0.11-1.08-0.58-1.99-1.39-2.72c-0.82-0.73-1.76-1.1-2.85-1.1c-1.1,0-2.05,0.37-2.86,1.11\n\tc-0.81,0.74-1.27,1.65-1.37,2.75l-0.06,0.5c0,0.12-0.07,0.19-0.2,0.19l-0.53,0.07c-0.83,0.07-1.53,0.41-2.1,1.04\n\ts-0.85,1.35-0.85,2.19c0,0.85,0.3,1.59,0.9,2.23s1.33,0.97,2.18,1.02c0.11,0,0.17,0.06,0.17,0.18v1.33c0,0.11-0.06,0.17-0.17,0.17\n\tc-1.34-0.04-2.47-0.54-3.4-1.5C5.1,19.42,4.64,18.27,4.64,16.95z M11,21.02c0-0.22,0.08-0.42,0.24-0.58\n\tc0.16-0.16,0.35-0.24,0.59-0.24c0.23,0,0.43,0.08,0.59,0.24c0.16,0.16,0.24,0.36,0.24,0.58c0,0.24-0.08,0.44-0.24,0.6\n\tc-0.16,0.17-0.35,0.25-0.59,0.25c-0.23,0-0.43-0.08-0.59-0.25C11.08,21.46,11,21.26,11,21.02z M11,24.65c0-0.24,0.08-0.44,0.24-0.6\n\tc0.16-0.15,0.35-0.23,0.58-0.23c0.23,0,0.43,0.08,0.59,0.23c0.16,0.16,0.24,0.35,0.24,0.59c0,0.24-0.08,0.43-0.24,0.59\n\tc-0.16,0.16-0.35,0.23-0.59,0.23c-0.23,0-0.43-0.08-0.59-0.23C11.08,25.08,11,24.88,11,24.65z M14.19,22.95\n\tc0-0.23,0.08-0.44,0.25-0.62c0.16-0.16,0.35-0.24,0.57-0.24c0.23,0,0.43,0.09,0.6,0.26c0.17,0.17,0.26,0.37,0.26,0.6\n\tc0,0.23-0.08,0.43-0.25,0.6c-0.17,0.17-0.37,0.25-0.61,0.25c-0.23,0-0.42-0.08-0.58-0.25S14.19,23.18,14.19,22.95z M14.19,19.33\n\tc0-0.23,0.08-0.43,0.25-0.6c0.18-0.16,0.37-0.24,0.57-0.24c0.24,0,0.44,0.08,0.61,0.25c0.17,0.17,0.25,0.36,0.25,0.6\n\tc0,0.23-0.08,0.43-0.25,0.59c-0.17,0.16-0.37,0.24-0.61,0.24c-0.23,0-0.42-0.08-0.58-0.24C14.27,19.76,14.19,19.56,14.19,19.33z\n\t M14.19,26.61c0-0.23,0.08-0.43,0.25-0.61c0.16-0.16,0.35-0.24,0.57-0.24c0.24,0,0.44,0.08,0.61,0.25c0.17,0.17,0.25,0.37,0.25,0.6\n\ts-0.08,0.43-0.25,0.59c-0.17,0.16-0.37,0.24-0.61,0.24c-0.23,0-0.42-0.08-0.58-0.24C14.27,27.03,14.19,26.84,14.19,26.61z\n\t M17.41,21.02c0-0.22,0.08-0.41,0.25-0.58c0.17-0.17,0.37-0.25,0.6-0.25c0.23,0,0.43,0.08,0.59,0.24c0.16,0.16,0.24,0.36,0.24,0.58\n\tc0,0.24-0.08,0.44-0.24,0.6c-0.16,0.17-0.35,0.25-0.59,0.25c-0.24,0-0.44-0.08-0.6-0.25C17.5,21.45,17.41,21.25,17.41,21.02z\n\t M17.41,24.65c0-0.22,0.08-0.42,0.25-0.6c0.16-0.15,0.36-0.23,0.6-0.23c0.24,0,0.43,0.08,0.59,0.23s0.23,0.35,0.23,0.59\n\tc0,0.24-0.08,0.43-0.23,0.59c-0.16,0.16-0.35,0.23-0.59,0.23c-0.24,0-0.44-0.08-0.6-0.24C17.5,25.07,17.41,24.88,17.41,24.65z\" />",
};
#[cfg(feature = "WiSnowWind")]
const WI_SNOW_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,16.95c0-1.16,0.35-2.18,1.06-3.08s1.62-1.48,2.74-1.76c0.31-1.36,1.01-2.48,2.1-3.36s2.34-1.31,3.75-1.31\n\tc1.38,0,2.6,0.43,3.68,1.28c1.08,0.85,1.78,1.95,2.1,3.29h0.32c0.89,0,1.72,0.22,2.48,0.66c0.76,0.44,1.37,1.04,1.81,1.8\n\tc0.44,0.76,0.67,1.59,0.67,2.48c0,1.32-0.46,2.47-1.39,3.42c-0.92,0.96-2.05,1.46-3.38,1.5c-0.13,0-0.2-0.06-0.2-0.17v-1.33\n\tc0-0.12,0.07-0.18,0.2-0.18c0.85-0.04,1.58-0.38,2.18-1.02s0.9-1.38,0.9-2.23c0-0.89-0.32-1.65-0.97-2.3s-1.42-0.97-2.32-0.97h-1.61\n\tc-0.12,0-0.18-0.06-0.18-0.17l-0.08-0.58c-0.11-1.08-0.58-1.99-1.39-2.72c-0.82-0.73-1.76-1.1-2.85-1.1c-1.1,0-2.05,0.37-2.86,1.11\n\tc-0.81,0.74-1.27,1.65-1.37,2.75l-0.06,0.5c0,0.12-0.07,0.19-0.2,0.19l-0.53,0.07c-0.83,0.07-1.53,0.41-2.1,1.04\n\ts-0.85,1.35-0.85,2.19c0,0.85,0.3,1.59,0.9,2.23s1.33,0.97,2.18,1.02c0.11,0,0.17,0.06,0.17,0.18v1.33c0,0.11-0.06,0.17-0.17,0.17\n\tc-1.34-0.04-2.47-0.54-3.4-1.5C5.1,19.42,4.64,18.27,4.64,16.95z M10.14,24.65c0-0.23,0.08-0.43,0.25-0.6\n\tc0.16-0.15,0.35-0.23,0.57-0.23c0.23,0,0.43,0.08,0.59,0.23s0.24,0.35,0.24,0.59c0,0.24-0.08,0.43-0.24,0.59\n\tc-0.16,0.16-0.35,0.23-0.59,0.23c-0.23,0-0.43-0.08-0.59-0.23C10.22,25.08,10.14,24.88,10.14,24.65z M11,21.02\n\tc0-0.22,0.08-0.42,0.24-0.58c0.16-0.16,0.35-0.24,0.59-0.24c0.23,0,0.43,0.08,0.59,0.24c0.16,0.16,0.24,0.36,0.24,0.58\n\tc0,0.24-0.08,0.44-0.24,0.6c-0.16,0.17-0.35,0.25-0.59,0.25c-0.23,0-0.43-0.08-0.59-0.25C11.08,21.46,11,21.26,11,21.02z\n\t M12.9,26.61c0-0.23,0.08-0.43,0.25-0.61c0.16-0.16,0.35-0.24,0.57-0.24c0.24,0,0.44,0.08,0.61,0.25c0.17,0.17,0.25,0.37,0.25,0.6\n\ts-0.08,0.43-0.25,0.59c-0.17,0.16-0.37,0.24-0.61,0.24c-0.23,0-0.42-0.08-0.58-0.24C12.99,27.03,12.9,26.84,12.9,26.61z\n\t M13.77,22.95c0-0.24,0.08-0.44,0.24-0.62c0.16-0.16,0.36-0.24,0.58-0.24c0.23,0,0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.61\n\tc0,0.23-0.08,0.43-0.25,0.6s-0.37,0.25-0.6,0.25c-0.23,0-0.42-0.08-0.58-0.25C13.85,23.38,13.77,23.18,13.77,22.95z M14.19,19.33\n\tc0-0.23,0.08-0.43,0.25-0.6c0.18-0.16,0.37-0.24,0.57-0.24c0.24,0,0.44,0.08,0.61,0.25c0.17,0.17,0.25,0.36,0.25,0.6\n\tc0,0.23-0.08,0.43-0.25,0.59c-0.17,0.16-0.37,0.24-0.61,0.24c-0.23,0-0.42-0.08-0.58-0.24C14.27,19.76,14.19,19.56,14.19,19.33z\n\t M16.56,24.65c0-0.23,0.08-0.43,0.24-0.6c0.16-0.15,0.36-0.23,0.6-0.23c0.24,0,0.43,0.08,0.59,0.23c0.16,0.16,0.23,0.35,0.23,0.59\n\tc0,0.24-0.08,0.43-0.23,0.59c-0.16,0.16-0.35,0.23-0.59,0.23c-0.24,0-0.44-0.08-0.6-0.24C16.64,25.07,16.56,24.88,16.56,24.65z\n\t M17.41,21.02c0-0.22,0.08-0.41,0.25-0.58c0.17-0.17,0.37-0.25,0.6-0.25c0.23,0,0.43,0.08,0.59,0.24c0.16,0.16,0.24,0.36,0.24,0.58\n\tc0,0.24-0.08,0.44-0.24,0.6c-0.16,0.17-0.35,0.25-0.59,0.25c-0.24,0-0.44-0.08-0.6-0.25C17.5,21.45,17.41,21.25,17.41,21.02z\" />",
};
#[cfg(feature = "WiSnowflakeCold")]
const WI_SNOWFLAKE_COLD: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.46,14.41c0-0.19,0.07-0.36,0.2-0.5c0.13-0.14,0.29-0.21,0.48-0.21c0.19,0,0.36,0.07,0.49,0.21c0.13,0.14,0.2,0.3,0.2,0.5\n\tc0,0.19-0.07,0.35-0.2,0.48c-0.13,0.13-0.3,0.2-0.48,0.2s-0.35-0.07-0.48-0.2C7.52,14.75,7.46,14.59,7.46,14.41z M9.44,14.41\n\tc0-0.19,0.07-0.36,0.2-0.5c0.13-0.14,0.29-0.21,0.48-0.21h3.23l-2.28-2.28c-0.14-0.12-0.21-0.28-0.21-0.47\n\tc0-0.19,0.07-0.35,0.21-0.49c0.14-0.14,0.3-0.21,0.49-0.21s0.35,0.07,0.49,0.21l2.27,2.27V9.52c0-0.19,0.07-0.36,0.21-0.5\n\ts0.3-0.21,0.5-0.21c0.19,0,0.35,0.07,0.48,0.21c0.13,0.14,0.2,0.3,0.2,0.5v3.23l2.29-2.3c0.14-0.14,0.3-0.21,0.48-0.21\n\tc0.18,0,0.35,0.07,0.49,0.21c0.12,0.14,0.18,0.3,0.18,0.49c0,0.19-0.06,0.35-0.18,0.47l-2.28,2.28h3.23c0.18,0,0.34,0.07,0.47,0.21\n\tc0.13,0.14,0.2,0.3,0.2,0.5c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.29,0.2-0.47,0.2h-3.23l2.29,2.29c0.12,0.12,0.18,0.28,0.18,0.47\n\ts-0.06,0.35-0.18,0.49c-0.14,0.14-0.31,0.21-0.49,0.21c-0.18,0-0.35-0.07-0.48-0.21l-2.29-2.3v3.24c0,0.19-0.07,0.36-0.2,0.49\n\tc-0.13,0.13-0.29,0.2-0.48,0.2c-0.19,0-0.36-0.07-0.5-0.2c-0.14-0.13-0.21-0.3-0.21-0.49v-3.22l-2.27,2.27\n\tc-0.14,0.14-0.3,0.21-0.49,0.21s-0.35-0.07-0.49-0.21s-0.21-0.3-0.21-0.49s0.07-0.34,0.21-0.47l2.3-2.29h-3.24\n\tc-0.19,0-0.35-0.07-0.48-0.2C9.5,14.75,9.44,14.59,9.44,14.41z M9.45,19.25c0-0.18,0.07-0.35,0.21-0.48\n\tc0.12-0.14,0.28-0.21,0.47-0.21c0.19,0,0.35,0.07,0.49,0.21c0.14,0.14,0.21,0.3,0.21,0.48c0,0.18-0.07,0.35-0.21,0.48\n\tc-0.14,0.14-0.3,0.21-0.49,0.21c-0.19,0-0.35-0.07-0.47-0.21C9.52,19.6,9.45,19.44,9.45,19.25z M9.45,9.54\n\tc0-0.18,0.07-0.35,0.21-0.48c0.12-0.14,0.28-0.21,0.47-0.21c0.19,0,0.35,0.07,0.49,0.21c0.14,0.14,0.21,0.3,0.21,0.48\n\ts-0.07,0.35-0.21,0.49c-0.14,0.14-0.3,0.21-0.49,0.21c-0.19,0-0.35-0.07-0.47-0.21C9.52,9.89,9.45,9.72,9.45,9.54z M14.3,21.25\n\tc0-0.18,0.07-0.34,0.21-0.47c0.14-0.13,0.3-0.2,0.5-0.2c0.19,0,0.35,0.07,0.48,0.2c0.13,0.13,0.2,0.29,0.2,0.47\n\tc0,0.19-0.07,0.36-0.2,0.49c-0.13,0.13-0.29,0.2-0.48,0.2c-0.19,0-0.36-0.07-0.5-0.2C14.37,21.61,14.3,21.45,14.3,21.25z M14.3,7.54\n\tc0-0.19,0.07-0.36,0.21-0.49s0.3-0.2,0.5-0.2c0.19,0,0.35,0.07,0.48,0.2c0.13,0.13,0.2,0.3,0.2,0.49c0,0.18-0.07,0.34-0.2,0.47\n\tc-0.13,0.13-0.29,0.2-0.48,0.2c-0.19,0-0.36-0.07-0.5-0.2C14.37,7.88,14.3,7.72,14.3,7.54z M19.17,19.25c0-0.18,0.07-0.35,0.21-0.48\n\tc0.12-0.14,0.28-0.21,0.47-0.21c0.19,0,0.35,0.07,0.49,0.21c0.14,0.14,0.21,0.3,0.21,0.48c0,0.18-0.07,0.35-0.21,0.48\n\tc-0.14,0.14-0.3,0.21-0.49,0.21c-0.19,0-0.35-0.07-0.47-0.21C19.24,19.6,19.17,19.44,19.17,19.25z M19.17,9.54\n\tc0-0.18,0.07-0.35,0.21-0.48c0.12-0.14,0.28-0.21,0.47-0.21c0.19,0,0.35,0.07,0.49,0.21c0.14,0.14,0.21,0.3,0.21,0.48\n\ts-0.07,0.35-0.21,0.49c-0.14,0.14-0.3,0.21-0.49,0.21c-0.19,0-0.35-0.07-0.47-0.21C19.24,9.89,19.17,9.72,19.17,9.54z M21.17,14.41\n\tc0-0.19,0.07-0.36,0.2-0.5c0.13-0.14,0.3-0.21,0.49-0.21c0.18,0,0.34,0.07,0.47,0.21c0.13,0.14,0.2,0.3,0.2,0.5\n\tc0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.29,0.2-0.47,0.2c-0.19,0-0.36-0.07-0.49-0.2C21.24,14.75,21.17,14.59,21.17,14.41z\" />",
};
#[cfg(feature = "WiSolarEclipse")]
const WI_SOLAR_ECLIPSE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.37,14.62c0-0.24,0.08-0.45,0.25-0.62c0.17-0.16,0.38-0.24,0.6-0.24h2.04c0.23,0,0.42,0.08,0.58,0.25\n\tc0.15,0.17,0.23,0.37,0.23,0.61S8,15.06,7.85,15.23c-0.15,0.17-0.35,0.25-0.58,0.25H5.23c-0.23,0-0.43-0.08-0.6-0.25\n\tC4.46,15.06,4.37,14.86,4.37,14.62z M7.23,21.55c0-0.23,0.08-0.43,0.23-0.61l1.47-1.43c0.15-0.16,0.35-0.23,0.59-0.23\n\tc0.24,0,0.44,0.08,0.6,0.23s0.24,0.34,0.24,0.57c0,0.24-0.08,0.46-0.24,0.64L8.7,22.14c-0.41,0.32-0.82,0.32-1.23,0\n\tC7.31,21.98,7.23,21.78,7.23,21.55z M7.23,7.71c0-0.23,0.08-0.43,0.23-0.61C7.66,6.93,7.87,6.85,8.1,6.85\n\tc0.22,0,0.42,0.08,0.59,0.24l1.43,1.47c0.16,0.15,0.24,0.35,0.24,0.59c0,0.24-0.08,0.44-0.24,0.6s-0.36,0.24-0.6,0.24\n\tc-0.24,0-0.44-0.08-0.59-0.24L7.47,8.32C7.31,8.16,7.23,7.95,7.23,7.71z M9.78,14.62c0-0.93,0.23-1.8,0.7-2.6s1.1-1.44,1.91-1.91\n\ts1.67-0.7,2.6-0.7c0.7,0,1.37,0.14,2.02,0.42c0.64,0.28,1.2,0.65,1.66,1.12c0.47,0.47,0.84,1.02,1.11,1.66\n\tc0.27,0.64,0.41,1.32,0.41,2.02c0,0.94-0.23,1.81-0.7,2.61c-0.47,0.8-1.1,1.43-1.9,1.9c-0.8,0.47-1.67,0.7-2.61,0.7\n\ts-1.81-0.23-2.61-0.7c-0.8-0.47-1.43-1.1-1.9-1.9C10.02,16.43,9.78,15.56,9.78,14.62z M14.14,22.4c0-0.24,0.08-0.44,0.25-0.6\n\ts0.37-0.24,0.6-0.24c0.24,0,0.45,0.08,0.61,0.24s0.24,0.36,0.24,0.6v1.99c0,0.24-0.08,0.45-0.25,0.62c-0.17,0.17-0.37,0.25-0.6,0.25\n\ts-0.44-0.08-0.6-0.25c-0.17-0.17-0.25-0.38-0.25-0.62V22.4z M14.14,6.9V4.86c0-0.23,0.08-0.43,0.25-0.6C14.56,4.09,14.76,4,15,4\n\ts0.43,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6V6.9c0,0.23-0.08,0.42-0.25,0.58S15.23,7.71,15,7.71s-0.44-0.08-0.6-0.23\n\tS14.14,7.13,14.14,6.9z M14.25,11.22c0.87,0.11,1.6,0.49,2.19,1.15c0.59,0.66,0.89,1.44,0.89,2.33c0,0.83-0.26,1.56-0.78,2.2\n\tc-0.52,0.63-1.18,1.04-1.98,1.21c0.2,0.02,0.35,0.04,0.44,0.04c0.97,0,1.81-0.35,2.5-1.04s1.04-1.52,1.04-2.5\n\tc0-0.96-0.35-1.78-1.04-2.47c-0.69-0.68-1.52-1.02-2.5-1.02C14.74,11.14,14.49,11.17,14.25,11.22z M19.66,20.08\n\tc0-0.23,0.08-0.42,0.23-0.56c0.15-0.16,0.34-0.23,0.56-0.23c0.24,0,0.44,0.08,0.6,0.23l1.46,1.43c0.16,0.17,0.24,0.38,0.24,0.61\n\tc0,0.23-0.08,0.43-0.24,0.59c-0.4,0.31-0.8,0.31-1.2,0l-1.42-1.42C19.74,20.55,19.66,20.34,19.66,20.08z M19.66,9.16\n\tc0-0.25,0.08-0.45,0.23-0.59l1.42-1.47c0.17-0.16,0.37-0.24,0.59-0.24c0.24,0,0.44,0.08,0.6,0.25c0.17,0.17,0.25,0.37,0.25,0.6\n\tc0,0.25-0.08,0.46-0.24,0.62l-1.46,1.43c-0.18,0.16-0.38,0.24-0.6,0.24c-0.23,0-0.41-0.08-0.56-0.24S19.66,9.4,19.66,9.16z\n\t M21.92,14.62c0-0.24,0.08-0.44,0.24-0.62c0.16-0.16,0.35-0.24,0.57-0.24h2.02c0.23,0,0.43,0.09,0.6,0.26\n\tc0.17,0.17,0.26,0.37,0.26,0.6s-0.09,0.43-0.26,0.6c-0.17,0.17-0.37,0.25-0.6,0.25h-2.02c-0.23,0-0.43-0.08-0.58-0.25\n\tS21.92,14.86,21.92,14.62z\" />",
};
#[cfg(feature = "WiSprinkle")]
const WI_SPRINKLE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,16.91c0-1.15,0.36-2.17,1.08-3.07c0.72-0.9,1.63-1.47,2.73-1.73c0.31-1.36,1.01-2.48,2.1-3.35s2.35-1.31,3.76-1.31\n\tc1.38,0,2.6,0.43,3.68,1.27c1.07,0.85,1.78,1.94,2.11,3.28h0.31c0.89,0,1.72,0.22,2.48,0.65s1.37,1.03,1.81,1.78\n\tc0.44,0.75,0.67,1.58,0.67,2.47c0,1.34-0.46,2.49-1.38,3.45s-2.05,1.47-3.38,1.51c-0.13,0-0.2-0.06-0.2-0.17v-1.33\n\tc0-0.12,0.07-0.18,0.2-0.18c0.86-0.04,1.58-0.38,2.18-1.02s0.9-1.39,0.9-2.26s-0.32-1.62-0.98-2.26c-0.65-0.64-1.42-0.96-2.31-0.96\n\th-1.6c-0.12,0-0.19-0.06-0.19-0.17l-0.07-0.58c-0.11-1.07-0.57-1.98-1.38-2.71c-0.82-0.73-1.77-1.1-2.85-1.1\n\tc-1.09,0-2.05,0.36-2.86,1.09c-0.81,0.73-1.27,1.63-1.38,2.71l-0.06,0.54c0,0.12-0.07,0.18-0.2,0.18l-0.53,0.03\n\tc-0.82,0.04-1.51,0.37-2.09,1s-0.86,1.37-0.86,2.22c0,0.87,0.3,1.62,0.9,2.26s1.33,0.98,2.18,1.02c0.11,0,0.17,0.06,0.17,0.18v1.33\n\tc0,0.11-0.06,0.17-0.17,0.17c-1.34-0.06-2.47-0.57-3.4-1.53S4.64,18.24,4.64,16.91z M10.57,17.79c0-0.24,0.12-0.57,0.37-0.99\n\tc0.24-0.42,0.47-0.75,0.68-1.01c0.21-0.24,0.34-0.38,0.38-0.42l0.36,0.4c0.26,0.28,0.5,0.61,0.72,1.02c0.22,0.4,0.33,0.74,0.33,1\n\tc0,0.39-0.13,0.72-0.4,0.98c-0.27,0.26-0.6,0.39-1,0.39c-0.39,0-0.73-0.13-1.01-0.4C10.71,18.5,10.57,18.17,10.57,17.79z\n\t M13.55,21.78c0-0.28,0.08-0.59,0.24-0.96s0.35-0.7,0.59-1.02c0.18-0.26,0.4-0.54,0.67-0.84c0.26-0.3,0.46-0.52,0.6-0.65\n\tc0.07-0.06,0.15-0.14,0.24-0.23l0.24,0.23c0.38,0.33,0.8,0.82,1.27,1.46c0.24,0.33,0.43,0.68,0.59,1.04s0.23,0.68,0.23,0.97\n\tc0,0.64-0.23,1.19-0.68,1.65s-1.01,0.68-1.66,0.68c-0.64,0-1.19-0.23-1.65-0.67C13.77,22.98,13.55,22.43,13.55,21.78z M15.02,15.12\n\tc0-0.42,0.32-0.95,0.97-1.6l0.24,0.25c0.18,0.21,0.33,0.45,0.48,0.71c0.14,0.26,0.22,0.47,0.22,0.64c0,0.26-0.09,0.48-0.28,0.66\n\tc-0.18,0.18-0.4,0.28-0.66,0.28c-0.27,0-0.5-0.09-0.69-0.28C15.11,15.6,15.02,15.38,15.02,15.12z\" />",
};
#[cfg(feature = "WiStars")]
const WI_STARS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.37,16.18c0.65-0.03,1.2-0.28,1.65-0.75c0.45-0.47,0.68-1.03,0.68-1.68c0,0.65,0.22,1.21,0.67,1.68\n\tc0.45,0.47,1,0.72,1.65,0.75c-0.65,0.03-1.2,0.28-1.65,0.75c-0.45,0.47-0.67,1.03-0.67,1.68c0-0.65-0.22-1.21-0.68-1.68\n\tC6.57,16.46,6.02,16.21,5.37,16.18z M7.7,8.98c1.26-0.06,2.33-0.55,3.21-1.47c0.88-0.92,1.32-2.01,1.32-3.28\n\tc0,1.27,0.44,2.36,1.32,3.28s1.95,1.4,3.22,1.47c-0.83,0.04-1.59,0.27-2.29,0.71c-0.69,0.43-1.24,1.01-1.65,1.73\n\tc-0.4,0.72-0.6,1.49-0.6,2.33c0-1.27-0.44-2.37-1.32-3.29C10.03,9.53,8.96,9.04,7.7,8.98z M11.02,19.75\n\tc0.95-0.04,1.76-0.41,2.42-1.1c0.66-0.69,0.99-1.51,0.99-2.47c0,0.96,0.33,1.78,0.99,2.47c0.66,0.69,1.46,1.06,2.41,1.1\n\tc-0.95,0.04-1.75,0.41-2.41,1.1c-0.66,0.69-0.99,1.51-0.99,2.47c0-0.96-0.33-1.78-0.99-2.47C12.77,20.16,11.97,19.8,11.02,19.75z\n\t M17.83,15.01c0.95-0.04,1.75-0.41,2.41-1.1c0.66-0.69,0.98-1.51,0.98-2.48c0,0.96,0.33,1.78,0.99,2.47s1.47,1.06,2.42,1.1\n\tc-0.95,0.04-1.76,0.41-2.42,1.1c-0.66,0.69-0.99,1.51-0.99,2.47c0-0.96-0.33-1.78-0.98-2.47C19.58,15.42,18.78,15.05,17.83,15.01z\" />",
};
#[cfg(feature = "WiStormShowers")]
const WI_STORM_SHOWERS: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.63,16.91c0,1.11,0.33,2.1,0.99,2.97s1.51,1.47,2.56,1.79l-0.65,1.68c-0.03,0.14,0.02,0.22,0.14,0.22h2.13l-1.04,3.78\n\th0.28l3.97-5.22c0.04-0.04,0.04-0.09,0.02-0.14s-0.07-0.07-0.14-0.07h-2.18l2.48-4.64c0.06-0.14,0.02-0.21-0.14-0.21h-2.94\n\tc-0.09,0-0.17,0.05-0.22,0.14L8.8,20.08c-0.71-0.18-1.3-0.57-1.77-1.16c-0.47-0.59-0.7-1.26-0.7-2.01c0-0.83,0.28-1.55,0.84-2.16\n\ts1.26-0.96,2.1-1.06l0.52-0.07c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.1-1.08,0.55-1.99,1.36-2.72c0.81-0.73,1.76-1.1,2.85-1.1\n\tc1.09,0,2.04,0.37,2.85,1.1c0.82,0.73,1.28,1.64,1.4,2.72l0.06,0.58c0,0.12,0.06,0.18,0.19,0.18h1.61c0.91,0,1.68,0.32,2.32,0.95\n\tc0.64,0.63,0.96,1.39,0.96,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.32,0.96-2.19,1.03c-0.13,0-0.2,0.06-0.2,0.19v1.37\n\tc0,0.11,0.07,0.17,0.2,0.17c0.65-0.02,1.27-0.17,1.86-0.44c0.59-0.27,1.1-0.63,1.52-1.07c0.42-0.44,0.76-0.96,1.01-1.57\n\tc0.25-0.6,0.38-1.23,0.38-1.88c0-0.9-0.22-1.73-0.67-2.49c-0.45-0.76-1.05-1.36-1.81-1.8c-0.76-0.44-1.59-0.66-2.48-0.66h-0.32\n\tc-0.33-1.33-1.03-2.42-2.11-3.26c-1.08-0.84-2.3-1.27-3.68-1.27c-1.41,0-2.67,0.44-3.76,1.31s-1.79,1.99-2.1,3.36\n\tc-1.1,0.26-2.01,0.83-2.73,1.73S4.63,15.76,4.63,16.91z M12.79,26.77c0,0.16,0.05,0.31,0.15,0.46c0.1,0.15,0.26,0.26,0.46,0.32\n\tc0.14,0.03,0.22,0.05,0.25,0.05c0.09,0,0.21-0.03,0.38-0.1c0.21-0.09,0.35-0.27,0.42-0.52l0.28-1.05c0.06-0.22,0.04-0.43-0.08-0.63\n\ts-0.29-0.33-0.53-0.4c-0.22-0.06-0.43-0.04-0.63,0.08c-0.2,0.12-0.34,0.29-0.41,0.53l-0.27,1C12.8,26.68,12.79,26.77,12.79,26.77z\n\t M14.13,22c0,0.14,0.05,0.29,0.15,0.44c0.1,0.15,0.25,0.26,0.45,0.33c0.22,0.07,0.44,0.06,0.64-0.05s0.33-0.28,0.4-0.52l0.3-1.04\n\tc0.06-0.22,0.03-0.43-0.08-0.63c-0.12-0.2-0.3-0.34-0.53-0.41c-0.23-0.06-0.44-0.04-0.65,0.08s-0.34,0.29-0.41,0.52l-0.24,1.01\n\tC14.14,21.9,14.13,21.99,14.13,22z M16.95,23.65c0,0.17,0.05,0.34,0.16,0.51c0.11,0.17,0.27,0.28,0.47,0.35\n\tc0.02,0,0.06,0.01,0.12,0.02c0.05,0.01,0.09,0.02,0.12,0.02c0.13,0,0.26-0.02,0.38-0.08c0.19-0.07,0.33-0.26,0.41-0.57l0.25-1.01\n\tc0.07-0.23,0.05-0.45-0.06-0.66c-0.11-0.21-0.28-0.35-0.5-0.42c-0.25-0.06-0.48-0.03-0.68,0.08c-0.2,0.12-0.33,0.3-0.37,0.53\n\tl-0.27,1.03C16.96,23.51,16.95,23.58,16.95,23.65z M18.31,18.86c-0.01,0.16,0.04,0.31,0.15,0.47c0.11,0.16,0.27,0.28,0.49,0.38\n\tc0.08,0.04,0.16,0.06,0.26,0.06c0.11,0,0.22-0.03,0.34-0.08c0.21-0.1,0.35-0.29,0.44-0.57l0.29-1.03c0.02-0.13,0.03-0.2,0.03-0.22\n\tc0-0.17-0.05-0.33-0.16-0.49s-0.27-0.27-0.49-0.33c-0.02,0-0.06-0.01-0.11-0.02C19.49,17,19.45,17,19.42,17\n\tc-0.17,0-0.33,0.05-0.49,0.15c-0.16,0.1-0.27,0.26-0.33,0.48l-0.27,1.01C18.32,18.72,18.31,18.79,18.31,18.86z\" />",
};
#[cfg(feature = "WiStormWarning")]
const WI_STORM_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.76,24.6V7.45h1.13V24.6H9.76z M11.7,14.05v-6.6h8.55v6.6H11.7z M14.06,12.05h3.81v-2.5h-3.81V12.05z\" />",
};
#[cfg(feature = "WiStrongWind")]
const WI_STRONG_WIND: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.1,16.97c0,0.24,0.09,0.45,0.28,0.62c0.16,0.19,0.37,0.28,0.63,0.28H18.7c0.29,0,0.53,0.1,0.73,0.3\n\tc0.2,0.2,0.3,0.45,0.3,0.74c0,0.29-0.1,0.53-0.3,0.72c-0.2,0.19-0.44,0.29-0.74,0.29c-0.29,0-0.54-0.1-0.73-0.29\n\tc-0.16-0.18-0.36-0.26-0.6-0.26c-0.25,0-0.46,0.09-0.64,0.26s-0.27,0.38-0.27,0.61c0,0.25,0.09,0.46,0.28,0.63\n\tc0.56,0.55,1.22,0.83,1.96,0.83c0.78,0,1.45-0.27,2.01-0.81c0.56-0.54,0.83-1.19,0.83-1.97s-0.28-1.44-0.84-2\n\tc-0.56-0.56-1.23-0.84-2-0.84H4.01c-0.25,0-0.46,0.09-0.64,0.26C3.19,16.51,3.1,16.72,3.1,16.97z M3.1,13.69\n\tc0,0.23,0.09,0.43,0.28,0.61c0.17,0.18,0.38,0.26,0.63,0.26h20.04c0.78,0,1.45-0.27,2.01-0.82c0.56-0.54,0.84-1.2,0.84-1.97\n\tc0-0.77-0.28-1.44-0.84-1.99s-1.23-0.83-2.01-0.83c-0.77,0-1.42,0.27-1.95,0.8c-0.18,0.16-0.27,0.38-0.27,0.67\n\tc0,0.26,0.09,0.47,0.26,0.63c0.17,0.16,0.38,0.24,0.63,0.24c0.24,0,0.45-0.08,0.63-0.24c0.19-0.21,0.42-0.31,0.7-0.31\n\tc0.29,0,0.53,0.1,0.73,0.3c0.2,0.2,0.3,0.44,0.3,0.73c0,0.29-0.1,0.53-0.3,0.72c-0.2,0.19-0.44,0.29-0.73,0.29H4.01\n\tc-0.25,0-0.46,0.09-0.64,0.26C3.19,13.23,3.1,13.44,3.1,13.69z\" />",
};
#[cfg(feature = "WiSunrise")]
const WI_SUNRISE: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.75,15.36c0-0.25,0.1-0.48,0.3-0.69c0.22-0.19,0.46-0.29,0.7-0.29h2.33c0.27,0,0.49,0.1,0.67,0.29\n\tc0.18,0.19,0.27,0.43,0.27,0.69c0,0.29-0.09,0.53-0.27,0.72c-0.18,0.19-0.41,0.29-0.67,0.29H3.75c-0.27,0-0.5-0.1-0.7-0.3\n\tC2.85,15.86,2.75,15.62,2.75,15.36z M6.08,7.38c0-0.27,0.09-0.5,0.26-0.68C6.57,6.5,6.8,6.4,7.05,6.4c0.26,0,0.49,0.1,0.68,0.29\n\tl1.64,1.65c0.19,0.22,0.28,0.45,0.28,0.69c0,0.28-0.09,0.52-0.27,0.7s-0.4,0.28-0.66,0.28c-0.24,0-0.48-0.1-0.7-0.29L6.34,8.11\n\tC6.17,7.9,6.08,7.65,6.08,7.38z M8.08,20.88c0-0.28,0.1-0.51,0.29-0.68c0.18-0.17,0.4-0.26,0.68-0.26h2.63l3.11-2.92\n\tc0.1-0.08,0.21-0.08,0.34,0l3.16,2.92h2.77c0.27,0,0.5,0.09,0.69,0.28c0.19,0.18,0.29,0.41,0.29,0.67c0,0.27-0.1,0.5-0.29,0.69\n\tc-0.19,0.19-0.42,0.29-0.69,0.29h-3.38c-0.1,0-0.2-0.02-0.29-0.07l-2.41-2.27l-2.39,2.27c-0.08,0.05-0.17,0.07-0.28,0.07H9.05\n\tc-0.27,0-0.5-0.1-0.69-0.29C8.17,21.38,8.08,21.15,8.08,20.88z M9,15.36c0,0.97,0.21,1.85,0.62,2.64c0.02,0.12,0.11,0.18,0.25,0.18\n\th1.88c0.07,0,0.12-0.03,0.15-0.08c0.03-0.06,0.02-0.12-0.02-0.19c-0.64-0.77-0.96-1.62-0.96-2.55c0-1.12,0.4-2.08,1.2-2.87\n\tc0.8-0.79,1.76-1.18,2.89-1.18c1.12,0,2.07,0.39,2.86,1.18c0.79,0.79,1.19,1.74,1.19,2.87c0,0.94-0.32,1.79-0.95,2.55\n\tc-0.04,0.07-0.05,0.13-0.03,0.19s0.07,0.08,0.15,0.08h1.9c0.13,0,0.21-0.06,0.23-0.18C20.8,17.23,21,16.35,21,15.36\n\tc0-0.81-0.16-1.59-0.48-2.32c-0.32-0.74-0.75-1.37-1.28-1.91c-0.53-0.53-1.17-0.96-1.91-1.28c-0.74-0.32-1.51-0.47-2.32-0.47\n\tc-0.81,0-1.59,0.16-2.33,0.47c-0.74,0.32-1.38,0.74-1.92,1.28c-0.54,0.53-0.97,1.17-1.29,1.91C9.16,13.77,9,14.54,9,15.36z\n\t M14.03,6.4v-2.3c0-0.29,0.09-0.52,0.28-0.71s0.43-0.28,0.71-0.28c0.28,0,0.51,0.09,0.7,0.28S16,3.83,16,4.11v2.3\n\tc0,0.29-0.09,0.52-0.28,0.71c-0.18,0.18-0.42,0.28-0.7,0.28c-0.29,0-0.52-0.09-0.71-0.28C14.12,6.93,14.03,6.69,14.03,6.4z\n\t M20.38,9.04c0-0.25,0.09-0.48,0.27-0.69l1.62-1.65c0.19-0.19,0.43-0.29,0.7-0.29c0.27,0,0.51,0.1,0.69,0.29\n\tc0.19,0.19,0.28,0.42,0.28,0.69c0,0.29-0.09,0.53-0.26,0.73L22,9.73c-0.21,0.19-0.45,0.29-0.7,0.29c-0.27,0-0.49-0.09-0.66-0.28\n\tS20.38,9.32,20.38,9.04z M22.99,15.36c0-0.27,0.09-0.5,0.27-0.69c0.18-0.19,0.4-0.29,0.66-0.29h2.35c0.27,0,0.5,0.1,0.69,0.29\n\tc0.19,0.19,0.29,0.43,0.29,0.69c0,0.28-0.1,0.51-0.29,0.71s-0.42,0.3-0.69,0.3h-2.35c-0.27,0-0.49-0.1-0.67-0.29\n\tC23.08,15.88,22.99,15.64,22.99,15.36z\" />",
};
#[cfg(feature = "WiSunset")]
const WI_SUNSET: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M2.88,15.47c0-0.28,0.1-0.5,0.3-0.68c0.17-0.18,0.4-0.26,0.68-0.26h2.31c0.27,0,0.49,0.09,0.67,0.27\n\tc0.17,0.18,0.26,0.4,0.26,0.67c0,0.28-0.09,0.52-0.27,0.71c-0.18,0.19-0.4,0.29-0.66,0.29H3.87c-0.27,0-0.5-0.1-0.69-0.3\n\tC2.98,15.97,2.88,15.74,2.88,15.47z M6.17,7.61c0-0.28,0.08-0.51,0.25-0.68c0.2-0.2,0.43-0.3,0.7-0.3c0.29,0,0.51,0.1,0.68,0.3\n\tl1.62,1.63c0.46,0.44,0.46,0.89,0,1.35c-0.19,0.19-0.4,0.28-0.65,0.28c-0.22,0-0.44-0.09-0.68-0.28L6.43,8.29\n\tC6.26,8.11,6.17,7.88,6.17,7.61z M8.14,20.89c0-0.26,0.1-0.49,0.3-0.69c0.18-0.18,0.41-0.27,0.68-0.27h3.22\n\tc0.11,0,0.2,0.02,0.28,0.08l2.35,2.22L17.36,20c0.07-0.05,0.17-0.08,0.29-0.08h3.3c0.27,0,0.5,0.09,0.69,0.28\n\tc0.19,0.19,0.29,0.42,0.29,0.68c0,0.27-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.69,0.29h-2.68l-3.14,2.84\n\tc-0.12,0.09-0.23,0.09-0.33,0l-3.08-2.84h-2.6c-0.27,0-0.5-0.1-0.69-0.29C8.24,21.39,8.14,21.16,8.14,20.89z M9.08,15.47\n\tc0,0.99,0.19,1.87,0.58,2.62c0.06,0.11,0.15,0.16,0.27,0.16h1.87c0.08,0,0.13-0.02,0.15-0.07c0.02-0.05-0.01-0.11-0.07-0.18\n\tc-0.59-0.74-0.89-1.59-0.89-2.53c0-1.1,0.39-2.04,1.18-2.81c0.79-0.77,1.74-1.16,2.85-1.16c1.1,0,2.04,0.39,2.83,1.16\n\tc0.78,0.78,1.18,1.71,1.18,2.8c0,0.94-0.3,1.79-0.89,2.53c-0.07,0.07-0.09,0.13-0.07,0.18c0.02,0.05,0.07,0.07,0.15,0.07h1.88\n\tc0.13,0,0.21-0.05,0.24-0.16c0.41-0.78,0.62-1.66,0.62-2.62c0-0.79-0.16-1.56-0.47-2.29s-0.74-1.37-1.27-1.9s-1.16-0.95-1.89-1.27\n\tc-0.73-0.32-1.5-0.47-2.3-0.47c-0.8,0-1.57,0.16-2.3,0.47c-0.73,0.32-1.36,0.74-1.89,1.27s-0.95,1.16-1.27,1.9\n\tS9.08,14.68,9.08,15.47z M14.04,6.66V4.33c0-0.27,0.1-0.5,0.29-0.69s0.42-0.29,0.69-0.29c0.27,0,0.5,0.1,0.69,0.29\n\ts0.29,0.42,0.29,0.69v2.32c0,0.27-0.1,0.5-0.29,0.69c-0.19,0.19-0.42,0.29-0.69,0.29c-0.27,0-0.5-0.1-0.69-0.29\n\tC14.13,7.15,14.04,6.93,14.04,6.66z M20.31,9.24c0-0.28,0.09-0.51,0.26-0.67l1.63-1.63c0.16-0.2,0.39-0.3,0.68-0.3\n\tc0.27,0,0.5,0.1,0.68,0.29c0.18,0.19,0.27,0.42,0.27,0.69c0,0.28-0.08,0.51-0.25,0.68l-1.66,1.63c-0.23,0.19-0.46,0.28-0.69,0.28\n\tc-0.26,0-0.48-0.09-0.66-0.28C20.4,9.74,20.31,9.51,20.31,9.24z M22.9,15.47c0-0.27,0.09-0.49,0.26-0.67\n\tc0.17-0.18,0.4-0.27,0.67-0.27h2.32c0.27,0,0.5,0.09,0.69,0.27c0.19,0.18,0.29,0.4,0.29,0.67c0,0.27-0.1,0.5-0.29,0.7\n\tc-0.19,0.2-0.42,0.3-0.69,0.3h-2.32c-0.26,0-0.48-0.1-0.66-0.29C22.99,15.99,22.9,15.75,22.9,15.47z\" />",
};
#[cfg(feature = "WiThermometer")]
const WI_THERMOMETER: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.91,19.56c0-0.85,0.2-1.64,0.59-2.38s0.94-1.35,1.65-1.84V5.42c0-0.8,0.27-1.48,0.82-2.03S14.2,2.55,15,2.55\n\tc0.81,0,1.49,0.28,2.04,0.83c0.55,0.56,0.83,1.23,0.83,2.03v9.92c0.71,0.49,1.25,1.11,1.64,1.84s0.58,1.53,0.58,2.38\n\tc0,0.92-0.23,1.78-0.68,2.56s-1.07,1.4-1.85,1.85s-1.63,0.68-2.56,0.68c-0.92,0-1.77-0.23-2.55-0.68s-1.4-1.07-1.86-1.85\n\tS9.91,20.48,9.91,19.56z M11.67,19.56c0,0.93,0.33,1.73,0.98,2.39c0.65,0.66,1.44,0.99,2.36,0.99c0.93,0,1.73-0.33,2.4-1\n\ts1.01-1.46,1.01-2.37c0-0.62-0.16-1.2-0.48-1.73c-0.32-0.53-0.76-0.94-1.32-1.23l-0.28-0.14c-0.1-0.04-0.15-0.14-0.15-0.29V5.42\n\tc0-0.32-0.11-0.59-0.34-0.81C15.62,4.4,15.34,4.29,15,4.29c-0.32,0-0.6,0.11-0.83,0.32c-0.23,0.21-0.34,0.48-0.34,0.81v10.74\n\tc0,0.15-0.05,0.25-0.14,0.29l-0.27,0.14c-0.55,0.29-0.98,0.7-1.29,1.23C11.82,18.35,11.67,18.92,11.67,19.56z M12.45,19.56\n\tc0,0.71,0.24,1.32,0.73,1.82s1.07,0.75,1.76,0.75s1.28-0.25,1.79-0.75c0.51-0.5,0.76-1.11,0.76-1.81c0-0.63-0.22-1.19-0.65-1.67\n\tc-0.43-0.48-0.96-0.77-1.58-0.85V9.69c0-0.06-0.03-0.13-0.1-0.19c-0.07-0.07-0.14-0.1-0.22-0.1c-0.09,0-0.16,0.03-0.21,0.08\n\tc-0.05,0.06-0.08,0.12-0.08,0.21v7.34c-0.61,0.09-1.13,0.37-1.56,0.85C12.66,18.37,12.45,18.92,12.45,19.56z\" />",
};
#[cfg(feature = "WiThermometerExterior")]
const WI_THERMOMETER_EXTERIOR: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M9.91,19.56c0-0.85,0.2-1.64,0.59-2.38s0.94-1.35,1.65-1.84V5.42c0-0.8,0.27-1.48,0.82-2.03S14.2,2.55,15,2.55\n\tc0.81,0,1.49,0.28,2.04,0.83c0.55,0.56,0.83,1.23,0.83,2.03v9.92c0.71,0.49,1.25,1.11,1.64,1.84s0.58,1.53,0.58,2.38\n\tc0,0.92-0.23,1.78-0.68,2.56s-1.07,1.4-1.85,1.85s-1.63,0.68-2.56,0.68c-0.92,0-1.77-0.23-2.55-0.68s-1.4-1.07-1.86-1.85\n\tS9.91,20.48,9.91,19.56z M11.67,19.56c0,0.93,0.33,1.73,0.98,2.39c0.65,0.66,1.44,0.99,2.36,0.99c0.93,0,1.73-0.33,2.4-1\n\ts1.01-1.46,1.01-2.37c0-0.62-0.16-1.2-0.48-1.73c-0.32-0.53-0.76-0.94-1.32-1.23l-0.28-0.14c-0.1-0.04-0.15-0.14-0.15-0.29V5.42\n\tc0-0.32-0.11-0.59-0.34-0.81C15.62,4.4,15.34,4.29,15,4.29c-0.32,0-0.6,0.11-0.83,0.32c-0.23,0.21-0.34,0.48-0.34,0.81v10.74\n\tc0,0.15-0.05,0.25-0.14,0.29l-0.27,0.14c-0.55,0.29-0.98,0.7-1.29,1.23C11.82,18.35,11.67,18.92,11.67,19.56z\" />",
};
#[cfg(feature = "WiThermometerInternal")]
const WI_THERMOMETER_INTERNAL: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M12.48,19.56c0,0.71,0.24,1.32,0.73,1.82c0.49,0.5,1.07,0.75,1.76,0.75s1.28-0.25,1.79-0.75c0.51-0.5,0.76-1.11,0.76-1.81\n\tc0-0.63-0.22-1.19-0.65-1.67c-0.43-0.48-0.96-0.77-1.57-0.85V9.69c0-0.06-0.03-0.13-0.1-0.19c-0.07-0.07-0.14-0.1-0.22-0.1\n\tc-0.09,0-0.16,0.03-0.21,0.08c-0.05,0.06-0.08,0.12-0.08,0.21v7.34c-0.61,0.09-1.13,0.37-1.56,0.85\n\tC12.69,18.37,12.48,18.92,12.48,19.56z\" />",
};
#[cfg(feature = "WiThunderstorm")]
const WI_THUNDERSTORM: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.63,16.91c0,1.11,0.33,2.1,0.99,2.97s1.52,1.47,2.58,1.79l-0.66,1.68c-0.03,0.14,0.02,0.22,0.14,0.22h2.13l-0.98,4.3h0.28\n\tl3.92-5.75c0.04-0.04,0.04-0.09,0.01-0.14c-0.03-0.05-0.08-0.07-0.15-0.07h-2.18l2.48-4.64c0.07-0.14,0.02-0.22-0.14-0.22h-2.94\n\tc-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.87c-0.71-0.18-1.3-0.57-1.77-1.16c-0.47-0.59-0.7-1.26-0.7-2.01c0-0.83,0.28-1.55,0.85-2.17\n\tc0.57-0.61,1.27-0.97,2.1-1.07l0.53-0.07c0.13,0,0.2-0.06,0.2-0.18l0.07-0.51c0.11-1.08,0.56-1.99,1.37-2.72\n\tc0.81-0.73,1.76-1.1,2.85-1.1c1.09,0,2.04,0.37,2.85,1.1c0.82,0.73,1.28,1.64,1.4,2.72l0.07,0.58c0,0.11,0.06,0.17,0.18,0.17h1.6\n\tc0.91,0,1.68,0.32,2.32,0.95c0.64,0.63,0.97,1.4,0.97,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.33,0.97-2.2,1.04\n\tc-0.13,0-0.2,0.06-0.2,0.18v1.37c0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.39-1.51s1.39-2.11,1.39-3.45\n\tc0-0.9-0.22-1.73-0.67-2.49c-0.44-0.76-1.05-1.36-1.81-1.8c-0.77-0.44-1.6-0.66-2.5-0.66H20.1c-0.33-1.33-1.04-2.42-2.11-3.26\n\ts-2.3-1.27-3.68-1.27c-1.41,0-2.67,0.44-3.76,1.31s-1.79,1.99-2.1,3.36c-1.11,0.26-2.02,0.83-2.74,1.73S4.63,15.76,4.63,16.91z\n\t M12.77,26.62c0,0.39,0.19,0.65,0.58,0.77c0.01,0,0.05,0,0.11,0.01c0.06,0.01,0.11,0.01,0.14,0.01c0.17,0,0.33-0.05,0.49-0.15\n\tc0.16-0.1,0.27-0.26,0.32-0.48l2.25-8.69c0.06-0.24,0.04-0.45-0.07-0.65c-0.11-0.19-0.27-0.32-0.5-0.39\n\tc-0.17-0.02-0.26-0.03-0.26-0.03c-0.16,0-0.32,0.05-0.47,0.15c-0.15,0.1-0.26,0.25-0.31,0.45l-2.26,8.72\n\tC12.78,26.44,12.77,26.53,12.77,26.62z M16.93,23.56c0,0.13,0.03,0.26,0.1,0.38c0.14,0.22,0.31,0.37,0.51,0.44\n\tc0.11,0.03,0.21,0.05,0.3,0.05s0.2-0.02,0.32-0.08c0.21-0.09,0.35-0.28,0.42-0.57l1.44-5.67c0.03-0.14,0.05-0.23,0.05-0.27\n\tc0-0.15-0.05-0.3-0.16-0.45s-0.26-0.26-0.46-0.32c-0.17-0.02-0.26-0.03-0.26-0.03c-0.17,0-0.33,0.05-0.47,0.15\n\tc-0.14,0.1-0.24,0.25-0.3,0.45l-1.46,5.7c0,0.02,0,0.05-0.01,0.11C16.93,23.5,16.93,23.53,16.93,23.56z\" />",
};
#[cfg(feature = "WiTime1")]
const WI_TIME1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,14.47V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24c0.22,0,0.42,0.08,0.59,0.24s0.25,0.36,0.25,0.59v3.53l0.75-1.3\n\tc0.12-0.2,0.29-0.32,0.52-0.38s0.44-0.03,0.64,0.09c0.2,0.11,0.32,0.27,0.39,0.5s0.04,0.43-0.08,0.63l-2.29,3.91\n\tc-0.13,0.35-0.38,0.53-0.76,0.53c-0.23,0-0.43-0.08-0.59-0.24S14.14,14.71,14.14,14.47z\" />",
};
#[cfg(feature = "WiTime10")]
const WI_TIME10: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M10.14,12.41c-0.07-0.22-0.04-0.43,0.07-0.63c0.11-0.2,0.28-0.33,0.51-0.4s0.44-0.04,0.64,0.07l2.78,1.57V7.81\n\tc0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v6.67c0,0.23-0.08,0.43-0.24,0.59\n\ts-0.36,0.24-0.59,0.24c-0.21,0-0.39-0.07-0.56-0.22l-3.88-2.17C10.34,12.8,10.21,12.63,10.14,12.41z\" />",
};
#[cfg(feature = "WiTime11")]
const WI_TIME11: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21c-1.35-0.79-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18C9.27,7.65,8.2,8.72,7.4,10.07\n\tS6.22,12.89,6.22,14.47z M11.84,10.24c0.06-0.22,0.19-0.39,0.38-0.5c0.2-0.12,0.41-0.15,0.64-0.09s0.4,0.19,0.51,0.38l0.78,1.3V7.81\n\tc0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v6.67c0,0.23-0.08,0.43-0.24,0.59\n\ts-0.36,0.24-0.59,0.24c-0.4,0-0.66-0.18-0.79-0.53l-2.26-3.91C11.81,10.67,11.78,10.46,11.84,10.24z\" />",
};
#[cfg(feature = "WiTime12")]
const WI_TIME12: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,14.47c0,0.22,0.08,0.42,0.24,0.59c0.16,0.17,0.36,0.25,0.59,0.25c0.22,0,0.42-0.08,0.59-0.25c0.17-0.17,0.25-0.36,0.25-0.59\n\tV7.81c0-0.23-0.08-0.43-0.25-0.59s-0.36-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.59,0.24s-0.24,0.36-0.24,0.59V14.47z\" />",
};
#[cfg(feature = "WiTime2")]
const WI_TIME2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,14.47V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v5.21l2.78-1.57\n\tc0.2-0.12,0.41-0.15,0.63-0.09s0.39,0.2,0.5,0.41c0.12,0.2,0.14,0.41,0.08,0.63s-0.19,0.4-0.39,0.51l-3.88,2.17\n\tc-0.17,0.15-0.35,0.22-0.56,0.22c-0.23,0-0.43-0.08-0.59-0.24S14.14,14.71,14.14,14.47z\" />",
};
#[cfg(feature = "WiTime3")]
const WI_TIME3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,14.47V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v5.82h3.78\n\tc0.23,0,0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59c0,0.22-0.08,0.42-0.24,0.59c-0.16,0.17-0.36,0.25-0.59,0.25h-4.44\n\tc-0.03,0.01-0.09,0.01-0.18,0.01c-0.23,0-0.43-0.08-0.59-0.24S14.14,14.71,14.14,14.47z\" />",
};
#[cfg(feature = "WiTime4")]
const WI_TIME4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,14.47V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v6.15l3.59,2.09\n\tc0.2,0.12,0.32,0.29,0.38,0.51s0.03,0.43-0.09,0.62c-0.16,0.28-0.4,0.42-0.72,0.42c-0.17,0-0.31-0.04-0.42-0.12l-3.82-2.23\n\tc-0.17-0.05-0.31-0.15-0.42-0.29S14.14,14.66,14.14,14.47z\" />",
};
#[cfg(feature = "WiTime5")]
const WI_TIME5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,14.47V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v6.42l2.15,3.84\n\tc0.12,0.21,0.14,0.43,0.08,0.65s-0.19,0.39-0.39,0.51c-0.11,0.06-0.24,0.09-0.41,0.09c-0.33,0-0.58-0.14-0.73-0.41l-2.2-3.9\n\tC14.2,14.85,14.14,14.68,14.14,14.47z\" />",
};
#[cfg(feature = "WiTime6")]
const WI_TIME6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M14.14,19.07V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v11.26\n\tc0,0.23-0.08,0.43-0.24,0.6s-0.36,0.25-0.59,0.25s-0.43-0.08-0.59-0.25S14.14,19.31,14.14,19.07z\" />",
};
#[cfg(feature = "WiTime7")]
const WI_TIME7: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M11.89,18.71c-0.06-0.22-0.04-0.44,0.08-0.65l2.17-3.84V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24\n\ts0.24,0.36,0.24,0.59v6.67c0,0.2-0.06,0.37-0.19,0.53l-2.18,3.9c-0.16,0.27-0.41,0.41-0.75,0.41c-0.16,0-0.29-0.03-0.4-0.09\n\tC12.09,19.1,11.96,18.93,11.89,18.71z\" />",
};
#[cfg(feature = "WiTime8")]
const WI_TIME8: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M10.17,16.56c0.06-0.22,0.19-0.39,0.38-0.51l3.59-2.09V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24s0.43,0.08,0.59,0.24\n\ts0.24,0.36,0.24,0.59v6.67c0,0.19-0.06,0.35-0.17,0.5s-0.25,0.24-0.42,0.29l-3.84,2.23c-0.12,0.08-0.25,0.12-0.41,0.12\n\tc-0.32,0-0.56-0.14-0.72-0.42C10.14,16.99,10.11,16.78,10.17,16.56z\" />",
};
#[cfg(feature = "WiTime9")]
const WI_TIME9: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.47c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16,3.74,14.47z M6.22,14.47\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.72,7.4,10.07S6.22,12.89,6.22,14.47z\n\t M9.51,14.46c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24h3.79V7.81c0-0.23,0.08-0.43,0.24-0.59s0.36-0.24,0.59-0.24\n\ts0.43,0.08,0.59,0.24s0.24,0.36,0.24,0.59v6.67c0,0.23-0.08,0.43-0.24,0.59s-0.36,0.24-0.59,0.24c-0.1,0-0.16,0-0.19-0.01h-4.44\n\tc-0.23,0-0.43-0.08-0.59-0.25C9.59,14.88,9.51,14.68,9.51,14.46z\" />",
};
#[cfg(feature = "WiTornado")]
const WI_TORNADO: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.13,15.19c0,0.69,0.36,1.28,1.08,1.77c1.32,0.93,3.31,1.39,5.98,1.39c1.2,0,2.31-0.1,3.34-0.31\n\tc1.08-0.23,1.97-0.6,2.65-1.1s1.03-1.08,1.03-1.76c0-0.21-0.04-0.41-0.12-0.62c1.39-0.34,2.48-0.8,3.27-1.38s1.19-1.25,1.19-2\n\tc0-0.19-0.03-0.39-0.09-0.6c2.29-0.81,3.43-1.9,3.43-3.28c0-0.88-0.5-1.66-1.49-2.34c-1.95-1.3-4.81-1.95-8.58-1.95\n\tc-1.78,0-3.39,0.16-4.83,0.47C9.42,3.8,8.16,4.3,7.2,4.98S5.76,6.44,5.76,7.31c0,0.52,0.16,0.99,0.48,1.42\n\tc-1.18,0.67-1.77,1.49-1.77,2.46c0,0.75,0.37,1.41,1.1,1.98C4.61,13.73,4.13,14.4,4.13,15.19z M4.73,19.69\n\tc0,0.73,0.45,1.31,1.35,1.72s2.04,0.62,3.41,0.62c1.39,0,2.53-0.21,3.44-0.62s1.36-0.99,1.36-1.72c0-0.27-0.09-0.5-0.26-0.69\n\ts-0.4-0.28-0.67-0.28c-0.22,0-0.42,0.08-0.6,0.23s-0.29,0.35-0.34,0.57c-0.2,0.16-0.56,0.3-1.1,0.43s-1.15,0.2-1.83,0.2\n\tc-1.1,0-2-0.16-2.68-0.47c0.16-0.16,0.24-0.36,0.26-0.6s-0.04-0.45-0.15-0.62c-0.16-0.21-0.36-0.35-0.61-0.4s-0.48,0-0.7,0.13\n\tC5.02,18.6,4.73,19.09,4.73,19.69z M6.01,15.19c0-0.01,0.06-0.07,0.19-0.18c0.09-0.09,0.28-0.2,0.56-0.34s0.61-0.25,0.96-0.35\n\tl0.12-0.06c1.62,0.54,3.51,0.81,5.67,0.81c0.95,0,1.81-0.05,2.58-0.16l0.26,0.23c-0.09,0.16-0.3,0.32-0.63,0.5\n\tc-0.4,0.21-1.02,0.41-1.86,0.57s-1.73,0.25-2.67,0.25s-1.83-0.08-2.67-0.25s-1.47-0.36-1.88-0.57C6.3,15.5,6.09,15.35,6.01,15.19z\n\t M6.12,23.61c0,0.63,0.36,1.12,1.08,1.46s1.61,0.51,2.67,0.51c1.08,0,1.99-0.17,2.72-0.51s1.1-0.83,1.1-1.46\n\tc0-0.25-0.09-0.48-0.28-0.67s-0.41-0.29-0.66-0.29c-0.47,0-0.78,0.24-0.92,0.72c-0.39,0.24-1.04,0.37-1.96,0.37\n\tc-0.8,0-1.44-0.12-1.92-0.37c-0.15-0.48-0.45-0.72-0.92-0.72c-0.25,0-0.47,0.09-0.64,0.28S6.12,23.34,6.12,23.61z M6.33,11.19\n\tc0-0.08,0.05-0.17,0.15-0.28c0.24-0.3,0.72-0.6,1.42-0.88c1.92,1.03,4.56,1.54,7.91,1.54c1.71,0,3.32-0.16,4.82-0.47v0.09\n\tc0,0.15-0.09,0.3-0.28,0.45c-0.41,0.36-1.17,0.7-2.29,1.03c-1.21,0.36-2.73,0.54-4.56,0.54c-1.84,0-3.36-0.18-4.57-0.54\n\tC7.77,12.35,7,12.01,6.61,11.65C6.42,11.5,6.33,11.35,6.33,11.19z M7.63,7.31c0-0.18,0.12-0.37,0.35-0.59\n\tC8.43,6.3,9.33,5.9,10.66,5.51c1.43-0.42,3.14-0.63,5.14-0.63c2.01,0,3.74,0.21,5.19,0.63c1.35,0.39,2.24,0.8,2.68,1.22\n\tc0.22,0.22,0.34,0.42,0.34,0.59s-0.11,0.35-0.34,0.56c-0.44,0.42-1.33,0.83-2.68,1.23c-1.45,0.42-3.17,0.63-5.19,0.63\n\tc-2,0-3.72-0.21-5.14-0.63C9.32,8.71,8.42,8.3,7.98,7.87C7.75,7.66,7.63,7.48,7.63,7.31z\" />",
};
#[cfg(feature = "WiTrain")]
const WI_TRAIN: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.25,12.68v-0.32c0-0.1,0.03-0.18,0.1-0.25c0.07-0.07,0.15-0.1,0.25-0.1h7.58c0.1,0,0.18,0.03,0.25,0.1\n\tc0.07,0.07,0.1,0.15,0.1,0.25v0.32c0,0.1-0.03,0.18-0.1,0.25c-0.07,0.07-0.15,0.1-0.25,0.1h-0.44v1.65h2.12\n\tc0.02-0.28,0.14-0.52,0.35-0.71c0.21-0.19,0.46-0.29,0.75-0.29c0.29,0,0.53,0.1,0.74,0.29c0.21,0.19,0.32,0.43,0.35,0.71h1.32v-3.39\n\tc-0.14-0.01-0.25-0.06-0.35-0.16c-0.1-0.1-0.15-0.23-0.15-0.37v-0.31c0-0.14,0.05-0.27,0.16-0.38s0.24-0.16,0.39-0.16h1.99\n\tc0.15,0,0.28,0.05,0.38,0.16s0.15,0.23,0.15,0.38v0.31c0,0.14-0.05,0.27-0.14,0.37c-0.09,0.1-0.2,0.16-0.34,0.16v3.39h1.56\n\tc0.27,0,0.51,0.1,0.71,0.3s0.3,0.44,0.3,0.71v2.93l3.73,4.87h-4.74v-3.04h-0.71c0.11,0.26,0.16,0.54,0.16,0.83\n\tc0,0.61-0.21,1.12-0.64,1.56c-0.43,0.43-0.95,0.65-1.55,0.65c-0.61,0-1.12-0.22-1.56-0.65c-0.43-0.43-0.65-0.95-0.65-1.56\n\tc0-0.29,0.05-0.57,0.16-0.83h-1c0.11,0.27,0.17,0.55,0.17,0.83c0,0.61-0.22,1.12-0.65,1.56s-0.95,0.65-1.56,0.65\n\tc-0.61,0-1.12-0.22-1.55-0.65s-0.64-0.95-0.64-1.56c0-0.29,0.05-0.57,0.16-0.83H9.97c0.12,0.29,0.18,0.57,0.18,0.83\n\tc0,0.61-0.22,1.12-0.65,1.56s-0.95,0.65-1.56,0.65s-1.12-0.22-1.56-0.65s-0.65-0.95-0.65-1.56c0-0.29,0.06-0.57,0.17-0.84\n\tc-0.24-0.04-0.45-0.15-0.61-0.34s-0.24-0.41-0.24-0.66v-0.86H5.03v-5.55H4.6c-0.1,0-0.18-0.03-0.25-0.1\n\tC4.28,12.86,4.25,12.78,4.25,12.68z M6.3,16.62c0,0.21,0.07,0.39,0.22,0.54c0.15,0.15,0.33,0.22,0.54,0.22h1.44\n\tc0.21,0,0.39-0.07,0.53-0.22c0.14-0.15,0.22-0.33,0.22-0.54v-2.3c0-0.21-0.07-0.38-0.22-0.53c-0.15-0.15-0.32-0.22-0.53-0.22H7.07\n\tc-0.21,0-0.39,0.07-0.54,0.23c-0.15,0.15-0.22,0.32-0.22,0.52V16.62z M15.78,5.43c0,0.41,0.16,0.76,0.47,1.04\n\tc0,0.2,0.09,0.43,0.26,0.68s0.36,0.4,0.56,0.44c0.04,0.22,0.15,0.41,0.31,0.57c0.16,0.15,0.36,0.25,0.59,0.3\n\tc-0.11,0.11-0.16,0.24-0.16,0.39c0,0.18,0.06,0.33,0.18,0.45c0.12,0.12,0.27,0.18,0.45,0.18c0.18,0,0.33-0.06,0.46-0.19\n\tc0.13-0.12,0.19-0.28,0.19-0.45c0-0.02,0-0.05-0.01-0.09c-0.01-0.04-0.01-0.08-0.01-0.1h0.03c0.21,0,0.39-0.08,0.54-0.23\n\tc0.15-0.15,0.23-0.34,0.23-0.55c0-0.1-0.04-0.22-0.12-0.38c0.17-0.09,0.31-0.25,0.41-0.47h0.45C21,7,21.34,6.85,21.61,6.57\n\tc0.28-0.28,0.42-0.61,0.42-1.01c0-0.34-0.11-0.64-0.33-0.9c-0.22-0.26-0.5-0.43-0.83-0.52c-0.08-0.4-0.29-0.73-0.62-0.99\n\ts-0.71-0.39-1.12-0.39c-0.41,0-0.77,0.13-1.08,0.38c-0.31,0.25-0.52,0.58-0.62,0.97h-0.11c-0.41,0-0.77,0.13-1.08,0.39\n\tC15.93,4.75,15.78,5.07,15.78,5.43z\" />",
};
#[cfg(feature = "WiTsunami")]
const WI_TSUNAMI: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.07,21.24c0-0.16,0.06-0.3,0.17-0.42c0.12-0.12,0.25-0.18,0.41-0.18h0.4c-0.21-0.66-0.39-1.35-0.53-2.07\n\tc-0.21-1.1-0.32-2.1-0.32-2.99c0-1.71,0.3-3.32,0.91-4.81C6.73,9.31,7.59,8.06,8.7,7.01c1.12-1.06,2.42-1.87,3.9-2.42\n\tc1.51-0.57,3.14-0.86,4.91-0.86c1.06,0,2.06,0.09,3,0.28c0.94,0.22,1.85,0.56,2.73,1.03l1.7,0.91l-1.88,0.39\n\tc-0.58,0.13-0.98,0.39-1.2,0.78c-0.16,0.32-0.15,0.69,0.03,1.11l0.41,0.95l-1.02,0.05c-0.43,0.03-0.83,0.12-1.18,0.27\n\tc-0.33,0.16-0.52,0.32-0.58,0.5c-0.11,0.23,0.01,0.56,0.36,1l0.81,0.96l-1.26,0.18c-1.55,0.23-2.82,0.55-3.81,0.96\n\ts-1.77,0.94-2.35,1.59c-0.56,0.62-0.98,1.42-1.25,2.37c-0.27,0.96-0.42,2.15-0.45,3.59h5.26v-2.78l-0.38,0.23\n\tc-0.14,0.09-0.29,0.11-0.45,0.07c-0.17-0.04-0.29-0.13-0.37-0.28c-0.09-0.14-0.11-0.29-0.08-0.45s0.12-0.29,0.27-0.38l3.82-2.38\n\tl0.02-0.02c0.01,0,0.01,0,0.01-0.01h0.02c0.01,0,0.02,0,0.03-0.01c0.07-0.02,0.14-0.05,0.23-0.07h0.06\n\tc0.01,0.01,0.02,0.02,0.03,0.02h0.07c0,0.01,0.01,0.01,0.02,0.01h0.03l0.02,0.01h0.02c0.01,0.01,0.02,0.02,0.02,0.03h0.02\n\tc0.01,0,0.01,0,0.01,0.01c0.02,0,0.03,0,0.03,0.01c0.01,0,0.02,0,0.03,0.01l0.02,0.01l3.82,2.35c0.14,0.09,0.23,0.22,0.27,0.38\n\tc0.03,0.17,0.01,0.32-0.08,0.46c-0.08,0.14-0.2,0.23-0.37,0.26s-0.32,0.01-0.45-0.08l-0.31-0.19v2.77h0.96\n\tc0.16,0,0.29,0.06,0.4,0.18c0.11,0.12,0.16,0.26,0.16,0.42c0.01,0.17-0.05,0.31-0.16,0.43c-0.11,0.12-0.25,0.18-0.4,0.18H5.65\n\tc-0.16,0-0.3-0.06-0.41-0.17C5.13,21.56,5.07,21.42,5.07,21.24z M6.62,15.58c0,0.71,0.1,1.62,0.3,2.73\n\tc0.15,0.81,0.33,1.52,0.54,2.12h2.69c0.05-1.45,0.2-2.65,0.45-3.59c0.35-1.27,0.88-2.31,1.6-3.09c0.73-0.82,1.69-1.47,2.89-1.96\n\tc0.82-0.34,1.86-0.63,3.11-0.87l-0.08-0.25c-0.1-0.46-0.07-0.87,0.09-1.23c0.22-0.51,0.65-0.92,1.28-1.21\n\tc0.07-0.03,0.13-0.06,0.19-0.07c-0.86-0.2-1.73-0.25-2.6-0.14c-0.99,0.12-1.92,0.41-2.78,0.85c-1.11,0.58-2.11,1.41-3.01,2.48\n\tc-0.1,0.12-0.23,0.18-0.38,0.18c-0.12,0-0.22-0.03-0.31-0.1c-0.1-0.09-0.16-0.2-0.17-0.34s0.02-0.26,0.11-0.37\n\tc1-1.19,2.11-2.1,3.34-2.73c0.98-0.49,2.03-0.81,3.14-0.95c0.57-0.07,1.12-0.08,1.67-0.02c0.54,0.06,0.92,0.12,1.14,0.17\n\ts0.37,0.09,0.45,0.12l0.08,0.03c0.05-0.31,0.13-0.59,0.24-0.84c0.16-0.29,0.37-0.56,0.64-0.8c-0.3-0.09-0.65-0.19-1.04-0.28\n\tc-0.8-0.18-1.7-0.26-2.69-0.26c-1.58,0-3.05,0.26-4.42,0.77c-1.34,0.51-2.48,1.22-3.42,2.14c-0.98,0.91-1.73,2-2.23,3.26\n\tC6.89,12.65,6.62,14.07,6.62,15.58z M18.05,20.64h3.88v-3.52l-1.98-1.21l-1.9,1.19V20.64z\" />",
};
#[cfg(feature = "WiUmbrella")]
const WI_UMBRELLA: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.64,14.65c0.01-0.34,0.18-0.86,0.5-1.58c0.32-0.72,0.76-1.48,1.33-2.3c1.86-2.61,4.49-3.98,7.88-4.13V6.21\n\tc0-0.21,0.07-0.37,0.21-0.5c0.14-0.13,0.3-0.19,0.48-0.19c0.19,0,0.35,0.06,0.5,0.19c0.14,0.13,0.22,0.3,0.22,0.5v0.44\n\tc0.98,0.04,1.9,0.19,2.75,0.45c0.85,0.26,1.59,0.59,2.22,1c0.63,0.41,1.17,0.83,1.61,1.27c0.45,0.43,0.85,0.9,1.2,1.41\n\tc0.41,0.59,0.77,1.23,1.06,1.9c0.29,0.67,0.5,1.21,0.61,1.61c0.11,0.4,0.17,0.6,0.18,0.61v0.19c0,0.18-0.07,0.32-0.21,0.44\n\ts-0.3,0.17-0.49,0.17c-0.31,0-0.51-0.09-0.6-0.26c-0.78-0.88-1.63-1.31-2.55-1.31c-0.34,0.02-0.69,0.1-1.03,0.23\n\tc-0.34,0.13-0.62,0.27-0.82,0.42c-0.21,0.14-0.4,0.29-0.58,0.44c-0.18,0.15-0.27,0.22-0.28,0.23c-0.19,0.17-0.37,0.26-0.53,0.26\n\tc-0.23,0-0.4-0.06-0.52-0.18c-0.73-0.73-1.39-1.17-2.01-1.32v7.57l0,0.17l-0.01,0.21l-0.04,0.23l-0.06,0.25l-0.09,0.26l-0.13,0.27\n\tl-0.17,0.26l-0.21,0.25c-0.51,0.59-1.23,0.88-2.18,0.88c-1.01,0-1.77-0.29-2.28-0.88c-0.12-0.12-0.22-0.25-0.31-0.38\n\tc-0.09-0.14-0.16-0.27-0.21-0.41c-0.05-0.13-0.09-0.26-0.12-0.38s-0.05-0.24-0.06-0.36C9.88,22,9.87,21.9,9.87,21.82s0-0.16,0-0.23\n\ts0.01-0.12,0.01-0.13c0-0.18,0.08-0.34,0.23-0.47c0.16-0.13,0.34-0.18,0.55-0.14c0.18,0,0.32,0.08,0.44,0.23s0.18,0.34,0.18,0.56\n\tc-0.06,0.41,0.02,0.76,0.25,1.05c0.21,0.29,0.65,0.44,1.32,0.44c0.52,0,0.9-0.12,1.13-0.36c0.13-0.13,0.23-0.29,0.29-0.48\n\tc0.06-0.19,0.09-0.34,0.08-0.47l-0.01-0.19v-7.36c-0.73,0.18-1.38,0.56-1.93,1.14c-0.04,0.08-0.12,0.16-0.23,0.23\n\ts-0.21,0.11-0.3,0.11c-0.18,0-0.38-0.11-0.6-0.34c-0.8-0.89-1.65-1.33-2.55-1.31c-0.4,0.01-0.78,0.07-1.12,0.2\n\tC7.26,14.43,7,14.56,6.82,14.69c-0.17,0.13-0.36,0.28-0.54,0.45s-0.29,0.27-0.32,0.29c-0.21,0.14-0.38,0.22-0.51,0.22\n\ts-0.3-0.06-0.48-0.17c-0.16-0.1-0.26-0.21-0.3-0.32C4.64,15.04,4.63,14.87,4.64,14.65z M6.73,13.23c0.68-0.36,1.32-0.53,1.92-0.53\n\th0.08c1.15,0,2.2,0.44,3.15,1.33c0.38-0.33,0.84-0.62,1.39-0.88c0.54-0.26,1.13-0.41,1.77-0.45h0.08c1.15,0,2.2,0.44,3.15,1.33\n\tc0.38-0.33,0.84-0.62,1.39-0.88c0.54-0.26,1.13-0.41,1.77-0.45h0.09c0.56,0,1.15,0.15,1.75,0.44c-0.44-0.86-0.74-1.41-0.88-1.66\n\tc-1.79-2.34-4.27-3.51-7.43-3.51c-1.58,0-2.99,0.3-4.24,0.9c-1.24,0.6-2.26,1.47-3.05,2.61C7.44,11.82,7.12,12.41,6.73,13.23z\" />",
};
#[cfg(feature = "WiVolcano")]
const WI_VOLCANO: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M7.39,22.61c-0.12-0.27-0.09-0.54,0.09-0.81l1.4-2.67c0.01-0.04,0.05-0.09,0.11-0.15c0.04-0.04,0.17-0.14,0.38-0.29\n\tc0.02-0.01,0.25-0.18,0.68-0.5c0.48-0.32,1.03-0.72,1.68-1.19l1.8-2.98c0.17-0.27,0.41-0.41,0.72-0.41h0.7\n\tc-0.16,0.19-0.31,0.39-0.45,0.6c-0.14,0.21-0.27,0.5-0.38,0.85c-0.12,0.36-0.18,0.71-0.18,1.07c0,0.36,0.09,0.77,0.28,1.25\n\tc0.19,0.47,0.48,0.94,0.88,1.39c0.27,0.31,0.44,0.62,0.5,0.93s0.02,0.58-0.1,0.83c-0.12,0.25-0.32,0.5-0.59,0.74\n\tc-0.27,0.24-0.56,0.45-0.88,0.63c-0.32,0.18-0.68,0.35-1.07,0.52c-0.39,0.17-0.75,0.3-1.05,0.41c-0.31,0.1-0.62,0.2-0.93,0.29H8.16\n\tc-0.16,0-0.32-0.05-0.46-0.14C7.55,22.89,7.45,22.77,7.39,22.61z M10.14,7.74c0-0.46,0.15-0.88,0.45-1.24\n\tc0.3-0.37,0.69-0.6,1.16-0.72c0.11-0.56,0.4-1.02,0.85-1.38s0.98-0.54,1.56-0.54c0.56,0,1.06,0.17,1.5,0.52s0.73,0.8,0.86,1.35h0.14\n\tc0.57,0,1.07,0.18,1.5,0.54c0.42,0.36,0.64,0.79,0.64,1.3c0,0.56-0.22,1.03-0.65,1.43c0,0.27-0.12,0.59-0.36,0.93\n\tc-0.24,0.35-0.5,0.55-0.78,0.61c-0.06,0.31-0.21,0.57-0.43,0.78c-0.23,0.22-0.5,0.36-0.82,0.43c0.15,0.16,0.22,0.34,0.22,0.54\n\tc0,0.25-0.09,0.46-0.26,0.63c-0.18,0.17-0.39,0.25-0.64,0.25c-0.24,0-0.45-0.09-0.63-0.26c-0.18-0.17-0.26-0.38-0.26-0.62\n\tc0-0.03,0.01-0.08,0.02-0.14s0.02-0.11,0.02-0.13H14.2c-0.29,0-0.54-0.11-0.75-0.32c-0.21-0.21-0.32-0.46-0.32-0.75\n\tc0-0.12,0.06-0.3,0.18-0.53c-0.24-0.12-0.43-0.33-0.57-0.63h-0.63c-0.54-0.05-1.01-0.27-1.39-0.65C10.34,8.76,10.14,8.29,10.14,7.74\n\tz M14.76,15.48c0-0.16,0.02-0.34,0.07-0.54c0.05-0.2,0.11-0.35,0.16-0.47c0.05-0.12,0.12-0.27,0.21-0.45\n\tc0.09-0.18,0.15-0.31,0.19-0.41h0.38c0.28,0,0.49,0.11,0.66,0.32l0.07,0.1l1.31,2.48l4.65,5.23l0.04,0.03\n\tc0.21,0.27,0.24,0.56,0.08,0.88c-0.15,0.31-0.4,0.46-0.75,0.46H16.2c0.17-0.16,0.32-0.29,0.44-0.39c0.12-0.11,0.27-0.27,0.45-0.49\n\ts0.33-0.43,0.42-0.61s0.17-0.42,0.23-0.69c0.06-0.27,0.07-0.53,0.01-0.79c-0.06-0.25-0.18-0.53-0.38-0.84\n\tc-0.19-0.31-0.46-0.61-0.81-0.91c-0.34-0.3-0.64-0.59-0.88-0.88c-0.24-0.28-0.43-0.54-0.56-0.76c-0.13-0.22-0.23-0.45-0.29-0.68\n\tC14.79,15.84,14.76,15.64,14.76,15.48z\" />",
};
#[cfg(feature = "WiWindBeaufort0")]
const WI_WIND_BEAUFORT0: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.01,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17H16c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21\n\tc-0.13-0.11-0.26-0.16-0.4-0.16c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41\n\tc0.36,0.36,0.78,0.53,1.28,0.53s0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27\n\tc-0.35-0.35-0.77-0.53-1.26-0.53H5.6c-0.16,0-0.3,0.06-0.42,0.17C5.07,13.21,5.01,13.34,5.01,13.5z M5.01,11.48\n\tc0,0.17,0.06,0.3,0.17,0.39c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27\n\tc0-0.49-0.17-0.91-0.52-1.26s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42\n\tc0,0.16,0.05,0.3,0.16,0.4c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18\n\tc0.17,0,0.33,0.06,0.46,0.18c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.6\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.07,11.18,5.01,11.32,5.01,11.48z M18.27,18.9c0,0.52,0.08,0.98,0.24,1.37s0.38,0.71,0.66,0.94\n\tc0.28,0.23,0.58,0.4,0.91,0.52c0.33,0.11,0.68,0.17,1.05,0.17c0.51,0,0.98-0.09,1.41-0.26c0.43-0.17,0.77-0.4,1.05-0.69\n\tc0.27-0.29,0.51-0.61,0.71-0.95c0.2-0.34,0.35-0.7,0.45-1.08s0.18-0.72,0.23-1.03s0.07-0.6,0.07-0.86c0-0.97-0.27-1.72-0.8-2.25\n\ts-1.24-0.8-2.12-0.8c-0.49,0-0.97,0.12-1.43,0.35s-0.87,0.56-1.23,0.98c-0.36,0.42-0.65,0.94-0.86,1.56\n\tC18.38,17.49,18.27,18.17,18.27,18.9z M20.32,18.96c0-0.15,0.01-0.34,0.04-0.58c0.03-0.23,0.08-0.51,0.16-0.83\n\tc0.08-0.32,0.18-0.62,0.3-0.89c0.12-0.27,0.29-0.5,0.52-0.69c0.22-0.19,0.47-0.29,0.75-0.29c0.27,0,0.49,0.09,0.65,0.26\n\tc0.16,0.17,0.23,0.44,0.23,0.79c0,0.96-0.17,1.78-0.5,2.45s-0.74,1.01-1.23,1.01C20.63,20.19,20.32,19.78,20.32,18.96z\" />",
};
#[cfg(feature = "WiWindBeaufort1")]
const WI_WIND_BEAUFORT1: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.76,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\ts0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H6.35\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.81,13.21,5.76,13.34,5.76,13.5z M5.76,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H6.35c-0.16,0-0.3,0.06-0.42,0.17\n\tC5.81,11.18,5.76,11.32,5.76,11.48z M18.65,21.85h2.47l1.65-7.98H20.3L18.65,21.85z\" />",
};
#[cfg(feature = "WiWindBeaufort10")]
const WI_WIND_BEAUFORT10: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.15,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\ts0.91-0.17,1.26-0.52s0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H3.75\n\tc-0.16,0-0.3,0.06-0.42,0.17C3.21,13.21,3.15,13.34,3.15,13.5z M3.15,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H3.75c-0.16,0-0.3,0.06-0.42,0.17\n\tC3.21,11.18,3.15,11.32,3.15,11.48z M15.97,21.8h2.46l1.64-7.94h-2.45L15.97,21.8z M20.16,18.88c0,0.52,0.08,0.98,0.24,1.38\n\ts0.38,0.72,0.66,0.95c0.27,0.23,0.58,0.4,0.9,0.52s0.68,0.17,1.05,0.17c0.61,0,1.16-0.12,1.64-0.38c0.48-0.25,0.86-0.56,1.13-0.93\n\tc0.27-0.37,0.5-0.79,0.68-1.25c0.18-0.47,0.3-0.89,0.37-1.27c0.06-0.38,0.09-0.73,0.09-1.05c0-0.97-0.27-1.72-0.8-2.25\n\ts-1.24-0.8-2.13-0.8c-1.03,0-1.93,0.46-2.7,1.37C20.54,16.26,20.16,17.44,20.16,18.88z M22.21,18.98c0-0.16,0.01-0.35,0.04-0.59\n\tc0.03-0.23,0.08-0.51,0.16-0.84c0.08-0.32,0.18-0.62,0.3-0.9c0.12-0.27,0.29-0.5,0.52-0.69c0.22-0.19,0.47-0.29,0.75-0.29\n\tc0.27,0,0.48,0.09,0.65,0.27c0.16,0.18,0.24,0.44,0.24,0.79c0,0.96-0.17,1.78-0.5,2.45s-0.75,1.01-1.23,1.01\n\tC22.52,20.19,22.21,19.79,22.21,18.98z\" />",
};
#[cfg(feature = "WiWindBeaufort11")]
const WI_WIND_BEAUFORT11: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.68,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21\n\tc-0.13-0.11-0.26-0.16-0.4-0.16c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41\n\tc0.36,0.36,0.78,0.53,1.28,0.53s0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27\n\tc-0.35-0.35-0.77-0.53-1.26-0.53H5.27c-0.16,0-0.3,0.06-0.42,0.17C4.74,13.21,4.68,13.34,4.68,13.5z M4.68,11.48\n\tc0,0.17,0.06,0.3,0.17,0.39c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27\n\tc0-0.49-0.17-0.91-0.52-1.26s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42\n\tc0,0.16,0.05,0.3,0.16,0.4c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18\n\tc0.17,0,0.33,0.06,0.46,0.18c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.27\n\tc-0.16,0-0.3,0.06-0.42,0.17C4.74,11.18,4.68,11.32,4.68,11.48z M17.57,21.9h2.47l1.65-7.99h-2.47L17.57,21.9z M21.3,21.9h2.46\n\tl1.65-7.99h-2.45L21.3,21.9z\" />",
};
#[cfg(feature = "WiWindBeaufort12")]
const WI_WIND_BEAUFORT12: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.07,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21\n\tc-0.13-0.11-0.26-0.16-0.4-0.16c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41\n\tc0.36,0.36,0.78,0.53,1.28,0.53s0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27\n\tc-0.35-0.35-0.77-0.53-1.26-0.53H3.66c-0.16,0-0.3,0.06-0.42,0.17C3.13,13.21,3.07,13.34,3.07,13.5z M3.07,11.48\n\tc0,0.17,0.06,0.3,0.17,0.39c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27\n\tc0-0.49-0.17-0.91-0.52-1.26s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42\n\tc0,0.16,0.05,0.3,0.16,0.4c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18\n\tc0.17,0,0.33,0.06,0.46,0.18c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H3.66\n\tc-0.16,0-0.3,0.06-0.42,0.17C3.13,11.18,3.07,11.32,3.07,11.48z M15.96,21.9h2.47l1.65-7.99h-2.47L15.96,21.9z M19.51,21.9h6.62\n\tl0.4-1.9h-3.67v-0.02c0.2-0.09,0.49-0.22,0.86-0.37c0.38-0.15,0.69-0.28,0.95-0.38s0.54-0.25,0.86-0.44\n\tc0.32-0.19,0.58-0.38,0.77-0.58s0.36-0.45,0.5-0.75s0.21-0.64,0.21-1c0-0.56-0.14-1.02-0.43-1.4c-0.29-0.38-0.65-0.64-1.08-0.8\n\tc-0.43-0.16-0.92-0.23-1.45-0.23c-0.97,0-1.76,0.26-2.37,0.78c-0.61,0.52-0.98,1.29-1.1,2.31h2.07c0-0.38,0.11-0.69,0.33-0.95\n\tc0.22-0.26,0.53-0.38,0.93-0.38c0.3,0,0.52,0.08,0.67,0.24c0.15,0.16,0.22,0.34,0.22,0.55c0,0.32-0.11,0.58-0.33,0.76\n\tc-0.22,0.18-0.63,0.42-1.25,0.72c-0.04,0.01-0.07,0.02-0.08,0.04c-0.89,0.43-1.44,0.7-1.65,0.83c-0.79,0.47-1.34,1.06-1.65,1.74\n\tC19.68,21.03,19.57,21.44,19.51,21.9z\" />",
};
#[cfg(feature = "WiWindBeaufort2")]
const WI_WIND_BEAUFORT2: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.94,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\ts0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.53\n\tc-0.16,0-0.3,0.06-0.42,0.17C4.99,13.21,4.94,13.34,4.94,13.5z M4.94,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16C19,9.66,19.15,9.6,19.34,9.6c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.53c-0.16,0-0.3,0.06-0.42,0.17\n\tC4.99,11.18,4.94,11.32,4.94,11.48z M17.66,21.85h6.62l0.4-1.89H21v-0.03c0.2-0.09,0.49-0.22,0.86-0.37\n\tc0.38-0.15,0.69-0.28,0.95-0.38s0.55-0.25,0.87-0.44s0.57-0.38,0.77-0.57c0.19-0.19,0.36-0.44,0.5-0.75s0.21-0.64,0.21-1\n\tc0-0.56-0.14-1.02-0.43-1.4s-0.65-0.65-1.08-0.81c-0.43-0.16-0.92-0.24-1.45-0.24c-0.97,0-1.76,0.26-2.38,0.78\n\tc-0.62,0.52-0.98,1.29-1.1,2.31h2.09c0-0.37,0.11-0.68,0.32-0.94c0.22-0.26,0.52-0.38,0.91-0.38c0.3,0,0.52,0.08,0.67,0.24\n\ts0.23,0.34,0.23,0.54c0,0.12-0.01,0.23-0.03,0.32s-0.07,0.19-0.15,0.28s-0.15,0.16-0.21,0.22s-0.17,0.13-0.34,0.23\n\tc-0.17,0.09-0.3,0.17-0.4,0.22c-0.1,0.05-0.27,0.13-0.53,0.25c-0.88,0.43-1.43,0.71-1.64,0.83c-0.8,0.48-1.35,1.07-1.66,1.78\n\tC17.82,21.01,17.71,21.41,17.66,21.85z\" />",
};
#[cfg(feature = "WiWindBeaufort3")]
const WI_WIND_BEAUFORT3: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.03,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\ts0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.62\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.09,13.21,5.03,13.34,5.03,13.5z M5.03,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51C18.06,9.09,18,9.23,18,9.38c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.62c-0.16,0-0.3,0.06-0.42,0.17\n\tC5.09,11.18,5.03,11.32,5.03,11.48z M18.12,19.52c0,0.27,0.05,0.53,0.16,0.79c0.11,0.26,0.27,0.5,0.5,0.75\n\tc0.23,0.24,0.55,0.43,0.96,0.58s0.9,0.22,1.46,0.22c1.21,0,2.08-0.24,2.63-0.72c0.55-0.48,0.82-1.13,0.82-1.95\n\tc0-0.36-0.1-0.69-0.3-0.99c-0.2-0.3-0.47-0.47-0.79-0.51v-0.02c0.43-0.08,0.79-0.27,1.07-0.58c0.28-0.31,0.43-0.69,0.43-1.12\n\tc0-0.31-0.06-0.58-0.17-0.82c-0.11-0.24-0.26-0.43-0.44-0.58c-0.18-0.15-0.39-0.27-0.64-0.37c-0.25-0.1-0.5-0.16-0.75-0.2\n\tc-0.25-0.04-0.52-0.06-0.8-0.06c-0.92,0-1.68,0.22-2.28,0.67c-0.59,0.45-0.96,1.12-1.1,2.01h2.03c0.04-0.31,0.17-0.55,0.38-0.72\n\tc0.21-0.17,0.47-0.26,0.78-0.26c0.29,0,0.51,0.06,0.68,0.18S23,16.11,23,16.32c0,0.47-0.42,0.7-1.27,0.7h-0.47l-0.29,1.4h0.44\n\tc0.68,0,1.02,0.23,1.02,0.7c0,0.31-0.11,0.55-0.34,0.72c-0.23,0.17-0.5,0.25-0.83,0.25c-0.38,0-0.66-0.11-0.83-0.34\n\tc-0.17-0.21-0.24-0.51-0.21-0.89h-2.07C18.13,19.06,18.12,19.27,18.12,19.52z\" />",
};
#[cfg(feature = "WiWindBeaufort4")]
const WI_WIND_BEAUFORT4: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.98,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\ts0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.57\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.04,13.21,4.98,13.34,4.98,13.5z M4.98,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.57c-0.16,0-0.3,0.06-0.42,0.17\n\tC5.04,11.18,4.98,11.32,4.98,11.48z M17.98,20.35h3.57l-0.32,1.55h2.2l0.36-1.55h1.01l0.36-1.9h-1l0.9-4.34h-2.22l-4.43,4.16\n\tL17.98,20.35z M20.23,18.45l2.24-2.21h0.03l-0.49,2.21H20.23z\" />",
};
#[cfg(feature = "WiWindBeaufort5")]
const WI_WIND_BEAUFORT5: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.97,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.13-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.28,0.53\n\tc0.49,0,0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.56\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.03,13.21,4.97,13.34,4.97,13.5z M4.97,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51C18,9.09,17.94,9.23,17.94,9.38c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.56c-0.16,0-0.3,0.06-0.42,0.17\n\tC5.03,11.18,4.97,11.32,4.97,11.48z M18.04,19.38c-0.02,0.32,0.01,0.62,0.12,0.91c0.1,0.29,0.27,0.56,0.5,0.81\n\tc0.23,0.25,0.55,0.44,0.98,0.59c0.42,0.15,0.92,0.22,1.49,0.22c0.58,0,1.09-0.08,1.53-0.23s0.8-0.34,1.05-0.57\n\tc0.25-0.22,0.45-0.49,0.61-0.79c0.16-0.3,0.27-0.57,0.32-0.82c0.05-0.25,0.08-0.49,0.08-0.74c0-0.67-0.21-1.21-0.64-1.61\n\ts-0.98-0.61-1.65-0.61c-0.69,0-1.18,0.14-1.45,0.43h-0.02l0.35-1.02h3.45l0.39-1.88h-5.24l-1.45,4.46h2\n\tc0.16-0.34,0.53-0.51,1.11-0.51c0.32,0,0.58,0.08,0.77,0.25c0.19,0.17,0.29,0.41,0.29,0.75c0,0.34-0.12,0.61-0.35,0.82\n\tc-0.23,0.21-0.57,0.31-1,0.31c-0.31,0-0.56-0.06-0.73-0.17c-0.21-0.11-0.33-0.31-0.36-0.6H18.04z\" />",
};
#[cfg(feature = "WiWindBeaufort6")]
const WI_WIND_BEAUFORT6: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.92,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.13-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.28,0.53\n\tc0.49,0,0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.51\n\tc-0.16,0-0.3,0.06-0.42,0.17C4.98,13.21,4.92,13.34,4.92,13.5z M4.92,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.51c-0.16,0-0.3,0.06-0.42,0.17\n\tC4.98,11.18,4.92,11.32,4.92,11.48z M18.33,18.72c0,0.96,0.25,1.73,0.75,2.31c0.5,0.58,1.26,0.87,2.29,0.87\n\tc0.95,0,1.73-0.29,2.35-0.87c0.62-0.58,0.92-1.34,0.92-2.28c0-0.64-0.22-1.17-0.67-1.57s-0.99-0.6-1.65-0.6\n\tc-0.73,0-1.3,0.25-1.72,0.75h-0.02c0.33-1.16,0.88-1.74,1.65-1.74c0.25,0,0.44,0.05,0.58,0.14c0.12,0.09,0.2,0.22,0.23,0.41h2.11\n\tc-0.01-0.31-0.08-0.59-0.19-0.84c-0.12-0.25-0.26-0.46-0.44-0.62s-0.39-0.3-0.63-0.4c-0.24-0.11-0.49-0.18-0.75-0.23\n\tc-0.26-0.04-0.52-0.07-0.8-0.07c-0.62,0-1.18,0.12-1.68,0.36s-0.88,0.54-1.17,0.89c-0.28,0.35-0.52,0.75-0.71,1.2\n\tc-0.19,0.45-0.31,0.86-0.38,1.23C18.36,18.04,18.33,18.39,18.33,18.72z M20.38,19.17c0-0.37,0.12-0.65,0.37-0.84\n\tc0.24-0.19,0.52-0.29,0.82-0.29c0.19,0,0.35,0.03,0.48,0.08s0.23,0.12,0.3,0.19c0.07,0.07,0.12,0.16,0.15,0.27\n\tc0.04,0.11,0.06,0.19,0.07,0.25c0.01,0.06,0.01,0.12,0.01,0.19c0,0.31-0.11,0.58-0.32,0.79s-0.5,0.32-0.85,0.32\n\tc-0.31,0-0.55-0.09-0.75-0.27C20.48,19.69,20.38,19.46,20.38,19.17z\" />",
};
#[cfg(feature = "WiWindBeaufort7")]
const WI_WIND_BEAUFORT7: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.83,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\ts0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.42\n\tc-0.16,0-0.3,0.06-0.42,0.17C4.89,13.21,4.83,13.34,4.83,13.5z M4.83,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.42c-0.16,0-0.3,0.06-0.42,0.17\n\tC4.89,11.18,4.83,11.32,4.83,11.48z M18.85,21.9h2.47c0.26-1.29,0.73-2.45,1.39-3.47c0.67-1.02,1.39-1.84,2.16-2.44l0.38-1.87h-5.96\n\tl-0.41,1.89h3.49C20.39,18,19.22,19.96,18.85,21.9z\" />",
};
#[cfg(feature = "WiWindBeaufort8")]
const WI_WIND_BEAUFORT8: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.99,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.13-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.28,0.53\n\tc0.49,0,0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.58\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.05,13.21,4.99,13.34,4.99,13.5z M4.99,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.58c-0.16,0-0.3,0.06-0.42,0.17\n\tC5.05,11.18,4.99,11.32,4.99,11.48z M17.99,19.67c0,0.73,0.29,1.29,0.86,1.66c0.57,0.38,1.34,0.57,2.31,0.57\n\tc0.59,0,1.12-0.06,1.57-0.18c0.46-0.12,0.81-0.27,1.07-0.44s0.46-0.38,0.62-0.62c0.16-0.24,0.26-0.46,0.31-0.66\n\tc0.05-0.2,0.08-0.4,0.08-0.61c0-0.41-0.12-0.77-0.36-1.06c-0.24-0.3-0.55-0.49-0.94-0.57l0.02-0.03v0.01\n\tc0.45-0.06,0.82-0.26,1.12-0.6c0.29-0.33,0.44-0.73,0.44-1.19c0-0.38-0.09-0.71-0.26-0.98s-0.41-0.48-0.71-0.61\n\tc-0.3-0.14-0.61-0.24-0.92-0.3c-0.31-0.06-0.65-0.09-1.01-0.09c-0.48,0-0.9,0.05-1.28,0.14c-0.38,0.09-0.69,0.22-0.93,0.37\n\tc-0.24,0.15-0.43,0.33-0.59,0.53s-0.27,0.4-0.33,0.6c-0.06,0.2-0.09,0.41-0.09,0.62c0,0.34,0.09,0.64,0.27,0.9\n\tc0.18,0.26,0.43,0.43,0.75,0.53v0.03c-0.56,0.06-1.04,0.27-1.42,0.61C18.18,18.67,17.99,19.12,17.99,19.67z M20.1,19.44\n\tc0-0.35,0.14-0.61,0.42-0.77s0.62-0.24,1.01-0.24c0.41,0,0.7,0.09,0.89,0.28c0.18,0.18,0.28,0.38,0.28,0.6v0.13\n\tc0,0.28-0.13,0.49-0.38,0.64c-0.25,0.14-0.58,0.22-0.97,0.22l0.03-0.01c-0.14,0-0.27-0.01-0.4-0.03s-0.27-0.06-0.41-0.11\n\tc-0.14-0.06-0.25-0.14-0.34-0.26C20.15,19.76,20.1,19.61,20.1,19.44z M20.86,16.37c0-0.32,0.12-0.55,0.37-0.69s0.55-0.22,0.9-0.22\n\tc0.3,0,0.55,0.07,0.76,0.2s0.31,0.35,0.31,0.63c0,0.07-0.02,0.15-0.05,0.23c-0.03,0.08-0.09,0.17-0.17,0.27\n\tc-0.08,0.1-0.21,0.18-0.39,0.24c-0.18,0.06-0.4,0.09-0.66,0.09c-0.4,0-0.68-0.08-0.84-0.23C20.94,16.75,20.86,16.57,20.86,16.37z\" />",
};
#[cfg(feature = "WiWindBeaufort9")]
const WI_WIND_BEAUFORT9: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M5.09,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.18,0,0.33,0.06,0.46,0.19\n\tc0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.13-0.11-0.26-0.16-0.4-0.16\n\tc-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53\n\tc0.49,0,0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.68\n\tc-0.16,0-0.3,0.06-0.42,0.17C5.14,13.21,5.09,13.34,5.09,13.5z M5.09,11.48c0,0.17,0.06,0.3,0.17,0.39\n\tc0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26\n\ts-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4\n\tc0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16c0.12-0.12,0.27-0.18,0.45-0.18c0.17,0,0.33,0.06,0.46,0.18\n\tc0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.68c-0.16,0-0.3,0.06-0.42,0.17\n\tC5.14,11.18,5.09,11.32,5.09,11.48z M18.18,19.76c0.02,0.39,0.11,0.73,0.28,1.02c0.17,0.29,0.39,0.51,0.67,0.67\n\tc0.28,0.16,0.58,0.27,0.9,0.34s0.67,0.11,1.04,0.11c0.57,0,1.09-0.11,1.55-0.32c0.47-0.21,0.84-0.48,1.13-0.81\n\tc0.29-0.33,0.53-0.7,0.73-1.13s0.33-0.84,0.41-1.23s0.12-0.78,0.12-1.15c0-1.06-0.27-1.87-0.81-2.43c-0.54-0.57-1.26-0.85-2.17-0.85\n\tc-0.93,0-1.72,0.28-2.36,0.85c-0.64,0.57-0.97,1.32-0.97,2.24c0,0.66,0.21,1.2,0.63,1.62c0.42,0.42,0.96,0.63,1.63,0.63\n\tc0.36,0,0.7-0.07,1.05-0.22c0.34-0.14,0.58-0.33,0.72-0.54h0.03c-0.12,0.48-0.31,0.88-0.58,1.22c-0.27,0.34-0.62,0.51-1.06,0.51\n\tc-0.29,0-0.48-0.03-0.59-0.1c-0.12-0.11-0.21-0.25-0.24-0.42H18.18z M20.75,16.88c0-0.31,0.1-0.58,0.29-0.81\n\tc0.19-0.23,0.48-0.34,0.86-0.34c0.34,0,0.6,0.09,0.77,0.26c0.18,0.17,0.27,0.43,0.27,0.76c0,0.09-0.02,0.2-0.06,0.31\n\ts-0.1,0.23-0.18,0.36c-0.08,0.12-0.2,0.23-0.37,0.31s-0.35,0.12-0.56,0.12s-0.39-0.04-0.54-0.11c-0.15-0.07-0.25-0.17-0.32-0.29\n\tc-0.07-0.12-0.11-0.22-0.14-0.31S20.75,16.96,20.75,16.88z\" />",
};
#[cfg(feature = "WiWindDeg")]
const WI_WIND_DEG: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M3.74,14.5c0-2.04,0.51-3.93,1.52-5.66s2.38-3.1,4.11-4.11s3.61-1.51,5.64-1.51c1.52,0,2.98,0.3,4.37,0.89\n\ts2.58,1.4,3.59,2.4s1.81,2.2,2.4,3.6s0.89,2.85,0.89,4.39c0,1.52-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.59s-2.2,1.8-3.59,2.39\n\ts-2.84,0.89-4.37,0.89c-1.53,0-3-0.3-4.39-0.89s-2.59-1.4-3.6-2.4s-1.8-2.2-2.4-3.58S3.74,16.03,3.74,14.5z M6.22,14.5\n\tc0,2.37,0.86,4.43,2.59,6.18c1.73,1.73,3.79,2.59,6.2,2.59c1.58,0,3.05-0.39,4.39-1.18s2.42-1.85,3.21-3.2s1.19-2.81,1.19-4.39\n\ts-0.4-3.05-1.19-4.4s-1.86-2.42-3.21-3.21s-2.81-1.18-4.39-1.18s-3.05,0.39-4.39,1.18S8.2,8.75,7.4,10.1S6.22,12.92,6.22,14.5z\n\t M11.11,20.35l3.75-13.11c0.01-0.1,0.06-0.15,0.15-0.15s0.14,0.05,0.15,0.15l3.74,13.11c0.04,0.11,0.03,0.19-0.02,0.25\n\ts-0.13,0.06-0.24,0l-3.47-1.3c-0.1-0.04-0.2-0.04-0.29,0l-3.5,1.3c-0.1,0.06-0.17,0.06-0.21,0S11.09,20.45,11.11,20.35z\" />",
};
#[cfg(feature = "WiWindy")]
const WI_WINDY: icondata_core::IconData = icondata_core::IconData {
    style: Some("enable-background:new 0 0 30 30;"),
    x: Some("0px"),
    y: Some("0px"),
    width: None,
    height: None,
    view_box: Some("0 0 30 30"),
    stroke_linecap: None,
    stroke_linejoin: None,
    stroke_width: None,
    stroke: None,
    fill: None,
    data: "<path d=\"M4.65,15.5c0-0.22,0.08-0.41,0.23-0.56c0.16-0.15,0.35-0.22,0.57-0.22h12.08c0.22,0,0.4,0.07,0.54,0.22\n\tc0.14,0.15,0.22,0.34,0.22,0.57c0,0.22-0.07,0.4-0.22,0.54c-0.14,0.14-0.32,0.22-0.54,0.22H5.45c-0.22,0-0.42-0.07-0.57-0.22\n\tC4.72,15.9,4.65,15.72,4.65,15.5z M7.06,12.6c0-0.22,0.08-0.4,0.23-0.55c0.15-0.15,0.34-0.23,0.56-0.23h12.09\n\tc0.21,0,0.39,0.08,0.54,0.23c0.15,0.15,0.22,0.33,0.22,0.55c0,0.22-0.07,0.4-0.22,0.56c-0.15,0.15-0.33,0.23-0.54,0.23H7.86\n\tc-0.22,0-0.41-0.08-0.56-0.23S7.06,12.82,7.06,12.6z M8.68,18.34c0-0.21,0.08-0.39,0.24-0.54c0.14-0.14,0.32-0.22,0.54-0.22h12.1\n\tc0.22,0,0.41,0.07,0.56,0.22c0.15,0.14,0.22,0.32,0.22,0.54s-0.08,0.41-0.23,0.56s-0.34,0.23-0.56,0.23H9.46\n\tc-0.22,0-0.4-0.08-0.56-0.23S8.68,18.56,8.68,18.34z M19.26,15.5c0-0.23,0.07-0.42,0.22-0.57c0.15-0.15,0.34-0.22,0.57-0.22h4.52\n\tc0.23,0,0.42,0.07,0.57,0.22c0.15,0.15,0.22,0.34,0.22,0.56c0,0.22-0.07,0.4-0.22,0.54c-0.15,0.14-0.34,0.22-0.56,0.22h-4.52\n\tc-0.23,0-0.42-0.07-0.57-0.22C19.33,15.9,19.26,15.72,19.26,15.5z\" />",
};
impl From<WiIcon> for icondata_core::IconData {
    fn from(icon: WiIcon) -> icondata_core::IconData {
        match icon {
            #[cfg(feature = "WiAlien")]
            WiIcon::WiAlien => WI_ALIEN,
            #[cfg(feature = "WiBarometer")]
            WiIcon::WiBarometer => WI_BAROMETER,
            #[cfg(feature = "WiCelsius")]
            WiIcon::WiCelsius => WI_CELSIUS,
            #[cfg(feature = "WiCloud")]
            WiIcon::WiCloud => WI_CLOUD,
            #[cfg(feature = "WiCloudDown")]
            WiIcon::WiCloudDown => WI_CLOUD_DOWN,
            #[cfg(feature = "WiCloudRefresh")]
            WiIcon::WiCloudRefresh => WI_CLOUD_REFRESH,
            #[cfg(feature = "WiCloudUp")]
            WiIcon::WiCloudUp => WI_CLOUD_UP,
            #[cfg(feature = "WiCloudy")]
            WiIcon::WiCloudy => WI_CLOUDY,
            #[cfg(feature = "WiCloudyGusts")]
            WiIcon::WiCloudyGusts => WI_CLOUDY_GUSTS,
            #[cfg(feature = "WiCloudyWindy")]
            WiIcon::WiCloudyWindy => WI_CLOUDY_WINDY,
            #[cfg(feature = "WiDayCloudy")]
            WiIcon::WiDayCloudy => WI_DAY_CLOUDY,
            #[cfg(feature = "WiDayCloudyGusts")]
            WiIcon::WiDayCloudyGusts => WI_DAY_CLOUDY_GUSTS,
            #[cfg(feature = "WiDayCloudyHigh")]
            WiIcon::WiDayCloudyHigh => WI_DAY_CLOUDY_HIGH,
            #[cfg(feature = "WiDayCloudyWindy")]
            WiIcon::WiDayCloudyWindy => WI_DAY_CLOUDY_WINDY,
            #[cfg(feature = "WiDayFog")]
            WiIcon::WiDayFog => WI_DAY_FOG,
            #[cfg(feature = "WiDayHail")]
            WiIcon::WiDayHail => WI_DAY_HAIL,
            #[cfg(feature = "WiDayHaze")]
            WiIcon::WiDayHaze => WI_DAY_HAZE,
            #[cfg(feature = "WiDayLightWind")]
            WiIcon::WiDayLightWind => WI_DAY_LIGHT_WIND,
            #[cfg(feature = "WiDayLightning")]
            WiIcon::WiDayLightning => WI_DAY_LIGHTNING,
            #[cfg(feature = "WiDayRain")]
            WiIcon::WiDayRain => WI_DAY_RAIN,
            #[cfg(feature = "WiDayRainMix")]
            WiIcon::WiDayRainMix => WI_DAY_RAIN_MIX,
            #[cfg(feature = "WiDayRainWind")]
            WiIcon::WiDayRainWind => WI_DAY_RAIN_WIND,
            #[cfg(feature = "WiDayShowers")]
            WiIcon::WiDayShowers => WI_DAY_SHOWERS,
            #[cfg(feature = "WiDaySleet")]
            WiIcon::WiDaySleet => WI_DAY_SLEET,
            #[cfg(feature = "WiDaySleetStorm")]
            WiIcon::WiDaySleetStorm => WI_DAY_SLEET_STORM,
            #[cfg(feature = "WiDaySnow")]
            WiIcon::WiDaySnow => WI_DAY_SNOW,
            #[cfg(feature = "WiDaySnowThunderstorm")]
            WiIcon::WiDaySnowThunderstorm => WI_DAY_SNOW_THUNDERSTORM,
            #[cfg(feature = "WiDaySnowWind")]
            WiIcon::WiDaySnowWind => WI_DAY_SNOW_WIND,
            #[cfg(feature = "WiDaySprinkle")]
            WiIcon::WiDaySprinkle => WI_DAY_SPRINKLE,
            #[cfg(feature = "WiDayStormShowers")]
            WiIcon::WiDayStormShowers => WI_DAY_STORM_SHOWERS,
            #[cfg(feature = "WiDaySunny")]
            WiIcon::WiDaySunny => WI_DAY_SUNNY,
            #[cfg(feature = "WiDaySunnyOvercast")]
            WiIcon::WiDaySunnyOvercast => WI_DAY_SUNNY_OVERCAST,
            #[cfg(feature = "WiDayThunderstorm")]
            WiIcon::WiDayThunderstorm => WI_DAY_THUNDERSTORM,
            #[cfg(feature = "WiDayWindy")]
            WiIcon::WiDayWindy => WI_DAY_WINDY,
            #[cfg(feature = "WiDegrees")]
            WiIcon::WiDegrees => WI_DEGREES,
            #[cfg(feature = "WiDirectionDown")]
            WiIcon::WiDirectionDown => WI_DIRECTION_DOWN,
            #[cfg(feature = "WiDirectionDownLeft")]
            WiIcon::WiDirectionDownLeft => WI_DIRECTION_DOWN_LEFT,
            #[cfg(feature = "WiDirectionDownRight")]
            WiIcon::WiDirectionDownRight => WI_DIRECTION_DOWN_RIGHT,
            #[cfg(feature = "WiDirectionLeft")]
            WiIcon::WiDirectionLeft => WI_DIRECTION_LEFT,
            #[cfg(feature = "WiDirectionRight")]
            WiIcon::WiDirectionRight => WI_DIRECTION_RIGHT,
            #[cfg(feature = "WiDirectionUp")]
            WiIcon::WiDirectionUp => WI_DIRECTION_UP,
            #[cfg(feature = "WiDirectionUpLeft")]
            WiIcon::WiDirectionUpLeft => WI_DIRECTION_UP_LEFT,
            #[cfg(feature = "WiDirectionUpRight")]
            WiIcon::WiDirectionUpRight => WI_DIRECTION_UP_RIGHT,
            #[cfg(feature = "WiDust")]
            WiIcon::WiDust => WI_DUST,
            #[cfg(feature = "WiEarthquake")]
            WiIcon::WiEarthquake => WI_EARTHQUAKE,
            #[cfg(feature = "WiFahrenheit")]
            WiIcon::WiFahrenheit => WI_FAHRENHEIT,
            #[cfg(feature = "WiFire")]
            WiIcon::WiFire => WI_FIRE,
            #[cfg(feature = "WiFlood")]
            WiIcon::WiFlood => WI_FLOOD,
            #[cfg(feature = "WiFog")]
            WiIcon::WiFog => WI_FOG,
            #[cfg(feature = "WiGaleWarning")]
            WiIcon::WiGaleWarning => WI_GALE_WARNING,
            #[cfg(feature = "WiHail")]
            WiIcon::WiHail => WI_HAIL,
            #[cfg(feature = "WiHorizon")]
            WiIcon::WiHorizon => WI_HORIZON,
            #[cfg(feature = "WiHorizonAlt")]
            WiIcon::WiHorizonAlt => WI_HORIZON_ALT,
            #[cfg(feature = "WiHot")]
            WiIcon::WiHot => WI_HOT,
            #[cfg(feature = "WiHumidity")]
            WiIcon::WiHumidity => WI_HUMIDITY,
            #[cfg(feature = "WiHurricane")]
            WiIcon::WiHurricane => WI_HURRICANE,
            #[cfg(feature = "WiHurricaneWarning")]
            WiIcon::WiHurricaneWarning => WI_HURRICANE_WARNING,
            #[cfg(feature = "WiLightning")]
            WiIcon::WiLightning => WI_LIGHTNING,
            #[cfg(feature = "WiLunarEclipse")]
            WiIcon::WiLunarEclipse => WI_LUNAR_ECLIPSE,
            #[cfg(feature = "WiMeteor")]
            WiIcon::WiMeteor => WI_METEOR,
            #[cfg(feature = "WiMoonAltFirstQuarter")]
            WiIcon::WiMoonAltFirstQuarter => WI_MOON_ALT_FIRST_QUARTER,
            #[cfg(feature = "WiMoonAltFull")]
            WiIcon::WiMoonAltFull => WI_MOON_ALT_FULL,
            #[cfg(feature = "WiMoonAltNew")]
            WiIcon::WiMoonAltNew => WI_MOON_ALT_NEW,
            #[cfg(feature = "WiMoonAltThirdQuarter")]
            WiIcon::WiMoonAltThirdQuarter => WI_MOON_ALT_THIRD_QUARTER,
            #[cfg(feature = "WiMoonAltWaningCrescent1")]
            WiIcon::WiMoonAltWaningCrescent1 => WI_MOON_ALT_WANING_CRESCENT1,
            #[cfg(feature = "WiMoonAltWaningCrescent2")]
            WiIcon::WiMoonAltWaningCrescent2 => WI_MOON_ALT_WANING_CRESCENT2,
            #[cfg(feature = "WiMoonAltWaningCrescent3")]
            WiIcon::WiMoonAltWaningCrescent3 => WI_MOON_ALT_WANING_CRESCENT3,
            #[cfg(feature = "WiMoonAltWaningCrescent4")]
            WiIcon::WiMoonAltWaningCrescent4 => WI_MOON_ALT_WANING_CRESCENT4,
            #[cfg(feature = "WiMoonAltWaningCrescent5")]
            WiIcon::WiMoonAltWaningCrescent5 => WI_MOON_ALT_WANING_CRESCENT5,
            #[cfg(feature = "WiMoonAltWaningCrescent6")]
            WiIcon::WiMoonAltWaningCrescent6 => WI_MOON_ALT_WANING_CRESCENT6,
            #[cfg(feature = "WiMoonAltWaningGibbous1")]
            WiIcon::WiMoonAltWaningGibbous1 => WI_MOON_ALT_WANING_GIBBOUS1,
            #[cfg(feature = "WiMoonAltWaningGibbous2")]
            WiIcon::WiMoonAltWaningGibbous2 => WI_MOON_ALT_WANING_GIBBOUS2,
            #[cfg(feature = "WiMoonAltWaningGibbous3")]
            WiIcon::WiMoonAltWaningGibbous3 => WI_MOON_ALT_WANING_GIBBOUS3,
            #[cfg(feature = "WiMoonAltWaningGibbous4")]
            WiIcon::WiMoonAltWaningGibbous4 => WI_MOON_ALT_WANING_GIBBOUS4,
            #[cfg(feature = "WiMoonAltWaningGibbous5")]
            WiIcon::WiMoonAltWaningGibbous5 => WI_MOON_ALT_WANING_GIBBOUS5,
            #[cfg(feature = "WiMoonAltWaningGibbous6")]
            WiIcon::WiMoonAltWaningGibbous6 => WI_MOON_ALT_WANING_GIBBOUS6,
            #[cfg(feature = "WiMoonAltWaxingCrescent1")]
            WiIcon::WiMoonAltWaxingCrescent1 => WI_MOON_ALT_WAXING_CRESCENT1,
            #[cfg(feature = "WiMoonAltWaxingCrescent2")]
            WiIcon::WiMoonAltWaxingCrescent2 => WI_MOON_ALT_WAXING_CRESCENT2,
            #[cfg(feature = "WiMoonAltWaxingCrescent3")]
            WiIcon::WiMoonAltWaxingCrescent3 => WI_MOON_ALT_WAXING_CRESCENT3,
            #[cfg(feature = "WiMoonAltWaxingCrescent4")]
            WiIcon::WiMoonAltWaxingCrescent4 => WI_MOON_ALT_WAXING_CRESCENT4,
            #[cfg(feature = "WiMoonAltWaxingCrescent5")]
            WiIcon::WiMoonAltWaxingCrescent5 => WI_MOON_ALT_WAXING_CRESCENT5,
            #[cfg(feature = "WiMoonAltWaxingCrescent6")]
            WiIcon::WiMoonAltWaxingCrescent6 => WI_MOON_ALT_WAXING_CRESCENT6,
            #[cfg(feature = "WiMoonAltWaxingGibbous1")]
            WiIcon::WiMoonAltWaxingGibbous1 => WI_MOON_ALT_WAXING_GIBBOUS1,
            #[cfg(feature = "WiMoonAltWaxingGibbous2")]
            WiIcon::WiMoonAltWaxingGibbous2 => WI_MOON_ALT_WAXING_GIBBOUS2,
            #[cfg(feature = "WiMoonAltWaxingGibbous3")]
            WiIcon::WiMoonAltWaxingGibbous3 => WI_MOON_ALT_WAXING_GIBBOUS3,
            #[cfg(feature = "WiMoonAltWaxingGibbous4")]
            WiIcon::WiMoonAltWaxingGibbous4 => WI_MOON_ALT_WAXING_GIBBOUS4,
            #[cfg(feature = "WiMoonAltWaxingGibbous5")]
            WiIcon::WiMoonAltWaxingGibbous5 => WI_MOON_ALT_WAXING_GIBBOUS5,
            #[cfg(feature = "WiMoonAltWaxingGibbous6")]
            WiIcon::WiMoonAltWaxingGibbous6 => WI_MOON_ALT_WAXING_GIBBOUS6,
            #[cfg(feature = "WiMoonFirstQuarter")]
            WiIcon::WiMoonFirstQuarter => WI_MOON_FIRST_QUARTER,
            #[cfg(feature = "WiMoonFull")]
            WiIcon::WiMoonFull => WI_MOON_FULL,
            #[cfg(feature = "WiMoonNew")]
            WiIcon::WiMoonNew => WI_MOON_NEW,
            #[cfg(feature = "WiMoonThirdQuarter")]
            WiIcon::WiMoonThirdQuarter => WI_MOON_THIRD_QUARTER,
            #[cfg(feature = "WiMoonWaningCrescent1")]
            WiIcon::WiMoonWaningCrescent1 => WI_MOON_WANING_CRESCENT1,
            #[cfg(feature = "WiMoonWaningCrescent2")]
            WiIcon::WiMoonWaningCrescent2 => WI_MOON_WANING_CRESCENT2,
            #[cfg(feature = "WiMoonWaningCrescent3")]
            WiIcon::WiMoonWaningCrescent3 => WI_MOON_WANING_CRESCENT3,
            #[cfg(feature = "WiMoonWaningCrescent4")]
            WiIcon::WiMoonWaningCrescent4 => WI_MOON_WANING_CRESCENT4,
            #[cfg(feature = "WiMoonWaningCrescent5")]
            WiIcon::WiMoonWaningCrescent5 => WI_MOON_WANING_CRESCENT5,
            #[cfg(feature = "WiMoonWaningCrescent6")]
            WiIcon::WiMoonWaningCrescent6 => WI_MOON_WANING_CRESCENT6,
            #[cfg(feature = "WiMoonWaningGibbous1")]
            WiIcon::WiMoonWaningGibbous1 => WI_MOON_WANING_GIBBOUS1,
            #[cfg(feature = "WiMoonWaningGibbous2")]
            WiIcon::WiMoonWaningGibbous2 => WI_MOON_WANING_GIBBOUS2,
            #[cfg(feature = "WiMoonWaningGibbous3")]
            WiIcon::WiMoonWaningGibbous3 => WI_MOON_WANING_GIBBOUS3,
            #[cfg(feature = "WiMoonWaningGibbous4")]
            WiIcon::WiMoonWaningGibbous4 => WI_MOON_WANING_GIBBOUS4,
            #[cfg(feature = "WiMoonWaningGibbous5")]
            WiIcon::WiMoonWaningGibbous5 => WI_MOON_WANING_GIBBOUS5,
            #[cfg(feature = "WiMoonWaningGibbous6")]
            WiIcon::WiMoonWaningGibbous6 => WI_MOON_WANING_GIBBOUS6,
            #[cfg(feature = "WiMoonWaxingCrescent1")]
            WiIcon::WiMoonWaxingCrescent1 => WI_MOON_WAXING_CRESCENT1,
            #[cfg(feature = "WiMoonWaxingCrescent2")]
            WiIcon::WiMoonWaxingCrescent2 => WI_MOON_WAXING_CRESCENT2,
            #[cfg(feature = "WiMoonWaxingCrescent3")]
            WiIcon::WiMoonWaxingCrescent3 => WI_MOON_WAXING_CRESCENT3,
            #[cfg(feature = "WiMoonWaxingCrescent4")]
            WiIcon::WiMoonWaxingCrescent4 => WI_MOON_WAXING_CRESCENT4,
            #[cfg(feature = "WiMoonWaxingCrescent5")]
            WiIcon::WiMoonWaxingCrescent5 => WI_MOON_WAXING_CRESCENT5,
            #[cfg(feature = "WiMoonWaxingCrescent6")]
            WiIcon::WiMoonWaxingCrescent6 => WI_MOON_WAXING_CRESCENT6,
            #[cfg(feature = "WiMoonWaxingGibbous1")]
            WiIcon::WiMoonWaxingGibbous1 => WI_MOON_WAXING_GIBBOUS1,
            #[cfg(feature = "WiMoonWaxingGibbous2")]
            WiIcon::WiMoonWaxingGibbous2 => WI_MOON_WAXING_GIBBOUS2,
            #[cfg(feature = "WiMoonWaxingGibbous3")]
            WiIcon::WiMoonWaxingGibbous3 => WI_MOON_WAXING_GIBBOUS3,
            #[cfg(feature = "WiMoonWaxingGibbous4")]
            WiIcon::WiMoonWaxingGibbous4 => WI_MOON_WAXING_GIBBOUS4,
            #[cfg(feature = "WiMoonWaxingGibbous5")]
            WiIcon::WiMoonWaxingGibbous5 => WI_MOON_WAXING_GIBBOUS5,
            #[cfg(feature = "WiMoonWaxingGibbous6")]
            WiIcon::WiMoonWaxingGibbous6 => WI_MOON_WAXING_GIBBOUS6,
            #[cfg(feature = "WiMoonrise")]
            WiIcon::WiMoonrise => WI_MOONRISE,
            #[cfg(feature = "WiMoonset")]
            WiIcon::WiMoonset => WI_MOONSET,
            #[cfg(feature = "WiNa")]
            WiIcon::WiNa => WI_NA,
            #[cfg(feature = "WiNightAltCloudy")]
            WiIcon::WiNightAltCloudy => WI_NIGHT_ALT_CLOUDY,
            #[cfg(feature = "WiNightAltCloudyGusts")]
            WiIcon::WiNightAltCloudyGusts => WI_NIGHT_ALT_CLOUDY_GUSTS,
            #[cfg(feature = "WiNightAltCloudyHigh")]
            WiIcon::WiNightAltCloudyHigh => WI_NIGHT_ALT_CLOUDY_HIGH,
            #[cfg(feature = "WiNightAltCloudyWindy")]
            WiIcon::WiNightAltCloudyWindy => WI_NIGHT_ALT_CLOUDY_WINDY,
            #[cfg(feature = "WiNightAltHail")]
            WiIcon::WiNightAltHail => WI_NIGHT_ALT_HAIL,
            #[cfg(feature = "WiNightAltLightning")]
            WiIcon::WiNightAltLightning => WI_NIGHT_ALT_LIGHTNING,
            #[cfg(feature = "WiNightAltPartlyCloudy")]
            WiIcon::WiNightAltPartlyCloudy => WI_NIGHT_ALT_PARTLY_CLOUDY,
            #[cfg(feature = "WiNightAltRain")]
            WiIcon::WiNightAltRain => WI_NIGHT_ALT_RAIN,
            #[cfg(feature = "WiNightAltRainMix")]
            WiIcon::WiNightAltRainMix => WI_NIGHT_ALT_RAIN_MIX,
            #[cfg(feature = "WiNightAltRainWind")]
            WiIcon::WiNightAltRainWind => WI_NIGHT_ALT_RAIN_WIND,
            #[cfg(feature = "WiNightAltShowers")]
            WiIcon::WiNightAltShowers => WI_NIGHT_ALT_SHOWERS,
            #[cfg(feature = "WiNightAltSleet")]
            WiIcon::WiNightAltSleet => WI_NIGHT_ALT_SLEET,
            #[cfg(feature = "WiNightAltSleetStorm")]
            WiIcon::WiNightAltSleetStorm => WI_NIGHT_ALT_SLEET_STORM,
            #[cfg(feature = "WiNightAltSnow")]
            WiIcon::WiNightAltSnow => WI_NIGHT_ALT_SNOW,
            #[cfg(feature = "WiNightAltSnowThunderstorm")]
            WiIcon::WiNightAltSnowThunderstorm => WI_NIGHT_ALT_SNOW_THUNDERSTORM,
            #[cfg(feature = "WiNightAltSnowWind")]
            WiIcon::WiNightAltSnowWind => WI_NIGHT_ALT_SNOW_WIND,
            #[cfg(feature = "WiNightAltSprinkle")]
            WiIcon::WiNightAltSprinkle => WI_NIGHT_ALT_SPRINKLE,
            #[cfg(feature = "WiNightAltStormShowers")]
            WiIcon::WiNightAltStormShowers => WI_NIGHT_ALT_STORM_SHOWERS,
            #[cfg(feature = "WiNightAltThunderstorm")]
            WiIcon::WiNightAltThunderstorm => WI_NIGHT_ALT_THUNDERSTORM,
            #[cfg(feature = "WiNightClear")]
            WiIcon::WiNightClear => WI_NIGHT_CLEAR,
            #[cfg(feature = "WiNightCloudy")]
            WiIcon::WiNightCloudy => WI_NIGHT_CLOUDY,
            #[cfg(feature = "WiNightCloudyGusts")]
            WiIcon::WiNightCloudyGusts => WI_NIGHT_CLOUDY_GUSTS,
            #[cfg(feature = "WiNightCloudyHigh")]
            WiIcon::WiNightCloudyHigh => WI_NIGHT_CLOUDY_HIGH,
            #[cfg(feature = "WiNightCloudyWindy")]
            WiIcon::WiNightCloudyWindy => WI_NIGHT_CLOUDY_WINDY,
            #[cfg(feature = "WiNightFog")]
            WiIcon::WiNightFog => WI_NIGHT_FOG,
            #[cfg(feature = "WiNightHail")]
            WiIcon::WiNightHail => WI_NIGHT_HAIL,
            #[cfg(feature = "WiNightLightning")]
            WiIcon::WiNightLightning => WI_NIGHT_LIGHTNING,
            #[cfg(feature = "WiNightPartlyCloudy")]
            WiIcon::WiNightPartlyCloudy => WI_NIGHT_PARTLY_CLOUDY,
            #[cfg(feature = "WiNightRain")]
            WiIcon::WiNightRain => WI_NIGHT_RAIN,
            #[cfg(feature = "WiNightRainMix")]
            WiIcon::WiNightRainMix => WI_NIGHT_RAIN_MIX,
            #[cfg(feature = "WiNightRainWind")]
            WiIcon::WiNightRainWind => WI_NIGHT_RAIN_WIND,
            #[cfg(feature = "WiNightShowers")]
            WiIcon::WiNightShowers => WI_NIGHT_SHOWERS,
            #[cfg(feature = "WiNightSleet")]
            WiIcon::WiNightSleet => WI_NIGHT_SLEET,
            #[cfg(feature = "WiNightSleetStorm")]
            WiIcon::WiNightSleetStorm => WI_NIGHT_SLEET_STORM,
            #[cfg(feature = "WiNightSnow")]
            WiIcon::WiNightSnow => WI_NIGHT_SNOW,
            #[cfg(feature = "WiNightSnowThunderstorm")]
            WiIcon::WiNightSnowThunderstorm => WI_NIGHT_SNOW_THUNDERSTORM,
            #[cfg(feature = "WiNightSnowWind")]
            WiIcon::WiNightSnowWind => WI_NIGHT_SNOW_WIND,
            #[cfg(feature = "WiNightSprinkle")]
            WiIcon::WiNightSprinkle => WI_NIGHT_SPRINKLE,
            #[cfg(feature = "WiNightStormShowers")]
            WiIcon::WiNightStormShowers => WI_NIGHT_STORM_SHOWERS,
            #[cfg(feature = "WiNightThunderstorm")]
            WiIcon::WiNightThunderstorm => WI_NIGHT_THUNDERSTORM,
            #[cfg(feature = "WiRain")]
            WiIcon::WiRain => WI_RAIN,
            #[cfg(feature = "WiRainMix")]
            WiIcon::WiRainMix => WI_RAIN_MIX,
            #[cfg(feature = "WiRainWind")]
            WiIcon::WiRainWind => WI_RAIN_WIND,
            #[cfg(feature = "WiRaindrop")]
            WiIcon::WiRaindrop => WI_RAINDROP,
            #[cfg(feature = "WiRaindrops")]
            WiIcon::WiRaindrops => WI_RAINDROPS,
            #[cfg(feature = "WiRefresh")]
            WiIcon::WiRefresh => WI_REFRESH,
            #[cfg(feature = "WiRefreshAlt")]
            WiIcon::WiRefreshAlt => WI_REFRESH_ALT,
            #[cfg(feature = "WiSandstorm")]
            WiIcon::WiSandstorm => WI_SANDSTORM,
            #[cfg(feature = "WiShowers")]
            WiIcon::WiShowers => WI_SHOWERS,
            #[cfg(feature = "WiSleet")]
            WiIcon::WiSleet => WI_SLEET,
            #[cfg(feature = "WiSmallCraftAdvisory")]
            WiIcon::WiSmallCraftAdvisory => WI_SMALL_CRAFT_ADVISORY,
            #[cfg(feature = "WiSmog")]
            WiIcon::WiSmog => WI_SMOG,
            #[cfg(feature = "WiSmoke")]
            WiIcon::WiSmoke => WI_SMOKE,
            #[cfg(feature = "WiSnow")]
            WiIcon::WiSnow => WI_SNOW,
            #[cfg(feature = "WiSnowWind")]
            WiIcon::WiSnowWind => WI_SNOW_WIND,
            #[cfg(feature = "WiSnowflakeCold")]
            WiIcon::WiSnowflakeCold => WI_SNOWFLAKE_COLD,
            #[cfg(feature = "WiSolarEclipse")]
            WiIcon::WiSolarEclipse => WI_SOLAR_ECLIPSE,
            #[cfg(feature = "WiSprinkle")]
            WiIcon::WiSprinkle => WI_SPRINKLE,
            #[cfg(feature = "WiStars")]
            WiIcon::WiStars => WI_STARS,
            #[cfg(feature = "WiStormShowers")]
            WiIcon::WiStormShowers => WI_STORM_SHOWERS,
            #[cfg(feature = "WiStormWarning")]
            WiIcon::WiStormWarning => WI_STORM_WARNING,
            #[cfg(feature = "WiStrongWind")]
            WiIcon::WiStrongWind => WI_STRONG_WIND,
            #[cfg(feature = "WiSunrise")]
            WiIcon::WiSunrise => WI_SUNRISE,
            #[cfg(feature = "WiSunset")]
            WiIcon::WiSunset => WI_SUNSET,
            #[cfg(feature = "WiThermometer")]
            WiIcon::WiThermometer => WI_THERMOMETER,
            #[cfg(feature = "WiThermometerExterior")]
            WiIcon::WiThermometerExterior => WI_THERMOMETER_EXTERIOR,
            #[cfg(feature = "WiThermometerInternal")]
            WiIcon::WiThermometerInternal => WI_THERMOMETER_INTERNAL,
            #[cfg(feature = "WiThunderstorm")]
            WiIcon::WiThunderstorm => WI_THUNDERSTORM,
            #[cfg(feature = "WiTime1")]
            WiIcon::WiTime1 => WI_TIME1,
            #[cfg(feature = "WiTime10")]
            WiIcon::WiTime10 => WI_TIME10,
            #[cfg(feature = "WiTime11")]
            WiIcon::WiTime11 => WI_TIME11,
            #[cfg(feature = "WiTime12")]
            WiIcon::WiTime12 => WI_TIME12,
            #[cfg(feature = "WiTime2")]
            WiIcon::WiTime2 => WI_TIME2,
            #[cfg(feature = "WiTime3")]
            WiIcon::WiTime3 => WI_TIME3,
            #[cfg(feature = "WiTime4")]
            WiIcon::WiTime4 => WI_TIME4,
            #[cfg(feature = "WiTime5")]
            WiIcon::WiTime5 => WI_TIME5,
            #[cfg(feature = "WiTime6")]
            WiIcon::WiTime6 => WI_TIME6,
            #[cfg(feature = "WiTime7")]
            WiIcon::WiTime7 => WI_TIME7,
            #[cfg(feature = "WiTime8")]
            WiIcon::WiTime8 => WI_TIME8,
            #[cfg(feature = "WiTime9")]
            WiIcon::WiTime9 => WI_TIME9,
            #[cfg(feature = "WiTornado")]
            WiIcon::WiTornado => WI_TORNADO,
            #[cfg(feature = "WiTrain")]
            WiIcon::WiTrain => WI_TRAIN,
            #[cfg(feature = "WiTsunami")]
            WiIcon::WiTsunami => WI_TSUNAMI,
            #[cfg(feature = "WiUmbrella")]
            WiIcon::WiUmbrella => WI_UMBRELLA,
            #[cfg(feature = "WiVolcano")]
            WiIcon::WiVolcano => WI_VOLCANO,
            #[cfg(feature = "WiWindBeaufort0")]
            WiIcon::WiWindBeaufort0 => WI_WIND_BEAUFORT0,
            #[cfg(feature = "WiWindBeaufort1")]
            WiIcon::WiWindBeaufort1 => WI_WIND_BEAUFORT1,
            #[cfg(feature = "WiWindBeaufort10")]
            WiIcon::WiWindBeaufort10 => WI_WIND_BEAUFORT10,
            #[cfg(feature = "WiWindBeaufort11")]
            WiIcon::WiWindBeaufort11 => WI_WIND_BEAUFORT11,
            #[cfg(feature = "WiWindBeaufort12")]
            WiIcon::WiWindBeaufort12 => WI_WIND_BEAUFORT12,
            #[cfg(feature = "WiWindBeaufort2")]
            WiIcon::WiWindBeaufort2 => WI_WIND_BEAUFORT2,
            #[cfg(feature = "WiWindBeaufort3")]
            WiIcon::WiWindBeaufort3 => WI_WIND_BEAUFORT3,
            #[cfg(feature = "WiWindBeaufort4")]
            WiIcon::WiWindBeaufort4 => WI_WIND_BEAUFORT4,
            #[cfg(feature = "WiWindBeaufort5")]
            WiIcon::WiWindBeaufort5 => WI_WIND_BEAUFORT5,
            #[cfg(feature = "WiWindBeaufort6")]
            WiIcon::WiWindBeaufort6 => WI_WIND_BEAUFORT6,
            #[cfg(feature = "WiWindBeaufort7")]
            WiIcon::WiWindBeaufort7 => WI_WIND_BEAUFORT7,
            #[cfg(feature = "WiWindBeaufort8")]
            WiIcon::WiWindBeaufort8 => WI_WIND_BEAUFORT8,
            #[cfg(feature = "WiWindBeaufort9")]
            WiIcon::WiWindBeaufort9 => WI_WIND_BEAUFORT9,
            #[cfg(feature = "WiWindDeg")]
            WiIcon::WiWindDeg => WI_WIND_DEG,
            #[cfg(feature = "WiWindy")]
            WiIcon::WiWindy => WI_WINDY,
        }
    }
}
