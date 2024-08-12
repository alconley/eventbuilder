use super::channel_map::{ChannelMap, ChannelType};
#[allow(unused_imports)]
use super::compass_data::{decompose_uuid_to_board_channel, CompassData};
use super::used_size::UsedSize;
use std::collections::BTreeMap;
use std::hash::Hash;

use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumCount, EnumIter};

use polars::prelude::*;

const INVALID_VALUE: f64 = -1.0e6;

#[derive(Debug, Clone, Hash, Eq, PartialOrd, Ord, PartialEq, EnumIter, EnumCount, AsRefStr)]
pub enum ChannelDataField {
    AnodeFrontEnergy,
    AnodeFrontShort,
    AnodeFrontTime,
    AnodeBackEnergy,
    AnodeBackShort,
    AnodeBackTime,
    ScintLeftEnergy,
    ScintLeftShort,
    ScintLeftTime,
    ScintRightEnergy,
    ScintRightShort,
    ScintRightTime,
    CathodeEnergy,
    CathodeShort,
    CathodeTime,
    DelayFrontLeftEnergy,
    DelayFrontLeftShort,
    DelayFrontLeftTime,
    DelayFrontRightEnergy,
    DelayFrontRightShort,
    DelayFrontRightTime,
    DelayBackLeftEnergy,
    DelayBackLeftShort,
    DelayBackLeftTime,
    DelayBackRightEnergy,
    DelayBackRightShort,
    DelayBackRightTime,
    MonitorEnergy,
    MonitorShort,
    MonitorTime,
    X1,
    X2,
    Xavg,
    Theta,

    Cebra0Energy,
    Cebra1Energy,
    Cebra2Energy,
    Cebra3Energy,
    Cebra4Energy,
    Cebra5Energy,
    Cebra6Energy,
    Cebra7Energy,
    Cebra8Energy,

    Cebra0Short,
    Cebra1Short,
    Cebra2Short,
    Cebra3Short,
    Cebra4Short,
    Cebra5Short,
    Cebra6Short,
    Cebra7Short,
    Cebra8Short,

    Cebra0Time,
    Cebra1Time,
    Cebra2Time,
    Cebra3Time,
    Cebra4Time,
    Cebra5Time,
    Cebra6Time,
    Cebra7Time,
    Cebra8Time,

    Cebra0RelTime,
    Cebra1RelTime,
    Cebra2RelTime,
    Cebra3RelTime,
    Cebra4RelTime,
    Cebra5RelTime,
    Cebra6RelTime,
    Cebra7RelTime,
    Cebra8RelTime,
}

impl ChannelDataField {
    //Returns a list of fields for iterating over
    pub fn get_field_vec() -> Vec<ChannelDataField> {
        ChannelDataField::iter().collect()
    }

