use std::ops::{Add, Div, Mul, Sub};

use chrono::TimeDelta;
use electricity_price_optimizer::time::MINUTES_PER_TIMESTEP;
use pyo3::{
    Bound, FromPyObject, IntoPyObjectExt, PyAny, PyResult, Python,
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
const NANOSECONDS_PER_HOUR: f64 = 3_600_000_000_000.0;

#[derive(FromPyObject)]
enum UnitOrTimeOrFloat {
    Watt(Watt),
    WattHour(WattHour),
    Euro(Euro),
    EuroPerWh(EuroPerWh),
    TimeDelta(TimeDelta),
    Float(f64),
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Watt {
    pub value: f64,
}
impl Add for &Watt {
    type Output = Watt;

    fn add(self, other: &Watt) -> Watt {
        self.__add__(other)
    }
}
impl Sub for &Watt {
    type Output = Watt;

    fn sub(self, other: &Watt) -> Watt {
        self.__sub__(other)
    }
}
impl Mul<TimeDelta> for &Watt {
    type Output = WattHour;

    fn mul(self, other: TimeDelta) -> WattHour {
        let hours = other.num_nanoseconds().unwrap() as f64 / NANOSECONDS_PER_HOUR;
        WattHour {
            value: self.value * hours,
        }
    }
}
impl Mul<f64> for &Watt {
    type Output = Watt;

    fn mul(self, other: f64) -> Watt {
        Watt {
            value: self.value * other,
        }
    }
}
impl Div<f64> for &Watt {
    type Output = Watt;

    fn div(self, other: f64) -> Watt {
        Watt {
            value: self.value / other,
        }
    }
}
impl Div<Watt> for &Watt {
    type Output = f64;

    fn div(self, other: Watt) -> f64 {
        self.value / other.value
    }
}
#[pymethods]
impl Watt {
    #[new]
    fn new(value: f64) -> Self {
        Watt { value }
    }
    fn __mul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::TimeDelta(td) => {
                let result = self * td;
                // .into_bound_py_any(py) is the modern way to convert to Bound<'_, PyAny>
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::Float(f) => {
                let result = self * f;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for multiplication with Watt. Expected TimeDelta or float.",
            )),
        }
    }
    fn __rmul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__mul__(py, other)
    }

    fn __add__(&self, other: &Watt) -> Watt {
        Watt {
            value: self.value + other.value,
        }
    }
    fn __sub__(&self, other: &Watt) -> Watt {
        Watt {
            value: self.value - other.value,
        }
    }
    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::Float(f) => {
                let result = self / f;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::Watt(w) => {
                let result = self / w;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for division with Watt. Expected float or Watt.",
            )),
        }
    }
    fn __repr__(&self) -> String {
        // format with 2 decimal places
        format!("{:.2} W", self.value)
    }

    fn get_value(&self) -> f64 {
        self.value
    }
    fn __eq__(&self, other: &Watt) -> bool {
        self.value == other.value
    }
    fn __lt__(&self, other: &Watt) -> bool {
        self.value < other.value
    }
}
impl Watt {
    pub fn to_milli_watt_hour_per_timestep(&self) -> f64 {
        let timestep_duration = TimeDelta::minutes(MINUTES_PER_TIMESTEP as i64);
        let wh = self * timestep_duration;
        wh.to_milli_wh()
    }
    pub fn from_milli_watt_hour_per_timestep(value: f64) -> Self {
        let timestep_duration = TimeDelta::minutes(MINUTES_PER_TIMESTEP as i64);
        let wh = WattHour::from_milli_wh(value);
        &wh / timestep_duration
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct WattHour {
    pub value: f64,
}
impl Mul<f64> for &WattHour {
    type Output = WattHour;

    fn mul(self, other: f64) -> WattHour {
        WattHour {
            value: self.value * other,
        }
    }
}
impl Div<TimeDelta> for &WattHour {
    type Output = Watt;

    fn div(self, other: TimeDelta) -> Watt {
        // calc in nanos for precision
        let hours = other.num_nanoseconds().unwrap() as f64 / NANOSECONDS_PER_HOUR;
        Watt {
            value: self.value / hours,
        }
    }
}
impl Div<Watt> for &WattHour {
    type Output = TimeDelta;

    fn div(self, other: Watt) -> TimeDelta {
        let hours = self.value / other.value;
        let nanos = (hours * NANOSECONDS_PER_HOUR) as i64;
        TimeDelta::nanoseconds(nanos)
    }
}
impl Div<&WattHour> for &WattHour {
    type Output = f64;

    fn div(self, other: &WattHour) -> f64 {
        self.value / other.value
    }
}
impl Div<f64> for &WattHour {
    type Output = WattHour;

    fn div(self, other: f64) -> WattHour {
        WattHour {
            value: self.value / other,
        }
    }
}
impl Add for &WattHour {
    type Output = WattHour;

    fn add(self, other: &WattHour) -> WattHour {
        self.__add__(other)
    }
}
impl Sub for &WattHour {
    type Output = WattHour;

    fn sub(self, other: &WattHour) -> WattHour {
        self.__sub__(other)
    }
}
impl Mul<&EuroPerWh> for &WattHour {
    type Output = Euro;

    fn mul(self, other: &EuroPerWh) -> Euro {
        Euro {
            value: self.value * other.value,
        }
    }
}
#[pymethods]
impl WattHour {
    #[new]
    fn new(value: f64) -> Self {
        WattHour { value }
    }

    fn __mul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::EuroPerWh(epw) => {
                let result = self * &epw;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::Float(f) => {
                let result = self * f;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for multiplication with WattHour. Expected EuroPerWh or float.",
            )),
        }
    }

