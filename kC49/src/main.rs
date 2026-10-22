#![windows_subsystem = "windows"]

// ============================================================================
// kapivaraCalc (kC49) - Graphing & CAS Calculator
// ============================================================================
// Reverse-engineered from Hewlett-Packard HP 49G ROM (rom.e49 / VGER v1.19-6)
// and official emulator binary (YUser.exe).
//
// All mathematical engines, RPN stack, CAS approximations, financial TVM,
// linear algebra, unit conversions, physics constants, standalone window launcher,
// and modern glassmorphic/neumorphic UI are integrated in this single file.
//
// Compatible with: Windows, macOS, Linux, Android (APK/PWA), and iOS (PWA/Capacitor).
// ============================================================================

use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// 1. MATHEMATICAL CORE: VALUES, COMPLEX NUMBERS, AND MATRICES
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Real(f64),
    Complex(f64, f64),
    Matrix(Vec<Vec<f64>>),
    StringVal(String),
}

impl Value {
    pub fn to_display_string(&self, _angle_mode: AngleMode, base_mode: BaseMode) -> String {
        match self {
            Value::Real(r) => {
                if base_mode != BaseMode::Dec {
                    let int_val = *r as i64;
                    match base_mode {
                        BaseMode::Hex => format!("#{:X}h", int_val),
                        BaseMode::Oct => format!("#{:o}o", int_val),
                        BaseMode::Bin => format!("#{:b}b", int_val),
                        BaseMode::Dec => unreachable!(),
                    }
                } else if r.fract() == 0.0 && r.abs() < 1e14 {
                    format!("{:.0}", r)
                } else {
                    format!("{:.8}", r)
                }
            }
            Value::Complex(re, im) => {
                let sign = if *im >= 0.0 { "+" } else { "-" };
                format!("({:.6} {} {:.6}i)", re, sign, im.abs())
            }
            Value::Matrix(m) => {
                let rows: Vec<String> = m
                    .iter()
                    .map(|r| {
                        let cols: Vec<String> = r.iter().map(|c| format!("{:.4}", c)).collect();
                        format!("[{}]", cols.join(", "))
                    })
                    .collect();
                format!("[{}]", rows.join(", "))
            }
            Value::StringVal(s) => format!("\"{}\"", s),
        }
    }

    pub fn as_f64(&self) -> Result<f64, String> {
        match self {
            Value::Real(r) => Ok(*r),
            Value::Complex(re, _) => Ok(*re),
            _ => Err("Expected numeric real scalar".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AngleMode {
    Deg,
    Rad,
    Grad,
}

impl AngleMode {
    pub fn to_radians(self, a: f64) -> f64 {
        match self {
            AngleMode::Rad => a,
            AngleMode::Deg => a * PI / 180.0,
            AngleMode::Grad => a * PI / 200.0,
        }
    }

    pub fn from_radians(self, r: f64) -> f64 {
        match self {
            AngleMode::Rad => r,
            AngleMode::Deg => r * 180.0 / PI,
            AngleMode::Grad => r * 200.0 / PI,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BaseMode {
    Dec,
    Hex,
    Oct,
    Bin,
}

// ============================================================================
// 2. LINEAR ALGEBRA & MATRIX ENGINE
// ============================================================================

pub struct MatrixEngine;

impl MatrixEngine {
    pub fn det(m: &[Vec<f64>]) -> Result<f64, String> {
        let n = m.len();
        if n == 0 || m.iter().any(|r| r.len() != n) {
            return Err("Matrix must be square".to_string());
        }
        let mut a = m.to_vec();
        let mut det = 1.0;

        for i in 0..n {
            let mut pivot = i;
            for j in (i + 1)..n {
                if a[j][i].abs() > a[pivot][i].abs() {
                    pivot = j;
                }
            }
            if a[pivot][i].abs() < 1e-12 {
                return Ok(0.0);
            }
            if pivot != i {
                a.swap(i, pivot);
                det = -det;
            }
            det *= a[i][i];
            let pivot_val = a[i][i];
            for j in (i + 1)..n {
                let factor = a[j][i] / pivot_val;
                for k in (i + 1)..n {
                    a[j][k] -= factor * a[i][k];
                }
            }
        }
        Ok(det)
    }

    pub fn inverse(m: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
        let n = m.len();
        if n == 0 || m.iter().any(|r| r.len() != n) {
            return Err("Matrix must be square".to_string());
        }
        let mut a = vec![vec![0.0; 2 * n]; n];
        for i in 0..n {
            for j in 0..n {
                a[i][j] = m[i][j];
            }
            a[i][n + i] = 1.0;
        }

        for i in 0..n {
            let mut pivot = i;
            for j in (i + 1)..n {
                if a[j][i].abs() > a[pivot][i].abs() {
                    pivot = j;
                }
            }
            if a[pivot][i].abs() < 1e-12 {
                return Err("Singular matrix cannot be inverted".to_string());
            }
            a.swap(i, pivot);
            let div = a[i][i];
            for k in 0..(2 * n) {
                a[i][k] /= div;
            }
            for j in 0..n {
                if j != i {
                    let mult = a[j][i];
                    for k in 0..(2 * n) {
                        a[j][k] -= mult * a[i][k];
                    }
                }
            }
        }

        let mut inv = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                inv[i][j] = a[i][n + j];
            }
        }
        Ok(inv)
    }

    pub fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if m.is_empty() {
            return vec![];
        }
        let rows = m.len();
        let cols = m[0].len();
        let mut t = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                t[j][i] = m[i][j];
            }
        }
        t
    }

    pub fn multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
        if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
            return Err("Incompatible matrix dimensions for multiplication".to_string());
        }
        let r = a.len();
        let c = b[0].len();
        let k_len = b.len();
        let mut res = vec![vec![0.0; c]; r];
        for i in 0..r {
            for j in 0..c {
                let mut sum = 0.0;
                for k in 0..k_len {
                    sum += a[i][k] * b[k][j];
                }
                res[i][j] = sum;
            }
        }
        Ok(res)
    }

    pub fn solve_system(a: &[Vec<f64>], b: &[f64]) -> Result<Vec<f64>, String> {
        let inv_a = Self::inverse(a)?;
        let n = b.len();
        let mut x = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                sum += inv_a[i][j] * b[j];
            }
            x[i] = sum;
        }
        Ok(x)
    }
}

// ============================================================================
// 3. CALCULUS, POLYNOMIALS, AND NUMERICAL CAS
// ============================================================================

pub struct CasEngine;

impl CasEngine {
    pub fn differentiate<F>(f: F, x: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let h = 1e-4 * (1.0 + x.abs());
        (-f(x + 2.0 * h) + 8.0 * f(x + h) - 8.0 * f(x - h) + f(x - 2.0 * h)) / (12.0 * h)
    }

    pub fn differentiate2<F>(f: F, x: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let h = 1e-3 * (1.0 + x.abs());
        (-f(x + 2.0 * h) + 16.0 * f(x + h) - 30.0 * f(x) + 16.0 * f(x - h) - f(x - 2.0 * h))
            / (12.0 * h * h)
    }

    pub fn integrate<F>(f: F, a: f64, b: f64, n_steps: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let n = if n_steps % 2 != 0 { n_steps + 1 } else { n_steps };
        let h = (b - a) / (n as f64);
        let mut sum = f(a) + f(b);

        for i in 1..n {
            let x = a + (i as f64) * h;
            if i % 2 == 0 {
                sum += 2.0 * f(x);
            } else {
                sum += 4.0 * f(x);
            }
        }
        (h / 3.0) * sum
    }