    pub fn get_filtered_field_vec(channel_map: &ChannelMap) -> Vec<ChannelDataField> {
        let all_delay_lines_present = channel_map
            .contains_channel_type(ChannelType::DelayFrontLeft)
            && channel_map.contains_channel_type(ChannelType::DelayFrontRight)
            && channel_map.contains_channel_type(ChannelType::DelayBackLeft)
            && channel_map.contains_channel_type(ChannelType::DelayBackRight);
        ChannelDataField::iter()
            .filter(|field| {
                match field {
                    // Include additional fields only if all delay line channels are present
                    ChannelDataField::X1
                    | ChannelDataField::X2
                    | ChannelDataField::Xavg
                    | ChannelDataField::Theta => all_delay_lines_present,
                    // Filter other fields based on the channel map
                    ChannelDataField::AnodeFrontEnergy
                    | ChannelDataField::AnodeFrontShort
                    | ChannelDataField::AnodeFrontTime => {
                        channel_map.contains_channel_type(ChannelType::AnodeFront)
                    }
                    ChannelDataField::AnodeBackEnergy
                    | ChannelDataField::AnodeBackShort
                    | ChannelDataField::AnodeBackTime => {
                        channel_map.contains_channel_type(ChannelType::AnodeBack)
                    }
                    ChannelDataField::ScintLeftEnergy
                    | ChannelDataField::ScintLeftShort
                    | ChannelDataField::ScintLeftTime => {
                        channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                    ChannelDataField::ScintRightEnergy
                    | ChannelDataField::ScintRightShort
                    | ChannelDataField::ScintRightTime => {
                        channel_map.contains_channel_type(ChannelType::ScintRight)
                    }
                    ChannelDataField::CathodeEnergy
                    | ChannelDataField::CathodeShort
                    | ChannelDataField::CathodeTime => {
                        channel_map.contains_channel_type(ChannelType::Cathode)
                    }
                    ChannelDataField::DelayFrontLeftEnergy
                    | ChannelDataField::DelayFrontLeftShort
                    | ChannelDataField::DelayFrontLeftTime => {
                        channel_map.contains_channel_type(ChannelType::DelayFrontLeft)
                    }
                    ChannelDataField::DelayFrontRightEnergy
                    | ChannelDataField::DelayFrontRightShort
                    | ChannelDataField::DelayFrontRightTime => {
                        channel_map.contains_channel_type(ChannelType::DelayFrontRight)
                    }
                    ChannelDataField::DelayBackLeftEnergy
                    | ChannelDataField::DelayBackLeftShort
                    | ChannelDataField::DelayBackLeftTime => {
                        channel_map.contains_channel_type(ChannelType::DelayBackLeft)
                    }
                    ChannelDataField::DelayBackRightEnergy
                    | ChannelDataField::DelayBackRightShort
                    | ChannelDataField::DelayBackRightTime => {
                        channel_map.contains_channel_type(ChannelType::DelayBackRight)
                    }
                    ChannelDataField::MonitorEnergy
                    | ChannelDataField::MonitorShort
                    | ChannelDataField::MonitorTime => {
                        channel_map.contains_channel_type(ChannelType::Monitor)
                    }

                    ChannelDataField::Cebra0Energy
                    | ChannelDataField::Cebra0Short
                    | ChannelDataField::Cebra0Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra0)
                    }
                    ChannelDataField::Cebra0RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra0)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }

                    ChannelDataField::Cebra1Energy
                    | ChannelDataField::Cebra1Short
                    | ChannelDataField::Cebra1Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra1)
                    }
                    ChannelDataField::Cebra1RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra1)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }

                    ChannelDataField::Cebra2Energy
                    | ChannelDataField::Cebra2Short
                    | ChannelDataField::Cebra2Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra2)
                    }
                    ChannelDataField::Cebra2RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra2)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }

                    ChannelDataField::Cebra3Energy
                    | ChannelDataField::Cebra3Short
                    | ChannelDataField::Cebra3Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra3)
                    }
                    ChannelDataField::Cebra3RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra3)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                    ChannelDataField::Cebra4Energy
                    | ChannelDataField::Cebra4Short
                    | ChannelDataField::Cebra4Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra4)
                    }
                    ChannelDataField::Cebra4RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra4)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                    ChannelDataField::Cebra5Energy
                    | ChannelDataField::Cebra5Short
                    | ChannelDataField::Cebra5Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra5)
                    }
                    ChannelDataField::Cebra5RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra5)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                    ChannelDataField::Cebra6Energy
                    | ChannelDataField::Cebra6Short
                    | ChannelDataField::Cebra6Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra6)
                    }
                    ChannelDataField::Cebra6RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra6)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                    ChannelDataField::Cebra7Energy
                    | ChannelDataField::Cebra7Short
                    | ChannelDataField::Cebra7Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra7)
                    }
                    ChannelDataField::Cebra7RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra7)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                    ChannelDataField::Cebra8Energy
                    | ChannelDataField::Cebra8Short
                    | ChannelDataField::Cebra8Time => {
                        channel_map.contains_channel_type(ChannelType::Cebra8)
                    }
                    ChannelDataField::Cebra8RelTime => {
                        channel_map.contains_channel_type(ChannelType::Cebra8)
                            && channel_map.contains_channel_type(ChannelType::ScintLeft)
                    }
                }
            })
            .collect()
    }
}

impl UsedSize for ChannelDataField {
    fn get_used_size(&self) -> usize {
        std::mem::size_of::<ChannelDataField>()
    }
}

#[derive(Debug, Clone)]
pub struct ChannelData {
    //Columns must always come in same order, so use sorted map
    pub fields: BTreeMap<ChannelDataField, Vec<f64>>,
    pub rows: usize,
}

impl Default for ChannelData {
    fn default() -> Self {
        let fields = ChannelDataField::get_field_vec();
        let mut data = ChannelData {
            fields: BTreeMap::new(),
            rows: 0,
        };
        fields.into_iter().for_each(|f| {
            data.fields.insert(f, vec![]);
        });
        data
    }
}

impl UsedSize for ChannelData {
    fn get_used_size(&self) -> usize {
        self.fields.get_used_size()
    }
}

impl ChannelData {
    // Constructor accepting a channel map to initialize only valid fields
    pub fn new(channel_map: &ChannelMap) -> Self {
        let fields = ChannelDataField::get_filtered_field_vec(channel_map);
        let mut data = ChannelData {
            fields: BTreeMap::new(),
            rows: 0,
        };
        fields.into_iter().for_each(|f| {
            data.fields.insert(f, vec![]);
        });
        data
    }

