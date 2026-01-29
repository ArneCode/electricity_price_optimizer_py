use std::{fmt::Debug, rc::Rc};

use chrono::{DateTime, TimeDelta, TimeZone, Timelike, Utc};
use electricity_price_optimizer::{
    optimizer_context::{
        OptimizerContext as RustOptimizerContext,
        action::{
            constant::AssignedConstantAction as RustAssignedConstantAction,
            constant::ConstantAction as RustConstantAction,
            variable::AssignedVariableAction as RustAssignedVariableAction,
            variable::VariableAction as RustVariableAction,
        },
        battery::AssignedBattery as RustAssignedBattery,
        battery::Battery as RustBattery,
        prognoses::Prognoses,
    },
    schedule::Schedule as RustSchedule,
    time::{MINUTES_PER_TIMESTEP, STEPS_PER_DAY, Time},
};
use pyo3::{
    Bound, FromPyObject, Py, PyAny, PyErr, PyResult, Python,
    exceptions::PyValueError,
    prelude::FromPyObjectOwned,
    pyclass, pyfunction, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
#[pyclass]
struct PrognosesProvider {
    get_data: Py<PyAny>,
}

#[pymethods]
impl PrognosesProvider {
    #[new]
    fn new(get_data: Py<PyAny>) -> Self {
        PrognosesProvider { get_data }
    }
}
fn time_to_datetime(time: Time, start_time: DateTime<Utc>) -> DateTime<Utc> {
    // 1. Get starting point in nanoseconds
    // .expect() is used here because Utc timestamps usually fit in i64 nanos
    // unless you're dealing with years far in the future/past.
    let start_ns = start_time
        .timestamp_nanos_opt()
        .expect("Timestamp out of range");

    // 2. Define our interval in nanoseconds
    let ns_per_minute: i64 = 60 * 1_000_000_000;
    let interval_ns = (MINUTES_PER_TIMESTEP as i64 * ns_per_minute) as i64;

    // 3. Calculate target time in nanoseconds
    let added_ns = time.get_minutes() as i64 * ns_per_minute;
    let target_ns = start_ns + added_ns;

    // 4. Round down to the nearest timestep
    // The modulo operation gives us the "overflow" past the last clean interval
    let rounded_ns = target_ns - (target_ns % interval_ns);

    // 5. Ensure we don't round back to a time before the start_time
    let res_ns = rounded_ns.max(start_ns);

    // 6. Convert nanoseconds back into a DateTime object
    Utc.timestamp_nanos(res_ns)
}

fn datetime_to_time(dt: DateTime<Utc>, start_time: DateTime<Utc>) -> Result<Time, PyErr> {
    if dt == start_time {
        return Ok(Time::from_timestep(0));
    }
    if dt < start_time {
        return Err(PyValueError::new_err(format!(
            "DateTime {} is before start time {}",
            dt, start_time
        )));
    }
    // dt must be on a timestep boundary
    // check:
    if (dt.minute() % MINUTES_PER_TIMESTEP) != 0
        || dt.second() != 0
        || dt.timestamp_subsec_nanos() != 0
    {
        return Err(PyValueError::new_err(format!(
            "DateTime is not on a timestep boundary: minute={}, second={}, nanos={}",
            dt.minute(),
            dt.second(),
            dt.timestamp_subsec_nanos()
        )));
    }
    let duration = dt.signed_duration_since(start_time);
    let total_minutes = duration.num_minutes() as u32;
    let timesteps = total_minutes / MINUTES_PER_TIMESTEP;
    Ok(Time::from_timestep(timesteps))
}

impl PrognosesProvider {
    fn get_prognoses<'py, T: Clone + Debug + Default + FromPyObjectOwned<'py>>(
        &self,
        py: Python<'py>,
        start_time: DateTime<Utc>,
    ) -> Result<Prognoses<T>, PyErr> {
        Prognoses::from_closure_result(|t: Time| {
            let curr_t = time_to_datetime(t, start_time);
            let next_t = time_to_datetime(t.get_next_timestep(), start_time);
            let result = self
                .get_data
                .call1(py, (curr_t, next_t))
                .map_err(Into::<PyErr>::into)?;
            Ok(result.extract::<T>(py).map_err(Into::into)?)
        })
    }
}
#[pyclass(unsendable)]
pub struct ConstantAction {
    /// The earliest time the action can start.
    pub start_from: DateTime<Utc>,
    /// The latest time the action must end before.
    pub end_before: DateTime<Utc>,
    /// The duration of the action.
    pub duration: TimeDelta,
    /// The fixed consumption amount of the action for every timestep.
    pub consumption: i32,
    id: u32,
}
#[pymethods]
impl ConstantAction {
    #[new]
    fn new(
        start_from: DateTime<Utc>,
        end_before: DateTime<Utc>,
        duration: TimeDelta,
        consumption: i32,
        id: u32,
    ) -> Self {
        ConstantAction {
            start_from,
            end_before,
            duration,
            consumption,
            id,
        }
    }
}
impl ConstantAction {
    fn to_rust<'py>(
        &self,
        py: Python<'py>,
        start_time: DateTime<Utc>,
    ) -> PyResult<RustConstantAction> {
        let duration = self.duration;
        if duration.num_days() != 0 {
            return Err(PyValueError::new_err("Duration must be less than 1 day"));
        }
        let duration_minutes = duration.num_minutes() as u32;
        if (duration_minutes % MINUTES_PER_TIMESTEP) != 0 {
            return Err(PyValueError::new_err(format!(
                "Duration must be a multiple of {} minutes",
                MINUTES_PER_TIMESTEP
            )));
        }
        let duration = Time::new(0, duration_minutes);

        let start_time_converted = datetime_to_time(self.start_from, start_time)?;
        let end_time_converted = datetime_to_time(self.end_before, start_time)?;

        Ok(RustConstantAction::new(
            start_time_converted,
            end_time_converted,
            duration,
            self.consumption,
            self.id,
        ))
    }
}

