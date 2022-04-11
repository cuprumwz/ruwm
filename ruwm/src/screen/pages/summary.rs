use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{Point, RgbColor, Size},
};

use crate::{
    battery::BatteryState,
    screen::{
        shapes::{self, BatteryChargedText},
        DrawTargetRef, RotateAngle, TransformingAdaptor,
    },
    valve::ValveState,
    water_meter::WaterMeterState,
};

pub struct Summary {
    valve_state: Option<Option<ValveState>>,
    water_meter_state: Option<WaterMeterState>,
    battery_state: Option<BatteryState>,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            valve_state: None,
            water_meter_state: None,
            battery_state: None,
        }
    }

    pub fn draw<D>(
        &mut self,
        target: &mut D,
        valve_state: Option<ValveState>,
        water_meter_state: WaterMeterState,
        battery_state: BatteryState,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        if self.valve_state != Some(valve_state) {
            self.valve_state = Some(valve_state);

            // TODO
        }

        if self.water_meter_state != Some(water_meter_state) {
            self.water_meter_state = Some(water_meter_state);

            let wm_shape = shapes::WaterMeterClassic::<8>::new(
                self.water_meter_state.map(|wm| wm.edges_count),
                1,
                true,
            );

            //let bbox = target.bounding_box();

            let mut target = TransformingAdaptor::display(DrawTargetRef::new(target))
                .translate(Point::new(0, 30));

            wm_shape.draw(&mut target)?;
        }

        if self.battery_state != Some(battery_state) {
            self.battery_state = Some(battery_state);

            let percentage = battery_state.voltage.map(|voltage| {
                (voltage as u32 * 100
                    / (BatteryState::MAX_VOLTAGE as u32 + BatteryState::LOW_VOLTAGE as u32))
                    as u8
            });

            let battery_shape = shapes::Battery::new(percentage, BatteryChargedText::No, false);

            let bbox = target.bounding_box();

            let mut target = TransformingAdaptor::display(DrawTargetRef::new(target))
                .translate(Point::new(bbox.size.width as i32 - 40, 0))
                .scale_from(shapes::Battery::SIZE, Size::new(20, 40))
                .rotate(RotateAngle::Degrees270);

            battery_shape.draw(&mut target)?;
        }

        Ok(())
    }
}