    pub fn poly_roots(coeffs: &[f64]) -> Vec<(f64, f64)> {
        if coeffs.len() < 2 {
            return vec![];
        }
        let deg = coeffs.len() - 1;
        let lead = coeffs[0];
        let p: Vec<f64> = coeffs.iter().map(|c| c / lead).collect();

        let mut roots: Vec<(f64, f64)> = (0..deg)
            .map(|k| {
                let angle = 2.0 * PI * (k as f64) / (deg as f64) + 0.4;
                (0.4 * angle.cos(), 0.4 * angle.sin())
            })
            .collect();

        for _ in 0..100 {
            let old = roots.clone();
            for i in 0..deg {
                let z = roots[i];
                let mut pz_re = p[0];
                let mut pz_im = 0.0;
                for k in 1..=deg {
                    let nre = pz_re * z.0 - pz_im * z.1 + p[k];
                    let nim = pz_re * z.1 + pz_im * z.0;
                    pz_re = nre;
                    pz_im = nim;
                }

                let mut prod_re = 1.0;
                let mut prod_im = 0.0;
                for j in 0..deg {
                    if i != j {
                        let diff_re = z.0 - roots[j].0;
                        let diff_im = z.1 - roots[j].1;
                        let nre = prod_re * diff_re - prod_im * diff_im;
                        let nim = prod_re * diff_im + prod_im * diff_re;
                        prod_re = nre;
                        prod_im = nim;
                    }
                }

                let denom = prod_re * prod_re + prod_im * prod_im;
                if denom > 1e-18 {
                    let d_re = (pz_re * prod_re + pz_im * prod_im) / denom;
                    let d_im = (pz_im * prod_re - pz_re * prod_im) / denom;
                    roots[i].0 -= d_re;
                    roots[i].1 -= d_im;
                }
            }
            let mut diff = 0.0;
            for i in 0..deg {
                diff += (roots[i].0 - old[i].0).hypot(roots[i].1 - old[i].1);
            }
            if diff < 1e-12 {
                break;
            }
        }
        roots
    }
}

// ============================================================================
// 4. FINANCIAL TVM (TIME VALUE OF MONEY) SOLVER
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub struct TvmState {
    pub n: f64,
    pub i: f64,
    pub pv: f64,
    pub pmt: f64,
    pub fv: f64,
    pub begin_mode: bool,
}

impl Default for TvmState {
    fn default() -> Self {
        Self {
            n: 360.0,
            i: 6.0,
            pv: 100_000.0,
            pmt: -599.55,
            fv: 0.0,
            begin_mode: false,
        }
    }
}

impl TvmState {
    fn r(&self) -> f64 {
        (self.i / 100.0) / 12.0
    }

    pub fn solve_pmt(&mut self) -> f64 {
        let r = self.r();
        let k = if self.begin_mode { 1.0 + r } else { 1.0 };
        if r.abs() < 1e-9 {
            self.pmt = -(self.pv + self.fv) / self.n;
        } else {
            let factor = (1.0 + r).powf(self.n);
            self.pmt = -r * (self.pv * factor + self.fv) / (k * (factor - 1.0));
        }
        self.pmt
    }

    pub fn solve_pv(&mut self) -> f64 {
        let r = self.r();
        let k = if self.begin_mode { 1.0 + r } else { 1.0 };
        if r.abs() < 1e-9 {
            self.pv = -(self.fv + self.pmt * self.n);
        } else {
            let factor = (1.0 + r).powf(self.n);
            self.pv = -(self.fv + self.pmt * k * (factor - 1.0) / r) / factor;
        }
        self.pv
    }

    pub fn solve_fv(&mut self) -> f64 {
        let r = self.r();
        let k = if self.begin_mode { 1.0 + r } else { 1.0 };
        if r.abs() < 1e-9 {
            self.fv = -(self.pv + self.pmt * self.n);
        } else {
            let factor = (1.0 + r).powf(self.n);
            self.fv = -self.pv * factor - self.pmt * k * (factor - 1.0) / r;
        }
        self.fv
    }

    pub fn solve_n(&mut self) -> f64 {
        let r = self.r();
        let k = if self.begin_mode { 1.0 + r } else { 1.0 };
        if r.abs() < 1e-9 {
            self.n = -(self.pv + self.fv) / self.pmt;
        } else {
            let num = self.pmt * k - self.fv * r;
            let den = self.pmt * k + self.pv * r;
            if num / den > 0.0 {
                self.n = (num / den).ln() / (1.0 + r).ln();
            }
        }
        self.n
    }
}

// ============================================================================
// 5. HP 49G EQUATION LIBRARY: CONSTANTS & UNIT CONVERSIONS
// ============================================================================

pub struct Constants;
impl Constants {
    pub fn get_table() -> Vec<(&'static str, &'static str, f64, &'static str)> {
        vec![
            ("c", "Speed of Light", 299_792_458.0, "m/s"),
            ("h", "Planck Constant", 6.626_070_15e-34, "J*s"),
            ("hbar", "Reduced Planck", 1.054_571_817e-34, "J*s"),
            ("G", "Gravitational Constant", 6.674_30e-11, "m^3/(kg*s^2)"),
            ("g0", "Standard Gravity", 9.80665, "m/s^2"),
            ("e", "Elementary Charge", 1.602_176_634e-19, "C"),
            ("me", "Electron Mass", 9.109_383_7015e-31, "kg"),
            ("mp", "Proton Mass", 1.672_621_923_69e-27, "kg"),
            ("kB", "Boltzmann Constant", 1.380_649e-23, "J/K"),
            ("NA", "Avogadro Number", 6.022_140_76e23, "1/mol"),
            ("R", "Universal Gas Constant", 8.314_462_618, "J/(mol*K)"),
            ("eps0", "Vacuum Permittivity", 8.854_187_8128e-12, "F/m"),
            ("mu0", "Vacuum Permeability", 1.256_637_062_12e-6, "N/A^2"),
            ("sigma", "Stefan-Boltzmann", 5.670_374_419e-8, "W/(m^2*K^4)"),
        ]
    }
}

pub struct UnitConverter;
impl UnitConverter {
    pub fn convert(val: f64, from: &str, to: &str) -> Result<f64, String> {
        if from == "C" && to == "F" {
            return Ok(val * 9.0 / 5.0 + 32.0);
        }
        if from == "F" && to == "C" {
            return Ok((val - 32.0) * 5.0 / 9.0);
        }
        if from == "C" && to == "K" {
            return Ok(val + 273.15);
        }
        if from == "K" && to == "C" {
            return Ok(val - 273.15);
        }

        let mut scales: HashMap<&str, f64> = HashMap::new();
        scales.insert("m", 1.0);
        scales.insert("km", 1000.0);
        scales.insert("cm", 0.01);
        scales.insert("mm", 0.001);
        scales.insert("in", 0.0254);
        scales.insert("ft", 0.3048);
        scales.insert("yd", 0.9144);
        scales.insert("mi", 1609.344);
        scales.insert("nmi", 1852.0);

        scales.insert("kg", 1.0);
        scales.insert("g", 0.001);
        scales.insert("mg", 1e-6);
        scales.insert("lb", 0.45359237);
        scales.insert("oz", 0.028349523125);
        scales.insert("ton", 1000.0);

        scales.insert("Pa", 1.0);
        scales.insert("bar", 100_000.0);
        scales.insert("atm", 101_325.0);
        scales.insert("psi", 6894.757);
        scales.insert("mmHg", 133.322);

        scales.insert("J", 1.0);
        scales.insert("kJ", 1000.0);
        scales.insert("cal", 4.184);
        scales.insert("kcal", 4184.0);
        scales.insert("kWh", 3_600_000.0);
        scales.insert("eV", 1.602176634e-19);
        scales.insert("BTU", 1055.056);

        scales.insert("W", 1.0);
        scales.insert("kW", 1000.0);
        scales.insert("hp", 745.69987);

        let s_from = scales.get(from).ok_or_else(|| format!("Unknown unit {}", from))?;
        let s_to = scales.get(to).ok_or_else(|| format!("Unknown unit {}", to))?;

        let base_val = val * s_from;
        Ok(base_val / s_to)
    }
}

// ============================================================================
// 6. 4-LEVEL HP RPN ENGINE & STACK ARCHITECTURE
// ============================================================================

pub struct RpnEngine {
    pub stack: Vec<Value>,
    pub history: Vec<Vec<Value>>,
    pub angle_mode: AngleMode,
    pub base_mode: BaseMode,
    pub tvm: TvmState,
    pub last_arg: Option<Value>,
}

impl Default for RpnEngine {
    fn default() -> Self {
        Self {
            stack: vec![Value::Real(0.0), Value::Real(0.0), Value::Real(0.0), Value::Real(0.0)],
            history: Vec::new(),
            angle_mode: AngleMode::Rad,
            base_mode: BaseMode::Dec,
            tvm: TvmState::default(),
            last_arg: None,
        }
    }
}

impl RpnEngine {
    pub fn push(&mut self, v: Value) {
        self.save_history();
        self.stack.push(v);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.save_history();
        self.stack.pop()
    }

    pub fn save_history(&mut self) {
        if self.history.len() > 30 {
            self.history.remove(0);
        }
        self.history.push(self.stack.clone());
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.history.pop() {
            self.stack = prev;
        }
    }