#[pyclass(unsendable)]
pub struct AssignedConstantAction {
    inner: RustAssignedConstantAction,
    start_timestamp: DateTime<Utc>,
}
#[pymethods]
impl AssignedConstantAction {
    fn get_start_time(&self) -> DateTime<Utc> {
        time_to_datetime(self.inner.get_start_time(), self.start_timestamp)
    }
    fn get_end_time(&self) -> DateTime<Utc> {
        time_to_datetime(self.inner.get_end_time(), self.start_timestamp)
    }
    fn get_id(&self) -> u32 {
        self.inner.get_id()
    }
}
#[pyclass(unsendable)]
pub struct VariableAction {
    /// The earliest time the action can start.
    pub start: DateTime<Utc>,
    /// The latest time the action must end.
    pub end: DateTime<Utc>,
    /// The total consumption amount of the action.
    pub total_consumption: i32,
    /// The maximum consumption amount of the action for every timestep.
    pub max_consumption: i32,
    /// The unique identifier for the action.
    id: u32,
}
#[pymethods]
impl VariableAction {
    #[new]
    fn new(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        total_consumption: i32,
        max_consumption: i32,
        id: u32,
    ) -> Self {
        VariableAction {
            start,
            end,
            total_consumption,
            max_consumption,
            id,
        }
    }
}
impl VariableAction {
    fn to_rust(&self, start_time: DateTime<Utc>) -> PyResult<RustVariableAction> {
        let start_time_converted = datetime_to_time(self.start, start_time)?;
        let end_time_converted = datetime_to_time(self.end, start_time)?;

        Ok(RustVariableAction::new(
            start_time_converted,
            end_time_converted,
            self.total_consumption,
            self.max_consumption,
            self.id,
        ))
    }
}
#[pyclass(unsendable)]
pub struct AssignedVariableAction {
    inner: RustAssignedVariableAction,
    start_timestamp: DateTime<Utc>,
}
#[pymethods]
impl AssignedVariableAction {
    fn get_consumption(&self, time: DateTime<Utc>) -> PyResult<u32> {
        let time_converted = datetime_to_time(time, self.start_timestamp)?;
        Ok(self.inner.get_consumption(time_converted))
    }
    fn get_id(&self) -> u32 {
        self.inner.get_id()
    }
}
#[pyclass(unsendable)]
pub struct Battery {
    pub capacity: i32,
    pub max_charge_rate: i32,
    pub max_discharge_rate: i32,
    pub initial_charge: i32,
    pub id: u32,
}
#[pymethods]
impl Battery {
    #[new]
    fn new(
        capacity: i32,
        max_charge_rate: i32,
        max_discharge_rate: i32,
        initial_charge: i32,
        id: u32,
    ) -> Self {
        Battery {
            capacity,
            max_charge_rate,
            max_discharge_rate,
            initial_charge,
            id,
        }
    }
}
impl Battery {
    fn to_rust(&self) -> RustBattery {
        RustBattery::new(
            self.capacity,
            self.initial_charge,
            self.max_charge_rate,
            self.max_discharge_rate,
            1.0,
            self.id,
        )
    }
}
#[pyclass(unsendable)]
pub struct AssignedBattery {
    inner: RustAssignedBattery,
    start_timestamp: DateTime<Utc>,
}
#[pymethods]
impl AssignedBattery {
    fn get_charge_level(&self, time: DateTime<Utc>) -> PyResult<u32> {
        let time_converted = datetime_to_time(time, self.start_timestamp)?;
        if let Some(result) = self.inner.get_charge_level(time_converted) {
            Ok(*result)
        } else {
            Err(PyValueError::new_err(
                "Time out of range for battery charge level FIXME",
            ))
        }
    }
    fn get_id(&self) -> u32 {
        self.inner.get_battery().get_id()
    }
}
#[pyclass(unsendable)]
struct OptimizerContext {
    electricity_price: Prognoses<i32>,
    generated_electricity: Prognoses<i32>,
    beyond_control_consumption: Prognoses<i32>,
    batteries: Vec<Rc<RustBattery>>,
    constant_actions: Vec<Rc<RustConstantAction>>,
    variable_actions: Vec<Rc<RustVariableAction>>,
    start_time: DateTime<Utc>,
}