    //To keep columns all same length, push invalid values as necessary
    fn push_defaults(&mut self) {
        for field in self.fields.iter_mut() {
            if field.1.len() < self.rows {
                field.1.push(INVALID_VALUE)
            }
        }
    }

    //Update the last element to the given value
    fn set_value(&mut self, field: &ChannelDataField, value: f64) {
        if let Some(list) = self.fields.get_mut(field) {
            if let Some(back) = list.last_mut() {
                *back = value;
            }
        }
    }

    pub fn append_event(
        &mut self,
        event: Vec<CompassData>,
        map: &ChannelMap,
        weights: Option<(f64, f64)>,
    ) {
        self.rows += 1;
        self.push_defaults();

        let mut dfl_time = INVALID_VALUE;
        let mut dfr_time = INVALID_VALUE;
        let mut dbl_time = INVALID_VALUE;
        let mut dbr_time = INVALID_VALUE;

        // for cebra relative time
        let mut scint_left_time = INVALID_VALUE;
        let mut cebra0_time = INVALID_VALUE;
        let mut cebra1_time = INVALID_VALUE;
        let mut cebra2_time = INVALID_VALUE;
        let mut cebra3_time = INVALID_VALUE;
        let mut cebra4_time = INVALID_VALUE;
        let mut cebra5_time = INVALID_VALUE;
        let mut cebra6_time = INVALID_VALUE;
        let mut cebra7_time = INVALID_VALUE;
        let mut cebra8_time = INVALID_VALUE;

        for hit in event.iter() {
            //Fill out detector fields using channel map
            let channel_data = match map.get_channel_data(&hit.uuid) {
                Some(data) => data,
                None => continue,
            };
            match channel_data.channel_type {
                ChannelType::ScintLeft => {
                    self.set_value(&ChannelDataField::ScintLeftEnergy, hit.energy);
                    self.set_value(&ChannelDataField::ScintLeftShort, hit.energy_short);
                    self.set_value(&ChannelDataField::ScintLeftTime, hit.timestamp);
                    scint_left_time = hit.timestamp;
                }

                ChannelType::ScintRight => {
                    self.set_value(&ChannelDataField::ScintRightEnergy, hit.energy);
                    self.set_value(&ChannelDataField::ScintRightShort, hit.energy_short);
                    self.set_value(&ChannelDataField::ScintRightTime, hit.timestamp);
                }

                ChannelType::Cathode => {
                    self.set_value(&ChannelDataField::CathodeEnergy, hit.energy);
                    self.set_value(&ChannelDataField::CathodeShort, hit.energy_short);
                    self.set_value(&ChannelDataField::CathodeTime, hit.timestamp);
                }

                ChannelType::DelayFrontRight => {
                    self.set_value(&ChannelDataField::DelayFrontRightEnergy, hit.energy);
                    self.set_value(&ChannelDataField::DelayFrontRightShort, hit.energy_short);
                    self.set_value(&ChannelDataField::DelayFrontRightTime, hit.timestamp);
                    dfr_time = hit.timestamp;
                }

                ChannelType::DelayFrontLeft => {
                    self.set_value(&ChannelDataField::DelayFrontLeftEnergy, hit.energy);
                    self.set_value(&ChannelDataField::DelayFrontLeftShort, hit.energy_short);
                    self.set_value(&ChannelDataField::DelayFrontLeftTime, hit.timestamp);
                    dfl_time = hit.timestamp;
                }

                ChannelType::DelayBackRight => {
                    self.set_value(&ChannelDataField::DelayBackRightEnergy, hit.energy);
                    self.set_value(&ChannelDataField::DelayBackRightShort, hit.energy_short);
                    self.set_value(&ChannelDataField::DelayBackRightTime, hit.timestamp);
                    dbr_time = hit.timestamp;
                }

                ChannelType::DelayBackLeft => {
                    self.set_value(&ChannelDataField::DelayBackLeftEnergy, hit.energy);
                    self.set_value(&ChannelDataField::DelayBackLeftShort, hit.energy_short);
                    self.set_value(&ChannelDataField::DelayBackLeftTime, hit.timestamp);
                    dbl_time = hit.timestamp;
                }

                ChannelType::AnodeFront => {
                    self.set_value(&ChannelDataField::AnodeFrontEnergy, hit.energy);
                    self.set_value(&ChannelDataField::AnodeFrontShort, hit.energy_short);
                    self.set_value(&ChannelDataField::AnodeFrontTime, hit.timestamp);
                }

                ChannelType::AnodeBack => {
                    self.set_value(&ChannelDataField::AnodeBackEnergy, hit.energy);
                    self.set_value(&ChannelDataField::AnodeBackShort, hit.energy_short);
                    self.set_value(&ChannelDataField::AnodeBackTime, hit.timestamp);
                }

                ChannelType::Cebra0 => {
                    self.set_value(&ChannelDataField::Cebra0Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra0Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra0Time, hit.timestamp);
                    cebra0_time = hit.timestamp;
                }

                ChannelType::Cebra1 => {
                    self.set_value(&ChannelDataField::Cebra1Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra1Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra1Time, hit.timestamp);
                    cebra1_time = hit.timestamp;
                }

                ChannelType::Cebra2 => {
                    self.set_value(&ChannelDataField::Cebra2Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra2Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra2Time, hit.timestamp);
                    cebra2_time = hit.timestamp;
                }

                ChannelType::Cebra3 => {
                    self.set_value(&ChannelDataField::Cebra3Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra3Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra3Time, hit.timestamp);
                    cebra3_time = hit.timestamp;
                }

                ChannelType::Cebra4 => {
                    self.set_value(&ChannelDataField::Cebra4Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra4Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra4Time, hit.timestamp);
                    cebra4_time = hit.timestamp;
                }

                ChannelType::Cebra5 => {
                    self.set_value(&ChannelDataField::Cebra5Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra5Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra5Time, hit.timestamp);
                    cebra5_time = hit.timestamp;
                }

                ChannelType::Cebra6 => {
                    self.set_value(&ChannelDataField::Cebra6Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra6Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra6Time, hit.timestamp);
                    cebra6_time = hit.timestamp;
                }

                ChannelType::Cebra7 => {
                    self.set_value(&ChannelDataField::Cebra7Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra7Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra7Time, hit.timestamp);
                    cebra7_time = hit.timestamp;
                }

                ChannelType::Cebra8 => {
                    self.set_value(&ChannelDataField::Cebra8Energy, hit.energy);
                    self.set_value(&ChannelDataField::Cebra8Short, hit.energy_short);
                    self.set_value(&ChannelDataField::Cebra8Time, hit.timestamp);
                    cebra8_time = hit.timestamp;
                }

                _ => continue,
            }
        }

        //Physics
        let mut x1 = INVALID_VALUE;
        let mut x2 = INVALID_VALUE;
        if dfr_time != INVALID_VALUE && dfl_time != INVALID_VALUE {
            x1 = (dfl_time - dfr_time) * 0.5 * 1.0 / 2.1;
            self.set_value(&ChannelDataField::X1, x1);
        }
        if dbr_time != INVALID_VALUE && dbl_time != INVALID_VALUE {
            x2 = (dbl_time - dbr_time) * 0.5 * 1.0 / 1.98;
            self.set_value(&ChannelDataField::X2, x2);
        }
        if x1 != INVALID_VALUE && x2 != INVALID_VALUE {
            let diff = x2 - x1;
            if diff > 0.0 {
                self.set_value(&ChannelDataField::Theta, (diff / 36.0).atan());
            } else if diff < 0.0 {
                self.set_value(
                    &ChannelDataField::Theta,
                    std::f64::consts::PI + (diff / 36.0).atan(),
                );
            } else {
                self.set_value(&ChannelDataField::Theta, std::f64::consts::PI * 0.5);
            }

            match weights {
                Some(w) => self.set_value(&ChannelDataField::Xavg, w.0 * x1 + w.1 * x2),
                None => self.set_value(&ChannelDataField::Xavg, INVALID_VALUE),
            };
        }

        if scint_left_time != INVALID_VALUE && cebra0_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra0RelTime,
                cebra0_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra1_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra1RelTime,
                cebra1_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra2_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra2RelTime,
                cebra2_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra3_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra3RelTime,
                cebra3_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra4_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra4RelTime,
                cebra4_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra5_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra5RelTime,
                cebra5_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra6_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra6RelTime,
                cebra6_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra7_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra7RelTime,
                cebra7_time - scint_left_time,
            );
        }
        if scint_left_time != INVALID_VALUE && cebra8_time != INVALID_VALUE {
            self.set_value(
                &ChannelDataField::Cebra8RelTime,
                cebra8_time - scint_left_time,
            );
        }
    }

    pub fn convert_to_series(self) -> Vec<Series> {
        let sps_cols: Vec<Series> = self
            .fields
            .into_iter()
            .map(|field| -> Series { Series::new(field.0.as_ref(), field.1) })
            .collect();

        sps_cols
    }
}