    fn __rmul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__mul__(py, other)
    }

    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::Float(f) => {
                let result = self / f;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::TimeDelta(td) => {
                let result = self / td;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::Watt(w) => {
                let result = self / w;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::WattHour(wh) => {
                let result = self / &wh;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for division with WattHour. Expected float or WattHour.",
            )),
        }
    }

    fn __add__(&self, other: &WattHour) -> WattHour {
        WattHour {
            value: self.value + other.value,
        }
    }
    fn __sub__(&self, other: &WattHour) -> WattHour {
        WattHour {
            value: self.value - other.value,
        }
    }
    fn __repr__(&self) -> String {
        // format with 2 decimal places
        format!("{:.2} Wh", self.value)
    }

    fn get_value(&self) -> f64 {
        self.value
    }
    fn __eq__(&self, other: &WattHour) -> bool {
        self.value == other.value
    }
    fn __lt__(&self, other: &WattHour) -> bool {
        self.value < other.value
    }
}
impl WattHour {
    pub fn to_milli_wh(&self) -> f64 {
        self.value * 1_000.0
    }
    pub fn from_milli_wh(value: f64) -> Self {
        WattHour::new(value / 1_000.0)
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Euro {
    pub value: f64,
}
impl Mul<f64> for &Euro {
    type Output = Euro;

    fn mul(self, other: f64) -> Euro {
        Euro {
            value: self.value * other,
        }
    }
}
impl Div<WattHour> for &Euro {
    type Output = EuroPerWh;

    fn div(self, other: WattHour) -> EuroPerWh {
        EuroPerWh {
            value: self.value / other.value,
        }
    }
}
impl Div<f64> for &Euro {
    type Output = Euro;

    fn div(self, other: f64) -> Euro {
        Euro {
            value: self.value / other,
        }
    }
}
impl Div<&Euro> for &Euro {
    type Output = f64;

    fn div(self, other: &Euro) -> f64 {
        self.value / other.value
    }
}
impl Add for &Euro {
    type Output = Euro;

    fn add(self, other: &Euro) -> Euro {
        self.__add__(other)
    }
}
impl Sub for &Euro {
    type Output = Euro;

    fn sub(self, other: &Euro) -> Euro {
        self.__sub__(other)
    }
}
#[pymethods]
impl Euro {
    #[new]
    fn new(value: f64) -> Self {
        Euro { value }
    }

    fn __mul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::Float(f) => {
                let result = self * f;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for multiplication with Euro. Expected float.",
            )),
        }
    }

    fn __rmul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__mul__(py, other)
    }

    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::Float(f) => {
                let result = self / f;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::WattHour(wh) => {
                let result = self / wh;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for division with Euro. Expected float or WattHour.",
            )),
        }
    }

    fn __add__(&self, other: &Euro) -> Euro {
        Euro {
            value: self.value + other.value,
        }
    }
    fn __sub__(&self, other: &Euro) -> Euro {
        Euro {
            value: self.value - other.value,
        }
    }
    fn __repr__(&self) -> String {
        // format with 2 decimal places
        format!("{:.2} €", self.value)
    }

    fn get_value(&self) -> f64 {
        self.value
    }
    fn __eq__(&self, other: &Euro) -> bool {
        self.value == other.value
    }
    fn __lt__(&self, other: &Euro) -> bool {
        self.value < other.value
    }
}
impl Euro {
    pub fn from_nano_euro(value: f64) -> Self {
        Euro::new(value / 1_000_000_000.0)
    }
    pub fn to_nano_euro(&self) -> f64 {
        self.value * 1_000_000_000.0
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct EuroPerWh {
    pub value: f64,
}
impl Mul<&WattHour> for &EuroPerWh {
    type Output = Euro;

    fn mul(self, other: &WattHour) -> Euro {
        Euro {
            value: self.value * other.value,
        }
    }
}
impl Mul<f64> for &EuroPerWh {
    type Output = EuroPerWh;

    fn mul(self, other: f64) -> EuroPerWh {
        EuroPerWh {
            value: self.value * other,
        }
    }
}

impl Div<f64> for &EuroPerWh {
    type Output = EuroPerWh;

    fn div(self, other: f64) -> EuroPerWh {
        EuroPerWh {
            value: self.value / other,
        }
    }
}
impl Div<&EuroPerWh> for &EuroPerWh {
    type Output = f64;

    fn div(self, other: &EuroPerWh) -> f64 {
        self.value / other.value
    }
}
impl Add for &EuroPerWh {
    type Output = EuroPerWh;

    fn add(self, other: &EuroPerWh) -> EuroPerWh {
        self.__add__(other)
    }
}
impl Sub for &EuroPerWh {
    type Output = EuroPerWh;

    fn sub(self, other: &EuroPerWh) -> EuroPerWh {
        self.__sub__(other)
    }
}
#[pymethods]
impl EuroPerWh {
    #[new]
    fn new(value: f64) -> Self {
        EuroPerWh { value }
    }

    fn __mul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::WattHour(wh) => {
                let result = self * &wh;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::Float(f) => {
                let result = self * f;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for multiplication with EuroPerWh. Expected WattHour or float.",
            )),
        }
    }
    fn __rmul__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__mul__(py, other)
    }
    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: UnitOrTimeOrFloat,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            UnitOrTimeOrFloat::Float(f) => {
                let result = self / f;
                Ok(result.into_bound_py_any(py)?)
            }
            UnitOrTimeOrFloat::EuroPerWh(epw) => {
                let result = self / &epw;
                Ok(result.into_bound_py_any(py)?)
            }
            _ => Err(PyTypeError::new_err(
                "Unsupported type for division with EuroPerWh. Expected float or EuroPerWh.",
            )),
        }
    }

    fn __add__(&self, other: &EuroPerWh) -> EuroPerWh {
        EuroPerWh {
            value: self.value + other.value,
        }
    }
    fn __sub__(&self, other: &EuroPerWh) -> EuroPerWh {
        EuroPerWh {
            value: self.value - other.value,
        }
    }
    fn __repr__(&self) -> String {
        // format with 6 decimal places
        format!("{:.6} €/Wh", self.value)
    }
    fn get_value(&self) -> f64 {
        self.value
    }
    fn __eq__(&self, other: &EuroPerWh) -> bool {
        self.value == other.value
    }
    fn __lt__(&self, other: &EuroPerWh) -> bool {
        self.value < other.value
    }
}
impl EuroPerWh {
    pub fn to_micro_euro_per_wh(&self) -> f64 {
        self.value * 1_000_000.0
    }
}

pub fn register_units_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let units_mod = PyModule::new(parent_module.py(), "units")?;

    // Add the unit classes to the submodule
    units_mod.add_class::<Watt>()?;
    units_mod.add_class::<WattHour>()?;
    units_mod.add_class::<Euro>()?;
    units_mod.add_class::<EuroPerWh>()?;

    // Add the submodule to the parent
    parent_module.add_submodule(&units_mod)?;
    Ok(())
}