#[pymethods]
impl OptimizerContext {
    #[new]
    fn new(
        py: Python<'_>,
        time: DateTime<Utc>,
        electricity_price: &PrognosesProvider,
    ) -> Result<Self, PyErr> {
        let electricity_price = electricity_price.get_prognoses::<i32>(py, time)?;
        let generated_electricity = Prognoses::from_closure(|_| 0);
        let beyond_control_consumption = Prognoses::from_closure(|_| 0);
        let batteries = vec![];
        let constant_actions = vec![];
        let variable_actions = vec![];
        let start_time = time;

        // let inner = RustOptimizerContext::new(electricity_price, generated_electricity, beyond_control_consumption, batteries, constant_actions, variable_actions, first_timestep_fraction)
        // OptimizerContext {
        //     inner,
        //     start_timestamp,
        // }
        Ok(OptimizerContext {
            electricity_price,
            generated_electricity,
            beyond_control_consumption,
            batteries,
            constant_actions,
            variable_actions,
            start_time,
        })
    }
    fn add_constant_action<'py>(
        &mut self,
        py: Python<'py>,
        action: &ConstantAction,
    ) -> PyResult<()> {
        self.constant_actions
            .push(Rc::new(action.to_rust(py, self.start_time)?));
        Ok(())
    }

    fn add_variable_action<'py>(
        &mut self,
        py: Python<'py>,
        action: &VariableAction,
    ) -> PyResult<()> {
        self.variable_actions
            .push(Rc::new(action.to_rust(self.start_time)?));
        Ok(())
    }
    fn add_battery(&mut self, battery: &Battery) -> PyResult<()> {
        self.batteries.push(Rc::new(battery.to_rust()));
        Ok(())
    }
    fn add_past_constant_action<'py>(
        &mut self,
        py: Python<'py>,
        action: &AssignedConstantAction,
    ) -> PyResult<()> {
        // find out how much time has passed since action start
        let end_time = action.get_end_time();
        let end_time = datetime_to_time(end_time, self.start_time)?;
        self.beyond_control_consumption += Prognoses::from_closure(|t: Time| {
            if t >= end_time {
                0
            } else {
                action.inner.get_action().get_consumption()
            }
        });
        Ok(())
    }
    fn add_generated_electricity_prognoses<'py>(
        &mut self,
        py: Python<'py>,
        provider: &PrognosesProvider,
    ) -> PyResult<()> {
        self.generated_electricity += provider.get_prognoses::<i32>(py, self.start_time)?;
        Ok(())
    }
}
impl OptimizerContext {
    fn to_rust(&self) -> RustOptimizerContext {
        // first_timestep fraction is the length of the first timestep that is remaining divided by full timestep length
        let first_timestep_fraction = {
            let start_time = self.start_time;
            let next_timestep = time_to_datetime(Time::from_timestep(1), start_time);
            let remaining_duration = next_timestep.signed_duration_since(start_time);
            // calculate as precise as possible
            let remaining_nanos = remaining_duration.num_nanoseconds().unwrap() as f64;
            let full_timestep_nanos = (MINUTES_PER_TIMESTEP as i64 * 60 * 1_000_000_000) as f64;
            remaining_nanos / full_timestep_nanos
        };
        RustOptimizerContext::new(
            self.electricity_price.clone(),
            self.generated_electricity.clone(),
            self.beyond_control_consumption.clone(),
            self.batteries.clone(),
            self.constant_actions.clone(),
            self.variable_actions.clone(),
            first_timestep_fraction as f32,
        )
    }
}

