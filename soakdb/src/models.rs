use sea_orm::ActiveValue;

#[derive(Debug, Clone)]
pub struct Visit {
    pub name: String,
    pub protein: String,
}

impl From<Visit> for crate::tables::soak_db::ActiveModel {
    fn from(value: Visit) -> Self {
        Self {
            id: ActiveValue::Set(1),
            version: ActiveValue::NotSet,
            lab_visit: ActiveValue::Set(Some(value.name)),
            path: ActiveValue::NotSet,
            protein: ActiveValue::Set(Some(value.protein)),
            drop_volume: ActiveValue::NotSet,
            crystals_per_batch: ActiveValue::NotSet,
            one_batch_per_plate: ActiveValue::NotSet,
            compound_stock: ActiveValue::NotSet,
            solvent_percent: ActiveValue::NotSet,
            cryo_stock: ActiveValue::NotSet,
            desired_cryo: ActiveValue::NotSet,
            cryo_location: ActiveValue::NotSet,
            desired_soak_time: ActiveValue::NotSet,
            crystal_start_number: ActiveValue::NotSet,
            beamline_visit: ActiveValue::NotSet,
            space_group: ActiveValue::NotSet,
            a: ActiveValue::NotSet,
            b: ActiveValue::NotSet,
            c: ActiveValue::NotSet,
            alpha: ActiveValue::NotSet,
            beta: ActiveValue::NotSet,
            gamma: ActiveValue::NotSet,
            recipe: ActiveValue::NotSet,
            resolution: ActiveValue::NotSet,
            centring_method: ActiveValue::NotSet,
            eub_open: ActiveValue::NotSet,
            i_next: ActiveValue::NotSet,
            covid19: ActiveValue::NotSet,
            ilo_xchem: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VisitReadback {
    pub name: Option<String>,
    pub protein: Option<String>,
}

impl From<Visit> for VisitReadback {
    fn from(value: Visit) -> Self {
        Self {
            name: Some(value.name),
            protein: Some(value.protein),
        }
    }
}

impl From<crate::tables::soak_db::Model> for VisitReadback {
    fn from(value: crate::tables::soak_db::Model) -> Self {
        Self {
            protein: value.protein,
            name: value.lab_visit,
        }
    }
}