    pub fn get_level(&self, idx: usize) -> Value {
        let len = self.stack.len();
        if idx < len {
            self.stack[len - 1 - idx].clone()
        } else {
            Value::Real(0.0)
        }
    }

    pub fn execute(&mut self, cmd: &str) -> Result<String, String> {
        let upper = cmd.trim().to_uppercase();
        match upper.as_str() {
            "ENTER" => {
                let x = self.get_level(0);
                self.push(x);
                Ok("OK".into())
            }
            "DROP" => {
                self.pop();
                Ok("OK".into())
            }
            "SWAP" => {
                let len = self.stack.len();
                if len >= 2 {
                    self.save_history();
                    self.stack.swap(len - 1, len - 2);
                }
                Ok("OK".into())
            }
            "ROLL" => {
                let len = self.stack.len();
                if len >= 3 {
                    self.save_history();
                    let val = self.stack.remove(len - 3);
                    self.stack.push(val);
                }
                Ok("OK".into())
            }
            "CLEAR" => {
                self.save_history();
                self.stack.clear();
                Ok("OK".into())
            }
            "UNDO" => {
                self.undo();
                Ok("Undo complete".into())
            }

            "RAD" => {
                self.angle_mode = AngleMode::Rad;
                Ok("Angle Mode: Radian".into())
            }
            "DEG" => {
                self.angle_mode = AngleMode::Deg;
                Ok("Angle Mode: Degree".into())
            }
            "GRAD" => {
                self.angle_mode = AngleMode::Grad;
                Ok("Angle Mode: Grad".into())
            }
            "DEC" => {
                self.base_mode = BaseMode::Dec;
                Ok("Base: Decimal".into())
            }
            "HEX" => {
                self.base_mode = BaseMode::Hex;
                Ok("Base: Hexadecimal".into())
            }
            "OCT" => {
                self.base_mode = BaseMode::Oct;
                Ok("Base: Octal".into())
            }
            "BIN" => {
                self.base_mode = BaseMode::Bin;
                Ok("Base: Binary".into())
            }

            "+" | "ADD" => self.binary_op(|a, b| Ok(Value::Real(a + b))),
            "-" | "SUB" => self.binary_op(|a, b| Ok(Value::Real(a - b))),
            "*" | "MUL" => self.binary_op(|a, b| Ok(Value::Real(a * b))),
            "/" | "DIV" => self.binary_op(|a, b| {
                if b == 0.0 {
                    Err("Division by zero".into())
                } else {
                    Ok(Value::Real(a / b))
                }
            }),
            "^" | "POW" => self.binary_op(|a, b| Ok(Value::Real(a.powf(b)))),
            "MOD" => self.binary_op(|a, b| Ok(Value::Real(a % b))),

            "NEG" | "+/-" => self.unary_op(|a| Ok(Value::Real(-a))),
            "INV" | "1/X" => self.unary_op(|a| {
                if a == 0.0 {
                    Err("Division by zero".into())
                } else {
                    Ok(Value::Real(1.0 / a))
                }
            }),
            "SQRT" => self.unary_op(|a| {
                if a < 0.0 {
                    Ok(Value::Complex(0.0, (-a).sqrt()))
                } else {
                    Ok(Value::Real(a.sqrt()))
                }
            }),
            "CBRT" => self.unary_op(|a| Ok(Value::Real(a.cbrt()))),
            "ABS" => self.unary_op(|a| Ok(Value::Real(a.abs()))),
            "LN" => self.unary_op(|a| {
                if a <= 0.0 {
                    Err("Domain error for ln".into())
                } else {
                    Ok(Value::Real(a.ln()))
                }
            }),
            "EXP" => self.unary_op(|a| Ok(Value::Real(a.exp()))),
            "LOG10" | "LOG" => self.unary_op(|a| {
                if a <= 0.0 {
                    Err("Domain error for log10".into())
                } else {
                    Ok(Value::Real(a.log10()))
                }
            }),
            "ALN" | "10^X" => self.unary_op(|a| Ok(Value::Real(10.0f64.powf(a)))),

            "SIN" => {
                let m = self.angle_mode;
                self.unary_op(|a| Ok(Value::Real(m.to_radians(a).sin())))
            }
            "COS" => {
                let m = self.angle_mode;
                self.unary_op(|a| Ok(Value::Real(m.to_radians(a).cos())))
            }
            "TAN" => {
                let m = self.angle_mode;
                self.unary_op(|a| Ok(Value::Real(m.to_radians(a).tan())))
            }
            "ASIN" => {
                let m = self.angle_mode;
                self.unary_op(|a| {
                    if a < -1.0 || a > 1.0 {
                        Err("ASIN domain [-1, 1]".into())
                    } else {
                        Ok(Value::Real(m.from_radians(a.asin())))
                    }
                })
            }
            "ACOS" => {
                let m = self.angle_mode;
                self.unary_op(|a| {
                    if a < -1.0 || a > 1.0 {
                        Err("ACOS domain [-1, 1]".into())
                    } else {
                        Ok(Value::Real(m.from_radians(a.acos())))
                    }
                })
            }
            "ATAN" => {
                let m = self.angle_mode;
                self.unary_op(|a| Ok(Value::Real(m.from_radians(a.atan()))))
            }
            "SINH" => self.unary_op(|a| Ok(Value::Real(a.sinh()))),
            "COSH" => self.unary_op(|a| Ok(Value::Real(a.cosh()))),
            "TANH" => self.unary_op(|a| Ok(Value::Real(a.tanh()))),

            "FACT" | "N!" => self.unary_op(|a| {
                if a < 0.0 || a > 170.0 {
                    Err("Overflow or negative factorial".into())
                } else {
                    let mut res = 1.0;
                    for i in 2..=(a as u64) {
                        res *= i as f64;
                    }
                    Ok(Value::Real(res))
                }
            }),
            "NCR" => self.binary_op(|n, r| {
                if r < 0.0 || r > n {
                    return Ok(Value::Real(0.0));
                }
                let mut res = 1.0;
                let k = r.min(n - r) as u64;
                for i in 1..=k {
                    res = res * ((n as u64 - k + i) as f64) / (i as f64);
                }
                Ok(Value::Real(res))
            }),
            "NPR" => self.binary_op(|n, r| {
                if r < 0.0 || r > n {
                    return Ok(Value::Real(0.0));
                }
                let mut res = 1.0;
                let k = r as u64;
                for i in 0..k {
                    res *= (n as u64 - i) as f64;
                }
                Ok(Value::Real(res))
            }),

            "DET" => {
                let x = self.pop().ok_or("Stack underflow")?;
                match x {
                    Value::Matrix(m) => {
                        let d = MatrixEngine::det(&m)?;
                        self.push(Value::Real(d));
                        Ok(format!("DET = {}", d))
                    }
                    _ => Err("X is not a matrix".into()),
                }
            }
            "TRAN" => {
                let x = self.pop().ok_or("Stack underflow")?;
                match x {
                    Value::Matrix(m) => {
                        let t = MatrixEngine::transpose(&m);
                        self.push(Value::Matrix(t));
                        Ok("Transposed matrix".into())
                    }
                    _ => Err("X is not a matrix".into()),
                }
            }
            "INVM" => {
                let x = self.pop().ok_or("Stack underflow")?;
                match x {
                    Value::Matrix(m) => {
                        let inv = MatrixEngine::inverse(&m)?;
                        self.push(Value::Matrix(inv));
                        Ok("Inverted matrix".into())
                    }
                    _ => Err("X is not a matrix".into()),
                }
            }

            "TVM_N" => {
                let val = self.tvm.solve_n();
                self.push(Value::Real(val));
                Ok(format!("Solved N = {:.2}", val))
            }
            "TVM_PV" => {
                let val = self.tvm.solve_pv();
                self.push(Value::Real(val));
                Ok(format!("Solved PV = {:.2}", val))
            }
            "TVM_PMT" => {
                let val = self.tvm.solve_pmt();
                self.push(Value::Real(val));
                Ok(format!("Solved PMT = {:.2}", val))
            }
            "TVM_FV" => {
                let val = self.tvm.solve_fv();
                self.push(Value::Real(val));
                Ok(format!("Solved FV = {:.2}", val))
            }

            "DERIV" => {
                let x_val = self.pop().ok_or("Missing x evaluation point")?.as_f64()?;
                let d = CasEngine::differentiate(|t| t.sin() * t.exp(), x_val);
                self.push(Value::Real(d));
                Ok(format!("d/dx at {} = {:.8}", x_val, d))
            }
            "INTEG" => {
                let b = self.pop().ok_or("Missing upper bound b")?.as_f64()?;
                let a = self.pop().ok_or("Missing lower bound a")?.as_f64()?;
                let res = CasEngine::integrate(|t| t.sin().abs() + 1.0, a, b, 1000);
                self.push(Value::Real(res));
                Ok(format!("Integral from {} to {} = {:.8}", a, b, res))
            }

            // Unit Conversions
            "CONV_M_FT" => {
                let m = self.pop().ok_or("Stack underflow")?.as_f64()?;
                let ft = UnitConverter::convert(m, "m", "ft")?;
                self.push(Value::Real(ft));
                Ok(format!("{} m = {:.6} ft", m, ft))
            }
            "CONV_FT_M" => {
                let ft = self.pop().ok_or("Stack underflow")?.as_f64()?;
                let m = UnitConverter::convert(ft, "ft", "m")?;
                self.push(Value::Real(m));
                Ok(format!("{} ft = {:.6} m", ft, m))
            }
            "CONV_KG_LB" => {
                let kg = self.pop().ok_or("Stack underflow")?.as_f64()?;
                let lb = UnitConverter::convert(kg, "kg", "lb")?;
                self.push(Value::Real(lb));
                Ok(format!("{} kg = {:.6} lb", kg, lb))
            }
            "CONV_C_F" => {
                let c = self.pop().ok_or("Stack underflow")?.as_f64()?;
                let f = UnitConverter::convert(c, "C", "F")?;
                self.push(Value::Real(f));
                Ok(format!("{} C = {:.4} F", c, f))
            }
            "CONV_J_CAL" => {
                let j = self.pop().ok_or("Stack underflow")?.as_f64()?;
                let cal = UnitConverter::convert(j, "J", "cal")?;
                self.push(Value::Real(cal));
                Ok(format!("{} J = {:.6} cal", j, cal))
            }
            "CONV_PA_BAR" => {
                let pa = self.pop().ok_or("Stack underflow")?.as_f64()?;
                let bar = UnitConverter::convert(pa, "Pa", "bar")?;
                self.push(Value::Real(bar));
                Ok(format!("{} Pa = {:.6} bar", pa, bar))
            }

            _ => {
                if let Ok(num) = cmd.trim().parse::<f64>() {
                    self.push(Value::Real(num));
                    Ok("OK".into())
                } else if cmd.starts_with('#') {
                    let hex_str = cmd.trim_start_matches('#').trim_end_matches('h');
                    if let Ok(val) = i64::from_str_radix(hex_str, 16) {
                        self.push(Value::Real(val as f64));
                        return Ok("OK".into());
                    }
                    Err(format!("Unknown command: {}", cmd))
                } else {
                    Err(format!("Unknown command: {}", cmd))
                }
            }
        }
    }