#[pyclass(unsendable)]
pub struct Schedule {
    inner: RustSchedule,
    start_timestamp: DateTime<Utc>,
}
#[pymethods]
impl Schedule {
    fn get_constant_action(&self, id: u32) -> Option<AssignedConstantAction> {
        if let Some(action) = self.inner.get_constant_action(id) {
            Some(AssignedConstantAction {
                inner: action.clone(),
                start_timestamp: self.start_timestamp,
            })
        } else {
            None
        }
    }
    fn get_variable_action(&self, id: u32) -> Option<AssignedVariableAction> {
        if let Some(action) = self.inner.get_variable_action(id) {
            Some(AssignedVariableAction {
                inner: action.clone(),
                start_timestamp: self.start_timestamp,
            })
        } else {
            None
        }
    }
    fn get_battery(&self, id: u32) -> Option<AssignedBattery> {
        if let Some(battery) = self.inner.get_battery(id) {
            Some(AssignedBattery {
                inner: battery.clone(),
                start_timestamp: self.start_timestamp,
            })
        } else {
            None
        }
    }
}

#[pyfunction]
fn run_simulated_annealing(
    py: Python<'_>,
    context: &OptimizerContext,
) -> PyResult<(i64, Schedule)> {
    let rust_context = context.to_rust();
    let (cost, rust_schedule) =
        electricity_price_optimizer::simulated_annealing::run_simulated_annealing(rust_context);
    Ok((
        cost,
        Schedule {
            inner: rust_schedule,
            start_timestamp: context.start_time,
        },
    ))
}

#[pymodule]
fn electricity_price_optimizer_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register classes
    m.add_class::<PrognosesProvider>()?;
    m.add_class::<ConstantAction>()?;
    m.add_class::<AssignedConstantAction>()?;
    m.add_class::<VariableAction>()?;
    m.add_class::<AssignedVariableAction>()?;
    m.add_class::<Battery>()?;
    m.add_class::<AssignedBattery>()?;
    m.add_class::<OptimizerContext>()?;
    m.add_class::<Schedule>()?;

    // Register functions
    m.add_function(wrap_pyfunction!(run_simulated_annealing, m)?)?;

    Ok(())
}