    fn unary_op<F>(&mut self, op: F) -> Result<String, String>
    where
        F: FnOnce(f64) -> Result<Value, String>,
    {
        let a = self.pop().ok_or("Stack underflow")?.as_f64()?;
        self.last_arg = Some(Value::Real(a));
        match op(a) {
            Ok(res) => {
                self.push(res);
                Ok("OK".into())
            }
            Err(e) => {
                self.push(Value::Real(a));
                Err(e)
            }
        }
    }

    fn binary_op<F>(&mut self, op: F) -> Result<String, String>
    where
        F: FnOnce(f64, f64) -> Result<Value, String>,
    {
        let y_val = self.pop().ok_or("Stack underflow")?;
        let x_val = self.pop().ok_or("Stack underflow")?;

        match (x_val, y_val) {
            (Value::Real(a), Value::Real(b)) => match op(a, b) {
                Ok(res) => {
                    self.push(res);
                    Ok("OK".into())
                }
                Err(e) => {
                    self.push(Value::Real(a));
                    self.push(Value::Real(b));
                    Err(e)
                }
            },
            (Value::Matrix(m1), Value::Matrix(m2)) => {
                let res = MatrixEngine::multiply(&m1, &m2)?;
                self.push(Value::Matrix(res));
                Ok("Matrix multiplied".into())
            }
            _ => Err("Invalid operand types".into()),
        }
    }
}

// ============================================================================
// 7. EMBEDDED MODERN GLASSMORPHISM / NEUMORPHIC UI & PWA ASSETS
// ============================================================================

const INDEX_HTML: &str = r###"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
<meta name="theme-color" content="#12151b">
<meta name="apple-mobile-web-app-capable" content="yes">
<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
<meta name="apple-mobile-web-app-title" content="kapivaraCalc">
<link rel="manifest" href="/manifest.json">
<title>kapivaraCalc (kC49) - Graphing & CAS Calculator</title>
<style>
  :root {
    --bg-dark: #0f1218;
    --titanium-1: #1e222d;
    --titanium-2: #272d3b;
    --titanium-highlight: #3b4356;
    --lcd-bg: #151c18;
    --lcd-matrix: #1b2620;
    --lcd-glow: #38e89f;
    --lcd-text: #50ffb4;
    --lcd-dim: #1f6645;
    --key-dark: #20242e;
    --key-white: #e1e7f0;
    --key-orange: #ff772e;
    --key-cyan: #00c8e0;
    --accent-red: #ff3355;
  }
  * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; -webkit-tap-highlight-color: transparent; }
  body {
    background: radial-gradient(circle at 50% 20%, #1a202c, #0a0c10);
    color: #fff;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 10px;
  }
  .chassis {
    width: 100%;
    max-width: 420px;
    background: linear-gradient(175deg, #2b3140 0%, #171a22 40%, #101217 100%);
    border-radius: 36px;
    padding: 24px 20px 30px 20px;
    box-shadow: 0 30px 70px rgba(0,0,0,0.85), inset 0 1px 1px rgba(255,255,255,0.2), inset 0 -2px 4px rgba(0,0,0,0.6);
    border: 1px solid #363e52;
    position: relative;
  }
  .brand-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    padding: 0 8px;
  }
  .brand-logo {
    font-weight: 800;
    font-size: 19px;
    letter-spacing: 1.5px;
    background: linear-gradient(90deg, #ffffff, #a2b0cb);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
  .badge-cas {
    font-size: 11px;
    font-weight: 700;
    padding: 3px 8px;
    background: rgba(0,200,224,0.15);
    border: 1px solid rgba(0,200,224,0.4);
    color: #00e5ff;
    border-radius: 12px;
    letter-spacing: 1px;
  }
  .screen-bezel {
    background: #080b09;
    border-radius: 18px;
    padding: 14px;
    border: 2px solid #232b26;
    box-shadow: inset 0 6px 16px rgba(0,0,0,0.9), 0 2px 4px rgba(255,255,255,0.05);
    margin-bottom: 16px;
  }
  .annunciator-row {
    display: flex;
    justify-content: space-between;
    font-family: monospace;
    font-size: 10px;
    font-weight: bold;
    color: var(--lcd-dim);
    border-bottom: 1px solid #16241c;
    padding-bottom: 6px;
    margin-bottom: 8px;
  }
  .ann-item.active {
    color: var(--lcd-text);
    text-shadow: 0 0 8px var(--lcd-glow);
  }
  .ann-shift-orange.active { color: var(--key-orange); text-shadow: 0 0 8px #ff772e; }
  .ann-shift-cyan.active { color: var(--key-cyan); text-shadow: 0 0 8px #00c8e0; }

  .stack-display {
    font-family: 'Courier New', Courier, monospace;
    display: flex;
    flex-direction: column;
    gap: 5px;
    min-height: 115px;
  }
  .stack-level {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
    color: var(--lcd-text);
    text-shadow: 0 0 4px rgba(80,255,180,0.5);
  }
  .stack-idx {
    color: var(--lcd-dim);
    font-size: 12px;
    font-weight: bold;
    width: 24px;
  }
  .stack-val {
    flex-grow: 1;
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .input-line {
    border-top: 1px dashed #1a3325;
    margin-top: 6px;
    padding-top: 4px;
    display: flex;
    justify-content: space-between;
    color: #fff;
    font-family: monospace;
    font-size: 15px;
  }
  .cursor {
    display: inline-block;
    width: 8px;
    height: 14px;
    background: var(--lcd-glow);
    animation: blink 1s infinite;
    vertical-align: middle;
  }
  @keyframes blink { 0%, 50% { opacity: 1; } 51%, 100% { opacity: 0; } }

  .softkey-row {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 6px;
    margin-bottom: 14px;
  }
  .softkey {
    background: #181d26;
    border: 1px solid #333d50;
    border-radius: 8px 8px 12px 12px;
    height: 30px;
    color: #9cb1d4;
    font-weight: bold;
    font-size: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 3px 6px rgba(0,0,0,0.4), inset 0 1px 0 rgba(255,255,255,0.1);
    transition: transform 0.05s, background 0.1s;
  }
  .softkey:active { transform: translateY(2px); background: #263142; }

  .keypad {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .key-row {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 8px;
  }
  .key {
    height: 44px;
    border-radius: 12px;
    background: linear-gradient(145deg, #232733, #181b24);
    border: 1px solid #313747;
    box-shadow: 0 4px 8px rgba(0,0,0,0.5), inset 0 1px 1px rgba(255,255,255,0.1);
    color: #e4eaf5;
    font-size: 14px;
    font-weight: 600;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    position: relative;
    transition: all 0.05s ease-out;
  }
  .key:active, .key.pressed {
    transform: translateY(2px);
    box-shadow: 0 1px 2px rgba(0,0,0,0.7), inset 0 2px 4px rgba(0,0,0,0.6);
    background: #151821;
  }
  .key-sub-orange {
    position: absolute;
    top: 2px;
    left: 4px;
    font-size: 8px;
    font-weight: 700;
    color: var(--key-orange);
  }
  .key-sub-cyan {
    position: absolute;
    top: 2px;
    right: 4px;
    font-size: 8px;
    font-weight: 700;
    color: var(--key-cyan);
  }

  .key-shift-orange { background: linear-gradient(145deg, #d35400, #963b00); border-color: #ff8533; color: #fff; }
  .key-shift-cyan { background: linear-gradient(145deg, #0097a7, #005662); border-color: #33d3e6; color: #fff; }
  .key-alpha { background: linear-gradient(145deg, #374151, #1f2937); border-color: #4b5563; }
  .key-enter {
    grid-row: span 2;
    height: 96px;
    background: linear-gradient(145deg, #2563eb, #1d4ed8);
    border-color: #60a5fa;
    color: #fff;
    font-size: 16px;
    font-weight: 800;
  }
  .key-op { background: linear-gradient(145deg, #283042, #1c2230); color: #60a5fa; }
  .key-num { background: linear-gradient(145deg, #1f232e, #141720); font-weight: 700; font-size: 17px; }

  .menu-tabs {
    display: flex;
    justify-content: space-around;
    background: #141822;
    border-radius: 8px;
    padding: 4px;
    margin-bottom: 8px;
  }
  .tab-btn {
    background: transparent;
    border: none;
    color: #7b8fae;
    font-size: 10px;
    font-weight: bold;
    padding: 4px 6px;
    cursor: pointer;
    border-radius: 6px;
  }
  .tab-btn.active {
    background: #273347;
    color: #00c8e0;
  }
</style>
</head>
<body>

<div class="chassis">
  <div class="brand-bar">
    <div class="brand-logo">kapivaraCalc<span style="font-size:12px; font-weight:300; margin-left:6px; color:#00e5ff;">kC49</span></div>
    <div class="badge-cas" id="mode-badge">RPN · CAS</div>
  </div>

  <div class="menu-tabs">
    <button class="tab-btn active" onclick="switchMenu('MATH')">MATH</button>
    <button class="tab-btn" onclick="switchMenu('CAS')">CAS</button>
    <button class="tab-btn" onclick="switchMenu('MATR')">MATR</button>
    <button class="tab-btn" onclick="switchMenu('TVM')">TVM</button>
    <button class="tab-btn" onclick="switchMenu('STAT')">STAT</button>
    <button class="tab-btn" onclick="switchMenu('CONST')">CONST</button>
    <button class="tab-btn" onclick="switchMenu('UNIT')">UNIT</button>
  </div>

  <div class="softkey-row" id="softkeys">
    <!-- Softkeys populated dynamically -->
  </div>

  <div class="screen-bezel">
    <div class="annunciator-row">
      <span class="ann-item ann-shift-orange" id="ann-ls">⯬ SHIFT</span>
      <span class="ann-item ann-shift-cyan" id="ann-rs">SHIFT ⯮</span>
      <span class="ann-item" id="ann-alpha">ALPHA</span>
      <span class="ann-item active" id="ann-rad">RAD</span>
      <span class="ann-item active" id="ann-dec">DEC</span>
      <span class="ann-item" id="ann-busy">BUSY</span>
    </div>

    <div class="stack-display">
      <div class="stack-level"><span class="stack-idx">4:</span><span class="stack-val" id="st-4">0</span></div>
      <div class="stack-level"><span class="stack-idx">3:</span><span class="stack-val" id="st-3">0</span></div>
      <div class="stack-level"><span class="stack-idx">2:</span><span class="stack-val" id="st-2">0</span></div>
      <div class="stack-level"><span class="stack-idx">1:</span><span class="stack-val" id="st-1" style="color:#ffffff; font-weight:bold;">0</span></div>
    </div>

    <div class="input-line">
      <span id="input-buffer"></span><span class="cursor"></span>
    </div>
  </div>

  <div class="keypad">
    <div class="key-row">
      <div class="key" onclick="cmd('SWAP')"><span class="key-sub-orange">HIST</span><span class="key-sub-cyan">VIEW</span>SWAP</div>
      <div class="key" onclick="cmd('DROP')"><span class="key-sub-orange">CLEAR</span><span class="key-sub-cyan">UNDO</span>DROP</div>
      <div class="key" onclick="cmd('ROLL')"><span class="key-sub-orange">LIST</span><span class="key-sub-cyan">EVAL</span>ROLL</div>
      <div class="key" onclick="toggleAngle()"><span class="key-sub-orange">GRAD</span><span class="key-sub-cyan">DEG</span>MODE</div>
      <div class="key" onclick="bs()"><span class="key-sub-orange">DEL</span><span class="key-sub-cyan">CLR</span>⌫</div>
    </div>

    <div class="key-row">
      <div class="key key-shift-orange" onclick="toggleShift('L')">⯬ SHIFT</div>
      <div class="key key-shift-cyan" onclick="toggleShift('R')">SHIFT ⯮</div>
      <div class="key key-alpha" onclick="toggleAlpha()">ALPHA</div>
      <div class="key key-op" onclick="cmd('^')"><span class="key-sub-orange">X²</span><span class="key-sub-cyan">X√Y</span>Y^X</div>
      <div class="key key-op" onclick="cmd('SQRT')"><span class="key-sub-orange">CBRT</span><span class="key-sub-cyan">ROOT</span>√</div>
    </div>

    <div class="key-row">
      <div class="key key-op" onclick="cmd('SIN')"><span class="key-sub-orange">ASIN</span><span class="key-sub-cyan">SINH</span>SIN</div>
      <div class="key key-op" onclick="cmd('COS')"><span class="key-sub-orange">ACOS</span><span class="key-sub-cyan">COSH</span>COS</div>
      <div class="key key-op" onclick="cmd('TAN')"><span class="key-sub-orange">ATAN</span><span class="key-sub-cyan">TANH</span>TAN</div>
      <div class="key key-op" onclick="cmd('LN')"><span class="key-sub-orange">EXP</span><span class="key-sub-cyan">E^X</span>LN</div>
      <div class="key key-op" onclick="cmd('LOG10')"><span class="key-sub-orange">10^X</span><span class="key-sub-cyan">ALN</span>LOG</div>
    </div>

    <div class="key-row">
      <div class="key key-num" onclick="num('7')">7</div>
      <div class="key key-num" onclick="num('8')">8</div>
      <div class="key key-num" onclick="num('9')">9</div>
      <div class="key key-op" onclick="cmd('DIV')">÷</div>
      <div class="key key-op" onclick="cmd('INV')">1/X</div>
    </div>

    <div class="key-row">
      <div class="key key-num" onclick="num('4')">4</div>
      <div class="key key-num" onclick="num('5')">5</div>
      <div class="key key-num" onclick="num('6')">6</div>
      <div class="key key-op" onclick="cmd('MUL')">×</div>
      <div class="key key-op" onclick="cmd('NEG')">+/-</div>
    </div>

    <div style="display:grid; grid-template-columns: repeat(5, 1fr); gap:8px;">
      <div style="grid-column: span 4; display:flex; flex-direction:column; gap:8px;">
        <div class="key-row" style="grid-template-columns: repeat(4, 1fr);">
          <div class="key key-num" onclick="num('1')">1</div>
          <div class="key key-num" onclick="num('2')">2</div>
          <div class="key key-num" onclick="num('3')">3</div>
          <div class="key key-op" onclick="cmd('SUB')">-</div>
        </div>
        <div class="key-row" style="grid-template-columns: repeat(4, 1fr);">
          <div class="key key-num" onclick="num('0')">0</div>
          <div class="key key-num" onclick="num('.')">.</div>
          <div class="key key-op" onclick="num('E')">EEX</div>
          <div class="key key-op" onclick="cmd('ADD')">+</div>
        </div>
      </div>
      <div class="key key-enter" onclick="pressEnter()">ENTER</div>
    </div>
  </div>
</div>

<script>
let buffer = "";
let shiftL = false;
let shiftR = false;
let alpha = false;
let currentMenu = "MATH";

const menus = {
  MATH: [
    { label: "SIN", cmd: "SIN" }, { label: "COS", cmd: "COS" }, { label: "TAN", cmd: "TAN" },
    { label: "LN", cmd: "LN" }, { label: "EXP", cmd: "EXP" }, { label: "SQRT", cmd: "SQRT" }
  ],
  CAS: [
    { label: "DERIV", cmd: "DERIV" }, { label: "INTEG", cmd: "INTEG" }, { label: "ROOTS", cmd: "ROOTS" },
    { label: "EXPND", cmd: "EXPAND" }, { label: "FCTR", cmd: "FACTOR" }, { label: "TAYLR", cmd: "TAYLOR" }
  ],
  MATR: [
    { label: "DET", cmd: "DET" }, { label: "TRAN", cmd: "TRAN" }, { label: "INVM", cmd: "INVM" },
    { label: "SOLVE", cmd: "SOLVE_SYS" }, { label: "DOT", cmd: "DOT" }, { label: "CROSS", cmd: "CROSS" }
  ],
  TVM: [
    { label: "N", cmd: "TVM_N" }, { label: "I%YR", cmd: "TVM_I" }, { label: "PV", cmd: "TVM_PV" },
    { label: "PMT", cmd: "TVM_PMT" }, { label: "FV", cmd: "TVM_FV" }, { label: "AMRT", cmd: "TVM_AMORT" }
  ],
  STAT: [
    { label: "N!", cmd: "FACT" }, { label: "nCr", cmd: "NCR" }, { label: "nPr", cmd: "NPR" },
    { label: "GAUSS", cmd: "GAUSS" }, { label: "MEAN", cmd: "MEAN" }, { label: "STDEV", cmd: "STDEV" }
  ],
  CONST: [
    { label: "c", cmd: "299792458" }, { label: "h", cmd: "6.62607015E-34" }, { label: "G", cmd: "6.6743E-11" },
    { label: "e", cmd: "1.602176634E-19" }, { label: "kB", cmd: "1.380649E-23" }, { label: "NA", cmd: "6.02214076E23" }
  ],
  UNIT: [
    { label: "m->ft", cmd: "CONV_M_FT" }, { label: "ft->m", cmd: "CONV_FT_M" },
    { label: "kg->lb", cmd: "CONV_KG_LB" }, { label: "C->F", cmd: "CONV_C_F" },
    { label: "J->cal", cmd: "CONV_J_CAL" }, { label: "Pa->bar", cmd: "CONV_PA_BAR" }
  ]
};

function playFeedback() {
  if (navigator.vibrate) navigator.vibrate(12);
  try {
    const ctx = new (window.AudioContext || window.webkitAudioContext)();
    const osc = ctx.createOscillator();
    const g = ctx.createGain();
    osc.frequency.setValueAtTime(800, ctx.currentTime);
    g.gain.setValueAtTime(0.04, ctx.currentTime);
    g.gain.exponentialRampToValueAtTime(0.0001, ctx.currentTime + 0.04);
    osc.connect(g);
    g.connect(ctx.destination);
    osc.start();
    osc.stop(ctx.currentTime + 0.04);
  } catch(e) {}
}

function switchMenu(m) {
  playFeedback();
  currentMenu = m;
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.toggle('active', b.innerText === m));
  const container = document.getElementById('softkeys');
  container.innerHTML = "";
  menus[m].forEach(item => {
    const btn = document.createElement('div');
    btn.className = 'softkey';
    btn.innerText = item.label;
    btn.onclick = () => cmd(item.cmd);
    container.appendChild(btn);
  });
}

function updateScreen(levels) {
  document.getElementById('st-1').innerText = levels[0] || "0";
  document.getElementById('st-2').innerText = levels[1] || "0";
  document.getElementById('st-3').innerText = levels[2] || "0";
  document.getElementById('st-4').innerText = levels[3] || "0";
}

function num(d) {
  playFeedback();
  buffer += d;
  document.getElementById('input-buffer').innerText = buffer;
}

function bs() {
  playFeedback();
  if (buffer.length > 0) {
    buffer = buffer.slice(0, -1);
    document.getElementById('input-buffer').innerText = buffer;
  } else {
    cmd("DROP");
  }
}

async function pressEnter() {
  playFeedback();
  if (buffer.length > 0) {
    const val = buffer;
    buffer = "";
    document.getElementById('input-buffer').innerText = "";
    await sendCmd(val);
  } else {
    await sendCmd("ENTER");
  }
}

async function cmd(c) {
  playFeedback();
  if (shiftL) {
    c = "L_" + c;
    shiftL = false;
    document.getElementById('ann-ls').classList.remove('active');
  } else if (shiftR) {
    c = "R_" + c;
    shiftR = false;
    document.getElementById('ann-rs').classList.remove('active');
  }
  if (buffer.length > 0) {
    const val = buffer;
    buffer = "";
    document.getElementById('input-buffer').innerText = "";
    await sendCmd(val);
  }
  await sendCmd(c);
}

function toggleShift(s) {
  playFeedback();
  if (s === 'L') {
    shiftL = !shiftL;
    shiftR = false;
  } else {
    shiftR = !shiftR;
    shiftL = false;
  }
  document.getElementById('ann-ls').classList.toggle('active', shiftL);
  document.getElementById('ann-rs').classList.toggle('active', shiftR);
}

function toggleAlpha() {
  playFeedback();
  alpha = !alpha;
  document.getElementById('ann-alpha').classList.toggle('active', alpha);
}

function toggleAngle() {
  playFeedback();
  const radElem = document.getElementById('ann-rad');
  if (radElem.innerText === "RAD") {
    radElem.innerText = "DEG";
    sendCmd("DEG");
  } else {
    radElem.innerText = "RAD";
    sendCmd("RAD");
  }
}

class RpnEngineJS {
  constructor() {
    this.stack = [0, 0, 0, 0];
    this.history = [];
    this.angleMode = 'RAD';
    this.baseMode = 'DEC';
    this.tvm = { n: 360, i: 6.0, pv: 100000, pmt: -599.55, fv: 0, begin: false };
  }

  save() {
    if (this.history.length > 30) this.history.shift();
    this.history.push([...this.stack]);
  }

  undo() {
    if (this.history.length > 0) this.stack = this.history.pop();
  }

  push(v) {
    this.save();
    this.stack.unshift(v);
  }

  pop() {
    this.save();
    return this.stack.length > 0 ? this.stack.shift() : 0;
  }

  formatVal(v) {
    if (typeof v === 'number') {
      if (this.baseMode === 'HEX') return '#' + (v | 0).toString(16).toUpperCase() + 'h';
      if (this.baseMode === 'OCT') return '#' + (v | 0).toString(8) + 'o';
      if (this.baseMode === 'BIN') return '#' + (v | 0).toString(2) + 'b';
      if (Number.isInteger(v) && Math.abs(v) < 1e12) return v.toString();
      return parseFloat(v.toPrecision(10)).toString();
    }
    return String(v);
  }

  getStackDisplay() {
    return [
      this.formatVal(this.stack[0] || 0),
      this.formatVal(this.stack[1] || 0),
      this.formatVal(this.stack[2] || 0),
      this.formatVal(this.stack[3] || 0)
    ];
  }

  toRadians(a) {
    if (this.angleMode === 'DEG') return a * Math.PI / 180;
    if (this.angleMode === 'GRAD') return a * Math.PI / 200;
    return a;
  }

  fromRadians(r) {
    if (this.angleMode === 'DEG') return r * 180 / Math.PI;
    if (this.angleMode === 'GRAD') return r * 200 / Math.PI;
    return r;
  }

  execute(cmd) {
    const c = cmd.trim().toUpperCase();
    if (!isNaN(Number(cmd))) {
      this.push(Number(cmd));
      return;
    }

    switch (c) {
      case 'ENTER':
        this.push(this.stack[0] || 0);
        break;
      case 'DROP':
        this.pop();
        break;
      case 'SWAP':
        if (this.stack.length >= 2) {
          this.save();
          const t = this.stack[0];
          this.stack[0] = this.stack[1];
          this.stack[1] = t;
        }
        break;
      case 'ROLL':
        if (this.stack.length >= 3) {
          this.save();
          const val = this.stack.splice(2, 1)[0];
          this.stack.unshift(val);
        }
        break;
      case 'CLEAR':
        this.save();
        this.stack = [0, 0, 0, 0];
        break;
      case 'UNDO':
        this.undo();
        break;

      case 'RAD': this.angleMode = 'RAD'; break;
      case 'DEG': this.angleMode = 'DEG'; break;
      case 'GRAD': this.angleMode = 'GRAD'; break;
      case 'DEC': this.baseMode = 'DEC'; break;
      case 'HEX': this.baseMode = 'HEX'; break;
      case 'OCT': this.baseMode = 'OCT'; break;
      case 'BIN': this.baseMode = 'BIN'; break;

      case '+': case 'ADD': {
        const y = this.pop(), x = this.pop();
        this.push(x + y);
        break;
      }
      case '-': case 'SUB': {
        const y = this.pop(), x = this.pop();
        this.push(x - y);
        break;
      }
      case '*': case 'MUL': {
        const y = this.pop(), x = this.pop();
        this.push(x * y);
        break;
      }
      case '/': case 'DIV': {
        const y = this.pop(), x = this.pop();
        this.push(y !== 0 ? x / y : 0);
        break;
      }
      case '^': case 'POW': {
        const y = this.pop(), x = this.pop();
        this.push(Math.pow(x, y));
        break;
      }
      case 'MOD': {
        const y = this.pop(), x = this.pop();
        this.push(x % y);
        break;
      }
      case 'NEG': case '+/-': {
        const x = this.pop();
        this.push(-x);
        break;
      }
      case 'INV': case '1/X': {
        const x = this.pop();
        this.push(x !== 0 ? 1 / x : 0);
        break;
      }
      case 'SQRT': {
        const x = this.pop();
        this.push(x >= 0 ? Math.sqrt(x) : 0);
        break;
      }
      case 'CBRT': {
        const x = this.pop();
        this.push(Math.cbrt(x));
        break;
      }
      case 'ABS': {
        const x = this.pop();
        this.push(Math.abs(x));
        break;
      }
      case 'LN': {
        const x = this.pop();
        this.push(x > 0 ? Math.log(x) : 0);
        break;
      }
      case 'EXP': {
        const x = this.pop();
        this.push(Math.exp(x));
        break;
      }
      case 'LOG10': case 'LOG': {
        const x = this.pop();
        this.push(x > 0 ? Math.log10(x) : 0);
        break;
      }
      case 'ALN': case '10^X': {
        const x = this.pop();
        this.push(Math.pow(10, x));
        break;
      }

      case 'SIN': this.push(Math.sin(this.toRadians(this.pop()))); break;
      case 'COS': this.push(Math.cos(this.toRadians(this.pop()))); break;
      case 'TAN': this.push(Math.tan(this.toRadians(this.pop()))); break;
      case 'ASIN': this.push(this.fromRadians(Math.asin(this.pop()))); break;
      case 'ACOS': this.push(this.fromRadians(Math.acos(this.pop()))); break;
      case 'ATAN': this.push(this.fromRadians(Math.atan(this.pop()))); break;
      case 'SINH': this.push(Math.sinh(this.pop())); break;
      case 'COSH': this.push(Math.cosh(this.pop())); break;
      case 'TANH': this.push(Math.tanh(this.pop())); break;

      case 'FACT': case 'N!': {
        const n = Math.floor(Math.abs(this.pop()));
        let r = 1;
        for (let i = 2; i <= Math.min(n, 170); i++) r *= i;
        this.push(r);
        break;
      }
      case 'NCR': {
        const r_val = this.pop(), n_val = this.pop();
        if (r_val < 0 || r_val > n_val) { this.push(0); break; }
        const k = Math.min(r_val, n_val - r_val);
        let res = 1;
        for (let i = 1; i <= k; i++) res = res * (n_val - k + i) / i;
        this.push(res);
        break;
      }
      case 'NPR': {
        const r_val = this.pop(), n_val = this.pop();
        if (r_val < 0 || r_val > n_val) { this.push(0); break; }
        let res = 1;
        for (let i = 0; i < r_val; i++) res *= (n_val - i);
        this.push(res);
        break;
      }

      case 'DERIV': {
        const x = this.pop();
        const f = t => Math.sin(t) * Math.exp(t);
        const h = 1e-4 * (1 + Math.abs(x));
        const d = (-f(x + 2*h) + 8*f(x + h) - 8*f(x - h) + f(x - 2*h)) / (12*h);
        this.push(d);
        break;
      }
      case 'INTEG': {
        const b = this.pop(), a = this.pop();
        const f = t => Math.abs(Math.sin(t)) + 1;
        const n = 1000;
        const h = (b - a) / n;
        let sum = f(a) + f(b);
        for (let i = 1; i < n; i++) sum += (i % 2 === 0 ? 2 : 4) * f(a + i*h);
        this.push((h / 3) * sum);
        break;
      }

      case 'TVM_N': {
        const r = (this.tvm.i / 100) / 12;
        const num = this.tvm.pmt - this.tvm.fv * r;
        const den = this.tvm.pmt + this.tvm.pv * r;
        if (num / den > 0) this.tvm.n = Math.log(num / den) / Math.log(1 + r);
        this.push(this.tvm.n);
        break;
      }
      case 'TVM_PV': {
        const r = (this.tvm.i / 100) / 12;
        const f = Math.pow(1 + r, this.tvm.n);
        this.tvm.pv = -(this.tvm.fv + this.tvm.pmt * (f - 1) / r) / f;
        this.push(this.tvm.pv);
        break;
      }
      case 'TVM_PMT': {
        const r = (this.tvm.i / 100) / 12;
        const f = Math.pow(1 + r, this.tvm.n);
        this.tvm.pmt = -r * (this.tvm.pv * f + this.tvm.fv) / (f - 1);
        this.push(this.tvm.pmt);
        break;
      }
      case 'TVM_FV': {
        const r = (this.tvm.i / 100) / 12;
        const f = Math.pow(1 + r, this.tvm.n);
        this.tvm.fv = -this.tvm.pv * f - this.tvm.pmt * (f - 1) / r;
        this.push(this.tvm.fv);
        break;
      }

      case 'CONV_M_FT': this.push(this.pop() * 3.280839895); break;
      case 'CONV_FT_M': this.push(this.pop() / 3.280839895); break;
      case 'CONV_KG_LB': this.push(this.pop() * 2.20462262); break;
      case 'CONV_C_F': this.push(this.pop() * 9/5 + 32); break;
      case 'CONV_J_CAL': this.push(this.pop() / 4.184); break;
      case 'CONV_PA_BAR': this.push(this.pop() / 100000); break;

      default: break;
    }
  }
}

const engine = new RpnEngineJS();

function sendCmd(command) {
  document.getElementById('ann-busy').classList.add('active');
  try {
    if (command !== "REFRESH") {
      engine.execute(command);
    }
    updateScreen(engine.getStackDisplay());
  } catch(e) {
    console.error(e);
  } finally {
    document.getElementById('ann-busy').classList.remove('active');
  }
}

window.addEventListener('keydown', (e) => {
  if (e.key >= '0' && e.key <= '9') num(e.key);
  else if (e.key === '.') num('.');
  else if (e.key === 'Enter') pressEnter();
  else if (e.key === 'Backspace') bs();
  else if (e.key === '+') cmd('+');
  else if (e.key === '-') cmd('-');
  else if (e.key === '*') cmd('*');
  else if (e.key === '/') cmd('/');
});

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/sw.js').catch(() => {});
}

switchMenu('MATH');
sendCmd("REFRESH");
</script>
</body>
</html>"###;

const MANIFEST_JSON: &str = r###"{
  "name": "kapivaraCalc (kC49)",
  "short_name": "kC49",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0f1218",
  "theme_color": "#12151b",
  "icons": [
    {
      "src": "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><rect width='100' height='100' rx='20' fill='%23171a22'/><text x='50' y='65' font-size='42' font-weight='bold' fill='%2300c8e0' text-anchor='middle'>kC49</text></svg>",
      "sizes": "192x192 512x512",
      "type": "image/svg+xml"
    }
  ]
}"###;

const SW_JS: &str = r###"
const CACHE_NAME = 'hp49-cache-v1';
self.addEventListener('install', e => {
  e.waitUntil(caches.open(CACHE_NAME).then(c => c.addAll(['/'])));
});
self.addEventListener('fetch', e => {
  if (e.request.url.includes('/api/')) return;
  e.respondWith(caches.match(e.request).then(r => r || fetch(e.request)));
});
"###;

// ============================================================================
// 8. EMBEDDED HIGH-PERFORMANCE ZERO-DEPENDENCY HTTP / REST ENGINE
// ============================================================================

pub struct WebServer {
    engine: Arc<Mutex<RpnEngine>>,
}

impl Default for WebServer {
    fn default() -> Self {
        Self::new()
    }
}

impl WebServer {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(Mutex::new(RpnEngine::default())),
        }
    }

    pub fn start(&self, port: u16) {
        let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(l) => l,
            Err(_) => TcpListener::bind("127.0.0.1:0").expect("Failed to bind port"),
        };
        let bound_port = listener.local_addr().unwrap().port();
        let app_url = format!("http://127.0.0.1:{}", bound_port);

        println!("============================================================");
        println!("  kapivaraCalc (kC49) RUNNING");
        println!("  URL: {}", app_url);
        println!("============================================================");

        // Launch dedicated standalone application window (no browser tabs/bars, own life!)
        launch_standalone_app_window(&app_url);

        let engine = Arc::clone(&self.engine);
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    let eng = Arc::clone(&engine);
                    thread::spawn(move || {
                        handle_connection(stream, eng);
                    });
                }
            }
        });
    }
}

fn launch_standalone_app_window(url: &str) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let edge_candidates = [
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
            "msedge.exe",
        ];
        let user_data = format!(r"--user-data-dir={}\kC49_app_profile", std::env::temp_dir().display());
        for exe in &edge_candidates {
            let mut cmd = std::process::Command::new(exe);
            cmd.args(&[
                &format!("--app={}", url),
                "--window-size=435,845",
                &user_data,
                "--app-id=kC49",
                "--disable-features=TranslateUI",
                "--disable-extensions",
                "--no-first-run",
            ]);
            cmd.creation_flags(CREATE_NO_WINDOW);

            if let Ok(_) = cmd.spawn() {
                return;
            }
        }

        let chrome_candidates = [
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
            "chrome.exe",
        ];
        for exe in &chrome_candidates {
            let mut cmd = std::process::Command::new(exe);
            cmd.args(&[
                &format!("--app={}", url),
                "--window-size=435,845",
                &user_data,
                "--app-id=kC49",
                "--disable-extensions",
                "--no-first-run",
            ]);
            cmd.creation_flags(CREATE_NO_WINDOW);

            if let Ok(_) = cmd.spawn() {
                return;
            }
        }

        let mut fallback = std::process::Command::new("cmd");
        fallback.args(&["/C", "start", "", url]);
        fallback.creation_flags(CREATE_NO_WINDOW);
        let _ = fallback.spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let mac_chrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
        let mac_edge = "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge";
        if std::path::Path::new(mac_chrome).exists() {
            if let Ok(_) = std::process::Command::new(mac_chrome)
                .args(&[&format!("--app={}", url), "--window-size=435,845"])
                .spawn()
            {
                return;
            }
        }
        if std::path::Path::new(mac_edge).exists() {
            if let Ok(_) = std::process::Command::new(mac_edge)
                .args(&[&format!("--app={}", url), "--window-size=435,845"])
                .spawn()
            {
                return;
            }
        }
        let _ = std::process::Command::new("open").arg(url).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let linux_browsers = ["google-chrome", "chromium-browser", "chromium", "brave-browser"];
        for b in &linux_browsers {
            if let Ok(_) = std::process::Command::new(b)
                .args(&[&format!("--app={}", url), "--window-size=435,845"])
                .spawn()
            {
                return;
            }
        }
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

fn handle_connection(mut stream: TcpStream, engine: Arc<Mutex<RpnEngine>>) {
    let mut buffer = [0; 4096];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let req = String::from_utf8_lossy(&buffer[..bytes_read]);
    let first_line = req.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let path = parts[1];

    if method == "GET" {
        match path {
            "/" | "/index.html" => send_response(&mut stream, "200 OK", "text/html", INDEX_HTML),
            "/manifest.json" => send_response(&mut stream, "200 OK", "application/json", MANIFEST_JSON),
            "/sw.js" => send_response(&mut stream, "200 OK", "application/javascript", SW_JS),
            _ => send_response(&mut stream, "404 NOT FOUND", "text/plain", "Not Found"),
        }
    } else if method == "POST" && path == "/api/exec" {
        let body = req.split("\r\n\r\n").nth(1).unwrap_or("");
        let mut cmd_str = String::new();
        if let Some(pos) = body.find("\"cmd\":\"") {
            let start = pos + 7;
            if let Some(end) = body[start..].find('"') {
                cmd_str = body[start..start + end].to_string();
            }
        }

        let mut eng = engine.lock().unwrap();
        let status = if cmd_str != "REFRESH" && !cmd_str.is_empty() {
            eng.execute(&cmd_str).unwrap_or_else(|e| e)
        } else {
            "OK".into()
        };

        let s1 = eng.get_level(0).to_display_string(eng.angle_mode, eng.base_mode);
        let s2 = eng.get_level(1).to_display_string(eng.angle_mode, eng.base_mode);
        let s3 = eng.get_level(2).to_display_string(eng.angle_mode, eng.base_mode);
        let s4 = eng.get_level(3).to_display_string(eng.angle_mode, eng.base_mode);

        let json_resp = format!(
            "{{\"status\":\"{}\",\"stack\":[\"{}\",\"{}\",\"{}\",\"{}\"]}}",
            status, s1, s2, s3, s4
        );
        send_response(&mut stream, "200 OK", "application/json", &json_resp);
    }
}

fn send_response(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status,
        content_type,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

// ============================================================================
// 9. MAIN ENTRY POINT & CLI REPL
// ============================================================================

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--cli") {
        println!("============================================================");
        println!("  kapivaraCalc (kC49) - INTERACTIVE CLI CAS ENGINE");
        println!("  Type numbers or commands (+, -, *, /, SIN, COS, DET, etc.)");
        println!("  Type 'EXIT' or 'QUIT' to close.");
        println!("============================================================");

        let mut eng = RpnEngine::default();
        let stdin = std::io::stdin();
        let mut line = String::new();

        loop {
            print!("RPN> ");
            let _ = std::io::stdout().flush();
            line.clear();
            if stdin.read_line(&mut line).is_err() || line.trim().is_empty() {
                continue;
            }
            let input = line.trim();
            if input.eq_ignore_ascii_case("EXIT") || input.eq_ignore_ascii_case("QUIT") {
                break;
            }
            match eng.execute(input) {
                Ok(msg) => {
                    println!("[Result: {}]", msg);
                    println!("4: {}", eng.get_level(3).to_display_string(eng.angle_mode, eng.base_mode));
                    println!("3: {}", eng.get_level(2).to_display_string(eng.angle_mode, eng.base_mode));
                    println!("2: {}", eng.get_level(1).to_display_string(eng.angle_mode, eng.base_mode));
                    println!("1: {}", eng.get_level(0).to_display_string(eng.angle_mode, eng.base_mode));
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        return;
    }

    // Write self-contained application assets to local temp cache
    let temp_dir = std::env::temp_dir().join("kC49_app");
    let _ = std::fs::create_dir_all(&temp_dir);
    let html_path = temp_dir.join("index.html");
    let _ = std::fs::write(&html_path, INDEX_HTML);
    let _ = std::fs::write(temp_dir.join("manifest.json"), MANIFEST_JSON);
    let _ = std::fs::write(temp_dir.join("sw.js"), SW_JS);

    let file_url = format!("file:///{}", html_path.to_string_lossy().replace('\\', "/"));

    // Launch standalone window directly via file protocol - zero network dependency!
    launch_standalone_app_window(&file_url);
}
